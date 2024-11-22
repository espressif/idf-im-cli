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
        let testRunner = null;

        beforeEach(async function () {
            this.timeout(10000);
            logger.debug(
                `Starting IDF terminal using activation script ${pathToIDFScript}, sample project copied at ${pathToProjectFolder}`
            );
            testRunner = new InteractiveCLITestRunner();
            try {
                await testRunner.runIDFTerminal(pathToIDFScript);
            } catch {
                logger.info("Error to start IDF terminal");
                logger.info(testRunner.output);
                throw new Error(
                    "One test in teh chain failed, aborting the entire suite."
                );
            }
        });

        afterEach(async function () {
            this.timeout(20000);
            if (this.currentTest.state === "failed") {
                logger.info(
                    `Terminal output on failure: >>\r ${testRunner.output}`
                );
            }
            try {
                await testRunner.stop(6000);
                testRunner = null;
            } catch {
                logger.debug("Error to clean up terminal after test");
            }
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

            expect(
                confirmFolderContent,
                "sdkconfig.ci file not shown after a ls command, file copy failed"
            ).to.be.true;
            expect(
                testRunner.output,
                "pytest_hello_world.py file not shown after a ls command, file copy failed"
            ).to.include("pytest_hello_world.py");
            expect(
                testRunner.output,
                "main folder not shown after a ls command, file copy failed"
            ).to.include("main");

            logger.info("sample project creation Passed");
        });

        it("Should set the target", async function () {
            /**
             * This test attempts to set a target MCU for the project created in the previous test.
             */
            this.timeout(600000);
            testRunner.sendInput(`cd ${pathToProjectFolder}\r`);
            testRunner.sendInput("cd hello_world\r");
            testRunner.sendInput(`idf.py set-target ${validTarget}\r`);

            const targetSet = await testRunner.waitForOutput(
                "Build files have been written to",
                600000
            );

            expect(
                targetSet,
                "expecting 'Build files have been written to', failed to complete the set-target task"
            ).to.be.true;
            expect(
                testRunner.output,
                "expecting 'configuring done', failed to complete the set-target task"
            ).to.include("Configuring done");
            expect(
                testRunner.output,
                "expecting 'Generating Done', failed to complete the set-target task"
            ).to.include("Generating done");

            logger.info("Set Target Passed");
        });

        it("Should build project for the selected target", async function () {
            /**
             * This test attempts to build artifacts for the project and targets selected above.
             * The test is successful if the success message is printed in the terminal.
             */
            this.timeout(600000);
            testRunner.sendInput(`cd ${pathToProjectFolder}\r`);
            testRunner.sendInput("cd hello_world\r");
            testRunner.sendInput("idf.py build\r");

            const buildComplete = await testRunner.waitForOutput(
                "Project build complete",
                450000
            );

            expect(
                buildComplete,
                "Expecting 'Project build complete', filed to build the sample project"
            ).to.be.true;
            expect(
                testRunner.output,
                "Expecting to successfully create target image, filed to build the sample project"
            ).to.include(`Successfully created ${validTarget} image`);
            logger.info("Build Passed");
        });
    });
}
