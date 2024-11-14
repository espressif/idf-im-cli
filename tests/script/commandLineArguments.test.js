import { expect } from "chai";
import { describe, it, before, after, beforeEach, afterEach } from "mocha";
import { InteractiveCLITestRunner } from "../classes/CLITestRunner.class.js";
import logger from "../classes/logger.class.js";

export function runArgumentsTests(pathToEim, eimVersion) {
    describe("CLI Arguments Tests", function () {
        let testRunner = null;

        beforeEach(function () {
            testRunner = new InteractiveCLITestRunner();
        });

        afterEach(async function () {
            if (this.currentTest.state === "failed") {
                logger.info(
                    `Terminal output on failure: >>\r ${testRunner.output}`
                );
            }
            if (!testRunner.exited) {
                logger.debug("Sending stop command to emulator");
                await testRunner.stop();
            }
            testRunner = null;
        });

        it("should show correct version number", async function () {
            await testRunner.start();
            testRunner.sendInput(`${pathToEim} -V\r`);
            const meetVersion = await testRunner.waitForOutput(eimVersion);
            expect(meetVersion).to.be.true;
        });

        it("should show help with --help argument", async function () {
            await testRunner.start();
            testRunner.sendInput(`${pathToEim} --help\r`);
            const printHelp = await testRunner.waitForOutput("Options:");
            expect(printHelp).to.be.true;
            expect(testRunner.output).to.include("Usage:");
        });

        it("should handle invalid arguments", async function () {
            await testRunner.start();
            testRunner.sendInput(`${pathToEim} --KK\r`);
            const wrongArgument = await testRunner.waitForOutput(
                "unexpected argument"
            );
            expect(wrongArgument).to.be.true;
        });
    });
}
