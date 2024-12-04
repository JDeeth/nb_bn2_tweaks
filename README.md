# Nimbus BN-2 Islander Tweaks

This is a strictly unofficial Rust plugin for the Nimbus BN-2 Islander for X-Plane 11/12.

https://store.x-plane.org/BN-2B-Islander_p_1243.html

It provides additional commands to facilitate hardware integration:

![Initial flap and magneto commands](img/initial_commands.png)

## Implemented

- Flaps
- Magnetos
- Fuel pumps

Switches have On, Off, and Toggle commands

The flaps switch is spring-loaded. Hold down the command to hold down the switch.

### Todo
- All the custom light switches
- Pitot heat controls
- Ice protection controls
- Starter isolation switch
- Mac/Linux binary if there's demand for it

## Installation (Windows)

Download the latest [Release](https://github.com/JDeeth/nb_bn2_tweaks/releases)

Copy the `nimbus-bn2-tweaks` folder into the Islander's `Plugins` folder

To confirm the plugin is running, load the aircraft and open Plugins > Show Plugin Admin.
The compilation date will be shown under Information > Nimbus BN2 Tweaks and the additional
commands should be visible in the control config.

## Build instructions

To build from source (and for Mac/Linux), clone this repo on a computer with Rust installed:

```
cargo build --release
```
Rename the `nb-bn2-tweaks.dll`/`.so`/`dylib` to `.xpl`

Place it in your DR400 plugins folder [as per the X-Plane SDK](https://developer.x-plane.com/article/building-and-installing-plugins/)

The `release.bat` file automates these steps (for my setup - customise as required!)

## Future

It should be possible to replace this with a generic "Commands to datarefs"
plugin which would be configured with a text file and provide standardised sets
of commands for pushbuttons, toggle switches, and multiposition switches.