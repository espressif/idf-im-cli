import pty from "node-pty";
import os from "os";
import logger from "./logger.class.js";

export class IDFTestRunner {
    constructor(IDFLoadScriptPath) {
        this.LoadScriptPath = IDFLoadScriptPath;
        this.process = null;
        this.output = "";
        this.exited = false;
        this.exitCode = null;
        this.error = null;
    }

    getPlatformSpecificTerminal() {
        if (os.platform() === "win32") {
            // Windows
            return {
                //Not working yet
                command: "powershell.exe -ExecutionPolicy Bypass -NoProfile",
                args: [], // [" -Command", `"& {. '${this.LoadScriptPath}'}"`],
            };
        } else {
            // Linux, macOS, and other Unix-like systems
            return {
                command: "bash",
                args: [],
            };
        }
    }

    startTerminal() {
        return new Promise((resolve, reject) => {
            logger.debug("Starting terminal...");
            const command =
                os.platform() !== "win32"
                    ? "bash"
                    : "powershell.exe -ExecutionPolicy Bypass -NoProfile";
            const args = [];
            this.process = pty.spawn(command, args, {
                name: "eim-terminal",
                cols: 80,
                rows: 30,
                cwd: process.cwd(),
                env: process.env,
            });
            const loadCommand =
                os.platform() !== "win32"
                    ? `source ${this.LoadScriptPath}`
                    : `. "${this.LoadScriptPath}"`;
            this.sendInput(`${loadCommand}\r`);
            this.exited = false;

            this.process.onData((data) => {
                try {
                    this.output += data;
                    logger.debug(data);
                } catch (error) {
                    logger.debug("Error in onData:", error);
                    this.error = error;
                    this.exited = true;
                    reject(error);
                }
            });
            this.process.onExit(({ exitCode }) => {
                logger.debug("Exiting with code:>>>", exitCode, "<<<");
                this.exited = true;
                this.exitCode = exitCode;
                if (!this.error) {
                    resolve();
                }
            });

            this.process.on("error", (error) => {
                logger.debug("Process error:>>>>", error, "<<<<<<");
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
        if (this.process && !this.exited) {
            try {
                this.process.write(input);
            } catch (error) {
                logger.debug("Error sending input:>>>>", error, "<<<<<<<<<<<");
                this.error = error;
                this.exited = true;
            }
        } else {
            logger.debug("Attempted to send input, but process is not running");
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
                this.process.write("\x03"); // Ctrl+C

                // Set up a timeout
                const timer = setTimeout(() => {
                    logger.debug(
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
