#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let timer1 = peripherals.TC1;
    // set wave-form generation mode 15 and Compare Output Mode 3
    timer1.tccr1a.write(
	|w| w
	    .wgm1().bits(0b11)  // Wave-form generation mode 15: FastPWM, TOP in OCR1A Update of OCR1A at BOTTOM, TOV1 Flag Set on TOP
	    .com1b().bits(0b11) // Set OC1A on compare match, clear OC1A at BOTTOM (inverting mode)
	    .com1a().bits(0b11) // Set OC1A on compare match, clear OC1A at BOTTOM (inverting mode)
    );
    timer1.tccr1b.write(|w| w.wgm1().bits(0b11)  // The other half of wgm mode config
			.cs1().bits(0b101)); // prescaler 1024

    timer1.ocr1a.write(|w| w.bits(125)); // TOP

    let pins = arduino_hal::pins!(peripherals);
    let output = pins.d10;
    output.into_output();
    loop {
	for top in 0..=125 {
            timer1.ocr1a.write(|w| w.bits(top));
	    arduino_hal::delay_ms(20); // control duty cycle
	}
    }
}
