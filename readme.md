# DDathome-rust 

Rust-based client for Project *DD@Home*.

Related repos:
- Cluster manager: https://github.com/dd-center/Cluster-center
- DD@Home-nodejs: https://github.com/dd-center/DDatHome-nodejs
- DD@Electron: https://github.com/dd-center/DDatElectron
- DD@Home-go: https://github.com/dd-center/DDatHome-go
- DD@Home-python: https://github.com/Radekyspec/DDatHome-python

*最近在学rust，于是造个轮子练练手。*

## Usage

### cli
Please download the executable from [github releases](https://github.com/Furffico/ddathome-rust/releases), then run the program in terminal with parameters as follows.
```
OPTIONS:
    -b, --baseurl <BASEURL>      The baseurl of the server [default: wss://cluster.vtbs.moe]
    -h, --help                   Print help information
    -i, --interval <INTERVAL>    The interval(ms) between task execution [default: 500]
    -n, --name <NAME>            The name of the client (for statistics) [default: ]
    -u, --uuid <UUID>            The uuid of this client (for statistics) [default: "random generated"]
    -V, --version                Print version information
```

### Docker

Please create the container with following commands, where `[OPTIONS]` follow the same rule as above:
```bash
$ docker pull furffy/ddathome-rust
$ docker run [DOCKER PARAMETERS] furffy/ddathome-rust [OPTIONS]
```
