Installation of IDF
===============================
Installing the ESP-IDF using EIM is straightforward. Simply open your preferred shell (we recommend PowerShell for Windows users) and run EIM, providing your installation preferences using any of the configuration <configuration> methods you choose. If you haven't provided all the necessary options, an interactive wizard will guide you through the remaining steps.

After Installation 
===============================

Windows
-------------------------
On Windows, the installer creates an icon on your desktop labeled IDF_PowerShell. Clicking this icon will launch PowerShell with the environment set up, allowing you to start using ESP-IDF immediately. If you've installed multiple versions of ESP-IDF, you will have multiple icons, one for each version.

macOS & Linux
---------------------------------
In the installation directory you selected, there will be a .sh script that, when sourced, activates the ESP-IDF environment in your current shell. It's important to note that this script should be sourced, not executed directly. If you've installed multiple versions of ESP-IDF, there will be a separate script for each version.

.. note::

    The script should be really sourced and not executed
