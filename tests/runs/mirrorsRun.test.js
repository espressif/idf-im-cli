import { testRun } from "./testRun.test.js";
import path from "path";
import fs from "fs";

/**
 * Setup the following environmental variables to execute this test:
 *
 * EIM_FILE_PATH to point to the eim application.
 * EIM_VERSION to specify expected version to be printed by the application.
 * IDF_VERSION to specify the default version of the IDF to be installed.
 *
 * use:
 * Windows: $env:<variable>="<value>"
 * Linux/mac: export <variable>="<value>"
 *
 * This script relies on test suite data stores on:
 * suites/basic_test.json
 */

const jsonFilePath = path.join(import.meta.dirname, "suites/mirrors_test.json");
const mirrorsTest = JSON.parse(fs.readFileSync(jsonFilePath, "utf-8"));

testRun(mirrorsTest);
