
use bluez_async::{BluetoothError, BluetoothSession};
use std::time::Duration;
use log::info;
use tokio::time;
const SCAN_DURATION: Duration = Duration::from_secs(20);


/// Start Bluetooth discovery and pair with the specified device
/// IMPORTANT: Device needs to be paired first
pub async fn start() -> Result<(), BluetoothError>{
        // pretty_env_logger::init();
    // Configure env_logger programmatically
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info) // Set default level
        .parse_default_env() // Still allow RUST_LOG to override this
        .init();

        // Create a new session. This establishes the D-Bus connection to talk to BlueZ. In this case we
        // ignore the join handle, as we don't intend to run indefinitely.
        let (_, session) = BluetoothSession::new().await?;

        // Start scanning for Bluetooth devices, and wait a few seconds for some to be discovered.
        session.start_discovery().await?;
        time::sleep(Duration::from_secs(5)).await;
        session.stop_discovery().await?;

        // Get a list of devices which are currently known.
        let devices = session.get_devices().await?;
        // let device_mac = get_mac();
            info!("Devices:P{}", get_mac());
              // Find the device we care about.
        let device = match devices
            .into_iter()
            .find(|device| device.mac_address.to_string().ends_with("E9:CB:AC"))
            // .find(|device| device.mac_address.to_string().ends_with("06:9A:F5"))
        {
                Some(device) => device,
                None => return Err(BluetoothError::AddressTypeParseError("mac not found".to_string())),
        };


        // Connect to it.
        if let Err(e ) = session.pair_with_timeout(&device.id, SCAN_DURATION).await{
            info!("Bluetooth pair with timeout {:?} : {}", &device.id, &e);
        }
        session.connect(&device.id).await?;
        Ok(())

}
fn get_mac() -> String {
    let arguments: Vec<String> = std::env::args().collect();
    for i in arguments {
        println!("{}", i);
    }

    return "/dev/class/bluetooth/mac".into();


}