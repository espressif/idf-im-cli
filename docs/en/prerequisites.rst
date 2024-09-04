Prerequisites
===============================
Below are the minimum requirements for running the ESP-IDF. The Installation Manager itself has no dependecies but during it's run checks the system if the dependencies of IDF are met.

Windows
----------------

To get started with ESP-IDF, you need Git, CMake, Ninja, and Python. The ESP-IDF Installation Manager will verify the required prerequisites on your system and install any that are missing.

For more details about ESP-IDF prerequisites, please refer to `the ESP-IDF prerequisites documentation <https://docs.espressif.com/projects/esp-idf/en/v4.2.2/esp32/get-started/windows-setup.html>`_.

.. note::

    If any of these prerequisites are missing, the installer will prompt you to install them. If you agree, the installer will automatically install and configure everything required to run ESP-IDF.

MacOS
----------------

- dfu-util
- cmake
- ninja
- python with pip capable of creating virtual environment and doing SSL requests

.. note::
  
    On POSIX systems, the installer will check for the required prerequisites. If they are not met, the installation will not proceed.

Linux
----------------

- git
- cmake
- ninja
- wget
- flex
- bison
- gperf
- ccache
- libffi-dev
- libssl-dev
- dfu-util
- libusb-1.0-0
- python with pip capable of creating virtual environment and doing SSL requests

.. note::
  
    On POSIX systems, the installer will check for the required prerequisites. If they are not met, the installation will not proceed.

