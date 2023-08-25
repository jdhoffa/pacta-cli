# pacta-cli

This is a simple wrapper around the [PACTA for Investors Docker image](https://ghcr.io/rmi-pacta/workflow.transition.monitor), that allows users to easily run PACTA from the command line. 

## Installation
### Homebrew
On MacOS, you can install this application via homebrew using:
``` bash
brew tap jdhoffa/taps
brew install jdhoffa/taps/pacta-cli
```

## Usage

This repository assumes existing knowledge of running PACTA, and is mainly intended for internal RMI users. 

To use it, you must have:
* A local copy of prepared PACTA input data, with .sqlite prepared datasets
* A local working directory, containing the usual `00_Log_File`, `10_Parameter_File`, etc... directory structure

With these, call:
``` bash
pacta-cli <PORTFOLIO_NAME> <PACTA_DATA_PATH> <WORKING_DIR_PATH>
```
