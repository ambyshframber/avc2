# avc2

This repository contains 2 things: avc2, the reference implementation of the AVC2 virtual CPU; and the version 1 specification for the CPU. This file is the documentation for avc2, not the specification, however it makes frequent reference to it. The specification can be found in `specification.md`. 

## Invoking avc2

The basic usage is `avc2 ROM`, where ROM is a version 1 AVC2 rom file. See the spec for what this means. No other options or arguments are present.

## Caveats

avc2 does not currently support the entire base spec: the `WAIT`, `STDIN` and `BUFLEN` ports on the system device are non-functional. This will be fixed soon.
