# ESP-IDF Installation Manager (EIM) - CLI

**Important Notice: This CLI tool has been integrated into the ESP-IDF Installation Manager(EIM) repository. While the CLI functionality is now primarily maintained within the GUI application, a dedicated CLI build is also available in the abovementioned repository for those who prefer a pure command-line experience and for headless or text-only environments (like CI). This repository is now deprecated.**

The ESP-IDF Installation Manager (EIM) was a cross-platform CLI application that simplified the setup process for ESP-IDF (Espressif IoT Development Framework). It offered a consistent and user-friendly experience for installing prerequisites, ESP-IDF itself, and essential development tools on macOS, Linux, and Windows.

**For the latest updates and continued development, please refer to the ESP-IDF Installation Manager(EIM) repository:**

[![EIM-UI Repository](https://img.shields.io/badge/GitHub-EIM-red?style=for-the-badge&logo=github)](https://github.com/espressif/idf-im-ui)

**Key Changes:**

* **Integration with EIM:** The CLI functionality has been incorporated into the EIM repository, providing a unified experience for both GUI and CLI users.
* **Dedicated CLI Build:** A pure CLI build is still available in the abovementioned repository for those who prefer a command-line interface.
* **New Version Management:** The updated tooling includes enhanced version management capabilities.
* **Deprecation of this Repository:** This repository is now considered deprecated. Please use the EIM repository for future updates and support.

**Accessing the CLI:**

* The CLI can be accessed through calling the EIM application from terminal ( `eim --help` ).
* A standalone CLI build is available in the repository.

**Original REAME.MD file (for reference):**

# ESP-IDF Installation Manager(EIM) - CLI

EIM is a cross-platform CLI application that simplifies the setup process for ESP-IDF (Espressif IoT Development Framework). Whether you’re working on macOS, Linux, or Windows, EIM offers a consistent and user-friendly experience for installing prerequisites, ESP-IDF itself, and essential development tools.

[![Documentation](https://img.shields.io/badge/documentation-white?style=for-the-badge&logo=readthedocs&logoColor=red)](https://docs.espressif.com/projects/idf-im-cli/en/latest/)

## Configuration

There are several ways how to configure the installer, you can supply a config file, specify cli arguments, set environmental variables or go through a wizard on the command line.

The precedence is that the config file is overwritten by env variables which can be overridden by cli arguments.
If you go through the wizard, your choices will have the higher precedence.

### File

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