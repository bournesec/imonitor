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

* imonitor - list interfaces:

<img width="730" height="392" alt="image" src="https://github.com/user-attachments/assets/9497ee18-bcb0-43e6-93ff-dff0094c61b5" />

* imonitor - statistic interface traffic:

<img width="628" height="448" alt="image" src="https://github.com/user-attachments/assets/d2bd9111-b718-45c4-80d9-a8bac7f13f94" />

* imonitor - statistic interface  traffic with filter:

<img width="675" height="505" alt="image" src="https://github.com/user-attachments/assets/960af5c3-a268-412e-b378-0502ab3b50b3" />

