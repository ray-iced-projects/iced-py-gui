from icedpygui import Window, Column, Container, MouseArea, Row, start_session, \
    add_image, add_svg, add_text, add_space, add_event_timer, \
    update_widget, update_timer, IpgImageParam, IpgSvgParam, IpgTimerParam, \
    IpgTextParam, IpgMousePointer

import os


# Setting up the image path
cwd = os.getcwd()

ferris = cwd + "/python_examples/resources/ferris_0.png"
tiger = cwd + "/python_examples/resources/tiger_0.svg"

# Global var for the ids.
ferris_ids = []
tiger_ids = []

show_ferris = [True, True, True, True, True]
show_tiger = [False, False, False, False, False]

text_ids = []
text_points = []

# Callback for when the image is selected
def image_selected(image_id: int, index: int):
    # Get the index of the image which is the index of the text widget
    update_widget(
            wid=text_ids[index], 
            param=IpgTextParam.Content, 
            value="You Pressed Me!")

# Callback for when the mouse is moving over the image.
def on_mouse_move(ma_id, point: list, index: int):
    x = '{:{}.{}}'.format(point[1], 10, 4)
    y = '{:{}.{}}'.format(point[3], 10, 4)
    update_widget(
            wid=text_points[index], 
            param=IpgTextParam.Content, 
            value=f"x={x}\ny={y}\n")


# On exit, reset the text widget
def on_mouse_exit(ma_id, index: int):
    update_widget(
            wid=text_points[index], 
            param=IpgTextParam.Content, 
            value="Point")


# On right_press, ferris shows
# We need to try both because the image_id could be either the tiger or ferris
def toggle_images(image_id, index: int):
    global show_ferris, show_tiger

    show_ferris[index] = not show_ferris[index]
    show_tiger[index] = not show_tiger[index]

    update_widget(wid=ferris_ids[index], param=IpgImageParam.Show, value=show_ferris[index])
    update_widget(wid=tiger_ids[index], param=IpgSvgParam.Show, value=show_tiger[index])


def on_tick(_timer_id: int, counter: int, _elapsed_ms: int):
    print(counter)
    radians = counter*0.048481
    update_widget(wid=ferris_ids[0], param=IpgImageParam.RotationRadians, value=radians)
    update_widget(wid=ferris_ids[1], param=IpgImageParam.RotationRadians, value=radians)
    update_widget(wid=ferris_ids[2], param=IpgImageParam.RotationRadians, value=radians)
    update_widget(wid=ferris_ids[3], param=IpgImageParam.RotationRadians, value=radians)

    update_widget(wid=tiger_ids[0], param=IpgSvgParam.RotationRadians, value=radians)
    update_widget(wid=tiger_ids[1], param=IpgSvgParam.RotationRadians, value=radians)
    update_widget(wid=tiger_ids[2], param=IpgSvgParam.RotationRadians, value=radians)
    update_widget(wid=tiger_ids[3], param=IpgSvgParam.RotationRadians, value=radians)
    
# add an event timer
timer_id = add_event_timer(duration_ms=1000, on_tick=on_tick)
value=False

def toggle_timer(_ma_id, _):
    global value
    value = not(value)
    update_timer(wid=timer_id, param=IpgTimerParam.Enable, value=value)



# Add the window
with Window(title="Date Picker Demo", center=True):

    # Add a column to hold the widgets
    with Column(fill=True, align_center=True, spacing=20):

        # Add a space for readability
        add_space(height=50.0)

        # Add some text info
        add_text(
                content="Pressing the left mouse button, while over an image, will display a message.  "
                "Pressing the right mouse button, while over the "
                "image, will toggle between ferris and the tiger.  "
                "While the mouse is over an image the the mouse position will be displayed.",
                width=600.0)


        # adding a row for the line of images
        with Row(spacing=20):
            
                # Looping to add the images, each will have the same callback
                # but they could be different depending on your needs.
                for i in range(0, 4):
                    with Column(spacing=20, align_center=True, width=200):
                        with Container(width=100, height=50):
                            with MouseArea(mouse_pointer=IpgMousePointer.Pointer,
                                        on_move=on_mouse_move,
                                        on_exit=on_mouse_exit,
                                        on_press=image_selected,
                                        on_right_press=toggle_images,
                                        on_middle_press=toggle_timer,
                                        user_data=i):
                                
                                ferris_ids.append(add_image(image_path=ferris))
                        
                                tiger_ids.append(add_svg(
                                                svg_path=tiger,
                                                width=150.0, 
                                                height=75.0,
                                                show=False))
                        
                        # Spacing was added last because because the two images occupy the same space
                        # So spacing is between the pairs
                        add_space(width=10.0)

                        # Add the text below each image.  There are a number of ways this could be done,
                        # Another way is to add a column with the image, info, and points then put the columns into row.
                        text_ids.append(add_text(
                                        content="Press image above me", 
                                        width=100.0))


                        text_points.append(add_text(
                                            content="Point", 
                                            width=100.0))


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
