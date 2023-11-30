# McUtils

A collection of various utilities for working with minecraft servers.
This project is mostly for use with [debris](https://github.com/Inky-developer/debris), but can also be used for other projects.

## Crates

### rcon
A simple, synchronous minecraft rcon implementation. An example console application can be found [here](https://github.com/Inky-developer/mc_utils/tree/master/examples/rcon.rs)

### server
Functionality that allows to run and configure a server.jar file.
This inlcudes access to the version manifest and downloading arbitrary server versions.

### data_generator
This crate provides functionality to extract data from a minecraft server, including a list of all blocks and blockstates.