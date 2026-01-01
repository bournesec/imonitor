# imonitor

A lightweight tool for real-time monitoring of network interface traffic.

# build

```
cargo build --release
```

# usage

```
./target/release/imonitor --help
imonitor 0.1.0
Real-time network traffic monitor

USAGE:
    imonitor [FLAGS] [OPTIONS]

FLAGS:
    -h, --help               Prints help information
    -l, --list-interfaces    
    -V, --version            Prints version information

OPTIONS:
    -f, --filter <filter>                      
    -i, --interface <interface>                 [default: eth0]
    -u, --update-interval <update-interval>     [default: 1000]
```

## snapshot

