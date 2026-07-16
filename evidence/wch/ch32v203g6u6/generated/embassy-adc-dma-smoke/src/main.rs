#![no_std]
#![no_main]

use core::{
    fmt::Write as _,
    panic::PanicInfo,
    ptr::{addr_of_mut, read_volatile, write_volatile},
};

use ch32v203g6u6_embassy_hal::{
    adc::{ADC1, DRV_ADC1_RUNTIME_RESOURCES},
    dma::{DMA1, DRV_DMA1_RUNTIME_RESOURCES},
    gpio::{DRV_GPIOA_RUNTIME_RESOURCES, GPIOA},
    rcc::{DRV_RCC_RUNTIME_RESOURCES, RCC},
    usb::{DRV_USBD_RUNTIME_RESOURCES, USBD, USBDUsbDriver},
    wch,
};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer, with_timeout};
use embassy_usb::{
    Builder, Config, UsbDevice,
    class::cdc_acm::{CdcAcmClass, State as CdcState},
};
use heapless::String;

const ADC_CHANNELS: &[u8] = &[7];
const ADC_SAMPLE_TIME_CODE: u8 = 7;
const ONE_SHOT_BUFFER_LEN: usize = 32;
const IDLE_RETRY_MS: u64 = 100;
const REPORT_PERIOD_MS: u64 = 1000;
const DMA_POLL_TIMEOUT_MS: u64 = 250;
const DMA_CHANNEL_INDEX: u32 = 1;

static mut CONFIG_DESCRIPTOR: [u8; 256] = [0; 256];
static mut BOS_DESCRIPTOR: [u8; 64] = [0; 64];
static mut MSOS_DESCRIPTOR: [u8; 64] = [0; 64];
static mut CONTROL_BUFFER: [u8; 128] = [0; 128];
static mut CDC_STATE: CdcState<'static> = CdcState::new();

const RCC_CFGR0: *mut u32 = 0x4002_1004 as *mut u32;
const GPIOA_CFGLR: *mut u32 = 0x4001_0800 as *mut u32;
const RCC_CFGR0_ADCPRE_MASK: u32 = 0x0000_C000;
const RCC_CFGR0_ADCPRE_DIV8: u32 = 0x0000_C000;
const GPIOA_CFGLR_PA7_MASK: u32 = 0xF000_0000;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

fn modify_u32(address: *mut u32, clear_mask: u32, set_mask: u32) {
    unsafe {
        let current = read_volatile(address);
        write_volatile(address, (current & !clear_mask) | set_mask);
    }
}

fn configure_adc_path() {
    // Match the WCH ADC DMA example: keep ADCCLK within spec and put PA7 in analog mode.
    modify_u32(RCC_CFGR0, RCC_CFGR0_ADCPRE_MASK, RCC_CFGR0_ADCPRE_DIV8);
    modify_u32(GPIOA_CFGLR, GPIOA_CFGLR_PA7_MASK, 0x0000_0000);
}

#[embassy_executor::task]
async fn usb_task(mut device: UsbDevice<'static, USBDUsbDriver>) -> ! {
    device.run().await
}

struct SampleStats {
    first: u16,
    last: u16,
    min: u16,
    max: u16,
    avg: u32,
}

fn sample_stats(samples: &[u16]) -> SampleStats {
    let mut min = u16::MAX;
    let mut max = 0u16;
    let mut sum = 0u32;
    for &sample in samples {
        if sample < min {
            min = sample;
        }
        if sample > max {
            max = sample;
        }
        sum = sum.wrapping_add(u32::from(sample));
    }
    SampleStats {
        first: samples[0],
        last: samples[samples.len() - 1],
        min,
        max,
        avg: sum / u32::try_from(samples.len()).unwrap_or(1),
    }
}

async fn write_line(cdc: &mut CdcAcmClass<'static, USBDUsbDriver>, line: &str) -> Result<(), ()> {
    for chunk in line.as_bytes().chunks(64) {
        cdc.write_packet(chunk).await.map_err(|_| ())?;
    }
    Ok(())
}

async fn run_one_shot(
    adc1: &ADC1,
    dma1: &DMA1,
    cdc: &mut CdcAcmClass<'static, USBDUsbDriver>,
) -> Result<(), ()> {
    let mut samples = [0u16; ONE_SHOT_BUFFER_LEN];
    configure_adc_path();
    write_line(cdc, "oneshot start\r\n").await?;
    match with_timeout(
        Duration::from_millis(DMA_POLL_TIMEOUT_MS),
        adc1.sample_one_shot_dma_u16_async(dma1, ADC_CHANNELS, ADC_SAMPLE_TIME_CODE, &mut samples),
    )
    .await
    {
        Ok(Ok(())) => {
            let stats = sample_stats(&samples);
            let mut line: String<96> = String::new();
            let _ = write!(
                line,
                "oneshot pa7 n={} first={} last={} min={} max={} avg={}\r\n",
                samples.len(),
                stats.first,
                stats.last,
                stats.min,
                stats.max,
                stats.avg
            );
            write_line(cdc, line.as_str()).await
        }
        _ => {
            let half = adc1.is_dma_half_transfer().map_err(|_| ())? as u8;
            let full = dma1.is_transfer_complete(DMA_CHANNEL_INDEX).map_err(|_| ())? as u8;
            let _ = dma1.disable_transfer_complete_interrupt(DMA_CHANNEL_INDEX);
            let _ = adc1.stop_dma_sampling();
            let mut line: String<80> = String::new();
            let _ = write!(
                line,
                "oneshot timeout half={} full={} first={} last={}\r\n",
                half,
                full,
                samples[0],
                samples[samples.len() - 1]
            );
            write_line(cdc, line.as_str()).await?;
            Err(())
        }
    }
}

#[embassy_executor::main(entry = "riscv_rt::entry")]
async fn main(spawner: Spawner) -> ! {
    let rcc = RCC::new(DRV_RCC_RUNTIME_RESOURCES).unwrap();
    rcc.configure_usb_fsdev_clock_48mhz().unwrap();
    wch::init_embassy_time_runtime().unwrap();

    let gpioa = GPIOA::new(DRV_GPIOA_RUNTIME_RESOURCES).unwrap();
    gpioa.enable_clock().unwrap();
    gpioa.release_reset().unwrap();
    configure_adc_path();

    let adc1 = ADC1::new(DRV_ADC1_RUNTIME_RESOURCES).unwrap();
    adc1.enable_clock().unwrap();
    adc1.release_reset().unwrap();
    adc1.apply_calibrate().unwrap();
    let dma1 = DMA1::new(DRV_DMA1_RUNTIME_RESOURCES).unwrap();

    let usbd = USBD::new(DRV_USBD_RUNTIME_RESOURCES).unwrap();
    let driver = usbd.embassy_usb_driver();

    let mut config = Config::new(0xCAFE, 0x4006);
    config.manufacturer = Some("TinyUSB");
    config.product = Some("CH32V203 ADC DMA Smoke");
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
            Timer::after(Duration::from_millis(IDLE_RETRY_MS)).await;
        }

        if write_line(&mut cdc, "adc-dma ready\r\n").await.is_err() {
            continue;
        }

        loop {
            if !cdc.dtr() {
                break;
            }
            if run_one_shot(&adc1, &dma1, &mut cdc).await.is_err() {
                break;
            }
            Timer::after(Duration::from_millis(REPORT_PERIOD_MS)).await;
        }
    }
}
