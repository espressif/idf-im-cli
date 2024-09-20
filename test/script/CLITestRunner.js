import pty from "node-pty";
import os from "os";
import path from "path";

export class InteractiveCLITestRunner {
  constructor(exePath) {
    this.exePath = path.join(os.homedir(), exePath);
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

    this.process.onData((data) => {
      this.output += data;
      // console.log("received data:", data)
    });
    this.process.onExit((exitCode) => {
      // console.log("Exiting with code:", exitCode);
      this.exited = true;
      this.exitCode = exitCode;
    });
  }

  sendInput(input) {
    if (this.process && !this.exited) {
      try {
        this.process.write(input + "\r");
      } catch (error) {
        console.error("Error sending input:", error);
        this.error = error;
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
        await new Promise((resolve) => setTimeout(resolve, 100));
        // console.log("Output on exit>>>>>", this.output, "<<<<<<<<<<<<<<<<")
        return this.output.includes(expectedLastOutput);
      }
      if (this.error) {
        throw this.error;
      }
      await new Promise((resolve) => setTimeout(resolve, 100));
    }
    return false;
  }

  async stop() {
    if (this.process && !this.exited) {
      this.process.kill();
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
