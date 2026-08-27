import subprocess

from invoke.context import Context
from invoke.tasks import task  # pyright: ignore[reportUnknownVariableType]


@task
def dev(_c: Context):
    """
    Run the dev server with bacon

    Replace current process with bacon, `bacon run -- cargo run`
    """
    _ = subprocess.run(["bacon", "run", "--", "cargo", "run"], check=True)
