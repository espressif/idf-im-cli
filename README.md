# ESP-IDF Installation Manager - CLI

idf-im-cli for the CLI tool.

## configuration

there are several ways how to configure the installer, you can supply a config file, specify cli arguments, set environmental variables or go through a wizard on the command line.

The precedence is that the config file is overwritten by env variables which can be overridden by cli arguments.
If you go through the wizard, your choices will have the higher precedence.

### file

the installer takes the config toml file. it searches for it in the default location ./config/default.toml but you can specify the path to the config with `--config` cli argument

example config:
```toml
path = "/tmp/esp-new/"
idf_path = "/tmp/esp-new/v5.3/esp-idf"
tool_download_folder_name = "dist"
tool_install_folder_name = "tools"
target = ["all"]
idf_versions = ["v5.3"]
tools_json_file = "tools/tools.json"
idf_tools_path = "./tools/idf_tools.py"
mirror = "https://github.com"
idf_mirror = "https://github.com"
recurse_submodules = false
```

### Env variables

you can override any of the settings by exporting env variable prefixed by `ESP_` like `ESP_TARGET`

### Cli

please refer to the --help for information about cli usage

## How to run

Download the executable for the suitable architecture.

it is recommended first to run the `--help` command 

Run it and proceed according to instructions in the terminal and you will have IDF installed.

for Chinese run it with the flag `-l=cn`
