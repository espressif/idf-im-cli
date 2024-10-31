#!/bin/bash

# Save the arguments as environment variables
export EIM_FILE_PATH="$1"
export EIM_VERSION="$2"

cd tests

# install node modules
# Node is being installed int eh folder by the CI, run this line if running it locally
# npm ci

# run tests
npm run default-test
npm run variation1-test