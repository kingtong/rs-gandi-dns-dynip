
# gandi-dns-dynip

Little tool to set an `A` record on a domain registered with Gandi.

The public IP set for the record is determined with [https://www.icanhazip.com/]() or by setting it manually from 
command line or configuration file.

## Installing

### Binary
**TODO**

### Cargo
You can build from source using `cargo` and crates.io. If you do not have a Rust compiler installed, go to
[rustup.rs](https://rustup.rs) to get one. Then you can run `cargo install rs-gandi-dns-dynip` and it will
be downloaded from crates.io and then built.

## Usage

```
USAGE:
    rs-gandi-dns-dynip [OPTIONS]

OPTIONS:
        --api-key <API_KEY>    [default: ]
        --config <CONFIG>
        --domain <DOMAIN>      [default: ]
    -h, --help                 Print help information
        --ip <IP>
        --record <RECORD>      [default: ]
    -V, --version              Print version information
```

## Configuration

The configuration file is a basic JSON file with the following structure:

```json
{
    "api_key": "gandi_api_key",
    "domain": "domain_name",
    "record": "domain_record_name",
    "ip": "ip (optional)"
}
```

## Reference
* https://www.gandi.net/
* https://api.gandi.net/docs/
* https://www.icanhazip.com/
