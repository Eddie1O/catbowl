use esp_idf_hal::{
    adc::{attenuation, config::Config, AdcChannelDriver, AdcDriver, ADC1},
    delay::{Delay, Ets, FreeRtos},
    gpio::{Gpio10, Gpio3, Gpio7, Output, PinDriver},
    timer::{self, TimerDriver},
};
use esp_idf_svc::hal::prelude::Peripherals;

// ####### GPIO PIN CONFIGURATION ########
// GPIO 0
// GPIO 1
// GPIO 2
// GPIO 3 - adc photosensor
// GPIO 4
// GPIO 5
// GPIO 6
// GPIO 7 - LED Laser
// GPIO 8
// GPIO 9
// GPIO 10 - Motor
// GPIO 20
// GPIO 21

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    let peripherals = Peripherals::take()?;
    let mut led = PinDriver::output(peripherals.pins.gpio8)?;
    led.set_high()?;
    let mut timer = TimerDriver::new(peripherals.timer00, &timer::config::Config::default())?;
    timer.enable(true)?;
    let delay: Delay = Default::default();

    let mut adc = AdcDriver::new(peripherals.adc1, &Config::new().calibration(true))?;
    let mut photosensor: esp_idf_hal::adc::AdcChannelDriver<0, _> =
        AdcChannelDriver::new(peripherals.pins.gpio3)?;

    let mut led = PinDriver::output(peripherals.pins.gpio7)?;
    led.set_low()?;

    let mut motor = PinDriver::output(peripherals.pins.gpio10)?;

    loop {
        //one loop is exactly 24 hours
        let counter_now = timer.counter().unwrap();
        let counter_after_24_hours = counter_now + 1000000 * 60 * 60 * 24;
        delay.delay_ms(2000);
        rotate(&mut motor, &mut adc, &mut photosensor, &mut led);
        delay.delay_ms(1000 * 60 * 60 * 4);
        rotate(&mut motor, &mut adc, &mut photosensor, &mut led);
        delay.delay_ms(1000 * 60 * 60 * 4);
        rotate(&mut motor, &mut adc, &mut photosensor, &mut led);

        let ms_till_tomorrow: u32 = ((counter_after_24_hours - timer.counter().unwrap()) / 1000)
            .try_into()
            .unwrap();

        delay.delay_ms(ms_till_tomorrow);
    }
}

fn rotate(
    motor: &mut PinDriver<Gpio10, Output>,
    adc: &mut AdcDriver<ADC1>,
    photosensor: &mut AdcChannelDriver<0, Gpio3>,
    led: &mut PinDriver<Gpio7, Output>,
) {
    led.set_high().unwrap();
    motor.set_high().unwrap();
    FreeRtos::delay_ms(2000);
    //
    loop {
        let mut count = 0;
        for _ in 0..250 {
            count += adc.read_raw(photosensor).unwrap();

            Ets::delay_us(10);
        }
        if count >= 5800 {
            FreeRtos::delay_ms(150);
            motor.set_low().unwrap();
            led.set_low().unwrap();
            break;
        }
    }
}
