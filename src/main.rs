pub mod dbus_bluetoothd;
use tokio::main;

#[tokio::main]
async fn main() {


    if let Err(e ) = dbus_bluetoothd::core::connect_bluetooth_dbus().await{
        eprintln!("ceruleus: Error -> {}", e);
    }


}
