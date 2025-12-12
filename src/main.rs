mod calculators;

use gilrs::{Button, Event, Gilrs};

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    for (_id, gamepad) in gilrs.gamepads() {
        println!("{:?} ")
    }
}
