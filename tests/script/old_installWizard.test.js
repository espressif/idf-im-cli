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
                await testRunner.start(["-v"]);
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
                it("Should install IDF using wizzard and default values", async function () {
                    const selectTargetQuestion = await testRunner.waitForOutput(
                        "Please select all of the target platforms"
                    );
                    if (!selectTargetQuestion) {
                        console.log(testRunner.output);
                    }
                    expect(selectTargetQuestion).to.be.true;
                    expect(testRunner.output).to.include("all");

                    console.log("Select Target Passed");
                    testRunner.output = "";
                    testRunner.sendInput("\r");

                    const selectIDFVersion = await testRunner.waitForOutput(
                        "Please select the desired ESP-IDF version"
                    );
                    if (!selectIDFVersion) {
                        console.log(testRunner.output);
                    }
                    expect(selectIDFVersion).to.be.true;
                    expect(testRunner.output).to.include("v5.3.1");

                    console.log("Select IDF Version passed");
                    testRunner.output = "";
                    testRunner.sendInput("\r");

                    const selectIDFMirror = await testRunner.waitForOutput(
                        "Select the source from which to download esp-idf"
                    );
                    if (!selectIDFMirror) {
                        console.log(testRunner.output);
                    }
                    expect(selectIDFMirror).to.be.true;
                    expect(testRunner.output).to.include("https://github.com");

                    console.log("Select IDF mirror passed");
                    testRunner.output = "";
                    testRunner.sendInput("\r");

                    const selectToolsMirror = await testRunner.waitForOutput(
                        "Select a source from which to download tools"
                    );
                    if (!selectToolsMirror) {
                        console.log(testRunner.output);
                    }
                    expect(selectToolsMirror).to.be.true;
                    expect(testRunner.output).to.include("https://github.com");

                    console.log("Select tools mirror passed");
                    testRunner.output = "";
                    testRunner.sendInput("\r");

                    const selectInstallPath = await testRunner.waitForOutput(
                        "Please select the ESP-IDF installation location"
                    );
                    if (!selectInstallPath) {
                        console.log(testRunner.output);
                    }
                    expect(selectInstallPath).to.be.true;
                    expect(testRunner.output).to.include("/.espressif)");

                    console.log("Select install path passed");
                    testRunner.output = "";
                    testRunner.sendInput("\r");

                    const installationCompleted =
                        await testRunner.waitForOutput(
                            "Do you want to save the installer configuration",
                            1200000
                        );
                    if (!installationCompleted) {
                        console.log(testRunner.output);
                    }
                    expect(installationCompleted).to.be.true;
                    expect(testRunner.output).to.not.include("error");
                    expect(testRunner.output).to.include(
                        "Finished fetching submodules"
                    );
                    expect(testRunner.output).to.include("Downloading tools");

                    console.log("Installation completed");
                    testRunner.output = "";
                    testRunner.sendInput("\r");

                    const installationSuccessful = await testRunner.waitForExit(
                        "Successfully installed IDF"
                    );
                    if (!installationSuccessful) {
                        console.log(testRunner.output);
                    }
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
                it("Should install IDF using wizzard and default values", async function () {
                    const promptRequisites = await testRunner.waitForOutput(
                        "Do you want to install prerequisites"
                    );
                    if (!promptRequisites) {
                        console.log(testRunner.output);
                    }
                    expect(promptRequisites).to.be.true;

                    console.log("Question to install prerequisites passed");
                    testRunner.output = "";
                    testRunner.sendInput("y");

                    const promptPython = await testRunner.waitForOutput(
                        "Do you want to install Python",
                        240000
                    );
                    if (!promptPython) {
                        console.log(testRunner.output);
                    }
                    expect(promptPython).to.be.true;
                    expect(testRunner.output).to.include(
                        "All prerequisites are satisfied"
                    );

                    console.log("Question to install python passed");
                    testRunner.output = "";
                    testRunner.sendInput("y");

                    const selectTargetQuestion = await testRunner.waitForOutput(
                        "Please select all of the target platforms",
                        240000
                    );
                    if (!selectTargetQuestion) {
                        console.log(testRunner.output);
                    }
                    expect(selectTargetQuestion).to.be.true;
                    expect(testRunner.output).to.include("all");
                    expect(testRunner.output).to.include(
                        "Python installed successfully"
                    );

                    console.log("Select Target Passed");
                    testRunner.output = "";
                    testRunner.sendInput("\r");

                    const selectIDFVersion = await testRunner.waitForOutput(
                        "Please select the desired ESP-IDF version"
                    );
                    if (!selectIDFVersion) {
                        console.log(testRunner.output);
                    }
                    expect(selectIDFVersion).to.be.true;
                    expect(testRunner.output).to.include("v5.3.1");

                    console.log("Select IDF Version passed");
                    testRunner.output = "";
                    testRunner.sendInput("\r");

                    const selectIDFMirror = await testRunner.waitForOutput(
                        "Select the source from which to download esp-idf"
                    );
                    if (!selectIDFMirror) {
                        console.log(testRunner.output);
                    }
                    expect(selectIDFMirror).to.be.true;
                    expect(testRunner.output).to.include("https://github.com");

                    console.log("Select IDF mirror passed");
                    testRunner.output = "";
                    testRunner.sendInput("\r");

                    const selectToolsMirror = await testRunner.waitForOutput(
                        "Select a source from which to download tools"
                    );
                    if (!selectToolsMirror) {
                        console.log(testRunner.output);
                    }
                    expect(selectToolsMirror).to.be.true;
                    expect(testRunner.output).to.include("https://github.com");

                    console.log("Select tools mirror passed");
                    testRunner.output = "";
                    testRunner.sendInput("\r");

                    const selectInstallPath = await testRunner.waitForOutput(
                        "Please select the ESP-IDF installation location"
                    );
                    if (!selectInstallPath) {
                        console.log(testRunner.output);
                    }
                    expect(selectInstallPath).to.be.true;
                    expect(testRunner.output).to.include("esp");

                    console.log("Select install path passed");
                    testRunner.output = "";
                    testRunner.sendInput("\r");

                    const installationCompleted =
                        await testRunner.waitForOutput(
                            "Do you want to save the installer configuration",
                            1200000
                        );
                    if (!installationCompleted) {
                        console.log(testRunner.output);
                    }
                    expect(installationCompleted).to.be.true;
                    expect(testRunner.output).to.not.include("error");
                    expect(testRunner.output).to.include(
                        "Finished fetching submodules"
                    );
                    expect(testRunner.output).to.include("Downloading tools");
                    expect(testRunner.output).to.include(
                        "Desktop shortcut created"
                    );

                    console.log("Installation completed");
                    testRunner.output = "";
                    testRunner.sendInput("\r");

                    const installationSuccessful = await testRunner.waitForExit(
                        "Successfully installed IDF"
                    );
                    if (!installationSuccessful) {
                        console.log(testRunner.output);
                    }
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