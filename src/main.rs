use std::io;
use cidr_utils::cidr::IpCidr;

#[allow(unused_must_use)]
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut cidr_input = String::new();
    stdin.read_line(&mut cidr_input);

    let cidr = IpCidr::from_str(&cidr_input.trim_end());

    if let Err(e) = cidr {
        println!("Error: {}", e);
        std::process::exit(1);
    }

    for ip in cidr.unwrap().iter_as_ip_addr() {
        println!("{}", ip);
    }

    Ok(())
}
