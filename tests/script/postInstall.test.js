import { expect } from "chai";
import { describe, it, before, after, beforeEach, afterEach } from "mocha";
import { IDFTestRunner } from "./IDFTestRunner.class.js";
import logger from "./logger.class.js";
import os from "os";
import path from "path";

let pathToIDFScript;

if (process.env.IDF_SCRIPT) {
    pathToIDFScript = process.env.IDF_SCRIPT;
} else {
    pathToIDFScript =
        os.platform() !== "win32"
            ? path.join(os.homedir(), ".espressif/activate_idf_v5.3.1.sh")
            : "C:\\esp\\v5.3.1\\Microsoft.PowerShell_profile.ps1";
}

// export function runPostInstallTests() {
describe("Check if IDF installation is functional", function () {
    let testRunner;

    before(async function () {
        this.timeout(5000); // Increase timeout for setup
        testRunner = new IDFTestRunner(pathToIDFScript);
        try {
            await testRunner.startTerminal();
        } catch (error) {
            logger.debug("Error starting process:", error);
            throw error;
        }
    });

    after(async function () {
        this.timeout(10000);
        if (testRunner) {
            await testRunner.stop();
        }
        testRunner = null;
    });

    it("Should create a new project based on a template", async function () {
        /**
         * This test should attempt to create a copy of the Hello World Project into the ~/esp folder
         * The commads might differ for each operating system, Starting with Linux
         * The assert is based on the existance of the project files in the expected folder.
         */
        testRunner.sendInput("mkdir ~/esp\r");
        testRunner.sendInput("cd ~/esp\r");
        testRunner.sendInput(
            `cp -r ${process.env.IDF_PATH}/examples/get-started/hello_world .\r`
        );
        testRunner.sendInput("cd hello_world\r");
        testRunner.sendInput("ls\r");
        const confirmFilesCopied = await testRunner.waitForOutput(
            "pytest_hello_world.py"
        );
        if (!confirmFilesCopied) {
            logger.info(testRunner.output);
        }
        expect(confirmFilesCopied).to.be.true;
        expect(testRunner.output).to.include("sdkconfig.ci");
        expect(testRunner.output).to.include("main");
        expect(testRunner.exited).to.not.be.true;
    });

    //     it("Should build the project for the default target ESP32", async function () {

    //      });
});
// }
