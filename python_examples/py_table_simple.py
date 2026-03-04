from imports import *

column_widths = [100.0] * 4
width = sum(column_widths)

headers = ["str", "one", "two", "three"]
body = [
    [0.0, 1.0, 2.0, 3.0],
    [0.0, 2.0, 4.0, 6.0],
    [0.0, 3.0, 6.0, 9.0],
    [0.0, 4.0, 8.0, 12.0],
    [0.0, 5.0, 10.0, 15.0],
    [0.0, 6.0, 12.0, 18.0],
    [0.0, 7.0, 14.0, 21.0],
    [0.0, 8.0, 16.0, 24.0],
    [0.0, 9.0, 18.0, 27.0],
    [0.0, 10.0, 20.0, 30.0],
    [0.0, 11.0, 22.0, 33.0],
]
footers = ["", "", "", ""]


# Add the window
add_window(
        id="main", 
        title="Table Demo",
        size=(700, 600),
        center=True)

# Add the container for centering the table
add_container(
        window_id="main", 
        id="cont",
        width_fill=True, 
        height_fill=True,
        center=True,)


# The table is added.
table_id = add_table(
        window_id="main",
        table_id="table",
        headers=headers,
        body=body,
        footers=footers,
        parent_id="cont",
        column_widths=column_widths,
        height=150.0)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
