from imports import *
import polars as pl

column_widths = [100.0] * 4
width = sum(column_widths)

data = {
    "str": ["H", "e", "l", "l", "o", " ", "W", "o", "r", "l", "d"],
    "one": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0],
    "two": [2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22],
    "three": [3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33],
    }

df = pl.DataFrame(data)
df_width = df.width
df_length = df.height


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
        polars_df=df,
        parent_id="cont",
        column_widths=column_widths,
        height=150.0)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
