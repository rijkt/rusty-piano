use arduino_hal::hal::port::PB2;
use arduino_hal::port::mode::Floating;
use arduino_hal::port::mode::Input;
use arduino_hal::port::Pin;

/// Configure chip to use fast pulse width modulation mode using 16-bit Timer1.
/// See the ATmega328P data sheet for an in-depth explanation of the involved
/// registers.
///
/// Note: Some register writes are split for documentation's sake. The calls to
/// r.<register>.bits() are used to preserve previously written data.
pub(crate) fn enable_fast_pwm(timer1: &arduino_hal::pac::TC1, oc1b: Pin<Input<Floating>, PB2>, mode: PrescaleMode) {
    set_wgm_15(timer1);
    set_com_3(timer1);
    set_prescaler(timer1, mode);
    set_top(timer1, 255);
    oc1b.into_output();
}

/// Wave-form generation mode 15
/// FastPWM, TOP in OCR1A Update of OCR1A at BOTTOM, TOV1 Flag Set on TOP
fn set_wgm_15(timer1: &arduino_hal::pac::TC1) {
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
fn set_com_3(timer1: &arduino_hal::pac::TC1) {
    timer1.tccr1a.modify(
        |r, w| w
            .wgm1().bits(r.wgm1().bits())
            .com1b().bits(0b11)
            .com1a().bits(0b11)
    );    
}

#[derive(Copy, Clone)]
pub(crate) enum PrescaleMode {
    Direct,
    Freq8,
    Freq64,
    Freq256,
    Freq1024
}

fn to_bits(mode: PrescaleMode) -> u8 {
    match mode {
	    PrescaleMode::Direct => 0b000,
	    PrescaleMode::Freq8 => 0b01,
	    PrescaleMode::Freq64 => 2,
	    PrescaleMode::Freq256 => 1,
	    PrescaleMode::Freq1024 => 0b101
    }
}

pub(crate) fn to_factor(mode: PrescaleMode) -> u16 {
    match mode {
	    PrescaleMode::Direct => 1,
	    PrescaleMode::Freq8 => 8,
	    PrescaleMode::Freq64 => 64,
	    PrescaleMode::Freq256 => 256,
	    PrescaleMode::Freq1024 => 1024
    }
}

/// Set prescale factor. One of Direct (1), 8, 64, 256 or 1024.
fn set_prescaler(timer1: &arduino_hal::pac::TC1, mode: PrescaleMode) {
    let selected = to_bits(mode);
    timer1.tccr1b.modify(|r, w| w
                         .wgm1().bits(r.wgm1().bits())
                         .cs1().bits(selected));
}

/// Set the timer TOP value using the OCR1A register.
/// Requires Waveform Generation Mode 15 for FastPWM
pub(crate) fn set_top(timer1: &arduino_hal::pac::TC1, top: u16) {
    timer1.ocr1a.write(|w| w.bits(top)); // top
    timer1.ocr1b.write(|w| w.bits(top/2)); // duty cycle
}

