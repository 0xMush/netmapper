# netmapper
 
A fast, async TCP port scanner written in Rust. It supports single ports, port ranges, and multiple port scanning with optional banner grabbing.
 
```
 ____ _____/ |_ _____ _____ ______ ______ ___________
/ \_/ __ \ __\/ \\__ \ \____ \\____ \_/ __ \_ __ \
| | \ ___/| | | Y Y \/ __ \| |_> > |_> > ___/| | \/
|___| /\___ >__| |__|_| (____ / __/| __/ \___ >__|
    \/     \/             \/\/|__|  |__|        \/
```
 
## Features
 
- **Async scanning** — powered by [Tokio](https://tokio.rs/) for high-performance concurrent port scanning
- **Banner grabbing** — reads the first 256 bytes from open ports to identify services
- **Port range scanning** — scan a range like `1-1000` concurrently
- **Single port scanning** — probe a specific port
- **Multiple port scanning** — provide a list of individual ports in one command
- **Timeout handling** — 2-second read timeout prevents hanging on silent services
## Requirements
 
- [Rust](https://www.rust-lang.org/tools/install) (stable, 2021 edition or later)
- Cargo (included with Rust)
## Installation
 
Clone the repository and build with Cargo:
 
```bash
git clone https://github.com/0xMush/netmapper.git
cd netmapper
cargo build --release
```
 
The compiled binary will be located at `./target/release/netmapper`.
 
## Usage
 
```
netmapper <target> -p <ports>
```
 
### Scan a port range
 
```bash
./netmapper example.com -p 1-1000
```
 
### Scan a single port
 
```bash
./netmapper example.com -p 22
```
 
### Scan multiple specific ports
 
```bash
./netmapper example.com -p 20 40 80 22
```
 
### Show help
 
```bash
./netmapper -h
```
 
## Example Output
 
```
Port is Open 22 SSH-2.0-OpenSSH_8.9p1
Port is Open 80
Port is Open 443
```
 
When a service returns a banner on connection, it is displayed alongside the port number.
 
## Dependencies
 
| Crate | Purpose |
|-------|---------|
| [`tokio`](https://crates.io/crates/tokio) | Async runtime for concurrent TCP connections |
 
## Project Structure
 
```
netmapper/
├── src/
│   └── main.rs      # Main entry point and port scanning logic
├── Cargo.toml       # Project manifest and dependencies
├── Cargo.lock       # Dependency lock file
└── .gitignore
```
 
## How It Works
 
1. Parses CLI arguments to get the target IP/hostname and port specification.
2. For **ranges**, spawns a Tokio task per port and runs all concurrently.
3. For **multiple ports**, same concurrent task approach.
4. For a **single port**, connects directly and awaits the result.
5. On a successful connection, attempts to read a banner within a 2-second timeout.
6. Prints open ports (with banner if available) to stdout; closed/filtered ports are silently ignored.
 
## Author
 
[0xMush](https://github.com/0xMush)
