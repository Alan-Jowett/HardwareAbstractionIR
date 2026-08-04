#![no_std]
#![no_main]

use core::{panic::PanicInfo, ptr::addr_of_mut};

use ch32v203g6u6_embassy_hal::{
    interrupt,
    rcc::{DRV_RCC_RUNTIME_RESOURCES, RCC},
    usb::{DRV_USBD_RUNTIME_RESOURCES, USBD, USBDUsbDriver},
    wch,
};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embassy_usb::{
    Builder, Config, UsbDevice,
    class::cdc_acm::{CdcAcmClass, State as CdcState},
};

const DTR_POLL_INTERVAL: Duration = Duration::from_millis(100);
const ARMED_MESSAGE: &[u8] =
    b"Reset smoke armed: close this COM port to reset the MCU.\r\n";

static mut CONFIG_DESCRIPTOR: [u8; 256] = [0; 256];
static mut BOS_DESCRIPTOR: [u8; 256] = [0; 256];
static mut MSOS_DESCRIPTOR: [u8; 256] = [0; 256];
static mut CONTROL_BUFFER: [u8; 128] = [0; 128];
static mut CDC_STATE: CdcState<'static> = CdcState::new();

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[embassy_executor::task]
async fn usb_task(mut device: UsbDevice<'static, USBDUsbDriver>) -> ! {
    device.run().await
}

#[embassy_executor::main(entry = "riscv_rt::entry")]
async fn main(spawner: Spawner) -> ! {
    let rcc = RCC::new(DRV_RCC_RUNTIME_RESOURCES).unwrap();
    rcc.configure_usb_fsdev_clock_48mhz().unwrap();
    wch::init_embassy_time_runtime().unwrap();

    let usbd = USBD::new(DRV_USBD_RUNTIME_RESOURCES).unwrap();
    let driver = usbd.embassy_usb_driver();

    let mut config = Config::new(0xCAFE, 0x4009);
    config.manufacturer = Some("TinyUSB");
    config.product = Some("CH32V203 reset smoke");
    config.serial_number = Some("0001");
    config.max_power = 100;

    let config_descriptor = unsafe { &mut *addr_of_mut!(CONFIG_DESCRIPTOR) };
    let bos_descriptor = unsafe { &mut *addr_of_mut!(BOS_DESCRIPTOR) };
    let msos_descriptor = unsafe { &mut *addr_of_mut!(MSOS_DESCRIPTOR) };
    let control_buffer = unsafe { &mut *addr_of_mut!(CONTROL_BUFFER) };
    let cdc_state = unsafe { &mut *addr_of_mut!(CDC_STATE) };

    let mut builder = Builder::new(
        driver,
        config,
        config_descriptor,
        bos_descriptor,
        msos_descriptor,
        control_buffer,
    );
    let mut cdc = CdcAcmClass::new(&mut builder, cdc_state, 64);
    let usb = builder.build();

    spawner.spawn(usb_task(usb)).unwrap();

    loop {
        cdc.wait_connection().await;
        while !cdc.dtr() {
            Timer::after(DTR_POLL_INTERVAL).await;
        }
        if cdc.write_packet(ARMED_MESSAGE).await.is_err() {
            continue;
        }
        while cdc.dtr() {
            Timer::after(DTR_POLL_INTERVAL).await;
        }
        interrupt::system_reset();
    }
}
