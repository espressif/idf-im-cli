import { expect } from "chai";
import { describe, it, before, after, beforeEach, afterEach } from "mocha";
import { InteractiveCLITestRunner } from "../classes/CLITestRunner.class.js";
import logger from "../classes/logger.class.js";
import os from "os";
import path from "path";

export function runPostInstallTest(
    pathToIDFScript,
    pathToProjectFolder,
    validTarget = "esp32",
    invalidTarget = ""
) {
    describe("create and build sample project", function () {
        this.timeout(600000);
        let testRunner;

        beforeEach(async function () {
            this.timeout(5000);
            logger.debug(
                `Starting IDF terminal using for idf script ${pathToIDFScript}, sample project copied at ${path.join(
                    os.homedir(),
                    pathToProjectFolder
                )}`
            );
            testRunner = new InteractiveCLITestRunner();
            try {
                await testRunner.runIDFTerminal(pathToIDFScript);
            } catch (error) {
                logger.debug(`Error starting process: ${error}`);
                throw error;
            }
        });

        afterEach(async function () {
            this.timeout(10000);
            if (this.currentTest.state === "failed") {
                logger.info(
                    `Terminal output on failure: >>>>>>>>>>>>>>>\r ${testRunner.output}`
                );
            }
            if (testRunner) {
                await testRunner.stop();
            }
            testRunner = null;
        });

        it("Should create a new project based on a template", async function () {
            /**
             * This test should attempt to create a copy of the Hello World Project into the ~/esp folder
             * The commands might differ for each operating system.
             * The assert is based on the existence of the project files in the expected folder.
             */
            testRunner.sendInput(`mkdir ${pathToProjectFolder}\r`);
            testRunner.sendInput(`cd ${pathToProjectFolder}\r`);

            testRunner.sendInput(
                os.platform() !== "win32"
                    ? `cp -r $IDF_PATH/examples/get-started/hello_world .\r`
                    : `xcopy /e /i $env:IDF_PATH\\examples\\get-started\\hello_world hello_world\r`
            );
            if (os.platform() === "win32") {
                const confirmFilesCopied = await testRunner.waitForOutput(
                    "copied"
                );
                expect(confirmFilesCopied).to.be.true;
            }
            testRunner.output = "";
            testRunner.sendInput("cd hello_world\r");
            testRunner.sendInput("ls\r");

            const confirmFolderContent = await testRunner.waitForOutput(
                "sdkconfig.ci"
            );

            expect(confirmFolderContent).to.be.true;
            expect(testRunner.output).to.include("pytest_hello_world.py");
            expect(testRunner.output).to.include("main");

            logger.info("sample project creation Passed");
        });

        it("Should set the target", async function () {
            /**
             * This test attempts to set a target MCU for the project created in the previous test.
             */
            this.timeout(600000);
            testRunner.output = "";
            testRunner.sendInput(`cd ${pathToProjectFolder}\r`);
            testRunner.sendInput("cd hello_world\r");
            testRunner.sendInput(`idf.py set-target ${validTarget}\r`);

            const targetSet = await testRunner.waitForOutput(
                "Build files have been written to",
                600000
            );

            expect(targetSet).to.be.true;
            expect(testRunner.output).to.include("Configuring done");
            expect(testRunner.output).to.include("Generating done");

            logger.info("Set Target Passed");
        });

        it("Should build project for the selected target", async function () {
            /**
             * This test attempts to build artifacts for the project and targets selected above.
             * The test is successful if the success message is printed in the terminal.
             */
            this.timeout(300000);
            testRunner.output = "";
            testRunner.sendInput(`cd ${pathToProjectFolder}\r`);
            testRunner.sendInput("cd hello_world\r");
            testRunner.sendInput("idf.py build\r");

            const buildComplete = await testRunner.waitForOutput(
                "Project build complete",
                450000
            );

            expect(buildComplete).to.be.true;
            expect(testRunner.output).to.include(
                `Successfully created ${validTarget} image`
            );
            logger.info("Build Passed");
        });
    });
}
