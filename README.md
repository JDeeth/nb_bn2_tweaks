# Nimbus BN-2 Islander Tweaks

This is a strictly unofficial Rust plugin for the Nimbus BN-2 Islander for X-Plane 11/12.

https://store.x-plane.org/BN-2B-Islander_p_1243.html

## Installation (Windows)

Copy the `nimbus-bn2-tweaks` folder from `compiled` to your Islander `plugins` folder

To confirm it's running, load the aircraft and open Plugins > Show Plugin Admin.
The compilation date will be shown under Information > Nimbus BN2 Tweaks

## Build instructions

```
cargo build --release
```
Rename the `nb-bn2-tweaks.dll`/`.so`/`dylib` to `.xpl`

Place it in your DR400 plugins folder [as per the X-Plane SDK](https://developer.x-plane.com/article/building-and-installing-plugins/)

The `release.bat` file automates these steps (for my setup - customise as required!)
