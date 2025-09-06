use std::time::Duration;
use dbus::blocking::Connection;
use dbus::arg;
use std::collections::HashMap;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use log::info;

// TODO: read from etc file later on
struct Bluetoothd {
        iphone_bluetooth_mac: String,
        bluetooth_dbus_interface: String,
        proxy_dest: String,
        proxy_path: String,
}




/// Get MAC of trusted device e.g. the iphone and try to connect to it
pub fn start() -> Result<(), String> {


        let mut connection_parameters: Bluetoothd = Bluetoothd {
                iphone_bluetooth_mac: "A0:78:2D:E9:CB:AC".to_string(),
                bluetooth_dbus_interface : "org.bluez.Adapter1".to_string(),
                proxy_dest: "org.bluez".to_string(),
                proxy_path: "/org/bluez/hci0".to_string(),
        };

        // Connect to the system bus
        let connection = Connection::new_system().map_err(|err| err.to_string())?;

        // Create a proxy to the BlueZ adapter (typically hci0)
        let proxy = connection.with_proxy(
                connection_parameters.proxy_dest.clone(),
                connection_parameters.proxy_path.clone(),
                Duration::from_millis(5000)
        );

        // Method 1: Get a specific property from the adapter
        info!("Getting BlueZ adapter Bluetooth Device Properties");

        let powered: bool = proxy.get("org.bluez.Adapter1", "Powered")
            .map_err(|err| format!("Failed to get Powered property: {}", err))?;
        println!("Adapter powered: {}", powered);

        let discoverable: bool = proxy.get("org.bluez.Adapter1", "Discoverable")
            .map_err(|err| format!("Failed to get Discoverable property: {}", err))?;
        println!("Adapter discoverable: {}", discoverable);

        // Method 2: Get all properties from the adapter
        // println!("\n=== All Adapter Properties ===");
        // let props: HashMap<String, arg::Variant<Box<dyn arg::RefArg>>> =
        //     proxy.get_all("org.bluez.Adapter1")
        //         .map_err(|err| format!("Failed to get all properties: {}", err))?;
        //
        // for (key, value) in props {
        //         println!("{}: {:?}", key, value);
        // }
        //
        // Method 3: Call a method to start discovery
        println!("\n=== Starting Discovery ===");
        // let found_devices: () = proxy.method_call(connection_parameters.bluetooth_adapter.clone(), "StartDiscovery", (".Name",))
        //     .map_err(|err| format!("Failed to start discovery: {}", err))?;
        let discoverable: bool = proxy.get("org.bluez.Adapter1", "Name")
            .map_err(|err| format!("Failed to get Discoverable property: {}", err))?;
        println!("Discovery started successfully! {discoverable}");

        Ok(())
}