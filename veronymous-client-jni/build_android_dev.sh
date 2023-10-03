#!/bin/bash

cross build --features dev-local --target aarch64-linux-android
cross build --features dev-local --target arm-linux-androideabi
cross build --features dev-local --target armv7-linux-androideabi
cross build --features dev-local --target i686-linux-android
cross build --features dev-local --target x86_64-linux-android
#cross build --release --features dev-local --target thumbv7neon-linux-androideabi
