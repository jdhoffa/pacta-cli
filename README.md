# pacta-cli

This is a simple wrapper around the [PACTA for Investors Docker image](https://ghcr.io/rmi-pacta/workflow.transition.monitor), that allows users to easily run PACTA from the command line. 

## Usage

This repository assumes existing knowledge of running PACTA, and is mainly intended for internal RMI users. 

To use it, you must have:
* A local copy of prepared PACTA input data, with .sqlite prepared datasets
* A local working directory, containing the usual `00_Log_File`, `10_Parameter_File`, etc... directory structure
* A working installation of Rust and Cargo

With these, simply clone the repo can call:
``` bash
cargo run -- <PORTFOLIO_NAME> <PACTA_DATA_PATH> <WORKING_DIR_PATH>
```
