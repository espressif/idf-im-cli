import { expect } from "chai";
import { describe, it, before, after, beforeEach, afterEach } from "mocha";
import { InteractiveCLITestRunner } from "../classes/CLITestRunner.class.js";
import logger from "../classes/logger.class.js";

export function runArgumentsTests(pathToEim, eimVersion) {
    describe("CLI Arguments Tests", function () {
        let testRunner;

        beforeEach(function () {
            testRunner = new InteractiveCLITestRunner();
        });

        afterEach(async function () {
            logger.debug(
                `Exited: ${testRunner.exited} with code ${testRunner.exitCode}`
            );
            if (!testRunner.exited) {
                await testRunner.stop();
            }
            testRunner = null;
        });

        it("should show correct version number", async function () {
            testRunner.runTerminal();
            testRunner.sendInput(`${pathToEim} -V\r`);
            const meetVersion = await testRunner.waitForOutput(eimVersion);
            if (!meetVersion) {
                logger.info(testRunner.output);
            }
            expect(meetVersion).to.be.true;
        });

        it("should show help with --help argument", async function () {
            testRunner.runTerminal();
            testRunner.sendInput(`${pathToEim} --help\r`);
            const printHelp = await testRunner.waitForOutput("Options:");
            if (!printHelp) {
                logger.info(testRunner.output);
            }
            expect(printHelp).to.be.true;
            expect(testRunner.output).to.include("Usage:");
        });

        it("should handle invalid arguments", async function () {
            testRunner.runTerminal();
            testRunner.sendInput(`${pathToEim} --KK\r`);
            const wrongArgument = await testRunner.waitForOutput(
                "unexpected argument"
            );
            if (!wrongArgument) {
                logger.info(testRunner.output);
            }
            expect(wrongArgument).to.be.true;
        });
    });
}
