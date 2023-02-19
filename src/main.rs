#![no_std]
#![no_main]

use panic_halt as _;
mod driver;

#[arduino_hal::entry]
fn main() -> ! {
    let prescale_mode = driver::PrescaleMode::Freq1024;
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let timer1 = peripherals.TC1;
    let pins = arduino_hal::pins!(peripherals);
    driver::enable_fast_pwm(&timer1, pins.d10, prescale_mode);
    const A_4: u32 = 440; // hz
    play_note(A_4, prescale_mode, timer1);
    
    loop {
    }
}

fn play_note(note: u32, prescale_mode: driver::PrescaleMode, timer1: arduino_hal::pac::TC1) {
    const SYSTEM_CLOCK: u32 = 16_000_000;
    let prescale_factor = driver::to_factor(prescale_mode);
    let timer_clock = SYSTEM_CLOCK / prescale_factor as u32;
    let top = (timer_clock / note) as u16 - 1;
    driver::set_top(&timer1, top);
}



