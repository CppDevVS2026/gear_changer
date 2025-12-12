// Cargo.toml dependencies:
// [dependencies]
// gilrs = "0.10"

use gilrs::{Button, Event, EventType, Gilrs};
use std::io::{self, Write};

struct Car {
    torque: f32,     // lb-ft
    horsepower: f32, // HP
    current_gear: u8,
    max_torque: f32, // Maximum possible torque for calculations
}

impl Car {
    fn new(torque: f32, horsepower: f32) -> Self {
        Self {
            torque,
            horsepower,
            current_gear: 3,
            max_torque: 1000.0, // Assuming max 1000 lb-ft for scaling
        }
    }

    fn calculate_rumble_intensity(&self, is_downshift: bool) -> f32 {
        // Base intensity from torque (0.0 to 1.0)
        let mut intensity = self.torque / self.max_torque;

        // Downshifts are 30% stronger, upshifts are 20% lighter
        if is_downshift {
            intensity *= 1.3;
        } else {
            intensity *= 0.8;
        }

        // Clamp between 0.0 and 1.0
        intensity.min(1.0).max(0.0)
    }

    fn upshift(&mut self, gamepad_id: gilrs::GamepadId, gilrs: &mut Gilrs) {
        if self.current_gear < 6 {
            self.current_gear += 1;
            let intensity = self.calculate_rumble_intensity(false);

            println!("\n🔼 UPSHIFT → Gear {}", self.current_gear);
            println!("   Rumble Intensity: {:.1}%", intensity * 100.0);

            // Trigger rumble
            self.trigger_rumble(gamepad_id, gilrs, intensity, false);
        } else {
            println!("\n⚠️  Already in highest gear!");
        }
    }

    fn downshift(&mut self, gamepad_id: gilrs::GamepadId, gilrs: &mut Gilrs) {
        if self.current_gear > 1 {
            self.current_gear -= 1;
            let intensity = self.calculate_rumble_intensity(true);

            println!("\n🔽 DOWNSHIFT → Gear {}", self.current_gear);
            println!("   Rumble Intensity: {:.1}%", intensity * 100.0);

            // Trigger rumble
            self.trigger_rumble(gamepad_id, gilrs, intensity, true);
        } else {
            println!("\n⚠️  Already in first gear!");
        }
    }

    fn trigger_rumble(
        &self,
        gamepad_id: gilrs::GamepadId,
        gilrs: &mut Gilrs,
        intensity: f32,
        is_downshift: bool,
    ) {
        let gamepad = gilrs.gamepad(gamepad_id);

        // Duration in milliseconds
        let duration = if is_downshift { 200 } else { 150 };

        // Try to trigger rumble
        if gamepad.is_ff_supported() {
            let strong_magnitude = (intensity * 65535.0) as u16;
            let weak_magnitude = (intensity * 0.7 * 65535.0) as u16;

            // Note: gilrs rumble support varies by platform
            // This creates a simple rumble effect
            let _ =
                gilrs
                    .gamepad(gamepad_id)
                    .set_rumble(strong_magnitude, weak_magnitude, duration);

            println!("   💥 Rumble triggered!");
        } else {
            println!("   ⚠️  Rumble not supported on this gamepad");
        }
    }

    fn display_status(&self) {
        println!("\n┌─────────────────────────────────┐");
        println!("│      CURRENT STATUS             │");
        println!("├─────────────────────────────────┤");
        println!("│ Gear:       {}                   │", self.current_gear);
        println!("│ Torque:     {:.0} lb-ft          │", self.torque);
        println!("│ Horsepower: {:.0} HP             │", self.horsepower);
        println!("└─────────────────────────────────┘");
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("╔═══════════════════════════════════════╗");
    println!("║  GEAR SHIFT HAPTIC FEEDBACK SIMULATOR ║");
    println!("╚═══════════════════════════════════════╝\n");

    // Get car specs from user
    let torque_input = get_input("Enter car torque (lb-ft) [e.g., 300]: ");
    let torque = torque_input.parse::<f32>().unwrap_or(300.0);

    let hp_input = get_input("Enter car horsepower [e.g., 400]: ");
    let horsepower = hp_input.parse::<f32>().unwrap_or(400.0);

    let mut car = Car::new(torque, horsepower);

    println!("\n✅ Car configured!");
    car.display_status();

    // Initialize gilrs
    let mut gilrs = match Gilrs::new() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("❌ Failed to initialize gamepad support: {}", e);
            return;
        }
    };

    // Check for connected gamepads
    let mut active_gamepad = None;
    for (_id, gamepad) in gilrs.gamepads() {
        println!("\n🎮 Gamepad found: {}", gamepad.name());
        active_gamepad = Some(gamepad.id());
        break;
    }

    if active_gamepad.is_none() {
        println!("\n⚠️  No gamepad detected! Please connect a gamepad and restart.");
        println!("Press Enter to exit...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        return;
    }

    println!("\n┌─────────────────────────────────┐");
    println!("│         CONTROLS                │");
    println!("├─────────────────────────────────┤");
    println!("│ X Button → Downshift (stronger)│");
    println!("│ B Button → Upshift (lighter)   │");
    println!("│ Start    → Exit                 │");
    println!("└─────────────────────────────────┘");
    println!("\n🏁 Ready! Start shifting...\n");

    // Main event loop
    loop {
        while let Some(Event { id, event, .. }) = gilrs.next_event() {
            match event {
                EventType::ButtonPressed(button, _) => {
                    match button {
                        Button::West => {
                            // X button = Downshift
                            if let Some(gamepad_id) = active_gamepad {
                                car.downshift(gamepad_id, &mut gilrs);
                            }
                        }
                        Button::East => {
                            // B button = Upshift
                            if let Some(gamepad_id) = active_gamepad {
                                car.upshift(gamepad_id, &mut gilrs);
                            }
                        }
                        Button::Start => {
                            println!("\n👋 Exiting...");
                            return;
                        }
                        _ => {}
                    }
                }
                EventType::Connected => {
                    println!("\n🎮 Gamepad connected!");
                    active_gamepad = Some(id);
                }
                EventType::Disconnected => {
                    println!("\n⚠️  Gamepad disconnected!");
                    active_gamepad = None;
                }
                _ => {}
            }
        }

        // Small delay to prevent CPU spinning
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
