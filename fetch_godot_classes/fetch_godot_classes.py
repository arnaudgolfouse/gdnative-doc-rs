#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
@author: arnaudgolfouse
@brief:  script that get godot classes for the 3.2, 3.3, 3.4 and 3.5 branches. Note that this is probably not portable :/
"""

# %%
import subprocess
import os
from pathlib import Path

GODOT_REPOSITORY_URL = "https://github.com/godotengine/godot"
GODOT_REPOSITORY_PATH = "godot"
CLASSES_PATH = "doc/classes"
VERSIONS = ["3.2", "3.3", "3.4", "3.5"]


script_path = Path(__file__).absolute().parent
os.chdir(script_path)

if not Path(GODOT_REPOSITORY_PATH).exists():
    os.mkdir(GODOT_REPOSITORY_PATH)
    os.chdir(GODOT_REPOSITORY_PATH)

    subprocess.call(["git", "init"])

    git_remote_add_args = ["git", "remote", "add"]
    for version in VERSIONS:
        git_remote_add_args.append("-t")
        git_remote_add_args.append(version)
    git_remote_add_args.append("origin")
    git_remote_add_args.append(GODOT_REPOSITORY_URL)
    subprocess.call(git_remote_add_args)

    subprocess.call(["git", "fetch", "--depth", "1"])

os.chdir(script_path)

for version in VERSIONS:
    godot_classes_file: str = f"godot_classes-{version}.txt"

    # change branch
    os.chdir(GODOT_REPOSITORY_PATH)
    subprocess.call(["git", "checkout", version])

    # get those files
    classes_names = []
    for file in Path(CLASSES_PATH).glob("*"):
        file_name = file.name
        if (not file_name.startswith("@")) and file_name.endswith(".xml"):
            classes_names.append(file_name[:-4])
    classes_names.sort()
    os.chdir(script_path)

    file_content = "// This file was automatically generated from the file names at " + \
        GODOT_REPOSITORY_URL + "/tree/" + version + "/doc/classes\n"
    file_content += "[\n"
    for class_name in classes_names:
        file_content += f"    \"{class_name}\",\n"
    file_content += "]"
    file_path = Path(godot_classes_file).absolute()
    file_path.write_text(file_content)
