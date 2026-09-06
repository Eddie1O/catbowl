## But why?

My `Cat Mate C300 Automatic 3 Meal Feeder` electronics died after few months of use. Instead of creating more waste I tried to use one of ESP32-C3's laying around to restore the functionality of this automatic feeder. I have used some spare parts and voilà! It works again and my cat is very happy about it.

Firmware is `no_std` [esp-hal](https://github.com/esp-rs/esp-hal) on RISC-V (`riscv32imc-unknown-none-elf`). Nightly is required only for Cargo `build-std`.

## Hardware:

1. Microcontroller: `ESP32C3`
2. Motor: simple 5V motor
3. Optoresistor for detecting in-between of bowl sections
4. Red LED to shine on optoresistor (covered by bowl, when not in-between of 3 bowl spaces)
