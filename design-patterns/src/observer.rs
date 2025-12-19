//!
//! # Observer Design Pattern
//!
//! To run/test, please run the following commands in your terminal
//!
//! ```sh
//! cargo run --bin observer
//! ```
//!
//! ```sh
//! cargo test --bin observer
//! ```
//!
//! An Observer Pattern defines a one to many dependency between objects so that
//! all dependent objects automatically get notified once the state changes in
//! one object.
//!
//! Observer pattern consists of 3 components:
//!  - `Subject`   : It is the entity being observed for the changes.
//!  - `Observer`  : It is an entity that observes changes in subjects.
//!  - `Client`    : It is an entity that uses the state of the subject.
//!
//! As Rustlang is a functional programming language with some OOP features, we
//! need to define traits and structs separately.
//!
//! The code snippet below uses observer pattern to notify users
//! once it receives readings in the weather data.
//!*/
use common::input; // common library for this repository
use rand;
use std::collections::HashMap;

// Define Observers
#[derive(PartialEq, Eq, Hash, Clone)]
enum SensorType {
    Rain,
    Wind,
}

type Subscriber = fn(value: u32);

#[derive(Default)]
struct Publisher {
    sensors: HashMap<SensorType, Vec<Subscriber>>,
}

impl Publisher {
    pub fn subscribe(&mut self, sensor_type: SensorType, listener: Subscriber) {
        self.sensors.entry(sensor_type.clone()).or_default();
        self.sensors.get_mut(&sensor_type).unwrap().push(listener);
    }
    pub fn unsubscribe(&mut self, sensor_type: SensorType, listener: Subscriber) {
        self.sensors
            .get_mut(&sensor_type)
            .unwrap()
            .retain(|&x| !std::ptr::fn_addr_eq(x, listener));
    }
    pub fn notify(&mut self, sensor_type: SensorType, value: u32) {
        let listeners = self.sensors.get(&sensor_type).unwrap();
        for listener in listeners {
            listener(value);
        }
    }
}

// Define Subjects

#[derive(Default)]
struct Sensor {
    publisher: Publisher,
}

impl Sensor {
    fn readings(&mut self) -> &mut Publisher {
        &mut self.publisher
    }

    fn get_new_reading(&mut self, sensor_type: SensorType) {
        self.publisher.notify(sensor_type, rand::random::<u32>())
    }
}

fn main() {
    let rain_listener: Subscriber = |value| {
        println!("RAIN:\t\t{value}mm");
    };
    let wind_listener: Subscriber = |value| {
        println!("WIND:\t\t{value}m/s2");
    };
    println!("Observer Pattern");
    let mut sensor = Sensor::default();

    sensor.readings().subscribe(SensorType::Rain, rain_listener);
    sensor.readings().subscribe(SensorType::Wind, wind_listener);
    loop {
        println!(
            r#"
            -------------------------------------------
            [r/R]: Read
            [sr/SR]: Subscribe to the Rain data
            [sw/SW]: Subscribe to the Wind data
            [ur/UR]: Unsubscribe to the Rain data
            [uw/UW]: Unsubscribe to the Wind data
            [e/E]: Exit
            -------------------------------------------
            "#
        );
        // ! input() is a common library function, not included in std
        match input("Enter option: ").as_str().trim() {
            "R" | "r" => {
                // display readings of subscribed sensors
                sensor.get_new_reading(SensorType::Rain);
                sensor.get_new_reading(SensorType::Wind);
            }
            "SR" | "sr" => {
                // Subscribe to rain sensor readings
                sensor.readings().subscribe(SensorType::Rain, rain_listener);
            }
            "SW" | "sw" => {
                // Subscribe to wind sensor readings
                sensor.readings().subscribe(SensorType::Wind, wind_listener);
            }
            "UR" | "ur" => {
                // Unsubscribe to rain sensor readings
                sensor
                    .readings()
                    .unsubscribe(SensorType::Rain, rain_listener);
            }
            "UW" | "uw" => {
                // Unsubscribe to wind sensor readings
                sensor
                    .readings()
                    .unsubscribe(SensorType::Wind, wind_listener);
            }
            _ => {
                break;
            }
        };
    }
}
