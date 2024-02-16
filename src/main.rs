use std::env;
use std::net::IpAddr;
use std::str::FromStr;

struct Arguments{
    flag:String,
    ipadder:IpAddr,
    thread:u16
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
}
