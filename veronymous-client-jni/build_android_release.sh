#!/bin/bash

cross build --release --features production --target aarch64-linux-android
cross build --release --features production --target arm-linux-androideabi
cross build --release --features production --target armv7-linux-androideabi
cross build --release --features production --target i686-linux-android
cross build --release --features production --target x86_64-linux-android
#cross build --release --features production --target thumbv7neon-linux-androideabi
