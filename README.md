# CIDR to IPs

## Installation
```sh
# https://doc.rust-lang.org/cargo/getting-started/installation.html
$ cargo install --path .
```

## Usage
```sh
$ echo 192.168.100.0/24 | cidr2ip
192.168.100.0
192.168.100.1
192.168.100.2
192.168.100.3
192.168.100.4
192.168.100.5
192.168.100.6
192.168.100.7
192.168.100.8
192.168.100.9
```

## Usage examples
```sh
$ echo 10.33.112/24 | cidr2ip | ffuf -u https://FUZZ -w -
```
