@echo off
:: Enter test folder
cd AutoTest

:: install node modules
npm ci

:: run tests
npm run AllTest