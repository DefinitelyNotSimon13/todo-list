#! /bin/bash

diesel database reset

cargo build --release --bin populate_table

/usr/bin/time -v ./target/release/populate_table
