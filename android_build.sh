#!/bin/sh
min_ver=22
cargo ndk --target aarch64-linux-android -- build --release
cargo ndk --target armv7-linux-androideabi -- build --release
cargo ndk --target i686-linux-android -- build --release
cargo ndk --target x86_64-linux-android -- build --release



jniLibs=../../tictactoe/android/app/src/main/jniLibs
libName=librustylib.so

rm -rf ${jniLibs}

mkdir ${jniLibs}
mkdir ${jniLibs}/arm64-v8a
mkdir ${jniLibs}/armeabi-v7a
mkdir ${jniLibs}/x86
mkdir ${jniLibs}/x86_64

cp target/aarch64-linux-android/release/${libName} ${jniLibs}/arm64-v8a/${libName}
cp target/armv7-linux-androideabi/release/${libName} ${jniLibs}/armeabi-v7a/${libName}
cp target/i686-linux-android/release/${libName} ${jniLibs}/x86/${libName}
cp target/x86_64-linux-android/release/${libName} ${jniLibs}/x86_64/${libName}