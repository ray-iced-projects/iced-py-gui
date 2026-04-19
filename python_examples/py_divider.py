#!/usr/bin/env python3
"""
Divider demo
"""

from icedpygui import Window, Column, Container, Row, start_session, \
    update_widget, ContainerParam, DividerParam, RowParam, Stack, StackParam, \
    add_text, add_divider, add_divider_style, DividerDirection, Color, add_container_style


def divider_row_change(div_id: int, data: tuple[int, float]):
    """Divider callback row changed"""
    index = data[0]
    value = data[1]
    # Get the difference to be added to the below row
    diff = state["rows"][index] - value

    # Update the local data
    state["rows"][index] = value
    if index < len(state["rows"])-1:
        state["rows"][index+1] += diff

    # Update the row above the divider
    update_widget(
        state["row_ids"][index],
        RowParam.Height,
        value)

    # Update the row below the divider
    if index < len(state["rows"])-1:
        update_widget(
            state["row_ids"][index+1],
            RowParam.Height,
            state["rows"][index+1])

    # Update the divider
    update_widget(
        div_id,
        DividerParam.Sizes,
        state["rows"])

    # Update the height of the column divider
    update_widget(
        col_div,
        DividerParam.HandleHeight,
        sum(state["rows"])
    )


def divider_col_change(div_id: int, data: tuple[int, float]):
    """Divider callback column changed"""
    index = data[0]
    value = data[1]
    # get the difference to be added to the right side of the divider
    diff = state["columns"][index] - value

    # update the local values for storing the data
    state["columns"][index] = value
    if index < len(state["columns"])-1:
        state["columns"][index+1] += diff

    for _ in range(0, len(state["rows"])):
        # Update all the containers on the left of the divider
        update_widget(
            state["container_ids"][i][index],
            ContainerParam.Width,
            value)

        # Update all the containers on the right of the divider
        if index < len(state["columns"])-1:
            update_widget(
                state["container_ids"][i][index+1],
                ContainerParam.Width,
                state["columns"][index+1])

    # Update the column divider
    update_widget(
        div_id,
        DividerParam.Sizes,
        state["columns"])

    # Update the width of the row divider
    update_widget(
        row_div,
        DividerParam.HandleWidth,
        sum(state["columns"])
    )

    # Update the stack width
    update_widget(
        stack_id,
        StackParam.Width,
        sum(state["columns"]))


state = {
    "rows": [100.0, 100.0, 100.0],
    "columns": [150.0, 148.0],
    "row_ids": [],
    "container_ids": [],

    "row_handle_width": 298.0,
    "row_handle_height": 4.0,

    "col_handle_width": 4.0,
    "col_handle_height": 300.0,
}

cont_style_id = add_container_style(
                        border_color=Color.WHITE,
                        border_width=1.0)

divider_style_id = add_divider_style(
                        background_transparent=True)


# Add a window first
with Window(title="Divider Demo",
            size=(600, 600), center=True):

    # Add a container to center the widgets in the middle
    with Container(fill=True, padding=[100, 0, 0, 100]):

        # add a column to hold the text and the stack
        with Column(spacing=30):

            add_text(content="Pace the cursor over the highlighted divider and drag")

            # make the stack to lay the dividers over the containers
            with Stack() as stack_id:

                # make a column to hold the two columns
                # this is added to stack
                with Column(width=state["row_handle_width"]):
                    for i, height in enumerate(state["rows"]):
                        with Row(height=height) as row_id:
                            state["row_ids"].append(row_id)

                            for j, width in enumerate(state["columns"]):
                                with Container(width=width,
                                    height_fill=True,
                                    style_id=cont_style_id) as cont_id:

                                    add_text(content=f"Cell {i} {j}")

                        state["container_ids"].append(cont_id)


                # Make the vertical divider (rows)
                row_div = add_divider(
                            direction=DividerDirection.Vertical,
                            sizes=state["rows"],
                            handle_width=state["row_handle_width"],
                            handle_height=state["row_handle_height"],
                            on_change=divider_row_change,
                            # use the style to see just the outline and not the divider
                            # style_id=divider_style_id
                            )

                #Make the horizontal divider (columns)
                col_div = add_divider(
                            direction=DividerDirection.Horizontal,
                            sizes=state["columns"],
                            handle_width=state["col_handle_width"],
                            handle_height=state["col_handle_height"],
                            on_change=divider_col_change,
                            # use the style to see just the outline and not the divider
                            # style_id=divider_style_id
                            )

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
