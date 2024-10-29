#!/bin/bash

# Save the arguments as environment variables
export EIM_FILE_PATH="$1"
export EIM_VERSION="$2"

# install node modules
npm ci

# run tests
npm run test