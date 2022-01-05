# upower-dbus

A Rust library which interfaces with UPower status information through dbus.

## Examples

### Detecting if the system is running on battery

```rust
extern crate upower_dbus;

use futures::stream::StreamExt;
use upower_dbus::UPowerProxy;

fn main() -> zbus::Result<()> {
    futures::executor::block_on(async move {
        let connection = zbus::Connection::system().await?;

        let upower = UPowerProxy::new(&connection).await?;

        println!("On Battery: {:?}", upower.on_battery().await);

        let mut stream = upower.receive_on_battery_changed().await;

        while let Some(event) = stream.next().await {
            println!("On Battery: {:?}", event.get().await);
        }

        Ok(())
    })
}

```

### Getting the current battery status as a percentage

```rust
extern crate upower_dbus;

use upower_dbus::{DeviceProxy, UPowerProxy};

fn main() -> zbus::Result<()> {
    futures::executor::block_on(async move {
        let connection = zbus::Connection::system().await?;

        let upower = UPowerProxy::new(&connection).await?;

        let display_device = upower.get_display_device().await?;

        let device = DeviceProxy::builder(&connection)
            .path(display_device)?
            .build()
            .await?;

        println!("Battery: {:?}", device.percentage().await);

        Ok(())
    })
}

```
