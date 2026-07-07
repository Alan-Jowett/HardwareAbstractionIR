#![no_std]
#![no_main]

use cortex_m_semihosting::debug;
use embassy_executor::Spawner;
use panic_halt as _;
use stm32_h405_generated::usart::{DRV_USART1_RESOURCES, Usart1};

const BRR_MANTISSA_115200_AT_16MHZ: u16 = 8;
const BRR_FRACTION_115200_AT_16MHZ: u8 = 11;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let usart1 = Usart1::new(DRV_USART1_RESOURCES).unwrap();

    usart1.enable_clock().unwrap();
    usart1.configure_tx_pa9_route().unwrap();
    usart1.configure_rx_pa10_route().unwrap();
    usart1.configure_8n1().unwrap();
    usart1
        .set_baud_divider(
            BRR_MANTISSA_115200_AT_16MHZ,
            BRR_FRACTION_115200_AT_16MHZ,
        )
        .unwrap();
    usart1.enable_transmitter().unwrap();
    usart1.enable().unwrap();
    usart1.write_bytes(b"Hello, USART1 from QEMU!\r\n").unwrap();
    usart1.flush().unwrap();

    debug::exit(debug::EXIT_SUCCESS);

    loop {
        cortex_m::asm::nop();
    }
}
