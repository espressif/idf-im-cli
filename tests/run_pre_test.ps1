# Load arguments as environmental variables

param (
    [Parameter(Mandatory=$true)]
    [string]$Path_to_eim,

    [Parameter(Mandatory=$true)]
    [string]$Version
)

# Save the arguments as environment variables
$env:EIM_FILE_PATH = $Path_to_eim
$env:EIM_VERSION = $Version

Set-Location -Path "./tests"

# Expand Node modules folder
Expand-Archive node_modules.zip

# Install node modules using npm ci
# This can be used if the node modules folder is not packed with the repo
# npm ci

# Run tests using npm run AllTest
npm run pre-test