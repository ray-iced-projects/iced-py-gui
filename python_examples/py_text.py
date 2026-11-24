#!/usr/bin/env python3
"""
Text use demo
"""

from icedpygui import (Window, Column, Container, Row, start_session,
    TextParam, TextStyleStd, Color,
    ContainerStyleStd, add_radio, add_text, add_text_format, update_widget,
    add_font_style, add_text_style, FontWeight, FontStyle, load_font)

state = {"value": 0}
align_labels = [
    "AlignBottomLeft",
    "AlignBottomCenter",
    "AlignBottomRight",
    "AlignCenterLeft",
    "AlignCenter",
    "AlignCenterRight",
    "AlignTopLeft - default",
    "AlignTopCenter",
    "AlignTopRight"]

wrap_labels = [
    "Word — wrap at word boundaries; (default)",
    "Glyph — wrap at any character",
    "None - No wrapping",
    "WordOrGlyph — try word then glyph"]

std_colors = [
    "None — default",
    "Base",
    "Danger",
    "Primary",
    "Secondary",
    "Success",
    "Warning"]

def wrapping_selected(_radio_id: int, index: int):
    """When radio selected, the coresponding text has it's wrapping set"""
    match index:
        case 0:
            fmt_id_ = add_text_format()
            update_widget(txt_id, TextParam.FormatId, fmt_id_)
        case 1:
            fmt_id_ = add_text_format(wrapping_glyph=True)
            update_widget(txt_id, TextParam.FormatId, fmt_id_)
        case 2:
            fmt_id_ = add_text_format(wrapping_none=True)
            update_widget(txt_id, TextParam.FormatId, fmt_id_)
        case 3:
            fmt_id_ = add_text_format(wrapping_word_glyph=True)
            update_widget(txt_id, TextParam.FormatId, fmt_id_)


def set_widths(_radio_id: int, data: int):
    """Callback sets the width of the text"""
    match data:
        case 0:  # shrink (default)
            fmt_id_ = add_text_format()
            update_widget(set_txt_id, TextParam.FormatId, fmt_id_)
        case 1:  # fill
            fmt_id_ = add_text_format(fill=True)
            update_widget(set_txt_id, TextParam.FormatId, fmt_id_)
        case 2:  # fixed
            fmt_id_ = add_text_format(width=100.0)
            update_widget(set_txt_id, TextParam.FormatId, fmt_id_)


def text_alignment(_radio_id: int, align_index: int):
    """Callback sets the alignment of the text"""

    align_type = align_labels[align_index]
    # to see the alignment, the text box size must be large than the text content,
    # so fill is used in this case to fill the container with the text box.
    match align_type:
        case "AlignBottomLeft":
            fmt_id_ = add_text_format(align_bottom_left=True, fill=True)
            update_widget(align_txt_id, TextParam.FormatId, fmt_id_)
        case "AlignBottomCenter":
            fmt_id_ = add_text_format(align_bottom_center=True, fill=True)
            update_widget(align_txt_id, TextParam.FormatId, fmt_id_)
        case "AlignBottomRight":
            fmt_id_ = add_text_format(align_bottom_right=True, fill=True)
            update_widget(align_txt_id, TextParam.FormatId, fmt_id_)
        case "AlignCenterLeft":
            fmt_id_ = add_text_format(align_center_left=True, fill=True)
            update_widget(align_txt_id, TextParam.FormatId, fmt_id_)
        case "AlignCenter":
            fmt_id_ = add_text_format(align_center=True, fill=True)
            update_widget(align_txt_id, TextParam.FormatId, fmt_id_)
        case "AlignCenterRight":
            fmt_id_ = add_text_format(align_center_right=True, fill=True)
            update_widget(align_txt_id, TextParam.FormatId, fmt_id_)
        case "AlignTopLeft - default":
            fmt_id_ = add_text_format(align_top_left=True, fill=True)
            update_widget(align_txt_id, TextParam.FormatId, fmt_id_)
        case "AlignTopCenter":
            fmt_id_ = add_text_format(align_top_center=True, fill=True)
            update_widget(align_txt_id, TextParam.FormatId, fmt_id_)
        case "AlignTopRight":
            fmt_id_ = add_text_format(align_top_right=True, fill=True)
            update_widget(align_txt_id, TextParam.FormatId, fmt_id_)


def color_selected(_radio_id: int, index: int):
    """Callback set the color of the text"""
    match std_colors[index]:
        case "None — default":
            update_widget(color_txt, TextParam.StyleStd, None)
        case "Base":
            update_widget(color_txt, TextParam.StyleStd, TextStyleStd.Base)
        case "Danger":
            update_widget(color_txt, TextParam.StyleStd, TextStyleStd.Danger)
        case "Primary":
            update_widget(color_txt, TextParam.StyleStd, TextStyleStd.Primary)
        case "Secondary":
            update_widget(color_txt, TextParam.StyleStd, TextStyleStd.Secondary)
        case "Success":
            update_widget(color_txt, TextParam.StyleStd, TextStyleStd.Success)
        case "Warning":
            update_widget(color_txt, TextParam.StyleStd, TextStyleStd.Warning)


black_id = add_font_style(family_name="Roboto", weight=FontWeight.Black)
bold_id = add_font_style(family_name="Roboto", weight=FontWeight.Bold)
bold_ex_id = add_font_style(family_name="Roboto", weight=FontWeight.ExtraBold)
light_ex_id = add_font_style(family_name="Roboto", weight=FontWeight.ExtraLight)
light_id = add_font_style(family_name="Roboto", weight=FontWeight.Light)
medium_id = add_font_style(family_name="Roboto", weight=FontWeight.Medium)
normal_id = add_font_style(family_name="Roboto", weight=FontWeight.Normal)
bold_semi_id = add_font_style(family_name="Roboto", weight=FontWeight.Semibold)
thin_id = add_font_style(family_name="Roboto", weight=FontWeight.Thin)

