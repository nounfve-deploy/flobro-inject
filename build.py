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


match platform.system():
    case "Windows":
        gh_platform = "windows-latest"
        exe_extension = ".exe"
    case "Linux":
        gh_platform = "ubuntu-latest"
        exe_extension = ""
    case _:
        raise Exception("unknoun system")
flobro_url = f"https://github.com/nounfve-deploy/flobro-app/releases/download/dev/dev-build-{gh_platform}.zip"

# build
exec("git clean -xfd ./working.temp/")
exec("npm run build")
exec("cargo build --release")
shutil.copy(f"target/release/webrtc_launcher{exe_extension}", "./working.temp/")

# package
os.chdir("./working.temp/")
exec(f"save_point meta --init")
exec(f"save_point meta include {flobro_url} --mount xbin")
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
