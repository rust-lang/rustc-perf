struct Emulator {
    state: nes::State,
}

impl nes::Context for Emulator {
    fn state(&self) -> &nes::State {
        &self.state
    }

    fn state_mut(&mut self) -> &mut nes::State {
        &mut self.state
    }
}

// Source of the ROM: https://github.com/christopherpow/nes-test-roms/tree/97720008e51db15dd281a2a1e64d4c65cf1bca4c/nes15-1.0.0
// Licensed under a BSD-style license.
const ROM: &[u8] = include_bytes!("../../data/nes15-NTSC.nes");

fn main() {
    use nes::Interface;
    benchlib::benchmark::run_benchmark_group(|group| {
        group.register_benchmark("pinky-nes15", || {
            let mut nes = Emulator {
                state: nes::State::new(),
            };

            nes.load_rom(ROM).unwrap();

            move || {
                // Trigger the autosolve mechanism in the ROM.
                for _ in 0..4 {
                    nes.execute_until_vblank().unwrap();
                }

                nes.press(nes::ControllerPort::First, nes::Button::Select);
                nes.execute_until_vblank().unwrap();
                nes.release(nes::ControllerPort::First, nes::Button::Select);

                for _ in 0..3 {
                    nes.execute_until_vblank().unwrap();
                }

                nes.press(nes::ControllerPort::First, nes::Button::Select);
                nes.execute_until_vblank().unwrap();
                nes.release(nes::ControllerPort::First, nes::Button::Select);

                // And then just let it run for a while.
                for _ in 0..2 * 60 {
                    nes.execute_until_vblank().unwrap();
                }

                nes
            }
        });
    });
}
