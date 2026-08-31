import subprocess
from typing import override

from invoke.context import Context
from invoke.tasks import task  # pyright: ignore[reportUnknownVariableType]
from watchdog.events import FileSystemEventHandler
from watchdog.observers import Observer


class DevHandler(FileSystemEventHandler):
    """Handler that restarts the cargo process on file changes"""

    def __init__(self):
        self.process = None  # pyright: ignore[reportUnannotatedClassAttribute]
        self.should_restart = False  # pyright: ignore[reportUnannotatedClassAttribute]
        self.start_cargo()

    def start_cargo(self):
        """Start the cargo run process"""
        if self.process:
            self.process.terminate()
            try:
                _ = self.process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                self.process.kill()
                _ = self.process.wait()

        print("\n🔄 Starting cargo run...\n")
        self.process = subprocess.Popen(["cargo", "run"])

    @override
    def on_modified(self, event):  # pyright: ignore[reportMissingParameterType]
        """Called when a file is modified"""
        if event.is_directory:
            return

        if event.src_path.endswith((".rs", ".sql", ".toml")):  # pyright: ignore[reportArgumentType]
            print(f"\n📝 File changed: {event.src_path}")
            self.start_cargo()

    def stop(self):
        """Clean shutdown"""
        if self.process:
            self.process.terminate()
            try:
                _ = self.process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                self.process.kill()
                _ = self.process.wait()


@task
def dev(_c: Context):
    """Run the dev server with file watching

    Press 'r' + Enter to manually restart
    Press Ctrl+C to stop
    """
    handler = DevHandler()
    observer = Observer()

    _ = observer.schedule(handler, "src", recursive=True)
    _ = observer.schedule(handler, "migrations", recursive=True)

    observer.start()

    print(
        "\n✅ Watching for changes... (Press 'r' + Enter to restart, Ctrl+C to stop)\n"
    )

    try:
        while True:
            user_input = input()
            if user_input.lower() == "r":
                print("🔄 Manual restart triggered")
                handler.start_cargo()
    except KeyboardInterrupt:
        print("\n⛔ Stopping...")
        observer.stop()
        handler.stop()

    observer.join()
