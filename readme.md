# avc2

This repository contains 2 things: avc2, the reference implementation of the AVC2 Virtual CPU; and the version 1 specification for the CPU. This file is the documentation for avc2, not the specification, however it makes frequent reference to it. The specification can be found in `specification.md`. 

## Invoking avc2

The basic usage is `avc2 [OPTIONS] ROM`, where ROM is a version 1 AVC2 rom file. See the spec for what this means.

The only available options are `-h/--help`, which prints a short help page, `-V/--version`, which prints the version information, and `-d DEVICE`, which specifies a non-system device. The format is `location;id;extradata`, where `location` is which of the 16 device slots to place it in (cannot be 0 as it is occupied by system) and `id` is the device id (see the specification). For example, to mount the drive in `./test.avd` to the device page, starting at 0xffa0, you would use `-d 10;2;test.avd`.
