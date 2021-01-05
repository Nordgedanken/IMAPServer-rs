#!/bin/bash
cargo build || exit $?
sudo target/debug/imap-server server