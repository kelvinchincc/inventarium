# Filename: tasks.py
# Package required: invoke, watchdog
# invoke must be pip installed, globally or venv, watchdog just require watchmedo command, either
# `pip` or `uv tool` are fine.

import os
import subprocess

from invoke.context import Context
from invoke.tasks import task  # pyright: ignore[reportUnknownVariableType]


@task
def dev(_c: Context):
    """Run and watch the development server using watchdog."""

    # fmt: off
    _ = subprocess.run([
        "watchmedo", "auto-restart",
        "-d", ".",
        "-p", "**/*.rs;**/*.sql", # file types to watch, saperate with ';'
        "--recursive",
        "--no-restart-on-command-exit", # don't restart if command exit, typically compilation error
        "--debounce-interval", "5.0",
        "--",
        "cargo", "run"
    ], check=True)
    # fmt: on


@task
def reset_db(_c: Context):
    """
    Remove the sqlite3's db file and hence reset the database. Migrations should create them on next startup.
    """

    project_name = "inventarium"
    files_to_delete = [
        f"./{project_name}.db",
        f"./{project_name}.db-shm",
        f"./{project_name}.db-wal",
    ]

    for f in files_to_delete:
        print(f"Removing {f}...")
        try:
            os.remove(f)
        except FileNotFoundError:
            print(f"{f} not found, skipping.")
