//! RP pico blinky med PIO
//!
//! Dette eksempelet er basert på https://github.com/rp-rs/rp2040-project-template
//!
//! Se https://blog.rustnekretser.no/rust-på-rp-pico/ for mer informasjon.

#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{
    gpio::{FunctionPio0, Pin},
    pac,
    pio::{PIOBuilder, PIOExt},
    sio::Sio,
};

use pio_proc::pio_asm;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // For å bruke en pinne i PIO må vi først konfigurere den som PIO.
    // Pinnen `led` er satt opp til pinne 25 som er koblet til LED'en på RP Pico.
    let _: Pin<_, FunctionPio0> = pins.led.into_mode();

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    // Vi kan skrive pioasm direkte i Rust-kildekoden
    let program = pio_asm!(
        "
        set pindirs, 1
        loop:
            set pins, 0 [31]
            set pins, 1 [30]
            jmp loop
        "
    );

    // Programmet må innstaleres i PIO-minnet
    let installed = pio.install(&program.program).unwrap();

    // Her konfigurer vi `sm0` til å kjøre programmet vårt med pinne 25
    // og klokkedeleren satt til 0 (0 betyr 65536 her).
    let (sm0, _, _) = PIOBuilder::from_program(installed)
        .set_pins(25, 1)
        .clock_divisor(0f32)
        .build(sm0);

    // Start tilstandsmaskinen
    sm0.start();

    // Selve blinkingen foregår uten bruk av CPU'en, så den kan settes i sovemodus
    loop {
        asm::wfi();
    }
}
