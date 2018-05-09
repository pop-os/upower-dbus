# upower-dbus

A Rust library which interfaces with UPower status information through dbus.

## Examples

### Detecting if the system is running on battery

```rust
extern crate upower_dbus;

use upower_dbus::UPower;

fn main() {
    match UPower::new(1000).and_then(|u| u.on_battery()) {
        Ok(true) => println!("system is using battery"),
        Ok(false) => println!("system is on AC"),
        Err(why) => eprintln!("failed to get battery status: {}", why)
    }
}
```

### Getting the current battery status as a percentage

```rust
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
```
