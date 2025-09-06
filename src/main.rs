use log::error;

pub mod dbus_bluetoothd;
mod daemon;

#[tokio::main]
async fn main() {


    //read mac from config file

    if let Err(e ) = dbus_bluetoothd::core::start().await{
        error!("{}", e.to_string());
    }


}
