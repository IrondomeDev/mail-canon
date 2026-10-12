"""Export the committed PHP package and shared fixtures for Packagist."""

import argparse
import json
from pathlib import Path
import shutil
import subprocess


def export(output: Path) -> None:
    root = Path(__file__).resolve().parents[1]
    output = output.resolve()
    if output.is_relative_to(root):
        raise SystemExit("Output must be outside the development repository.")
    if output.exists() and (not output.is_dir() or any(output.iterdir())):
        raise SystemExit("Output must be a new or empty directory.")
    changed = subprocess.check_output(
        ["git", "status", "--porcelain", "--", "php", "testdata", "tools"], cwd=root
    )
    if changed:
        raise SystemExit("Commit PHP, fixture, and export-tool changes before exporting.")
    revision = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip()
    files = subprocess.check_output(
        ["git", "ls-files", "-z", "php/", "testdata/"], cwd=root
    ).decode().split("\0")
    output.mkdir(parents=True, exist_ok=True)
    for name in filter(None, files):
        relative = Path(name)
        destination = output / (Path(*relative.parts[1:]) if relative.parts[0] == "php" else relative)
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(root / relative, destination)
    readme = output / "README.md"
    notice = (
        "> Generated distribution of [IronDome Mail Canon]"
        "(https://github.com/IrondomeDev/mail-canon). "
        "Submit issues and changes to the development repository.\n\n"
    )
    readme.write_text(notice + readme.read_text(encoding="utf-8"), encoding="utf-8", newline="\n")
    (output / ".gitattributes").write_text("* text=auto eol=lf\n", encoding="utf-8")
    (output / ".gitignore").write_text("vendor/\ncomposer.lock\n", encoding="utf-8")
    workflow = output / ".github/workflows/ci.yml"
    workflow.parent.mkdir(parents=True)
    shutil.copyfile(root / "tools/php-distribution-ci.yml", workflow)
    (output / "SOURCE.json").write_text(
        json.dumps({"repository": "https://github.com/IrondomeDev/mail-canon", "commit": revision,
                    "path": "php", "generator": "tools/export_php.py"}, indent=2) + "\n",
        encoding="utf-8", newline="\n",
    )
    print(f"Exported PHP distribution from {revision} to {output}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    export(parser.parse_args().output)
