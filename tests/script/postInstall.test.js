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
logger.debug(`Path to script = ${pathToIDFScript}`)

// export function runPostInstallTests() {
describe("Check if IDF installation is functional", function () {
    let testRunner;

    before(async function () {
        this.timeout(5000);
        testRunner = new IDFTestRunner(pathToIDFScript);
        try {
            await testRunner.startTerminal();
        } catch (error) {
            logger.debug("Error starting process:", error);
            throw error;
        }
        logger.debug(`Terminal started for IDF PATH ${process.env.IDF_PATH}`)
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
         * The commads might differ for each operating system.
         * The assert is based on the existance of the project files in the expected folder.
         */
        testRunner.sendInput(`mkdir ${os.homedir()}/esp\r`);
        testRunner.sendInput(`cd ${os.homedir()}/esp\r`);

        testRunner.sendInput(
                os.platform() !== "win32"
                    ? `cp -r ${process.env.IDF_PATH}/examples/get-started/hello_world .\r`
                    : `xcopy /e /i $env:IDF_PATH\\examples\\get-started\\hello_world hello_world\r`
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

    it("Should set the target to ESP32", async function () {
        /**
         * This test attempts to set a target MCU for the project created in the previous test.
         */
        this.timeout(18000);
        testRunner.sendInput("idf.py set-target esp32\r")

        const targetSet = await testRunner.waitForOutput(
            "Build files have been written to", 15000
        );
        if (!targetSet) {
            logger.info(testRunner.output);
        }
        expect(targetSet).to.be.true;
        expect(testRunner.output).to.include("Configuring done");
        expect(testRunner.output).to.include("Generating done");
        expect(testRunner.exited).to.not.be.true;
    });

    it("Should build project for the selected target", async function () {
        /**
         * This test attempts to build artifacts for the project and targets selected above.
         * The test is successfull if the succss message is printed in the terminal.
         */
        this.timeout(43000);
        testRunner.sendInput("idf.py build\r")

        const buildComplete = await testRunner.waitForOutput(
            "Project build complete", 40000
        );
        if (!buildComplete) {
            logger.info(testRunner.output);
        }
        expect(buildComplete).to.be.true;
        expect(testRunner.output).to.include("Successfully created esp32 image");
        expect(testRunner.exited).to.not.be.true;
    });

});
// }
