import pty from "node-pty";
import os from "os";
import logger from "./logger.class.js";

export class InteractiveCLITestRunner {
    constructor() {
        this.process = null;
        this.output = "";
        this.exited = false;
        this.exitCode = null;
        this.error = null;
    }

    runTerminal() {
        const command = os.platform() !== "win32" ? "bash" : "powershell.exe";
        const args =
            os.platform() !== "win32"
                ? []
                : ["-ExecutionPolicy", "Bypass", "-NoProfile"];

        logger.debug(`Starting terminal ${command} with args ${args}`);
        this.start(command, args);
    }

    runIDFTerminal(loadScript) {
        this.runTerminal();
        const loadCommand =
            os.platform() !== "win32"
                ? `source ${loadScript}`
                : `. "${loadScript}"`;
        logger.debug(`Script load command sent to terminal ${loadCommand}`);
        this.sendInput(`${loadCommand}\r`);
    }

    start(command, fullArgs = []) {
        return new Promise((resolve, reject) => {
            logger.debug("Starting terminal emulator process...");
            this.process = pty.spawn(command, fullArgs, {
                name: "eim-terminal",
                cols: 80,
                rows: 30,
                cwd: process.cwd(),
                env: process.env,
            });
            this.exited = false;

            this.process.onData((data) => {
                try {
                    this.output += data;
                    logger.debug(data);
                } catch (error) {
                    logger.debug(`Error receiving data: ${error}`);
                    this.error = error;
                    this.exited = true;
                    reject(error);
                }
            });
            this.process.onExit(({ exitCode }) => {
                logger.debug(`Exiting with code:>>>${exitCode}<<<`);
                this.exited = true;
                this.exitCode = exitCode;
                if (!this.error) {
                    resolve();
                }
            });

            this.process.on("error", (error) => {
                logger.debug(`Process error:>>>>${error}<<<<<<`);
                this.error = error;
                this.exited = true;
                reject(error);
            });

            // Resolve after a short delay if the process hasn't exited or errored
            setTimeout(() => {
                if (!this.exited && !this.error) {
                    resolve();
                }
            }, 1000);
        });
    }

    sendInput(input) {
        logger.debug(`Sending ${input.replace(/\r$/, "")} to terminal`);
        if (this.process && !this.exited) {
            try {
                this.process.write(input);
            } catch (error) {
                logger.info(`Error sending input:>>>>${error}<<<<<<<<<<<`);
                this.error = error;
                this.exited = true;
            }
        } else {
            logger.info("Attempted to send input, but process is not running");
        }
    }

    async waitForOutput(expectedOutput, timeout = 10000) {
        const startTime = Date.now();
        while (Date.now() - startTime < timeout) {
            if (this.output.includes(expectedOutput)) {
                return true;
            }
            if (this.exited) {
                return false;
            }
            await new Promise((resolve) => setTimeout(resolve, 100));
        }
        return false;
    }

    async waitForExit(expectedLastOutput, timeout = 5000) {
        const startTime = Date.now();
        while (Date.now() - startTime < timeout) {
            if (this.exited) {
                await new Promise((resolve) => setTimeout(resolve, 1000));
                return this.output.includes(expectedLastOutput);
            }
            await new Promise((resolve) => setTimeout(resolve, 200));
        }
        return false;
    }

    async stop(timeout = 3000) {
        if (this.process && !this.exited) {
            return new Promise((resolve) => {
                // First, try to send a termination signal
                this.process.write("\x03");
                this.process.write("exit\r");

                // Set up a timeout
                const timer = setTimeout(() => {
                    logger.info(
                        "Process didn't exit gracefully, forcing termination"
                    );
                    this.process.kill();
                    resolve();
                }, timeout);

                // Listen for the process to exit on its own
                this.process.onExit(() => {
                    clearTimeout(timer);
                    this.process = null;
                    this.exited = true;
                    this.output = "";
                    resolve();
                });
            });
        }
    }
}
