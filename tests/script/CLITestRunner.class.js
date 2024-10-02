import pty from "node-pty";
import os from "os";

export class InteractiveCLITestRunner {
    constructor(exePath) {
        this.exePath = exePath;
        this.process = null;
        this.output = "";
        this.exited = false;
        this.exitCode = null;
        this.error = null;
    }

    getPlatformSpecificCommand(command, args) {
        if (os.platform() === "win32") {
            // Windows
            return {
                command: "powershell.exe",
                args: ["-Command", `& '${command}' ${args.join(" ")}`],
            };
        } else {
            // Linux, macOS, and other Unix-like systems
            return {
                command: command,
                args: args,
            };
        }
    }

    start(args = []) {
        return new Promise((resolve, reject) => {
            // console.log("Starting process...");
            const { command, args: fullArgs } = this.getPlatformSpecificCommand(
                this.exePath,
                args
            );
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
                    // console.log(data);
                } catch (error) {
                    console.error("Error in onData:", error);
                    this.error = error;
                    this.exited = true;
                    reject(error);
                }
                // console.log("received data:>>>>>>", data, "<<<<<<<<<<<<<");
            });
            this.process.onExit((exitCode) => {
                // console.log(
                //     "Exiting with code:>>>>>>>",
                //     exitCode,
                //     "<<<<<<<<<<<<"
                // );
                this.exited = true;
                this.exitCode = exitCode;
                if (!this.error) {
                    resolve();
                }
            });

            this.process.on("error", (error) => {
                // console.error("Process error:>>>>", error, "<<<<<<");
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
                console.error("Error sending input:>>>>", error, "<<<<<<<<<<<");
                this.error = error;
                this.exited = true;
            }
        } else {
            console.warn("Attempted to send input, but process is not running");
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
                // console.log(
                //     "Output on exit>>>>>",
                //     this.output,
                //     "<<<<<<<<<<<<<<<<"
                // );
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
                    console.warn(
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

    async runWithArgs(args) {
        const { command, args: fullArgs } = this.getPlatformSpecificCommand(
            this.exePath,
            args
        );
        return new Promise((resolve) => {
            const proc = pty.spawn(command, fullArgs, {
                name: "eim-terminal",
                cols: 80,
                rows: 30,
                cwd: process.cwd(),
                env: process.env,
            });

            let output = "";
            proc.onData((data) => {
                output += data;
            });

            proc.onExit(({ exitCode }) => {
                resolve({
                    output: output,
                    code: exitCode,
                });
            });
        });
    }
}
