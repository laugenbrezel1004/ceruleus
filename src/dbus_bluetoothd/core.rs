use zbus::{Connection, Result, proxy};

#[proxy(
        interface = "org.bluez.Adapter1",
        default_service = "org.zbus.MyGreeter",
        default_path = "/org/zbus/MyGreeter"
)]
trait MyGreeter {
        async fn say_hello(&self, name: &str) -> Result<String>;
}

// Although we use `tokio` here, you can use any async runtime of choice.
pub async fn connect_bluetooth_dbus() -> Result<()> {
        let connection = Connection::session().await?;

        // `proxy` macro creates `MyGreaterProxy` based on `MyGreeter` trait.
        let proxy = MyGreeterProxy::new(&connection).await?;
        let reply = proxy.say_hello("Maria").await?;
        println!("{reply}");

        Ok(())
}
// pub fn connect_bluetooth_bus() -> Result<(), String > {
//     // First open up a connection to the system  bus.
//     let connection_system_bus = Connection::new_session().map_err(|e| format!("Error creating dbus session: {}", e))?;
//
//     // Second, create a wrapper struct around the connection that makes it easy
//     // to send method calls to a specific destination and path.
//     //TODO: Work later on with variables from ex tern to avoid recompiling when bluetoothd changes
//     let proxy = connection_system_bus.with_proxy("org.bluez", "/org/bluez/hci0", Duration::from_millis(5000));
//
//     // Now make the method call. The ListNames method call takes zero input parameters and
//     // one output parameter which is an array of strings.
//     // Therefore the input is a zero tuple "()", and the output is a single tuple "(names,)".
//
//    // let temp  = proxy.introspect().unwrap();
//     let name = proxy.introspect().map_err(|e| format!("Error getting bluetooth name: {}", e))?;
//     // let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus.Introspectable.", "Name", ()).map_err(|e| format!("Error listing names: {}", e))?;
//     println!("{:?}", name);
//
//     // for name in names { println!("{}", name); }
//
//     // Let's print all the names to stdout.
//
//     Ok(())
// }