# Load user fonts using load_font()
load_font("python_examples/resources/Newsreader/static/Newsreader_14pt-Regular.ttf")
load_font("python_examples/resources/Newsreader/static/Newsreader_14pt-BoldItalic.ttf")
news_regular_id = add_font_style(family_name="Newsreader 14pt")
news_bold_italic_id = add_font_style(family_name="Newsreader 14pt",
                                     weight=FontWeight.Bold, style=FontStyle.Italic)



# Add a window
with Window(title="Text Demo", size=[1000, 850], center=True):

    with Row(fill=True):
        # Left column
        with Column(width=250.0, height_fill=True,
                    spacing=5.0, padding=[10.0]):

            add_text(content="Text Size")

            # add container for the background
            with Container(width_fill=True, height=100.0,
                        style_std=ContainerStyleStd.BorderedBox):
                # add the row to hold the togglers
                with Column(spacing=10.0, padding=[10.0], fill=True):
                    add_text(content="Text Size (default=14.0)")
                    fmt_id = add_text_format(size=20.0)
                    add_text(content="Text Size: 20", format_id=fmt_id)

            with Column(width_fill=True, spacing=10.0):

                add_text(content="Text Line Height:\nRelative value")

                # add container for the background
                with Container(width_fill=True, height=200.0,
                            style_std=ContainerStyleStd.BorderedBox):

                    # add the column to hold the widgets
                    with Column(spacing=10.0, padding=[10.0], fill=True):

                        # adding a container with outline to show how the
                        # line height works
                        with Container(style_std=ContainerStyleStd.RoundedBox):
                            add_text(content="Text Line Height (default=1.3)")
                        with Container(style_std=ContainerStyleStd.RoundedBox):
                            fmt_id = add_text_format(line_height=5.0)
                            add_text(content="Text Line Height 5", format_id=fmt_id)

                add_text(content="This text is black weight", font_id=black_id)
                add_text(content="This text is bold weight", font_id=bold_id)
                add_text(content="This text is extra bold weight", font_id=bold_ex_id)
                add_text(content="This text is extra light weight", font_id=light_ex_id)
                add_text(content="This text is light weight", font_id=light_id)
                add_text(content="This text is medium weight", font_id=medium_id)
                add_text(content="This text is normal weight", font_id=normal_id)
                add_text(content="This text is semi bold weight", font_id=bold_semi_id)
                add_text(content="This text is thin weight", font_id=thin_id)
                add_text(content="Newsreader Regular (loaded via load_font)",
                         font_id=news_regular_id)
                add_text(content="Newsreader Bold Italic (loaded via load_font)",
                         font_id=news_bold_italic_id)


        # Middle column
        with Column(width=350, height_fill=True, spacing=10.0, padding=[20.0]):

            add_text(content="Text Wrapping, Width and Alignment:\n" + \
                     "Select radio buttons to see the effect")

            # add column to hold the widgets
            with Column(fill=True, spacing=20.0):
                add_text(content="Width types")
                add_radio(labels=["shrink (default)",
                                  "fill-for text same as shrink unless using aligh right",
                                  "fixed=100.0 - Causes a wrap since smaller"],
                    on_selected=set_widths,
                    radio_spacing=10.0,
                    selected_index=0)

                with Container(width_fill=True, style_std=ContainerStyleStd.RoundedBox):
                    fmt_id = add_text_format(width_fill=True)
                    set_txt_id = add_text(content="Select radio to set my width.", format_id=fmt_id)

                add_text(content="Alignment")
                add_radio(labels=align_labels,
                    on_selected=text_alignment,
                    radio_spacing=10.0,
                    selected_index=6)

                # used to highlight the text positon with a background color.
                with Container(width_fill=True, height=100,
                               style_std=ContainerStyleStd.RoundedBox):
                    # for the alignment to work, it's content box must be bigger than
                    # the text, so use fill=True to have the text box fill the container
                    # or some height and width.
                    fmt_id = add_text_format(fill=True)
                    align_txt_id = add_text(content="Select radio to set alignment.", format_id=fmt_id)

        # right column
        with Column(spacing=20.0, padding=[10.0], fill=True):

            add_text(content="Select wrapping type to show the effect")
            add_radio(labels=wrap_labels,
                    on_selected=wrapping_selected,
                    radio_spacing=10.0,
                    selected_index=0)
            # add container for the background and width sizing
            with Container(width=240.0, height=150.0,
                    style_std=ContainerStyleStd.BorderedBox) as txt_cont:

                txt_id = add_text(content="This is some text that will show wrapping" + \
                        " when the radio button is selected\n\n" + \
                        "ThisisalongwordthatwillshowWordOrGlyph")

            with Column(spacing=10.0, padding=[10.0], fill=True):
                add_text(content="Select a standard color")
                add_radio(labels=std_colors,
                        on_selected=color_selected,
                        radio_spacing=10.0,
                        selected_index=0)
                color_txt = add_text(content="This will change color when radio selected")

                # The color parameter will override the color_std parameter.
                style_id = add_text_style(color=Color.LIGHT_GREEN)
                add_text(content="This text was set by add_text_style() method", style_id=style_id)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
