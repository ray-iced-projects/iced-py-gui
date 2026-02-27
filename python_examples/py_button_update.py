from imports import *

# In this example, we'll use a timer to update the buuton style.
# This demonstratesusing a timer to doing something and how to update

# A callback id is the id of the widget making the callback.  If you need
# update other widgets, use their ids as show in this callback.
def update_button(btn_id: int):
    # changing the radius using a float
    # Since the radius is a style, then we are adding a styling id
    # which is defined below.
    update_widget(button_id, 
        IpgButtonParam.StyleId, 
        style_id,
        )
    
    update_widget(button_id, 
        IpgButtonParam.Label, 
        "Corner Radius Changed")

    # changing the label
    update_widget(label_btn, 
        IpgButtonParam.Label, 
        "Label Changed")

    # Changing the width
    update_widget(width_btn, 
        IpgButtonParam.Width, 
        300.0)
    
    update_widget(width_btn, 
        IpgButtonParam.Label, 
        "Width Changed")

    # Changing the height
    update_widget(height_btn, 
        IpgButtonParam.Height, 
        100.0)
    
    update_widget(height_btn, 
        IpgButtonParam.Label, 
        "Height Changed")

    # Changing the padding around the label
    update_widget(padding_btn, 
        IpgButtonParam.Padding, 
        [30.0])
    
    update_widget(padding_btn, 
        IpgButtonParam.Label, 
        "Padding Changed")

    # Changing the style
    update_widget(style_btn, 
        IpgButtonParam.StyleStandard, 
        value=IpgButtonStyleStandard.Text)
    
    update_widget(style_btn, 
        IpgButtonParam.Label, 
        "Styling Changed")

    # Changing the Arrow
    update_widget(arrow_btn, 
        IpgButtonParam.ArrowStyle, 
        IpgArrow.ArrowDown)

    # Hide the button
    update_widget(show_btn, 
        IpgButtonParam.Show, 
        False)
    
    # Show the button
    update_widget(hidden_btn, 
        IpgButtonParam.Show, 
        True)

counter = 0
def on_press(timer_id: int):
    global counter
    print(counter)
    counter += 1*3
    
    if counter > 10:
        counter = 0
        update_widget(style_id, IpgButtonStyleParam.BorderRadius, [0.0])
        
    update_widget(style_id, IpgButtonStyleParam.BorderRadius, [float(counter)*3.0])


# Add styling to change the border radius or many other properties.
style_id = add_button_style(border_radius=[20.0])

# A window widget needs to be added first, except for styles.
add_window(window_id="main", 
               title="Button Update", 
               size=(500, 650),
               center=True)

# Adding a container helps in aligning widgets since it has an x and y centering.
# The IpgContainerAlignment.Center is used to center widgets.  The container defaults
# to center so these are not needed in this case but put in to show use.
# A container can have only one widget, so generally a column or row follows.
# THis container may or may not be needed, it depends on your layout.
add_container(window_id="main", 
                  container_id="cont",
                  width_fill=True, 
                  height_fill=True,
                  center=True)

# A column is added next because the plan is to arrange then in a column.
# If you  set the width_fill or height_fill to true when the outer container
# is also true usually doesn't work, the column or row will expand out of view.
# The containers width_fill defaults to shrink to keep it that way unless needed.
# Sometime you'll need to give them specific amounts to get alignments correct for your layout
add_column(
        window_id="main", 
        container_id="col", 
        parent_id="cont",
        spacing=10.0,
        align_x=IpgAlignment.Center)

# This is the only active button needed for this demo, so it's the only one with a callback
# On some IDE setting, when you type in the callback name, it puts a () after the name.
# If this happens, simply remove the ().  If you leave it in, you will get an error
# about missing parameters.  This is not a function that is called but a python object
# passed to rust to let it know what function needs to be called from rust.
add_button(
        parent_id="col", 
        label="Press to Change Buttons Below", 
        on_press=on_press)

# This timer button will change the button style widget
# add_timer(
#         parent_id="col", 
#         duration_ms=500,
#         label="Continually change the button radius by press this button",
#         on_tick=on_tick)

# The radius of this button is styled by using the style_id defined above.

button_id = add_button(
                parent_id="col", 
                style_id=style_id,
                label="Corner Radius Will Change")

label_btn = add_button(
                parent_id="col", 
                label="Label Will Change")

width_btn = add_button(
                parent_id="col", 
                label="Width Will Change")

height_btn = add_button(
                parent_id="col", 
                label="Height Will Change")

padding_btn = add_button(
                parent_id="col", 
                label="Padding Will Change")

style_btn = add_button(
                parent_id="col", 
                label="Styling Will Change")

# On many parameters that are updated, you will need to import the proper
# dataclass so that the parameter can be selected.  In this case, you are working
# with a button arrow, so import the IpgButtonArrow and select the one you want.
# This method greatly cuts down on typos, if you had to use strings for the parameters.
arrow_btn = add_button(
                parent_id="col", 
                label="",
                style_arrow=IpgArrow.ArrowUp)

show_btn = add_button(
                parent_id="col", 
                label="This button will disappear")

hidden_btn = add_button(
                parent_id="col", 
                label="This button was hidden",
                show=False)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
