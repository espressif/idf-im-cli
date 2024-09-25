import { expect } from "chai";
import { describe, it, before, after, beforeEach, afterEach } from "mocha";
import { InteractiveCLITestRunner } from "./CLITestRunner.js"
import os from "os";
import path from "path";

let pathToEim;

if (process.env.EIM_FILE_PATH) {
  pathToEim = process.env.EIM_FILE_PATH;
} else {
  pathToEim = path.join(os.homedir(), "espressif/eim");
}

export function runInstallWizzardTests(){
    describe("Check if Install Wizzard steps", function () {
        let testRunner;

        beforeEach(async function () {
            testRunner = new InteractiveCLITestRunner(pathToEim);
            testRunner.start();
            await new Promise((resolve) => setTimeout(resolve, 1000));
        });

        afterEach(async function () {
            this.timeout(10000);
            if (testRunner) {
                await testRunner.stop();
        }
        });



        //Tests will enter here


    });
};