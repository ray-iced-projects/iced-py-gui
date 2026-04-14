#!/usr/bin/env python3
"""
Image use demo
"""
import os
from icedpygui import Window, Container, start_session, \
    add_image


# Setting up the image path
cwd = os.getcwd()

print(cwd)
tiger = cwd + "/python_examples/resources/png_svg/ferris.png"



# Add the window
with Window(title="Image Demo", center=True):

    with Container(align_center=True, fill=True):
        add_image(path=tiger, width=400.0, height=400.0)



# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
