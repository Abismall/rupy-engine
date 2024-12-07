import subprocess
import threading
import time


class RustProcessManager:
    def __init__(self, script_path):
        self.script_path = script_path
        self.process = None
        self.running = False

    def start_rust_process(self):
        try:
            self.process = subprocess.Popen(
                ["cmd.exe", "/c", self.script_path],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                shell=True
            )
            self.running = True
            print(f"Rust process started with PID: {self.process.pid}")

            threading.Thread(target=self._monitor_stdout, daemon=True).start()
            threading.Thread(target=self._monitor_stderr, daemon=True).start()

            threading.Thread(target=self._monitor_process, daemon=True).start()

        except Exception as e:
            print(f"Failed to start Rust process: {e}")
            self.running = False

    def _monitor_stdout(self):
        if self.process and self.process.stdout:
            for line in iter(self.process.stdout.readline, ''):
                print(f"[Rust STDOUT]: {line.strip()}")
        print("Stopped reading Rust stdout.")

    def _monitor_stderr(self):
        if self.process and self.process.stderr:
            for line in iter(self.process.stderr.readline, ''):
                print(f"[Rust STDERR]: {line.strip()}")
        print("Stopped reading Rust stderr.")

    def _monitor_process(self):
        if self.process:
            self.process.wait()
            self.running = False
            print("Rust process has exited.")

    def is_running(self):
        return self.running

    def stop_rust_process(self):
        if self.process and self.process.poll() is None:
            print(f"Terminating Rust process with PID: {self.process.pid}")
            self.process.terminate()
            self.process.wait()
            self.running = False
            print("Rust process terminated.")
