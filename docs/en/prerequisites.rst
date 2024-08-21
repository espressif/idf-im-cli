Prerequisites
===============================
Below are the minimum requirements for running the ESP-IDF Installation Manager.

Windows
----------------

- git
- cmake
- ninja
- python with pip capable of creating virtual environment and doing SSL requests

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

