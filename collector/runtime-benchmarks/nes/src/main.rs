//! This benchmark runs a simple game under a mostly cycle-accurate NES emulator,
//! which under the hood is essentially just a fancy bytecode interpreter.

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
                // By default the game which we're emulating stays on the main menu
                // screen without anything happening on screen; it doesn't even try
                // to play any music.
                //
                // Pressing any button on the gamepad actually starts the game, and
                // once the game is started pressing the select button triggers
                // an autosolve mechanism which makes the game automatically play itself.
                //
                // So let's trigger this here.

                // We need to wait for four frames until the game accepts any input.
                //
                // I don't know why; I just started to empirically increase the number
                // of frames until it worked.
                for _ in 0..4 {
                    nes.execute_until_vblank().unwrap();
                }

                // Now we can press a button to start the game.
                nes.press(nes::ControllerPort::First, nes::Button::Select);
                nes.execute_until_vblank().unwrap();
                nes.release(nes::ControllerPort::First, nes::Button::Select);

                // We need to wait for at least three frames until we can trigger
                // the autosolve mechanism.
                for _ in 0..3 {
                    nes.execute_until_vblank().unwrap();
                }

                // Now we can press select to make the game start playing itself.
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
