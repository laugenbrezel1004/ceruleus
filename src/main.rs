mod bluetooth;
mod daemon;

fn main() {
    match daemon::birth::run() {
        Ok(_) => println!("nice "),
        Err(e) => println!("ceruleus: Error -> {}", e),
    }
}
