import { expect } from "chai";
import { describe, it, before, after, beforeEach, afterEach } from "mocha";
import { InteractiveCLITestRunner } from "./CLITestRunner.js"

const pathToEim = process.env.EIM_FILE_PATH
const eimVersion = process.env.EIM_VERSION

export function runLineArgumentsTests(){
    describe("CLI Line Arguments Tests", function () {
        let testRunner;

        before(function () {
            testRunner = new InteractiveCLITestRunner(pathToEim);
        });

        after(async function () { //Need to check if this is necessary
            if (testRunner) {
            await testRunner.stop();
            }
        });

        describe("Command-line Arguments", function () {
            it("should handle valid arguments", async function () {
            const { output, code } = await testRunner.runWithArgs(["-V"]);
            expect(code).to.equal(0);
            expect(output).to.include(eimVersion);
            });

            it("should show help with --help argument", async function () {
            const { output, code } = await testRunner.runWithArgs(["--help"]);
            expect(code).to.equal(0);
            expect(output).to.include("Usage:");
            expect(output).to.include("Options:");
            });

            it("should handle invalid arguments", async function () {
            const { output, code } = await testRunner.runWithArgs(["-KK"]);
            expect(code).to.not.equal(0);
            expect(output).to.include("unexpected argument");
            });
        });
    });
};