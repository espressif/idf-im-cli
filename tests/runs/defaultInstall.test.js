import { describe, it, before, after } from "mocha";
import { runArgumentsTests } from "../script/commandLineArguments.test.js";
import { runInstallWizardTests } from "../script/installWizard.test.js";
import { runPostInstallTest } from "../script/postInstall.test.js";
import logger from "../classes/logger.class.js";
import os from "os";
import path from "path";

/**
 * Setup the following environmental variables to execute this test:
 *
 * EIM_FILE_PATH to point to the eim application.
 * EIM_VERSION to specify expected version to be printed by the application.
 *
 * use:
 * Windows: $env:<variable>="<value>"
 * Linux/mac: export <variable>="<value>"
 *
 */

let pathToEim;
let eimVersion;

if (process.env.EIM_FILE_PATH) {
    pathToEim = process.env.EIM_FILE_PATH;
} else {
    pathToEim = path.join(os.homedir(), "espressif/eim");
}

if (process.env.EIM_VERSION) {
    eimVersion = process.env.EIM_VERSION;
} else {
    eimVersion = "idf-im-cli 0.1.3";
}

const pathToIDFScript =
    os.platform() !== "win32"
        ? path.join(os.homedir(), ".espressif/activate_idf_v5.3.1.sh")
        : "C:\\esp\\v5.3.1\\Microsoft.PowerShell_profile.ps1";

const pathToProjectFolder =
    os.platform() !== "win32"
        ? path.join(os.homedir(), ".espressif/project")
        : "C:\\esp\\project";

describe("Installation using default settings", function () {
    this.timeout(2400000);

    runArgumentsTests(pathToEim, eimVersion);

    runInstallWizardTests(pathToEim);

    runPostInstallTest(pathToIDFScript, pathToProjectFolder);
});

logger.info("Completed Default Installation test");
