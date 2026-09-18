#!/usr/bin/env python3

import os
import platform
import shutil
import subprocess
from pathlib import Path


def exec(cmd: str):
    subprocess.run(cmd, shell=True, check=True)


def append(path: str, content: str):
    os.makedirs(Path(path).absolute().parent, exist_ok=True)
    with open(path, "a") as file:
        file.write(content)


def flobro_url():
    match platform.system():
        case "Windows":
            build = "windows-latest"
        case "Linux":
            build = "ubuntu-latest"
        case _:
            raise Exception("unknoun system")
    return f"https://github.com/nounfve-deploy/flobro-app/releases/download/dev-0.1/dev-build-{build}.zip"


# build
exec("git clean -xfd ./working.temp/")
exec("npm run build")
exec("cargo build --release")
shutil.copy("target/debug/webrtc_launcher", "./working.temp/")

# package
os.chdir("./working.temp/")
exec(f"save_point meta include {flobro_url()} --mount xbin")
# fmt: off
append(".sip.dir/.sip.yaml", """\
exec:
  entry: webrtc_launcher
ignore:
  - main.*.js
""")
append(".sip.dir/build.yaml", """\
desc_template:
  title: a webrtc streaming page
  label:
  - tag: dev.sip.shelf/id
    val: webrtc_streaming
""")
# fmt: on
exec(f"save_point archive create")
exec(f"save_point archive export webrtc_streaming.{platform.system()}.zip")
