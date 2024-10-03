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

export function runInstallWizzardTests() {
    describe("Check if Install Wizzard steps", function () {
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

        /** Run install wizzard on Linux/MAC
         *
         * Tests below will only be executed on Unix Based systems
         * It is expected to have all requirements installed
         * Test should accept all default settings for installation
         *
         */

        (os.platform() !== "win32" ? describe : describe.skip)(
            "Linux/MAC wizzard workflow",
            function () {
                it("Should launch the installation manager and request target platforms", async function () {
                    const selectTargetQuestion = await testRunner.waitForOutput(
                        "Please select all of the target platforms"
                    );
                    expect(selectTargetQuestion).to.be.true;
                    expect(testRunner.output).to.include("all");
                });

                it("Should request ESP-IDF version", async function () {
                    testRunner.sendInput("\r");
                    const selectTargetQuestion = await testRunner.waitForOutput(
                        "Please select the desired ESP-IDF version"
                    );
                    expect(selectTargetQuestion).to.be.true;
                    expect(testRunner.output).to.include("v5.3.1");
                });

                it("Should request the mirror to download ESP-IDF", async function () {
                    testRunner.sendInput("\r");
                    const selectIDFMirror = await testRunner.waitForOutput(
                        "Select the source from which to download esp-idf"
                    );
                    expect(selectIDFMirror).to.be.true;
                    expect(testRunner.output).to.include("https://github.com");
                });

                it("Should request the mirror to download Tools", async function () {
                    testRunner.sendInput("\r");
                    const selectToolsMirror = await testRunner.waitForOutput(
                        "Select a source from which to download tools"
                    );
                    expect(selectToolsMirror).to.be.true;
                    expect(testRunner.output).to.include("https://github.com");
                });

                it("Should request installation path", async function () {
                    testRunner.sendInput("\r");
                    const selectInstallPath = await testRunner.waitForOutput(
                        "Please select the ESP-IDF installation location"
                    );
                    expect(selectInstallPath).to.be.true;
                    expect(testRunner.output).to.include("/.espressif)");
                });

                it("Should complete installation", async function () {
                    testRunner.sendInput("\r");
                    const installationCompleted =
                        await testRunner.waitForOutput(
                            "Do you want to save the installer configuration",
                            1200000
                        );
                    expect(installationCompleted).to.be.true;
                    expect(testRunner.output).to.not.include("error");
                    expect(testRunner.output).to.include(
                        "Finished fetching submodules"
                    );
                    expect(testRunner.output).to.include("Downloading tools");
                });

                it("Should exit with concluded installation", async function () {
                    testRunner.sendInput("\r");
                    const installationSuccessful = await testRunner.waitForExit(
                        "Successfully installed IDF"
                    );
                    expect(installationSuccessful).to.be.true;
                    expect(testRunner.exitCode).to.equal(0);
                    expect(testRunner.output).to.include(
                        "to activate the environment, run the following command in your terminal:"
                    );
                });
            }
        );

        /** Run install wizzard on Windows
         *
         * Tests below will only be executed on windows systems
         * It is expected the installed will perform all steps, including installation of the prerequisites.
         * Test should accept all default settings for installation
         *
         */

        (os.platform() === "win32" ? describe : describe.skip)(
            "Windows wizzard workflow",
            function () {
                it("should offer to install prerequisites", async function () {
                    this.timeout(10000);
                    const promptReceived = await testRunner.waitForOutput(
                        "Do you want to install prerequisites"
                    );
                    expect(promptReceived).to.be.true;
                });

                it("should offer to install python", async function () {
                    testRunner.sendInput("y");
                    const selectTargetQuestion = await testRunner.waitForOutput(
                        "Do you want to install Python",
                        240000
                    );
                    expect(selectTargetQuestion).to.be.true;
                    expect(testRunner.output).to.include(
                        "All prerequisites are satisfied"
                    );
                });

                it("Should request target platforms", async function () {
                    testRunner.sendInput("y");
                    const selectTargetQuestion = await testRunner.waitForOutput(
                        "Please select all of the target platforms",
                        240000
                    );
                    expect(selectTargetQuestion).to.be.true;
                    expect(testRunner.output).to.include("all");
                    expect(testRunner.output).to.include(
                        "Python installed successfully"
                    );
                });

                it("Should request ESP-IDF version", async function () {
                    testRunner.sendInput("\r");
                    const selectTargetQuestion = await testRunner.waitForOutput(
                        "Please select the desired ESP-IDF version"
                    );
                    expect(selectTargetQuestion).to.be.true;
                    expect(testRunner.output).to.include("v5.3.1");
                });

                it("Should request the mirror to download ESP-IDF", async function () {
                    testRunner.sendInput("\r");
                    const selectIDFMirror = await testRunner.waitForOutput(
                        "Select the source from which to download esp-idf"
                    );
                    expect(selectIDFMirror).to.be.true;
                    expect(testRunner.output).to.include("https://github.com");
                });

                it("Should request the mirror to download Tools", async function () {
                    testRunner.sendInput("\r");
                    const selectToolsMirror = await testRunner.waitForOutput(
                        "Select a source from which to download tools"
                    );
                    expect(selectToolsMirror).to.be.true;
                    expect(testRunner.output).to.include("https://github.com");
                });

                it("Should request installation path", async function () {
                    testRunner.sendInput("\r");
                    const selectInstallPath = await testRunner.waitForOutput(
                        "Please select the ESP-IDF installation location"
                    );
                    expect(selectInstallPath).to.be.true;
                    expect(testRunner.output).to.include("esp");
                });

                it("Should complete installation", async function () {
                    testRunner.sendInput("\r");
                    const installationCompleted =
                        await testRunner.waitForOutput(
                            "Do you want to save the installer configuration",
                            1200000
                        );
                    expect(installationCompleted).to.be.true;
                    expect(testRunner.output).to.not.include("error");
                    expect(testRunner.output).to.include(
                        "Finished fetching submodules"
                    );
                    expect(testRunner.output).to.include("Downloading tools");
                    expect(testRunner.output).to.include(
                        "Desktop shortcut created"
                    );
                });

                it("Should exit with concluded installation", async function () {
                    testRunner.sendInput("\r");
                    const installationSuccessful = await testRunner.waitForExit(
                        "Successfully installed IDF"
                    );
                    expect(installationSuccessful).to.be.true;
                    expect(testRunner.exitCode).to.equal(0);
                    expect(testRunner.output).to.include(
                        "placed shortcuts for PowerShell terminal with activated ESP-IDF"
                    );
                });
            }
        );
    });
}
