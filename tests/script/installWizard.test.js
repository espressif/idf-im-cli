import { expect } from "chai";
import { describe, it, before, after, beforeEach, afterEach } from "mocha";
import { InteractiveCLITestRunner } from "../classes/CLITestRunner.class.js";
import logger from "../classes/logger.class.js";

export function runInstallWizardTests(pathToEim) {
    describe("Check IDF Install Wizard steps", function () {
        let testRunner = null;

        before(async function () {
            logger.debug(`Starting installation wizard with default options`);
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

        after(async function () {
            this.timeout(10000);
            if (testRunner) {
                await testRunner.stop();
            }
            testRunner = null;
        });

        /** Run install wizard
         *
         * It is expected to have all requirements installed
         * The step to install the prerequisites in windows is not tested
         *
         */

        it("Should install IDF using wizard and default values", async function () {
            const selectTargetQuestion = await testRunner.waitForOutput(
                "Please select all of the target platforms"
            );
            if (!selectTargetQuestion) {
                logger.info(testRunner.output);
            }
            expect(selectTargetQuestion).to.be.true;
            expect(testRunner.output).to.include("all");

            logger.info("Select Target Passed");
            testRunner.output = "";
            testRunner.sendInput("\r");

            const selectIDFVersion = await testRunner.waitForOutput(
                "Please select the desired ESP-IDF version"
            );
            if (!selectIDFVersion) {
                logger.info(testRunner.output);
            }
            expect(selectIDFVersion).to.be.true;
            expect(testRunner.output).to.include("v5.3.1");

            logger.info("Select IDF Version passed");
            testRunner.output = "";
            testRunner.sendInput("\r");

            const selectIDFMirror = await testRunner.waitForOutput(
                "Select the source from which to download esp-idf"
            );
            if (!selectIDFMirror) {
                logger.info(testRunner.output);
            }
            expect(selectIDFMirror).to.be.true;
            expect(testRunner.output).to.include("https://github.com");

            logger.info("Select IDF mirror passed");
            testRunner.output = "";
            testRunner.sendInput("\r");

            const selectToolsMirror = await testRunner.waitForOutput(
                "Select a source from which to download tools"
            );
            if (!selectToolsMirror) {
                logger.info(testRunner.output);
            }
            expect(selectToolsMirror).to.be.true;
            expect(testRunner.output).to.include("https://github.com");

            logger.info("Select tools mirror passed");
            testRunner.output = "";
            testRunner.sendInput("\r");

            const selectInstallPath = await testRunner.waitForOutput(
                "Please select the ESP-IDF installation location"
            );
            if (!selectInstallPath) {
                logger.info(testRunner.output);
            }
            expect(selectInstallPath).to.be.true;
            expect(testRunner.output).to.include("esp");

            logger.info("Select install path passed");
            testRunner.output = "";
            testRunner.sendInput("\r");

            const installationCompleted = await testRunner.waitForOutput(
                "Do you want to save the installer configuration",
                1200000
            );
            if (!installationCompleted) {
                logger.info(testRunner.output);
            }
            expect(installationCompleted).to.be.true;
            expect(testRunner.output).to.not.include("error");
            expect(testRunner.output).to.include(
                "Finished fetching submodules"
            );
            expect(testRunner.output).to.include("Downloading tools");

            logger.info("Installation completed");
            testRunner.output = "";
            testRunner.sendInput("\r");

            const installationSuccessful = await testRunner.waitForOutput(
                "Successfully installed IDF"
            );
            if (!installationSuccessful) {
                logger.info(testRunner.output);
            }
            expect(installationSuccessful).to.be.true;
        });
    });
}
