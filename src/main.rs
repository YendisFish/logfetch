extern crate whoami;
extern crate systemstat;

use std::env;
use systemstat::{System, Platform};


fn main() {
    let mut system = System::new();

    //end info
    println!("{}", whoami::distro());
    println!("======================================================");
    println!("Username: {}", whoami::username());
    println!("Desktop Environment: {}", whoami::desktop_env());
    println!("Device Name: {}", whoami::devicename());
    println!("Battery Life: {}%", &system.battery_life().unwrap().remaining_capacity * 100.0);
    println!("Total Memory: {}", &system.memory().unwrap().total);
    println!("Free Memory: {}", &system.memory().unwrap().free);
    println!("======================================================");
}