// \file cortex_m4.rs
// \copyright Copyright (C) Infineon Technologies AG 2023
// 
// Use of this file is subject to the terms of use agreed between (i) you or the company in which ordinary course of
// business you are acting and (ii) Infineon Technologies AG or its licensees. If and as long as no such terms of use
// are agreed, use of this file is subject to following:
// 
// Boost Software License - Version 1.0 - August 17th, 2003
// 
// Permission is hereby granted, free of charge, to any person or organization obtaining a copy of the software and
// accompanying documentation covered by this license (the "Software") to use, reproduce, display, distribute, execute,
// and transmit the Software, and to prepare derivative works of the Software, and to permit third-parties to whom the
// Software is furnished to do so, all subject to the following:
// 
// The copyright notices in the Software and this entire statement, including the above license grant, this restriction
// and the following disclaimer, must be included in all copies of the Software, in whole or in part, and all
// derivative works of the Software, unless such copies or derivative works are solely in the form of
// machine-executable object code generated by a source language processor.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
// WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, TITLE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// COPYRIGHT HOLDERS OR ANYONE DISTRIBUTING THE SOFTWARE BE LIABLE FOR ANY DAMAGES OR OTHER LIABILITY, WHETHER IN
// CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
// ---

use cortex_m::delay::Delay;
use cortex_m_semihosting::hprintln;

use cyt2b7 as pac;
use pac::gpio as GPIO;

/// Executes before the main function and can be used for HW initialization.
#[cortex_m_rt::pre_init]
unsafe fn before_main() {
}

/// CM4 "main" function
/// 
/// Demonstates how to use "safe rust" to access peripherals by taking ownership
/// of the `cyt2b7::Peripherals` instance.
/// The demo periodically toggles LED4 (port 19, pin 0).
#[cortex_m_rt::entry]
fn main() -> ! {
    _ = hprintln!("! CM4: Entering main()...");

    // Core peripheral registers...
    let cp = cortex_m::Peripherals::take().unwrap();
    let  syst = cp.SYST;

    // Peripheral registers...
    let p = pac::Peripherals::take().unwrap();

    let gpio = p.GPIO;
    configure_led(&gpio);
    
    let mut delay = Delay::new(syst, 100_000_000);
    let mut state = false;
    loop  {
        // Set GPIO state
        gpio.prt19.out_inv.write(|w| w.out0().bit(state));
        
        // Wait and toggle GPIO state
        delay.delay_ms(250);
        state = state^true;
    }
}

/// Set-up the relevant GPIO port/pin for the LED
fn configure_led(gpio: &pac::GPIO) {
    let strong_value: u8 = GPIO::prt::cfg::DRIVE_MODE0_A::STRONG.into();
    gpio.prt19.cfg.write(|w| w.drive_mode0().bits(strong_value));
}
