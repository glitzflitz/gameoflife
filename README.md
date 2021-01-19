# Game of Life
This is an implementation of Conway's game of life that runs
in x86_64 bare metal freestanding environment.  
This project was featured in [Qemu Advent Calender 2020](https://www.qemu-advent-calendar.org/2020/)
on day 12

## Run the binary
```
git clone https://github.com/glitzflitz/gameoflife
cd gameoflife
chmod +x run.sh
./run.sh
```

## Build
```
git clone https://github.com/glitzflitz/gameoflife
cd gameoflife
cargo install bootimage
cargo install cargo-xbuild
rustup component add rust-src
#Install qemu
cargo xrun
```
