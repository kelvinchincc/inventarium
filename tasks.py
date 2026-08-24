# /// script
# requires-python = ">=3.14"
# dependencies = [
#     "typer",
# ]
# ///

import subprocess

from typer import Typer

app = Typer()


@app.command()
def dev():
    """
    Run the dev server with bacon

    Replace current process with bacon, `bacon run -- cargo run`
    """

    _ = subprocess.run(["bacon", "run", "--", "cargo", "run"], check=True)


if __name__ == "__main__":
    app()
