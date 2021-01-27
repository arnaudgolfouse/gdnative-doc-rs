#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
@author: arnaudgolfouse
@brief:  script that get godot classes for the 3.2 branch. Note that this is probably not portable :/
"""

# %%
import subprocess
import os
from pathlib import Path

GODOT_REPOSITORY_URL = "https://github.com/godotengine/godot"
GODOT_REPOSITORY_PATH = "godot"
CLASSES_PATH = "doc/classes"
GODOT_CLASSES_FILE = "../gdnative-doc/godot_classes"

script_path = Path(__file__).absolute().parent
os.chdir(script_path)

# clone if not already done
if not Path(GODOT_REPOSITORY_PATH).exists():
    subprocess.call(["git", "clone", "--no-checkout", "--branch", "3.2",
                     "--single-branch", "--depth", "1", GODOT_REPOSITORY_URL, GODOT_REPOSITORY_PATH])

# sparse checkout
os.chdir(GODOT_REPOSITORY_PATH)
subprocess.call(["git", "sparse-checkout", "init", "--cone"])
subprocess.call(["git", "sparse-checkout", "set", CLASSES_PATH])

# get those files
file_names = "[\n"
for file in Path(CLASSES_PATH).glob("*"):
    file_name = file.name
    if (not file_name.startswith("@")) and file_name.endswith(".xml"):
        file_names += "    \""
        file_name = file_name[:-4]
        file_names += file_name
        file_names += "\",\n"
file_names += "]"
Path("../" + GODOT_CLASSES_FILE).write_text(file_names)
