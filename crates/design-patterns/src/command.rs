//!
//! # Command Design Pattern
//!
//! To run/test, please run the following commands in your terminal
//!
//! ```sh
//! cargo run --bin command
//! ```
//!
//! ```sh
//! cargo test --bin command
//! ```
//!
//! Command pattern is a behavioral design pattern that turns a request into a
//! stand-alone object that contains all the information about the request.
//!
//! Key components in this pattern are as follows:
//!
//! | component        | description                                  |
//! | ---------------- | -------------------------------------------- |
//! | command          | Trait that defines the common method         |
//! | Concrete Command | Each struct that implements Command trait    |
//! | Invoker          | function that invokes the concrete command   |
//! | Receiver         | Function that performs actual work           |
//! | Client           | Part of a code that uses the above mechanism |
//!

use std::{thread::sleep, time::Duration};

// Example below uses Command pattern to fire a weapon in a shooting game in
// which firing command directly fires any weapon without knowing the detail
// about each element's fire procedure.
use console::Term;

const MAX_AMMO: usize = 30;
// Define Command
trait Command {
    fn execute(&self) -> u8;
}

// Define Weapon Classes and traits

trait WeaponTrait {
    fn fire(&mut self) -> u8; // it returns number of bullets fired
}

#[derive(Clone)]
struct AssultRifle {
    fire_rate: u8,
}

#[derive(Clone)]
struct Sniper {
    fire_rate: u8,
    efficiency: u8,
}

impl WeaponTrait for AssultRifle {
    fn fire(&mut self) -> u8 {
        // sleep(Duration::from_millis((100 - self.fire_rate as u64) * 10));
        self.fire_rate
    }
}

impl WeaponTrait for Sniper {
    fn fire(&mut self) -> u8 {
        // sleep(Duration::from_millis((100 - self.fire_rate as u64) * 10));
        (self.fire_rate as f32 * self.efficiency as f32 * 0.01) as u8
    }
}

// Define Concrete Command classes and implementations

#[derive(Clone)]
struct FireAssultRifleCommand {
    weapon: AssultRifle,
}

struct FireSniperCommand {
    weapon: Sniper,
}

impl Command for FireAssultRifleCommand {
    fn execute(&self) -> u8 {
        self.weapon.clone().fire()
    }
}

impl Command for FireSniperCommand {
    fn execute(&self) -> u8 {
        self.weapon.clone().fire()
    }
}

fn main() {
    let assult_rifle = AssultRifle { fire_rate: 90 };
    let sniper = Sniper {
        fire_rate: 30,
        efficiency: 90,
    };

    let fire_assult_rifle_command = FireAssultRifleCommand {
        weapon: assult_rifle,
    };
    let fire_sniper_command = FireSniperCommand { weapon: sniper };

    let commands: Vec<Box<dyn Command>> = vec![
        Box::new(fire_assult_rifle_command),
        Box::new(fire_sniper_command),
    ];

    let mut selected = 0;
    let mut ammo = MAX_AMMO;
    let stdout = Term::stdout();

    loop {
        stdout.clear_screen().unwrap();
        let ammo_emoji = if selected == 0 { "🔫" } else { "🚀" };
        println!(
            r#"
===============================================================================
[ 1 ]: Rifle     [ 2 ]: Sniper    [ F ]: Fire    [ R ]: Reload   [ Q ]: Quit
===============================================================================

[{:03}] {}
-------------------------------------------------------------------------------
{}

"#,
            ammo,
            if ammo > 0 {
                (ammo_emoji).repeat(ammo)
            } else {
                "!! NO AMMO !!".to_string()
            },
            if selected == 0 {
                "[Rifle] ︻デ═一💥"
            } else {
                "[Sniper] 💥╾━╤デ╦︻"
            },
        );
        if let Ok(character) = stdout.read_char() {
            match character {
                '1' => {
                    if selected != 0 {
                        selected = 0;
                        ammo = MAX_AMMO;
                    }
                }
                '2' => {
                    if selected != 1 {
                        selected = 1;
                        ammo = MAX_AMMO;
                    }
                }
                'f' => {
                    if ammo > 0 {
                        let command = commands.get(selected).clone().unwrap();
                        let fire_rate = command.execute();
                        if ammo > 0 {
                            ammo -= 1;
                        }
                        // it pauses the execution depending the firerate. 100 means no pause
                        // We are currently dealing with the rust std library without async, so the firing will not
                        // block the key press buffer, so if you repeatedly press f, it takes time to complete firing.
                        sleep(Duration::from_millis((100 - fire_rate as u64) * 10));
                    }
                }
                'r' => {
                    ammo = MAX_AMMO;
                }
                _ => {
                    break;
                }
            }
        }
    }
}
