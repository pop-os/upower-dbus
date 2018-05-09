extern crate upower_dbus;

use std::process::exit;
use upower_dbus::UPower;

fn main() {
    let upower = match UPower::new(1000) {
        Ok(upower) => upower,
        Err(why) => {
            eprintln!("failed to get dbus connection: {}", why);
            exit(1);
        }
    };

    match upower.on_battery() {
        Ok(true) => {
            match upower.get_percentage() {
                Ok(percentage) => println!("battery is at {}%", percentage),
                Err(why) => {
                    eprintln!("could not get battery percentage: {}", why);
                    exit(1);
                }
            }
        }
        Ok(false) => println!("battery is not active"),
        Err(why) => {
            eprintln!("could not get battery status: {}", why);
            exit(1);
        }
    }
}
