import { expect } from "chai";
import { describe, it, before, after, beforeEach, afterEach } from "mocha";
import { InteractiveCLITestRunner } from "../classes/CLITestRunner.class.js";
import logger from "../classes/logger.class.js";
import os from "os";
import path from "path";

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
            await testRunner.runApp(pathToEim);
        } catch (error) {
            logger.debug("Error starting process:", error);
            throw error;
        }
    });

    afterEach(async function () {
        this.timeout(10000);
        if (testRunner) {
            await testRunner.stop();
        }
        testRunner = null;
    });

    /** Linux/MAC Specific Tests
     *
     *
     * Tests below will only be executed on Unix Based systems
     *
     *
     */

    // The following test can only be executed if the prerequisites have not been installed in the OS.
    (os.platform() !== "win32" ? describe : describe.skip)(
        "Linux/MAC specific tests",
        function () {
            it("Should detect missing requirements", async function () {
                this.timeout(20000);
                const missingRequisites = await testRunner.waitForExit(
                    "Error: Please install the missing prerequisites",
                    20000
                );
                if (!missingRequisites) {
                    logger.info(testRunner.output);
                }
                expect(missingRequisites).to.be.true;
                expect(testRunner.exitCode).to.equal(0);
            });
        }
    );

    /** Windows Specific Tests
     *
     *
     * Tests below will only be executed on win32 platform
     *
     *
     */

    (os.platform() === "win32" ? describe : describe.skip)(
        "Windows-specific tests",
        function () {
            it("should offer to install prerequisites and exit upon negative answer", async function () {
                this.timeout(20000);
                const promptRequisites = await testRunner.waitForOutput(
                    "Do you want to install prerequisites?"
                );
                if (!promptRequisites) {
                    logger.info(testRunner.output);
                }
                expect(promptRequisites).to.be.true;

                testRunner.sendInput("n");

                const terminalExited = await testRunner.waitForExit(
                    "Please install the missing prerequisites and try again"
                );
                if (!terminalExited) {
                    logger.info(testRunner.output);
                }
                expect(terminalExited).to.be.true;
                expect(testRunner.exitCode).to.equal(0);
            });

            //This should be re-enabled to run locally or when fixed on github runners powershell 7

            // it("should install prerequisites and offer to install python and exit upon negative answer", async function () {
            //     this.timeout(240000);
            //     const promptRequisites = await testRunner.waitForOutput(
            //         "Do you want to install prerequisites"
            //     );
            //     if (!promptRequisites) {
            //         logger.info(testRunner.output);
            //     }
            //     expect(promptRequisites).to.be.true;

            //     logger.info("Question to install prerequisites passed");
            //     testRunner.output = "";
            //     testRunner.sendInput("y");

            //     const promptPython = await testRunner.waitForOutput(
            //         "Do you want to install Python",
            //         240000
            //     );
            //     if (!promptPython) {
            //         logger.info(testRunner.output);
            //     }
            //     expect(promptPython).to.be.true;
            //     expect(testRunner.output).to.include(
            //         "All prerequisites are satisfied"
            //     );

            //     testRunner.sendInput("n");

            //     const terminalExited = await testRunner.waitForExit(
            //         "Please install python3 with pip and SSL support and try again"
            //     );
            //     if (!terminalExited) {
            //         logger.info(testRunner.output);
            //     }
            //     expect(terminalExited).to.be.true;
            //     expect(testRunner.exitCode).to.equal(0);
            // });

            // it("should install python and proceed with installation", async function () {
            //     this.timeout(240000);
            //     const promptRequisites = await testRunner.waitForOutput(
            //         "Do you want to install prerequisites"
            //     );
            //     if (!promptRequisites) {
            //         logger.info(testRunner.output);
            //     }
            //     expect(promptRequisites).to.be.true;

            //     logger.info("Question to install prerequisites passed");
            //     testRunner.output = "";
            //     testRunner.sendInput("y");

            //     const promptPython = await testRunner.waitForOutput(
            //         "Do you want to install Python",
            //         240000
            //     );
            //     if (!promptPython) {
            //         logger.info(testRunner.output);
            //     }
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
            //     if (!selectTargetQuestion) {
            //         logger.info(testRunner.output);
            //     }
            //     expect(selectTargetQuestion).to.be.true;
            //     expect(testRunner.exitCode).to.not.equal(0);
            // });
        }
    );
});
