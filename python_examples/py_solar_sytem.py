#!/usr/bin/env python3
"""
Solor System using Canvas widgets demo

"""

import os
from icedpygui import Window, Column, Row, start_session, \
        Color


def point_on_circle(center_x, center_y, radius, angle):

    x = center_x + radius * math.cos(angle)
    y = center_y + radius * math.sin(angle)
    return [x, y]


def on_start(timer_id: int):
    update_widget(timer_id, CanvasTimerParam.Label, "Stop Timer")


# When the timer is stopped, 2nd parameter is sent for elapsed time.
def on_stop(timer_id: int, _elapsed: int):
    update_widget(timer_id, CanvasTimerParam.Label, "Start Timer")


def on_tick(timer_id: int, elapsed: int):
    earth_rotation = 3.142 * elapsed
    earth_orbit = earth_rotation/365.25
    moon_orbit = earth_rotation/27.3
    sun_rotation = earth_rotation/27
    earth_point = point_on_circle(canvas_width/2.0, canvas_height/2.0, 150.0, earth_orbit)
    update_canvas_item(earth_id, CanvasGeometryParam.Position, earth_point)
    update_canvas_item(earth_id, CanvasGeometryParam.Rotation, earth_rotation)
    moon_point = point_on_circle(earth_point[0], earth_point[1], 15.0, moon_orbit)
    update_canvas_item(moon_id, CanvasGeometryParam.Position, moon_point)
    update_canvas_item(sun_id, CanvasGeometryParam.Rotation, sun_rotation)


global default_file_path
cwd = os.getcwd()
sun_path = f"{cwd}/python_examples/resources/solar_system_assets/sun.png"
earth_path = f"{cwd}/python_examples/resources/solar_system_assets/earth.png"
moon_path = f"{cwd}/python_examples/resources/solar_system_assets/moon.png"

canvas_width = 800.0
canvas_height = 725.0

with Window(title="Canvas", size=(800.0, 800.0), centered=True):

    with Column(fill=True):

        with Row():

            add_canvas_timer(
                duration_ms=15,
                on_tick=on_tick,
                on_start=on_start,
                on_stop=on_stop)

        with Canvas(
            width=canvas_width,
            height=canvas_height,
            background_ipg_color=Color.BLACK)

            sun_id = add_canvas_image(
                     image_path=sun_path,
                     width=140.0,
                     height=140.0,
                     position_xy=(canvas_width/2.0, canvas_height/2.0),
                     align_center=True,
                     )

            earth_start = point_on_circle(canvas_width/2.0, canvas_height/2.0, 150.0, 0)

            earth_id = add_canvas_image(
                     image_path=earth_path,
                     width=24.0,
                     height=24.0,
                     position_xy=earth_start,
                     align_center=True,
                     )

            moon_start = point_on_circle(earth_start[0], earth_start[1], 15.0, 0)

            moon_id = add_canvas_image(
                     image_path=moon_path,
                     width=8.0,
                     height=8.0,
                     position_xy=moon_start,
                     align_center=True,
                     )

            earth_obit_id = add_circle(
                position_xy=(canvas_width/2.0, canvas_height/2.0),
                radius=150.0,
                stroke_width=1.0,
                stroke_ipg_color=Color.WHITE,
                stroke_color_alpha=0.1,
                stroke_dash_offset=0,
                stroke_dash_segments=[3.0, 6.0])

# generate a random star pattern
for _ in range(0, 100):
    x = float(random.randint(0, int(canvas_width)))
    y = float(random.randint(0, int(canvas_height)))
    add_rectangle(canvas_id="canvas",
                 top_left_xy=(x, y),
                 width=0.1,
                 height=0.1,
                 fill_ipg_color=Color.WHITE
                 )

start_session()
