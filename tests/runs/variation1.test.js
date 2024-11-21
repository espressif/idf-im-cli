import { describe, it, before, after } from "mocha";
import { runInstallCustom } from "../script/installCustom.test.js";
import { runPostInstallTest } from "../script/postInstall.test.js";
import logger from "../classes/logger.class.js";
import os from "os";
import path from "path";

/**
 * Setup the following environmental variables to execute this test:
 *
 * EIM_FILE_PATH to point to the eim application.
 *
 * use:
 * Windows: $env:<variable>="<value>"
 * Linux/mac: export <variable>="<value>"
 *
 */

let pathToEim;

if (process.env.EIM_FILE_PATH) {
    pathToEim = process.env.EIM_FILE_PATH;
} else {
    pathToEim = path.join(os.homedir(), "eim-cli/eim");
}

logger.debug(`Starting custom installation using EIM on ${pathToEim}`);

const targetList = ["esp32s2"]; // targets used for IDF installation
const idfVersionList = ["v5.2.3"]; // IDF versions to be installed
const installFolder = ".espressif2";
const projectFolder = "project";

let installArgs = [];
installArgs.push(` -p ${path.join(os.homedir(), installFolder)}`); // Install Path
installArgs.push(` -t ${targetList.join(",")}`); // Targets (in case of multiple separate with ,)
installArgs.push(` -i ${idfVersionList.join(",")}`); // IDF versions (in case of multiple separate with ,)
installArgs.push(` -m https://github.com`); // IDF tools mirror
installArgs.push(` --idf-mirror https://github.com`); // ESP-IDF mirror
installArgs.push(` -r true`); // recursive submodules init

const pathToIDFScript =
    os.platform() !== "win32"
        ? path.join(
              os.homedir(),
              installFolder,
              `activate_idf_${idfVersionList[0]}.sh`
          )
        : path.join(
              os.homedir(),
              installFolder,
              idfVersionList[0],
              `Microsoft.PowerShell_profile.ps1`
          );

describe("Installation using custom settings", function () {
    this.timeout(2400000);

    runInstallCustom(pathToEim, installArgs);

    runPostInstallTest(
        pathToIDFScript,
        path.join(os.homedir(), installFolder, projectFolder),
        targetList[0],
        "esp32c6"
    );
});
