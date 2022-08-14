use std::io::Read;
use virtual_interface::tap::VirtualInterface;

fn main() {
    let mut interface = VirtualInterface::create("dev0").unwrap();
    let mut buffer = [0; 4096];
    println!("starting read loop for device `dev0`");
    loop {
        let n = interface.device().read(&mut buffer[..]).unwrap();
        println!("The bytes: {:?}", &buffer[..n]);
    }
}
