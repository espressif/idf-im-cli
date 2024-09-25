#!/bin/bash

# Save the arguments as environment variables
export EIM_FILE_PATH="$1"
export EIM_VERSION="$2"

#Enter test folder
cd script

# install node modules
npm ci

# run tests
npm run test