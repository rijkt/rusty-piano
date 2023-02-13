#![no_std]
#![no_main]

use arduino_hal::pac;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let timer1 = peripherals.TC1;
    enable_fast_pwm(&timer1);
    let pins = arduino_hal::pins!(peripherals);
    let oc1b = pins.d10;
    oc1b.into_output(); // todo: use low-level library, move to pwm fn

    loop {
        for top in 0..=125 {
            set_top(&timer1, top);
            arduino_hal::delay_ms(20); // control duty cycle
        }
    }
}

/// Configure chip to use fast pulse width modulation mode using 16-bit Timer1.
/// See the ATmega328P data sheet for an in-depth explanation of the involved
/// registers.
///
/// Note: Some register writes are split for documentation's sake. The calls to
/// r.<register>.bits() are used to preserve previously written data.
fn enable_fast_pwm(timer1: &pac::TC1) {
    set_wgm_15(&timer1);
    set_com_3(&timer1);
    set_prescaler(&timer1);
}


/// Wave-form generation mode 15
/// FastPWM, TOP in OCR1A Update of OCR1A at BOTTOM, TOV1 Flag Set on TOP
fn set_wgm_15(timer1: &pac::TC1) {
    timer1.tccr1a.modify(|r, w| w
                         .wgm1().bits(0b11)
                         .com1b().bits(r.com1b().bits())
                         .com1a().bits(r.com1a().bits())
    );
    timer1.tccr1b.modify(|r, w| w
                         .wgm1().bits(0b11)
                         .cs1().bits(r.cs1().bits())
    );
}

/// Compare Output Mode 3.
/// Set OC1A/OC1B on compare match, clear OC1A/OC1B at BOTTOM (inverting mode).
/// Relevant for us is OC1B: our output pin.
fn set_com_3(timer1: &pac::TC1) {
    timer1.tccr1a.modify(
        |r, w| w
            .wgm1().bits(r.wgm1().bits())
            .com1b().bits(0b11)
            .com1a().bits(0b11)
    );    
}

/// Set prescale factor. One of Direct (1), 8, 64, 256 or 1024.
fn set_prescaler(timer1: &pac::TC1) {
    // todo: parameterize mode
    timer1.tccr1b.modify(|r, w| w
                         .wgm1().bits(r.wgm1().bits())
                         .cs1().bits(0b101)); // prescaler 1024
}

/// Set the timer TOP value using the OCR1A register.
/// Requires Waveform Generation Mode 15 for FastPWM
fn set_top(timer1: &pac::TC1, top: u16) {
    timer1.ocr1a.write(|w| w.bits(top));
}
