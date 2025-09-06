pub mod dbus_bluetoothd;
mod daemon;


fn main() {


    //read mac from config file

    if let Err(e ) = dbus_bluetoothd::core::start(){
        eprintln!("ceruleus: Error -> {}", e);
    }


}
