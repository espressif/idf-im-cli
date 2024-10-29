import { expect } from "chai";
import { describe, it, before, after, beforeEach, afterEach } from "mocha";
import { InteractiveCLITestRunner } from "../classes/CLITestRunner.class.js";
import logger from "../classes/logger.class.js";
import os from "os";
import path from "path";

let pathToEim;
let eimVersion;

if (process.env.EIM_FILE_PATH) {
    pathToEim = process.env.EIM_FILE_PATH;
} else {
    pathToEim = path.join(os.homedir(), "espressif/eim");
}

if (process.env.EIM_VERSION) {
    eimVersion = process.env.EIM_VERSION;
} else {
    eimVersion = "idf-im-cli 0.1.0";
}

export function runLineArgumentsTests() {
    describe("CLI Arguments Tests", function () {
        let testRunner;

        before(function () {
            testRunner = new InteractiveCLITestRunner();
        });

        after(async function () {
            if (!testRunner.exited) {
                await testRunner.stop();
            }
            testRunner = null;
        });

        describe("Command-line Arguments", function () {
            it("should show correct version number", async function () {
                testRunner.runApp(pathToEim, ["-V"]);
                const meetVersion = await testRunner.waitForExit(eimVersion);
                expect(meetVersion).to.be.true;
                expect(testRunner.exitCode).to.equal(0);
            });

            it("should show help with --help argument", async function () {
                testRunner.runApp(pathToEim, ["--help"]);
                const meetVersion = await testRunner.waitForExit("Options:");
                expect(meetVersion).to.be.true;
                expect(testRunner.output).to.include("Usage:");
                expect(testRunner.exitCode).to.equal(0);
            });

            it("should handle invalid arguments", async function () {
                testRunner.runApp(pathToEim, ["-KK"]);
                const meetVersion = await testRunner.waitForExit(
                    "unexpected argument"
                );
                expect(meetVersion).to.be.true;
                expect(testRunner.exitCode).to.not.equal(0);
            });
        });
    });
}
