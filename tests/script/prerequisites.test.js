import { expect } from "chai";
import { describe, it, before, after, beforeEach, afterEach } from "mocha";
import { InteractiveCLITestRunner } from "./CLITestRunner.class.js";
import os from "os";
import path from "path";

let pathToEim;

if (process.env.EIM_FILE_PATH) {
    pathToEim = process.env.EIM_FILE_PATH;
} else {
    pathToEim = path.join(os.homedir(), "espressif/eim");
}

export function runPrerequisitesCheckTests() {
    describe("Check if prerequisites are installed", function () {
        let testRunner;

        before(async function () {
            this.timeout(5000); // Increase timeout for setup
            testRunner = new InteractiveCLITestRunner(pathToEim);
            try {
                await testRunner.start();
            } catch (error) {
                console.error("Error starting process:", error);
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

        /** Linux/MAC Specific Tests
         *
         *
         * Tests below will only be executed on Unix Based systems
         *
         *
         */

        // // The following test can only be executed if the prerequisites have not been installed in the OS.
        // (os.platform() !== "win32" ? describe : describe.skip)(
        //     "Linux/MAC specific tests",
        //     function () {
        // it("Should detect missing requirements", async function () {
        //     const missingRequisites = await testRunner.waitForExit(
        //         "Please install the missing prerequisites and try again"
        //     );
        //     expect(missingRequisites).to.be.true;
        //     expect(testRunner.exitCode).to.not.equal(0);
        // });
        //     }
        // );

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
                    this.timeout(10000);
                    const promptReceived = await testRunner.waitForOutput(
                        "Do you want to install prerequisites?"
                    );
                    expect(promptReceived).to.be.true;

                    testRunner.sendInput("n");
                    // await new Promise((resolve) =>
                    //     setTimeout(resolve, 2000)
                    // );
                    const terminalExited = await testRunner.waitForExit(
                        "Please install the missing prerequisites and try again"
                    );
                    expect(terminalExited).to.be.true;
                });
            }
        );
    });
}
