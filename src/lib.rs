//! # upower-dbus
//!
//! A Rust library which interfaces with UPower status information through dbus.
//!
//! ## Examples
//!
//! ### Detecting if the system is running on battery
//!
//! ```rust
//! extern crate upower_dbus;
//!
//! use upower_dbus::UPower;
//!
//! fn main() {
//!     match UPower::new(1000).and_then(|u| u.on_battery()) {
//!         Ok(true) => println!("system is using battery"),
//!         Ok(false) => println!("system is on AC"),
//!         Err(why) => eprintln!("failed to get battery status: {}", why)
//!     }
//! }
//! ```
//!
//! ### Getting the current battery status as a percentage
//!
//! ```rust
//! extern crate upower_dbus;
//!
//! use std::process::exit;
//! use upower_dbus::UPower;
//!
//! fn main() {
//!     let upower = match UPower::new(1000) {
//!         Ok(upower) => upower,
//!         Err(why) => {
//!             eprintln!("failed to get dbus connection: {}", why);
//!             exit(1);
//!         }
//!     };
//!
//!     match upower.on_battery() {
//!         Ok(true) => {
//!             match upower.get_percentage() {
//!                 Ok(percentage) => println!("battery is at {}%", percentage),
//!                 Err(why) => {
//!                     eprintln!("could not get battery percentage: {}", why);
//!                     exit(1);
//!                 }
//!             }
//!         }
//!         Ok(false) => println!("battery is not active"),
//!         Err(why) => {
//!             eprintln!("could not get battery status: {}", why);
//!             exit(1);
//!         }
//!     }
//! }
//! ```

extern crate dbus;
extern crate failure;
#[macro_use]
extern crate failure_derive;

use dbus::{BusType, ConnPath, Connection, Message};
use dbus::stdintf::org_freedesktop_dbus::Properties;

macro_rules! device_property {
    (dev => $device:expr, $prop:expr, $type:ty) => {{
        $device.connection_path(match $device.get_display_device()? {
            Some(device) => device,
            None => {
                return Err(UPowerError::NoDisplayDevice);
            }
        }).get::<$type>("org.freedesktop.UPower.Device", $prop)
    }};

    (path => $device:expr, $path:expr, $prop:expr, $type:ty) => {{
        $device.connection_path($path).get::<$type>("org.freedesktop.UPower.Device", $prop)
    }};
}

#[derive(Debug, Fail)]
pub enum UPowerError {
    #[fail(display = "method call could not be created: {}", why)]
    MethodCall { why: String },
    #[fail(display = "dbus error occurred: {}", why)]
    Dbus { why: String },
    #[fail(display = "no display device could be obtained")]
    NoDisplayDevice
}

impl From<dbus::Error> for UPowerError {
    fn from(why: dbus::Error) -> UPowerError {
        UPowerError::Dbus { why: format!("{}", why) }
    }
}

pub struct UPower {
    connection: Connection,
    timeout: i32
}

impl UPower {
    /// Creates a new dbus connection for interfacing with UPower.
    ///
    /// The timeout value specified will be used with each dbus request.
    pub fn new(timeout: i32) -> Result<UPower, UPowerError> {
        let result = Connection::get_private(BusType::System)
            .map(|connection| UPower { connection, timeout })?;

        Ok(result)
    }

    fn connection_path<'a, P: Into<dbus::Path<'a>>>(&'a self, path: P) -> ConnPath<'a, &'a Connection> {
        self.connection.with_path("org.freedesktop.UPower", path, self.timeout)
    }

    /// A composite device that represents the status icon to show in desktop environments.
    /// You can also access the object directly as its path is guaranteed to be
    /// `/org/freedesktop/UPower/devices/DisplayDevice`.
    pub fn get_display_device(&self) -> Result<Option<dbus::Path>, UPowerError> {
        let reply = self.connection.send_with_reply_and_block(
            Message::new_method_call(
                "org.freedesktop.UPower",
                "/org/freedesktop/UPower",
                "org.freedesktop.UPower",
                "GetDisplayDevice"
            ).map_err(|why| UPowerError::MethodCall { why })?,
            self.timeout
        )?;

        Ok(reply.get1::<dbus::Path>())
    }

    /// Indicates whether the system is running on battery power. This property is provided for convenience.
    pub fn on_battery(&self) -> Result<bool, UPowerError> {
        let result = self.connection_path("/org/freedesktop/UPower")
            .get::<bool>("org.freedesktop.UPower", "OnBattery")?;

        Ok(result)
    }

    /// The amount of energy left in the power source expressed as a percentage between 0 and 100.
    ///
    /// Typically this is the same as (energy - energy-empty) / (energy-full - energy-empty).
    /// However, some primitive power sources are capable of only reporting percentages and in
    /// this case the energy-* properties will be unset while this property is set.
    pub fn get_percentage(&self) -> Result<f64, UPowerError> {
        let result = device_property!(dev => self, "Percentage", f64)?;

        Ok(result)
    }

    /// The amount of energy left in the power source expressed as a percentage between 0 and 100.
    ///
    /// Typically this is the same as (energy - energy-empty) / (energy-full - energy-empty).
    /// However, some primitive power sources are capable of only reporting percentages and in
    /// this case the energy-* properties will be unset while this property is set.
    pub fn get_percentage_of(&self, path: dbus::Path) -> Result<f64, UPowerError> {
        let result = device_property!(path => self, path, "Percentage", f64)?;

        Ok(result)
    }

    /// Amount of energy (measured in Wh) currently available in the power source.
    pub fn get_energy(&self) -> Result<f64, UPowerError> {
        let result = device_property!(dev => self, "Energy", f64)?;

        Ok(result)
    }

    /// Amount of energy (measured in Wh) currently available in the power source.
    pub fn get_energy_of(&self, path: dbus::Path) -> Result<f64, UPowerError> {
        let result = device_property!(path => self, path, "Energy", f64)?;

        Ok(result)
    }

    /// Amount of energy (measured in Wh) in the power source when it's considered full.
    pub fn get_energy_full(&self) -> Result<f64, UPowerError> {
        let result = device_property!(dev => self, "EnergyFull", f64)?;

        Ok(result)
    }

    /// Amount of energy (measured in Wh) in the power source when it's considered full.
    pub fn get_energy_full_of(&self, path: dbus::Path) -> Result<f64, UPowerError> {
        let result = device_property!(path => self, path, "EnergyFull", f64)?;

        Ok(result)
    }

    /// Whether power is currently being provided through line power.
    pub fn get_online(&self) -> Result<bool, UPowerError> {
        let result = device_property!(dev => self, "Online", bool)?;

        Ok(result)
    }

    /// Whether power is currently being provided through line power.
    pub fn get_online_of(&self, path: dbus::Path) -> Result<bool, UPowerError> {
        let result = device_property!(path => self, path, "Online", bool)?;

        Ok(result)
    }
}
