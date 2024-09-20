# EMI-Test

This repository contains the test structure for the ESP Installation Manager.  
It is intended ofr temporary use only to develop a proff of concept, once functional this data will be transferred to the project repository.


## Concepts

The EMI application should have a test structure that would allow validation or customer use cases on the final artifacts. At an initial stage
the tests will be executed manually and using an structure that will allow evolution to be triggered by github actions using local or remote windows and linux runners.

All tests are developed in Node.js using Chain and Mocha as test libraries in combination with Node-PTY for teminal emulation. It is required to install node on the test runner machine.


## Environment Setup

On the test machine, the first step is to copy the testing artifacts. The location of the artifacts can be set using environment variable, or the test will look for the `eim` file in the default location:

Windows: C:\espressif\
Linux/MacOS: ~/espressif

Make sure Node version 14 or higher and Git are installed.

### Windows

Install Node.js:
https://nodejs.org/en/download/prebuilt-installer/current

Using fnm package manager:
`winget install Schniz.fnm`
`fnm env --use-on-cd | Out-String | Invoke-Expression`
`fnm use --install-if-missing 22`


VSCode build tools required to build node-pty module


### Linux:

Install Git and curl and python 2.7
`sudo apt install -y git curl python2.7`
`curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash`
Start a new terminal (to load nvm)
`nvm install 20`

Clone the test trunk from the public repository:

`git clone -b autotest https://github.com/espressif/idf-im-cli.git TestSetup`


### MacOS




## Commands summary

Navigate to the TestSetup folder, where the repository was cloned.

#### Windows

To execute tests on windows, use the script
`.\run_test.bat`

#### Linux
To execute tests on linux, use the script:

(if needed) give execution permission
`chmod +x run_test.sh`

`./run_test.sh`

>       cd TestSetup
        npm ci
        npm test


#### MacOS

To executing testins in MacOS, use the script:










# Installation Manager Usage

##Application arguments
```
Options:
  -p, --path <PATH>
          Base Path to which all the files and folder will be installed
  -c, --config <FILE>
  -t, --target <TARGET>
          You can provide multiple targets separated by comma
  -i, --idf-versions <IDF_VERSIONS>
          you can provide multiple versions of ESP-IDF separated by comma
      --tool-download-folder-name <TOOL_DOWNLOAD_FOLDER_NAME>
      --tool-install-folder-name <TOOL_INSTALL_FOLDER_NAME>
      --idf-tools-path <IDF_TOOLS_PATH>
          Path to tools.json file relative from ESP-IDF installation folder
      --tools-json-file <TOOLS_JSON_FILE>
          Path to idf_tools.py file relative from ESP-IDF installation folder
  -n, --non-interactive <NON_INTERACTIVE>
          [possible values: true, false]
  -m, --mirror <MIRROR>
          url for download mirror to use instead of github.com
      --idf-mirror <IDF_MIRROR>
          url for download mirror to use instead of github.com for downloading esp-idf
  -v, --verbose...
          Increase verbosity level (can be used multiple times)
  -l, --locale <LOCALE>
          Set the language for the wizard (en, cn)
      --log-file <LOG_FILE>
          file in which logs will be stored (default: eim.log)
  -r, --recurse-submodules <RECURSE_SUBMODULES>
          Should the installer recurse into submodules of the ESP-IDF repository (derfault true) 
          [possible values: true, false]
  -h, --help
          Print help (see a summary with '-h')
  -V, --version
          Print version
```

## Example config file:

file config.toml (Linux)
```
path = "/home/virtual/.esp"
idf_path = "/home/virtual/.esp/v5.3.1/esp-idf"
tool_download_folder_name = "dist"
tool_install_folder_name = "tools"
target = ["all"]
idf_versions = ["v5.3.1"]
tools_json_file = "tools/tools.json"
idf_tools_path = "./tools/idf_tools.py"
mirror = "https://github.com"
idf_mirror = "https://github.com"
```

file config.toml (Windows)
```
path = 'C:\esp\'
idf_path = 'C:\esp\v5.3.1\esp-idf'
tool_download_folder_name = "dist"
tool_install_folder_name = "tools"
target = ["all"]
idf_versions = ["v5.3.1"]
tools_json_file = "tools/tools.json"
idf_tools_path = "./tools/idf_tools.py"
mirror = "https://github.com"
idf_mirror = "https://github.com"
```

## Full arguments

#### Windows:
`.\eim.exe -p c:\espressif -t all -i v5.3.1 --tool-download-folder-name dist --tool-install-folder-name tools --idf-tools-path ./tools/idf_tools.py --tools-json-file tools/tools.json -m https://github.com --idf-mirror https://github.com -r true`

`.\eim.exe -c config.toml`

`.\eim.exe --log-file InstManager.log`


#### Linux & MacOS

`./eim -p ~/.espressif -t all -i v5.3.1 --tool-download-folder-name dist --tool-install-folder-name tools --idf-tools-path ./tools/idf_tools.py --tools-json-file tools/tools.json -m https://github.com --idf-mirror https://github.com`

`./eim -c config.toml`

`./eim --log-file InstManager.log`


## References

Packages required by EIM:

Windows: eim shoudl be able to perform all requirements installation

Linux:

sudo apt install git cmake ninja-build wget flex bison gperf ccache libffi-dev libssl-dev dfu-util libusb-dev python3 python3-venv python3-pip

MacOS:

Install homebrew and load the application to the terminal profile

`/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`

Then run:

brew install dfu-util cmake ninja python3

