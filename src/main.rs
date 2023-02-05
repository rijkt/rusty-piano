#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let timer1 = peripherals.TC1;
    timer1.tccr1a.modify(
	|_, w| w
	    .wgm1().bits(0b11)  // Wave-form generation mode 15: FastPWM, TOP in OCR1A Update of OCR1A at BOTTOM, TOV1 Flag Set on TOP
	    .com1b().bits(0b11) // Set OC1A on compare match, clear OC1A at BOTTOM (inverting mode)
	    .com1a().bits(0b11) // Set OC1A on compare match, clear OC1A at BOTTOM (inverting mode)
    );
    timer1.tccr1b.modify(|_, w| w.wgm1().bits(0b11)); // The other half of wgm mode config
    // TODO: set prescaler
    loop {
    }
}
