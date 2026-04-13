from icedpygui import Window, Container, Column, Row, Stack, start_session, \
    update_widget, ColumnParam, DividerParam, DividerDirection, TextParam, \
    add_container_style, Color, add_divider_style, add_text, \
    add_button, add_divider, add_toggler, ContainerStyleStd

def divider_change(div_id: int, data: tuple[int, float]):
    index, value = data
    # Get the difference to be added to the right column
    diff = heights[index] - value
    
    # Update the top locally and in ipg
    heights[index] = value
    update_widget(
            wid=column_ids[index],
            param=ColumnParam.Height,
            value=value)
    
    # Update the bottom locally and in ipg
    if index < len(heights)-1:
            heights[index+1] += diff
            update_widget(
                wid=column_ids[index+1],
                param=ColumnParam.Height,
                value=heights[index+1])
    
    # Update the divider
    update_widget(
                wid=div_id,
                param=DividerParam.Sizes,
                value=heights)
    
    # Update the two text items
    update_widget(wid=text_ids[index],
                    param=TextParam.Content,
                    value=f"Width={value}")
    
    if index < len(heights)-1:
        update_widget(wid=text_ids[index+1],
                    param=TextParam.Content,
                    value=f"Width={heights[index+1]}")


heights = [175.0, 175.0]
column_ids = []
text_ids = []
handle_width = 200.0  
handle_height = 4.0
        
cont_style_id = add_container_style(border_color=Color.YELLOW,
                                        border_width=4.0)

divider_style_id = add_divider_style(background_transparent=True)


# Add a window first
with Window( 
        title="CheckBox Demo",
        size=(600, 600),  
        center=True):

    # Add a container to center the widgets in the middle
    with Container(fill=True, align_center=True):

        # add a column to hold the text and the stack
        with Column(spacing=20.0):

            content = "Place the cursor over the divider and drag"

            add_text(content=content)

            # make the stack to lay the dividers over the containers
            with Stack():

                # make a column to hold the two columns
                # this is added to stack
                # The outer container used in the stack 
                # cannot have any padding, since divider
                # cannot detect whether padding is used
                # it becomes misaligned.
                with Column(width=handle_width):

                    for index, height in enumerate(heights):
                        # add a container for styling purposes
                        with Container(style_std=ContainerStyleStd.BorderedBox):
                            with Column(
                                    width=handle_width,
                                    height=height,
                                    spacing=10) as col:
                                
                                column_ids.append(col)
                                
                                text_ids.append(add_text(content=f"Width={height}"))
                                
                                add_button(label="Some Button")
                                
                                add_button(label="Another Button")

                                add_toggler(label="Toggler")

                # Make the divider
                add_divider(
                        direction=DividerDirection.Vertical,
                        sizes=heights,
                        handle_width=handle_width,
                        handle_height=handle_height,
                        on_change=divider_change)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
