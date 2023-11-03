#!/bin/bash

cp ../target/x86_64-linux-android/release/libveronymous_client_jni.so ./app/android-veronymous-client/src/main/jniLibs/x86_64/


cp ../target/i686-linux-android/release/libveronymous_client_jni.so ./app/android-veronymous-client/src/main/jniLibs/x86/


cp ../target/aarch64-linux-android/release/libveronymous_client_jni.so ./app/android-veronymous-client/src/main/jniLibs/arm64-v8a/


cp ../target/arm-linux-androideabi/release/libveronymous_client_jni.so ./app/android-veronymous-client/src/main/jniLibs/armeabi/


cp ../target/armv7-linux-androideabi/release/libveronymous_client_jni.so ./app/android-veronymous-client/src/main/jniLibs/armeabi-v7a/
