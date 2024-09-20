import { expect } from "chai";
import { describe, it, before, after, beforeEach, afterEach } from "mocha";
import { InteractiveCLITestRunner } from "./CLITestRunner.js"

export function runInstallWizzardTests(){
    describe("Check if Install Wizzard steps", function () {
        let testRunner;

        beforeEach(async function () {
            testRunner = new InteractiveCLITestRunner("/espressif/eim");
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