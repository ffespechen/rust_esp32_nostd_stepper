// Control de un motor paso a paso 28BYJ-48 con driver ULN2003
// Usando no_std para ESP32

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, delay::Delay, gpio::IO};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    // Aplica patrón singleton para periféricos
    // https://docs.rs/esp32-hal/0.18.1/esp32_hal/peripherals/struct.IO_MUX.html#impl-IO_MUX
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // LEDs
    let mut in1 = io.pins.gpio19.into_push_pull_output();
    let mut in2 = io.pins.gpio18.into_push_pull_output();
    let mut in3 = io.pins.gpio5.into_push_pull_output();
    let mut in4 = io.pins.gpio17.into_push_pull_output();

    let mut direccion_horaria = io.pins.gpio25.into_push_pull_output();
    let mut direccion_antihoraria = io.pins.gpio26.into_push_pull_output();

    // Constantes de acuerdo a las características del motor
    const PASOS_POR_VUELTA: i32 = 4096;
    const ESPERA_MOTOR: u32 = 1200;

    // Secuencia para medios pasos
    let pasos: [u8; 8] = [
        0b00001000,
        0b00001100,
        0b00000100,
        0b00000110,
        0b00000010,
        0b00000011,
        0b00000001,
        0b00001001
    ];

    // Estados iniciales de los LEDs
    in1.set_low();
    in2.set_low();
    in3.set_low();
    in4.set_low();

    direccion_antihoraria.set_low();
    direccion_horaria.set_low();

        
    loop {

        // Giro en sentido horario
        direccion_horaria.set_high();
        direccion_antihoraria.set_low();

        for i in 1..(PASOS_POR_VUELTA * 2) {
            for paso in pasos {
                if paso & 1 == 1 { in1.set_high() } else { in1.set_low() };
                if paso & 2 == 2 { in2.set_high() } else { in2.set_low() };
                if paso & 4 == 4 { in3.set_high() } else { in3.set_low() };
                if paso & 8 == 8 { in4.set_high() } else { in4.set_low() };
                delay.delay_micros(ESPERA_MOTOR);
            }
        }

        delay.delay_millis(1000);

        // Giro en sentido anti-horario
        direccion_horaria.set_low();
        direccion_antihoraria.set_high();

        for _i in 1..(PASOS_POR_VUELTA * 2) {
            for j in (0..(pasos.len() - 1)).rev() {
                if pasos[j] & 1 == 1 { in1.set_high() } else { in1.set_low() };
                if pasos[j] & 2 == 2 { in2.set_high() } else { in2.set_low() };
                if pasos[j] & 4 == 4 { in3.set_high() } else { in3.set_low() };
                if pasos[j] & 8 == 8 { in4.set_high() } else { in4.set_low() };
                delay.delay_micros(ESPERA_MOTOR);
            }
        }

        delay.delay_millis(1000);

    }
}
