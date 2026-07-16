#![no_std]
#![no_main]

use core::{fmt::Write as _, panic::PanicInfo, ptr::addr_of_mut};

use ch32v203g6u6_embassy_hal::{
    gpio::{DRV_GPIOA_RESOURCES, GPIOA, GPIOAInput, Pull},
    rcc::{DRV_RCC_RESOURCES, RCC},
    usb::{DRV_USBD_RESOURCES, USBD, USBDUsbDriver},
    wch,
};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer, with_timeout};
use embassy_usb::{
    Builder, Config, UsbDevice,
    class::cdc_acm::{CdcAcmClass, State as CdcState},
};
use embedded_hal_async::digital::Wait;
use heapless::String;

const EDGE_TIMEOUT: Duration = Duration::from_secs(5);
const ACTIVE_HOST_POLL: Duration = Duration::from_millis(100);
const INTER_STAGE_DELAY: Duration = Duration::from_millis(250);
const DISCONNECT_POLL: Duration = Duration::from_millis(250);
const CYCLE_COUNT: usize = 3;

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
    let rcc = RCC::new(DRV_RCC_RESOURCES).unwrap();
    rcc.configure_usb_fsdev_clock_48mhz().unwrap();
    wch::init_embassy_time_runtime().unwrap();

    let gpioa = GPIOA::new(DRV_GPIOA_RESOURCES).unwrap();
    gpioa.enable_clock().unwrap();
    gpioa.release_reset().unwrap();
    let mut exti_input = gpioa.pa7().into_input(Pull::None).unwrap();

    let usbd = USBD::new(DRV_USBD_RESOURCES).unwrap();
    let driver = usbd.embassy_usb_driver();

    let mut config = Config::new(0xCAFE, 0x4007);
    config.manufacturer = Some("TinyUSB");
    config.product = Some("CH32V203 EXTI Wait Smoke");
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
        wait_for_active_host(&mut cdc).await;

        log_line(
            &mut cdc,
            "EXTI wait smoke armed: drive PA7 with a clean 0-3.3V square wave.",
        )
        .await;

        match run_smoke_session(&mut cdc, &mut exti_input).await {
            Ok(()) => {
                log_line(&mut cdc, "PASS: EXTI wait operations completed on PA7.").await;
            }
            Err(message) => {
                log_line(&mut cdc, message).await;
            }
        }

        log_line(
            &mut cdc,
            "Session complete. Drop DTR or reconnect to rerun the smoke.",
        )
        .await;

        wait_for_disconnect(&mut cdc).await;
    }
}

async fn run_smoke_session(
    cdc: &mut CdcAcmClass<'static, USBDUsbDriver>,
    exti_input: &mut GPIOAInput,
) -> Result<(), &'static str> {
    wait_step(cdc, "wait_for_high()", exti_input.wait_for_high()).await?;
    wait_step(cdc, "wait_for_low()", exti_input.wait_for_low()).await?;

    for cycle in 1..=CYCLE_COUNT {
        log_cycle_step(cdc, cycle, "waiting for rising edge").await;
        with_timeout(EDGE_TIMEOUT, exti_input.wait_for_rising_edge())
            .await
            .map_err(|_| "FAIL: timed out waiting for a rising edge on PA7.")?
            .map_err(|_| "FAIL: wait_for_rising_edge() reported a GPIO/EXTI error.")?;
        log_cycle_step(cdc, cycle, "observed rising edge").await;
        Timer::after(INTER_STAGE_DELAY).await;

        log_cycle_step(cdc, cycle, "waiting for falling edge").await;
        with_timeout(EDGE_TIMEOUT, exti_input.wait_for_falling_edge())
            .await
            .map_err(|_| "FAIL: timed out waiting for a falling edge on PA7.")?
            .map_err(|_| "FAIL: wait_for_falling_edge() reported a GPIO/EXTI error.")?;
        log_cycle_step(cdc, cycle, "observed falling edge").await;
        Timer::after(INTER_STAGE_DELAY).await;
    }

    for index in 1..=CYCLE_COUNT {
        let mut message = String::<96>::new();
        let _ = write!(&mut message, "wait_for_any_edge() sample {} armed", index);
        log_line(cdc, message.as_str()).await;
        with_timeout(EDGE_TIMEOUT, exti_input.wait_for_any_edge())
            .await
            .map_err(|_| "FAIL: timed out waiting for any edge on PA7.")?
            .map_err(|_| "FAIL: wait_for_any_edge() reported a GPIO/EXTI error.")?;
        let mut message = String::<96>::new();
        let _ = write!(
            &mut message,
            "wait_for_any_edge() sample {} observed",
            index
        );
        log_line(cdc, message.as_str()).await;
        Timer::after(INTER_STAGE_DELAY).await;
    }

    Ok(())
}

async fn wait_step<F>(
    cdc: &mut CdcAcmClass<'static, USBDUsbDriver>,
    label: &str,
    future: F,
) -> Result<(), &'static str>
where
    F: core::future::Future<Output = Result<(), ch32v203g6u6_embassy_hal::metadata::Error>>,
{
    let mut message = String::<96>::new();
    let _ = write!(&mut message, "{} armed", label);
    log_line(cdc, message.as_str()).await;
    with_timeout(EDGE_TIMEOUT, future)
        .await
        .map_err(|_| "FAIL: timed out waiting for the requested PA7 level.")?
        .map_err(|_| "FAIL: wait_for_high()/wait_for_low() reported a GPIO/EXTI error.")?;
    let mut message = String::<96>::new();
    let _ = write!(&mut message, "{} observed", label);
    log_line(cdc, message.as_str()).await;
    Timer::after(INTER_STAGE_DELAY).await;
    Ok(())
}

async fn log_cycle_step(cdc: &mut CdcAcmClass<'static, USBDUsbDriver>, cycle: usize, step: &str) {
    let mut message = String::<96>::new();
    let _ = write!(&mut message, "cycle {}: {}", cycle, step);
    log_line(cdc, message.as_str()).await;
}

async fn log_line(cdc: &mut CdcAcmClass<'static, USBDUsbDriver>, message: &str) {
    let _ = cdc.write_packet(message.as_bytes()).await;
    let _ = cdc.write_packet(b"\r\n").await;
}

async fn wait_for_active_host(cdc: &mut CdcAcmClass<'static, USBDUsbDriver>) {
    while !cdc.dtr() {
        Timer::after(ACTIVE_HOST_POLL).await;
    }
}

async fn wait_for_disconnect(cdc: &mut CdcAcmClass<'static, USBDUsbDriver>) {
    while cdc.dtr() {
        Timer::after(DISCONNECT_POLL).await;
    }
}
