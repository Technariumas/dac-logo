# Draw on oscilloscope using RaspberryPi hardware PWM.

## How to build

Assumes installed Rust and Cargo on the host system.

1. (on RPI) Enable hardware PWM by adding `dtoverlay=pwm-2chan` to `/boot/config.txt`
2. (on host) Install Rust standard library for 'armv7-unknown-linux-gnueabihf' target `rustup target add armv7-unknown-linux-gnueabihf`
3. (on host) Install RPi toolchain by `git clone https://github.com/raspberrypi/tools` (in ~/raspi, for example)
4. (on host) Say to the Cargo where to get the linker - add

\[target.armv7-unknown-linux-gnueabihf\]  
linker = "/home/user/raspi/tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/arm-linux-gnueabihf-gcc"

to the ~/.cargo/config

5. (on host) Build and deploy `cargo build --target=armv7-unknown-linux-gnueabihf && scp -C target/armv7-unknown-linux-gnueabihf/debug/xyrust pi@raspberrypi:`
6. (on RPi) Run `sudo RUST_BACKTRACE=1 ./xyrust data.txt`

Reference: https://www.kernel.org/doc/Documentation/pwm.txt
