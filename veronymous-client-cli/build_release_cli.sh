#!/bin/bash

cross build --release --features production --target x86_64-unknown-linux-gnu
cross build --release --features production --target i686-unknown-linux-gnu
cross build --release --features production --target aarch64-unknown-linux-gnu
cross build --release --features production --target armv7-unknown-linux-gnueabihf
