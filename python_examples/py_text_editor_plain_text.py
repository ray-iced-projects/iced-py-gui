#!/usr/bin/env python3
"""
Text Editor demo

This is just simplest example of the text_editor.
The highlighter parameter for the editor is not used
and therefore defaults to "txt", which is plain text.
See the highligter method for more details.
You could make this a complete text editor by adding a menu bar
with the needed droipdown items that point to varous method
such as loading/saving files, cut/paste, etc.
Given time, a demo method may be made in the furture.
"""

import os
from icedpygui import (Window, Container, start_session,
    add_text_editor)

cwd = os.getcwd()
FILE_PATH = os.path.join(cwd, "python_examples", "resources", "text_editor", "demo_file.txt")

try:
    with open(FILE_PATH, "r", encoding='utf-8') as file:
        content = file.read()
except FileNotFoundError:
    print(f"The file does not exist using {FILE_PATH}.")

with Window(title="Text Editor", center=True):
    with Container(fill=True, align_center=True):
        add_text_editor(content=content, fill=True)

start_session()
