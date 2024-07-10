# ESP-IDF Installation Manager - CLI

idf-im-cli for the CLI tool. Please check more details on this project [here](https://gitlab.espressif.cn:6688/idf/idf-im-ui/-/wikis/ESP-IDF-Installation-Manager).

## configuration

there are several ways how to configure the installer, you can supply config file, specify cli arguments, set enviromental variables or go trough wizard on command line.

The precendence is that config file is overwriten by env variables which can be overriden by cli arguments.
If you go trought the wizard, your choices will have the higher precedence.

### file

the installer takes config toml file. it serches for it in the default location ./config/default.toml but you can specify path to the config with `--config` cli argument

### Env variables

you can override any of the settings by exporting env variable prefixed by `ESP_` like `ESP_TARGET`

### Cli

please refer to the --help for information about cli usage

## How to run

Download the executable for the suitable architecture.

it is recomanded to first run the `--help` command 

Run it and proceed according to instruction in the terminal and you will have IDF installed.

for chinese run it with flag `-l=cn`
