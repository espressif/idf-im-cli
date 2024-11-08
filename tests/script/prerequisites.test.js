import { expect } from "chai";
import { describe, it, before, after, beforeEach, afterEach } from "mocha";
import { InteractiveCLITestRunner } from "../classes/CLITestRunner.class.js";
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

describe("Check if prerequisites are installed", function () {
    this.timeout(600000);
    let testRunner;

    beforeEach(async function () {
        this.timeout(5000); // Increase timeout for setup
        testRunner = new InteractiveCLITestRunner();
        try {
            await testRunner.runTerminal();
            testRunner.sendInput(`${pathToEim}\r`);
        } catch (error) {
            logger.debug("Error starting process:", error);
            throw error;
        }
    });

    afterEach(async function () {
        this.timeout(10000);
        if (!testRunner.exited) {
            await testRunner.stop();
        }
        testRunner = null;
    });

    /** Linux/MAC Specific Tests
     * Tests below will only be executed on Unix Based systems
     */

    // The following test can only be executed if the prerequisites have not been installed in the OS.
    (os.platform() !== "win32" ? describe : describe.skip)(
        "Pre-Requisites test on non windows platform",
        function () {
            afterEach(async function () {
                this.timeout(10000);
                if (this.currentTest.state === "failed") {
                    logger.info(
                        `Terminal output on failure: >>>>>>>>>>>>>>>\r ${testRunner.output}`
                    );
                }
            });

            it("Should detect missing requirements", async function () {
                this.timeout(20000);
                const missingRequisites = await testRunner.waitForOutput(
                    "Error: Please install the missing prerequisites",
                    20000
                );
                expect(missingRequisites).to.be.true;
            });
        }
    );

    /** Windows Specific Tests
     * Tests below will only be executed on win32 platform
     */

    (os.platform() === "win32" ? describe : describe.skip)(
        "Pre-requisites test on Windows",
        function () {
            afterEach(async function () {
                this.timeout(10000);
                if (this.currentTest.state === "failed") {
                    logger.info(
                        `Terminal output on failure: >>>>>>>>>>>>>>>\r ${testRunner.output}`
                    );
                }
            });

            it("should offer to install prerequisites and exit upon negative answer", async function () {
                this.timeout(20000);
                const promptRequisites = await testRunner.waitForOutput(
                    "Do you want to install prerequisites?"
                );

                expect(promptRequisites).to.be.true;

                testRunner.sendInput("n");

                const terminalExited = await testRunner.waitForOutput(
                    "Please install the missing prerequisites and try again"
                );
                expect(terminalExited).to.be.true;
            });

            //This should be re-enabled to run locally or when fixed on github runners powershell 7

            // it("should install prerequisites and offer to install python and exit upon negative answer", async function () {
            //     this.timeout(240000);
            //     const promptRequisites = await testRunner.waitForOutput(
            //         "Do you want to install prerequisites"
            //     );

            //     expect(promptRequisites).to.be.true;

            //     logger.info("Question to install prerequisites passed");
            //     testRunner.output = "";
            //     testRunner.sendInput("y");

            //     const promptPython = await testRunner.waitForOutput(
            //         "Do you want to install Python",
            //         240000
            //     );

            //     expect(promptPython).to.be.true;
            //     expect(testRunner.output).to.include(
            //         "All prerequisites are satisfied"
            //     );

            //     testRunner.sendInput("n");

            //     const terminalExited = await testRunner.waitForOutput(
            //         "Please install python3 with pip and SSL support and try again"
            //     );

            //     expect(terminalExited).to.be.true;
            // });

            // it("should install python and proceed with installation", async function () {
            //     this.timeout(240000);
            //     const promptRequisites = await testRunner.waitForOutput(
            //         "Do you want to install prerequisites"
            //     );

            //     expect(promptRequisites).to.be.true;

            //     logger.info("Question to install prerequisites passed");
            //     testRunner.output = "";
            //     testRunner.sendInput("y");

            //     const promptPython = await testRunner.waitForOutput(
            //         "Do you want to install Python",
            //         240000
            //     );

            //     expect(promptPython).to.be.true;
            //     expect(testRunner.output).to.include(
            //         "All prerequisites are satisfied"
            //     );

            //     logger.info("Question to install python passed");
            //     testRunner.output = "";
            //     testRunner.sendInput("y");

            //     const selectTargetQuestion = await testRunner.waitForOutput(
            //         "Please select all of the target platforms",
            //         240000
            //     );

            //     expect(selectTargetQuestion).to.be.true;
            // });
        }
    );
});
