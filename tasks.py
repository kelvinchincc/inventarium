import subprocess

from invoke.context import Context
from invoke.tasks import task  # pyright: ignore[reportUnknownVariableType]


@task
def dev(_c: Context):
    """
    Run the dev server with watchdog

    Replace current process with watchmedo auto-restart, which will watch for changes in the source code and restart the
    server automatically. Watchdog must be installed first via pip or uv tool, eg: `uv tool install watchdog`. This is a
    workaround for the fact that bacon and watchexec-cli having issues with file watching on Windows. See
    https://github.com/Canop/bacon/issues/62
    """

    # fmt: off
    _ = subprocess.run([
        "watchmedo",
        "auto-restart",
        "-d", ".",
        "-p", "**/*.rs;**/*.sql;**/*.toml",
        "--recursive",
        "--",
        "cargo", "run"
    ], check=True)
    # fmt: on
