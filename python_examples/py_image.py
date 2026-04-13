from icedpygui import Window, Column, Container, MouseArea, Row, start_session, \
    add_image

import os


# Setting up the image path
cwd = os.getcwd()


tiger = cwd + "/python_examples/resources/png_svg/tiger_0.svg"



# Add the window
with Window(title="Date Picker Demo", center=True):

    with Container(align_center=True):
        add_image(path=tiger)
        


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
