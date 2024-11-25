import { describe, it, before, after } from "mocha";
import { runPostInstallTest } from "../script/postInstall.test.js";
import { runInstallNonInteractive } from "../script/installNonInteractive.test.js";
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

logger.debug(`Starting installation using mirror jihulab and dl.espressif.cn`);

const targetList = ["esp32"]; // targets used for IDF installation
const idfVersionList = ["v5.0.7"]; // IDF versions to be installed
const installFolder = ".espressif5";
const projectFolder = "project";

let installArgs = [];
installArgs.push(` -p ${path.join(os.homedir(), installFolder)}`); // Install Path
installArgs.push(` -t ${targetList.join(",")}`); // Targets
installArgs.push(` -i ${idfVersionList.join(",")}`); // IDF versions
installArgs.push(` -m https://dl.espressif.cn/github_assets`); // IDF tools mirror
installArgs.push(` --idf-mirror https://jihulab.com/esp-mirror`); // ESP-IDF mirror
installArgs.push(` -r false`); // recursive submodules init

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

describe("using mirror jihulab and dl.espressif.cn", function () {
    this.timeout(2400000);

    runInstallNonInteractive(pathToEim, installArgs);

    runPostInstallTest(
        pathToIDFScript,
        path.join(os.homedir(), installFolder, projectFolder),
        targetList[0],
        "esp32c6"
    );
});
