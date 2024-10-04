import { describe, it, before, after } from "mocha";
import { runLineArgumentsTests } from "./commandLineArguments.test.js";
import { runPrerequisitesCheckTests } from "./prerequisites.test.js";
import { runInstallWizzardTests } from "./installWizard.test.js";

describe("Installation Manager Tests", function () {
    this.timeout(2400000);

    before(function () {
        // Any setup code that needs to run before all tests
    });

    after(function () {
        // Any cleanup code that needs to run after all tests
    });

    // Run all test suites
    runLineArgumentsTests();
    runPrerequisitesCheckTests();
    runInstallWizzardTests();
});
