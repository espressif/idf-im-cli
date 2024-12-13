import { expect } from "chai";
import { describe, it, before, after, beforeEach, afterEach } from "mocha";
import { InteractiveCLITestRunner } from "../classes/CLITestRunner.class.js";
import logger from "../classes/logger.class.js";

export function runInstallNonInteractive(pathToEim, args = []) {
    describe("run custom installation using non interactive terminal", function () {
        let testRunner = null;

        before(async function () {
            logger.debug(`Starting non interactive installation`);
            logger.debug(`Using parameters ${args.join(" ")}`);
            this.timeout(5000);
            testRunner = new InteractiveCLITestRunner();
            try {
                await testRunner.start();
            } catch {
                logger.info("Error to start terminal");
            }
        });

        afterEach(function () {
            if (this.currentTest.state === "failed") {
                logger.info(
                    `Terminal output on failure: >>\r ${testRunner.output}`
                );
                testRunner.sendInput("\x03");
            }
        });

        after(async function () {
            logger.info("Custom installation routine completed");
            this.timeout(20000);
            try {
                await testRunner.stop();
            } catch (error) {
                logger.info("Error to clean up terminal after test");
                logger.info(` Error: ${error}`);
            }
        });

        /** Run installation with non-interactive shell.
         *
         * It is expected for all setting to fall back to default in case it is not passed as argument
         * Config file would also overwrite default setting.
         *
         */

        it("Should install IDF using specified parameters", async function () {
            logger.info(`Starting test - IDF non-interactive installation`);
            testRunner.sendInput(`${pathToEim} ${args.join(" ")} -n true\r`);

            const installationSuccessful = await testRunner.waitForOutput(
                "Successfully installed IDF",
                1200000
            );

            expect(
                installationSuccessful,
                "Failed to complete installation, missing 'Successfully Installed IDF'"
            ).to.be.true;

            // expect(
            //     testRunner.output,
            //     "Error message during installation"
            // ).to.not.include("error");

            expect(
                testRunner.output,
                "Failed to complete installation, missing 'Now you can start using IDF tools'"
            ).to.include("Now you can start using IDF tools");

            if ("-r true" in args || "--recursive-submodules" in args) {
                expect(
                    testRunner.output,
                    "Failed to download submodules, missing 'Finished fetching submodules'"
                ).to.include("Finished fetching submodules");
            }
        });
    });
}
