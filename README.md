# Qsnap
> Quickly snapshot and restore Qumulo clusters in AWS.

Qsnap is a tool I've built that allows AWS VPC administrators to easily snapshot and restore Qumulo AWS clusters without having to manually modify infrastructure. Qsnap requires cluster EC2 instances to be turned off before taking a snapshot or restoring a cluster.

## Installation

Binaries need to be built from source right now. In the future qsnap will be uploaded to a package manager or binaries uploaded to a public server. See "Development setup" for more information.

## Usage example

Take a snapshot
```
$> qsnap snapshot -i 10.0.0.1 10.0.0.2 10.0.0.3 10.0.0.4 -d "Taking this snapshot before removing two users."
```

List cluster snapshots taken with this tool

```
$> qsnap list -i 10.0.0.1 10.0.0.2 10.0.0.3 10.0.0.4
+--------------+-----------------------+-------------+
| Created Time | Unique Identifier Tag | Description |
+--------------+-----------------------+-------------+
| UTC Time     |  snapshot-unique-id   |  First snap |
+--------------+-----------------------+-------------+
```

Restore a cluster
```
$> qsnap restore -i 10.0.0.1 10.0.0.2 10.0.0.3 10.0.0.4 -u snapshot-unique-id
```

Get help
```
$> qsnap --help
```

## Development setup
This project was written in and requires Rust 2018.

Build project and download dependencies
```
cargo build
```

## Backlog
* Allow users to select their region (right now qsnap only works in us-west-2)
* Gracefully handle failures. Too many chances for a bad AWS HTTP request to cause a panic.
* Parallelize restoration process by multithreading volume create/attach

## Release History
* 0.1.0
  * Untested, development only release


## Meta

Grant Gumina – [@gum_ina_package](https://twitter.com/gum_ina_package)

[https://github.com/grantgumina/](https://github.com/grantgumina/)

## Contributing

1. Fork it (<https://github.com/yourname/yourproject/fork>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request
