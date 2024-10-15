# Espressif Installation Manager automated tests

## Concepts

The EMI application should have a test structure that would allow validation or customer use cases on the final artifacts. At an initial stage
the tests will be executed manually and using an structure that will allow evolution to be triggered by github actions using local or remote windows and linux runners.

All tests are developed in Node.js using Chain and Mocha as test libraries in combination with Node-PTY for teminal emulation. It is required to install node on the test runner machine.

## Environment Setup

On the test machine, the first step is to copy the testing artifacts. The location of the artifacts can be set using environment variable, or the test will look for the `eim` file in the default location:

Windows: $USERPROFILE\espressif\
Linux/MacOS: $HOME/espressif

### Windows

Install chocolatey package manager:
https://docs.chocolatey.org/en-us/choco/setup/
Run this command with administrator priviledges.
`Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))`

Install Node.js:
https://nodejs.org/en/download/prebuilt-installer/current
`choco install nodejs --version="20.17.0" -y`

Install git:
https://git-scm.com/download/win
`choco install git.install -y`

Clone the test trunk from the public repository:

`git clone -b autotest https://github.com/espressif/idf-im-cli.git`

### Linux:

Install Git and curl and build-essential packages
`sudo apt install -y git curl build-essential`
`curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash`

Start a new terminal (to load nvm)
`nvm install 20`

Clone the test trunk from the public repository:
`git clone -b autotest https://github.com/espressif/idf-im-cli.git`

At his point test for prerequisits can be run, the remaining tests requires the pre-requisits to be installed.
The pre-requisites tests have been commented out of the test scripts.

`sudo apt install git cmake ninja-build wget flex bison gperf ccache libffi-dev libssl-dev dfu-util libusb-dev python3 python3-venv python3-pip`

### MacOS

## Commands summary

Navigate to the idf-im-cli folder, where the repository was cloned.
The scripts should be executed passing as arguments the path to the `eim` application and the version of the file being tested.

#### Windows

Open Powershell, and enable script execution:
`Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass`

To execute tests on windows, use the script
`.\tests\run_test.ps1 "<PATH TO EIM.EXE>" "<Version being tested>"`

Default arguments are:
`.\tests\run_test.ps1 "$env:USERPROFILE\espressif\eim.exe" "idf-im-cli 0.1.0"`

#### Linux

(if needed) Give execution permission to the test script
`chmod +x ./tests/run_test.sh`

To execute tests on linux, use the script:
`. ./tests/run_test.sh "<PATH TO EIM>" "<Version being tested>"`

Default arguments are:
`. ./tests/run_test.sh "$HOME/espressif/eim" "idf-im-cli 0.1.0"`

#### MacOS

To executing testins in MacOS, use the script:

<TODO>

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

`./eim -p ~/.espressif -t all -i v5.3.1 --tool-download-folder-name dist --tool-install-folder-name tools --idf-tools-path ./tools/idf_tools.py --tools-json-file tools/tools.json -m https://github.com --idf-mirror https://github.com -r true`

`./eim -c config.toml`

`./eim --log-file InstManager.log`

## References

Packages required by EIM:

Windows: eim should be able to perform all requirements installation

Linux: sudo apt install git cmake ninja-build wget flex bison gperf ccache libffi-dev libssl-dev dfu-util libusb-dev python3 python3-venv python3-pip

MacOS:
Install homebrew and load the application to the terminal profile

`/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`

Then run: brew install dfu-util cmake ninja python3

## Running the tests manually

Follow the instructions below to setup a test machine for development, debug, or run the tests manually.
Node and NPM should be installed in the machine before any test script is able to execute. Follow the instrucitons provided above on the machine setup to get node installed.

Clone the repository to a desired location, recommended to be on the user home folder.
The test will look for the default location for both the installation manager and later for the location of IDF. If no environmental variable is provided the default location will be uses.

### Windows

### Linux

Before installing the pre-requisites, it is possible to run the rerequisites test, to verify eim is able to identify the missing packages.

Copy the eim installation manager application to ~/espressif

Navigate to the folder where the repository was cloned /tests/cripts
execute the command

`export EIM_FILE_PATH="/home/<USER>/espressif/eim"` If using the default folder this line can be skipped
`export EIM_VERSION="idf-im-cli 0.1.0"` If using the default version this line can be skipped
`export LOG_TO_FILE="true"`
`export DEBUG="true"` Thsi line is only needed if debug messages are needed
`npm run pre-test`

Running this test will show that the test scripts is able to execute and if passing it indicated eim could identfy the missing packages.

Install the pre-requisites manually and then run:

`npm run test`

This test will install IDF in the default location, check that IDF is installed at USERHOME/.espressif
There should be an activation script for each version of IDF installed. The default installation should automatically install the latest version of IDF.

### Node commands

If entering the node interactive terminal, the test class can be imported and executed as follows:

export IDF_SCRIPT="C:\esp\v5.3.1\Microsoft.PowerShell_profile.ps1"
export DEBUG="true"
export LOG_TO_FILE="true"
node
const {IDFTestRunner} = await import("./IDFTestRunner.class.js")
const terminal = new IDFTestRunner(process.env.IDF_SCRIPT)
terminal.startTerminal()
