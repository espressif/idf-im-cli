import { describe, it, before, after } from "mocha";
import { runLineArgumentsTests } from "./commandLineArgumentsTest.js";
import { runPrerequisitesCheckTests } from "./prerequisitesTest.js"
import { runInstallWizzardTests } from "./installWizardTest.js";

describe("Installation Manager Tests", function () {
  this.timeout(10000);

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
