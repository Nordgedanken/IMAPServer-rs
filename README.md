[![Build Status](https://travis-ci.org/Nordgedanken/IMAPServer-rs.svg?branch=master)](https://travis-ci.org/Nordgedanken/IMAPServer-rs) [![Build status](https://ci.appveyor.com/api/projects/status/ao4boyu11mhnr7rp/branch/master?svg=true)](https://ci.appveyor.com/project/MTRNord/imapserver-rs/branch/master) [![codecov](https://codecov.io/gh/Nordgedanken/IMAPServer-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/Nordgedanken/IMAPServer-rs)
# IMAPServer-rs
A Basic IMAP Server written in Rust (WIP)

Join the discussion at https://matrix.to/#/#IMAPServer-rs:matrix.ffslfl.net !

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites

What things you need to install the software and how to install them

1. You need to install cargo. Use [rustup](https://www.rustup.rs) to install it
2. You need a installation of [MYSQL](https://www.mysql.com) or [MariaDB](https://mariadb.org)

### Installing

A step by step series of examples that tell you have to get the IMAP server running

Clone the repository

```
git clone https://github.com/Nordgedanken/IMAPServer-rs.git
```

Build the binary

```
cd IMAPServer-rs
cargo build
```

Open the needed Ports

```
143
```

## Running the tests

After cloning this repository Cargo has a simple test command. You can simply use

```
cargo test --release
```

## Built With

* [Rust](https://rust-lang.org) - The web framework used
* [Mysql](https://www.mysql.com) - Database

<!--## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of conduct, and the process for submitting pull requests to us.
-->
## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/your/project/tags). 

## Authors

* **MTRNord** - *Initial work* - [MTRNord](https://github.com/MTRNord)

See also the list of [contributors](https://github.com/Nordgedanken/IMAPServer-rs/contributors) who participated in this project.

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE.md](LICENSE.md) file for details

<!--## Acknowledgments

* Hat tip to anyone who's code was used
* Inspiration
* etc
-->
