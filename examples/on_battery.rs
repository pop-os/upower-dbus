extern crate upower_dbus;

use upower_dbus::UPower;

fn main() {
    match UPower::new(1000).and_then(|u| u.on_battery()) {
        Ok(true) => println!("system is using battery"),
        Ok(false) => println!("system is on AC"),
        Err(why) => eprintln!("failed to get battery status: {}", why)
    }
}
