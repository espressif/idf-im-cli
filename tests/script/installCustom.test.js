import { expect } from "chai";
import { describe, it, before, after, beforeEach, afterEach } from "mocha";
import { InteractiveCLITestRunner } from "../classes/CLITestRunner.class.js";
import logger from "../classes/logger.class.js";

export function runInstallCustom(
    pathToEim,
    installPath,
    targetList,
    idfVersionList,
    recursiveSubmodules
) {
    describe("run custom installation using given parameters", function () {
        let testRunner;

        before(async function () {
            logger.debug(
                `Installing IDF version: ${idfVersionList} on path: ${installPath}`
            );
            logger.debug(`Installing IDF for targets ${targetList}`);
            logger.debug(`Recurse submodules active? : ${recursiveSubmodules}`);
            this.timeout(5000); // Increase timeout for setup
            testRunner = new InteractiveCLITestRunner();
        });

        afterEach(function () {
            if (this.currentTest.state === "failed") {
                logger.info(
                    `Terminal output on failure: >>>>>>>>>>>>>>>\r ${testRunner.output}`
                );
            }
        });

        after(async function () {
            this.timeout(10000);
            if (testRunner) {
                await testRunner.stop();
            }
            testRunner = null;
        });

        /** Run installation with full parameters, no need to ask questions
         *
         * It is expected to have all requirements installed
         *
         */

        it("Should install IDF using specified parameters", async function () {
            testRunner.runTerminal();
            logger.info("Sent command line for IDF installation");
            testRunner.sendInput(
                `${pathToEim} -p ${installPath} -t ${targetList} -i ${idfVersionList} --tool-download-folder-name dist --tool-install-folder-name tools --idf-tools-path ./tools/idf_tools.py --tools-json-file tools/tools.json -m https://github.com --idf-mirror https://github.com -r ${recursiveSubmodules}\r`
            );
            const installationCompleted = await testRunner.waitForOutput(
                "Do you want to save the installer configuration",
                1200000
            );
            expect(installationCompleted).to.be.true;
            expect(testRunner.output).to.not.include("error");

            logger.info("Installation completed");
            testRunner.output = "";
            testRunner.sendInput("n");

            const installationSuccessful = await testRunner.waitForOutput(
                "Successfully installed IDF"
            );

            expect(installationSuccessful).to.be.true;
            expect(testRunner.output).to.include(
                "Now you can start using IDF tools"
            );
        });
    });
}
