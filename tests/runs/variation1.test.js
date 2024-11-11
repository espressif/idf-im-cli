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
    pathToEim = path.join(os.homedir(), "espressif/eim");
}

logger.debug(`Starting custom installation using EIM on ${pathToEim}`);

const installPath = path.join(os.homedir(), ".espressif2");
const targetList = ["esp32s2"];
const idfVersionList = ["v5.2.3"];
const recursiveSubmodules = true;
const pathToProjectFolder = path.join(os.homedir(), ".espressif2/project");

const pathToIDFScript =
    os.platform() !== "win32"
        ? path.join(installPath, `activate_idf_${idfVersionList[0]}.sh`)
        : path.join(
              installPath,
              idfVersionList[0],
              `Microsoft.PowerShell_profile.ps1`
          );

describe("Installation using custom settings", function () {
    this.timeout(2400000);

    runInstallCustom(
        pathToEim,
        installPath,
        targetList.join(","),
        idfVersionList.join(","),
        recursiveSubmodules
    );

    runPostInstallTest(
        pathToIDFScript,
        pathToProjectFolder,
        targetList[0],
        "esp32c6"
    );
});
