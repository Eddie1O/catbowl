#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};
use esp_backtrace as _;
use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::timer::timg::TimerGroup;
use log::info;

esp_bootloader_esp_idf::esp_app_desc!();

// GPIO 3  - ADC photosensor
// GPIO 7  - LED laser
// GPIO 8  - status LED
// GPIO 10 - motor
const PHOTOSENSOR_SAMPLES: u32 = 250;
const PHOTOSENSOR_THRESHOLD: u32 = 1646;
const MEAL_SPACING: Duration = Duration::from_secs(60 * 60 * 4);
const DAY: Duration = Duration::from_secs(24 * 60 * 60);

#[esp_rtos::main]
async fn main(_spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);

    let _status_led = Output::new(peripherals.GPIO8, Level::High, OutputConfig::default());
    let mut laser = Output::new(peripherals.GPIO7, Level::Low, OutputConfig::default());
    let mut motor = Output::new(peripherals.GPIO10, Level::Low, OutputConfig::default());

    let mut adc_config = AdcConfig::new();
    let mut photosensor = adc_config.enable_pin(peripherals.GPIO3, Attenuation::_11dB);
    let mut adc = Adc::new(peripherals.ADC1, adc_config);
    let delay = Delay::new();
    info!("catbowl started");

    let mut rotate = || {
        info!("feeding");
        laser.set_high();
        motor.set_high();
        delay.delay_millis(2000);

        loop {
            let mut count = 0;
            for _ in 0..PHOTOSENSOR_SAMPLES {
                let result = nb::block!(adc.read_oneshot(&mut photosensor)).unwrap();
                count += u32::from(result);
                delay.delay_micros(10);
            }
            count = count / PHOTOSENSOR_SAMPLES as u32;
            esp_println::println!("{count}");
            if count >= PHOTOSENSOR_THRESHOLD {
                delay.delay_millis(150);
                motor.set_low();
                laser.set_low();
                break;
            }
        }
    };

    loop {
        let day_start = Instant::now();

        Timer::after(Duration::from_secs(2)).await;
        rotate();

        Timer::after(MEAL_SPACING).await;
        rotate();

        Timer::after(MEAL_SPACING).await;
        rotate();

        let elapsed = day_start.elapsed();
        if elapsed < DAY {
            Timer::after(DAY - elapsed).await;
        }
    }
}
