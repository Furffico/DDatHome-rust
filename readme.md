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
### Executable

Download and run the executable binary for your platform from [github releases](https://github.com/Furffico/ddathome-rust/releases).

### Docker

Of course you should have docker engine preinstalled on your device, otherwise please use the method above.

Create the container with the following commands:
```bash
$ docker pull furffy/ddathome-rust
$ docker run [DOCKER PARAMETERS] furffy/ddathome-rust [OPTIONS]
```

## Configuration

Priority: **defaults** < **ENV** < **config file** < **cli parameters**

| cli | env | json key | default | description | 
|----|-----|-----|----|----|
| `-b <baseurl>` | `BASEURL` | `"baseurl"` | `"wss://cluster.vtbs.moe"` | The baseurl of the server |
| `-n <name>` | `NAME` | `"name"` | `""` | The name of the client |
| `-u <uuid>` | `UUID` | `"uuid"` | (randomly generated) | The uuid of this client |
| `-i <interval>` | `INTERVAL` | `"interval"`| `500` | The interval(ms) between task execution |
| `-r <retry>` | `RETRY` | `"retry"` | `5` | Count of retries on connection failure.<br> Set to 0 for infinite retries. |
| `-c <config>` | `CONFIG` | -- | -- | Path to config file.<br>If the file does not exist, one with default config will be created. |
| `-h` | -- | -- | -- | Print help information |
| `-v` | -- | -- | -- | Print version information |

This is an example for cli options:
```bash
$ ./ddathome-rust -n ddknight -u 65469de3-31c8-4f0d-b85c-8ea8c9422619 -i 1000
```

This is an example for config file (config.json):
```json
{
    "name": "ddknight",
    "baseurl": "wss://cluster.vtbs.moe",
    "uuid": "c5a14755-137e-4dc9-8e94-b01c03f1e7a4",
    "interval": 500,
    "retry": 5
}
```

## To-dos

- [ ] refactor with [tokio](https://tokio.rs/) to support asyncio.
- [ ] 仿照 [nodejs版本](https://github.com/dd-center/DDatHome-nodejs) 写出弹幕转发功能（咕咕咕）。
- [ ] 根据学的时候看到的更好的写法对代码进行优化。
