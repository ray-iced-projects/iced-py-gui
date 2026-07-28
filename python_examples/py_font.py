#!/usr/bin/env python3
"""
Font demo - shows how to use fonts, font styles, and icons.

This demonstrates:
- load_font(): Load custom fonts (.ttf, .otf)
- add_font_style(): Create fonts with specific properties (weight, style, stretch)
- add_icon(): Add bootstrap icons and arrows to widgets
- arrow_to_str(): Convert arrow enums to characters
- arrow_variants(): Get all available arrow variants
"""
import os
from icedpygui import (
    Window, start_session,
    Column, Row,
    add_text, add_button, add_space,
    add_font_style, load_font, add_icon,
    arrow_to_str, arrow_variants,
    FontWeight, FontStyle, FontStretch, FontFamily,
    Arrow, Icon,
)


# Get the current directory for font paths
cwd = os.getcwd()
FONTS_PATH = os.path.join(cwd, "src", "graphics", "fonts")

# ==============================================================================
# 1. LOAD CUSTOM FONTS
# ==============================================================================
# Must be called BEFORE start_session() to make fonts available
# Supports .ttf and .otf file formats
# The Newsreader font is actually already loaded but for this example,
# we'll pretend that it is a new font that you want to use.
# The already loaded fonts are FiraSans-Regular, Newsreader, and Roboto.
# The Newsreader font is moderately common and growing in popularity.
# It is a free, open-source serif typeface designed by Production Type
# and commissioned by Google Fonts. It is widely used for digital publishing,
# blogs, and corporate brand guidelines."


load_font(f"{FONTS_PATH}/Newsreader.ttf")

# ==============================================================================
# 2. CREATE FONT STYLES
# ==============================================================================
# add_font_style() creates a font configuration with specific properties.
# Use FontFamily, FontWeight, FontStyle, FontStretch enums to customize.

# Default system font
font_default = add_font_style()

# Newsreader font (loaded custom font by name)
font_newsreader = add_font_style(family_name="Newsreader")

# FiraSans with Bold weight
font_firasans_bold = add_font_style(
    family_name="FiraSans-Regular",
    weight=FontWeight.Bold
)

# Monospace font with different weights
font_mono_normal = add_font_style(family=FontFamily.Monospace)
font_mono_bold = add_font_style(
    family=FontFamily.Monospace,
    weight=FontWeight.Bold
)

# Sans Serif with Italic style
font_sans_italic = add_font_style(
    family=FontFamily.SansSerif,
    style=FontStyle.Italic
)

# Sans Serif with Italic and Bold
font_sans_bold_italic = add_font_style(
    family=FontFamily.SansSerif,
    weight=FontWeight.Bold,
    style=FontStyle.Italic
)

# Different font weights
font_thin = add_font_style(family_name="Newsreader", weight=FontWeight.Thin)
font_extra_light = add_font_style(
    family_name="Newsreader",
    weight=FontWeight.ExtraLight
)
font_light = add_font_style(family_name="Newsreader", weight=FontWeight.Light)
font_normal = add_font_style(family_name="Newsreader", weight=FontWeight.Normal)
font_medium = add_font_style(family_name="Newsreader", weight=FontWeight.Medium)
font_semibold = add_font_style(
    family_name="Newsreader",
    weight=FontWeight.Semibold
)
font_extra_bold = add_font_style(
    family_name="Newsreader",
    weight=FontWeight.ExtraBold
)
font_black = add_font_style(family_name="Newsreader", weight=FontWeight.Black)

# Different font stretches
font_condensed = add_font_style(
    family=FontFamily.SansSerif,
    stretch=FontStretch.Condensed
)
font_normal_stretch = add_font_style(
    family=FontFamily.SansSerif,
    stretch=FontStretch.Normal
)
font_expanded = add_font_style(
    family=FontFamily.SansSerif,
    stretch=FontStretch.Expanded
)

# ==============================================================================
# 3. CREATE ICONS
# ==============================================================================
# add_icon() creates icon descriptors that can be used in widgets
# You can use:
# - Bootstrap arrows (Arrow.*)
# - Bootstrap icons (Icon.*)
# - Custom fonts with code_point

# Bootstrap arrow icons
arrow_down_icon = add_icon(arrow=Arrow.ArrowDown, size=20)
arrow_up_icon = add_icon(arrow=Arrow.ArrowUp, size=20)
arrow_left_icon = add_icon(arrow=Arrow.ArrowLeft, size=20)
arrow_right_icon = add_icon(arrow=Arrow.ArrowRight, size=20)

# Bootstrap regular icons
folder_icon = add_icon(icon=Icon.Folder, size=24)
file_icon = add_icon(icon=Icon.File, size=24)
check_icon = add_icon(icon=Icon.Check, size=24)
alert_icon = add_icon(icon=Icon.Exclamation, size=24)

# ==============================================================================
# 4. ARROW CONVERSION AND VARIANTS
# ==============================================================================
# arrow_to_str() converts arrow enum to its character representation
# arrow_variants() returns all available arrow enum variants

arrow_char = arrow_to_str(Arrow.ArrowDown)
all_arrow_variants = arrow_variants()

# ==============================================================================
# BUILD THE DEMO UI
# ==============================================================================

with Window(title="Font Demo", center=True):
    with Row(width_fill=True):
        with Column(spacing=10, padding=[20]):

            # ======== SECTION 1: Font Family Examples ========
            add_text(content="=== FONT FAMILIES ===", size=20)
            add_text(content="Default (FiraSans):", size=16)
            add_text(
                content="Size 14: The quick brown fox jumps over the lazy dog",
                size=14
            )

            add_text(content="Newsreader (Pretend Custom Font):", font_id=font_newsreader, size=18)
            add_text(
                content="Size 16: The quick brown fox jumps over the lazy dog",
                font_id=font_newsreader,
                size=16
            )

            add_space(height=20)

            # ======== SECTION 2: Font Styles (Italic, Bold) ========
            add_text(content="=== FONT STYLES ===", size=16)

            add_text(content="Default-Normal, Size=14")
            add_text(
                content="The quick brown fox",
                font_id=font_default,
                size=14
            )

            add_text(content="Default-Italic:", font_id=font_sans_italic)
            add_text(
                content="The quick brown fox",
                font_id=font_sans_italic,
                size=14
            )

            add_text(content="Default-Bold:", font_id=font_firasans_bold)
            add_text(
                content="The quick brown fox",
                font_id=font_firasans_bold,
                size=14
            )

            add_text(content="Default-Bold Italic:", font_id=font_sans_bold_italic)
            add_text(
                content="The quick brown fox",
                font_id=font_sans_bold_italic,
                size=14
            )

            add_space(height=20)

            # ======== SECTION 3: Font Weights ========
            add_text(content="=== FONT WEIGHTS ===", size=16)
            add_text(content=("For the remaining cases, the Newsreader font is used\n"
                              "because it has better coverage for these styles\n"))

            weights = [
                ("Thin", font_thin, 18),
                ("ExtraLight", font_extra_light, 17),
                ("Light", font_light, 17),
                ("Normal", font_normal, 16),
                ("Medium", font_medium, 16),
                ("Semibold", font_semibold, 15),
                ("Bold", font_firasans_bold, 14),
                ("ExtraBold", font_extra_bold, 13),
                ("Black", font_black, 12),
            ]

            for label, font_id, size in weights:
                add_text(
                    content=f"{label}: The quick brown fox",
                    font_id=font_id,
                    size=size
                )

        with Column(spacing=5):
            # ======== SECTION 4: Font Stretches ========
            add_text(content="=== FONT STRETCHES ===", size=16)

            add_text(
                content="Condensed: The quick brown fox jumps over the lazy dog",
                font_id=font_condensed,
                size=12
            )
            add_text(
                content="Normal: The quick brown fox jumps over the lazy dog",
                font_id=font_normal_stretch,
                size=12
            )
            add_text(
                content="Expanded: The quick brown fox jumps over the lazy dog",
                font_id=font_expanded,
                size=12
            )

            add_space(height=20)

            # ======== SECTION 5: Arrow Icons in Buttons ========
            add_text(content="=== BOOTSTRAP ARROW ICONS ===", size=16)
            add_text(
                content="add_icon() with arrow parameter, then use in buttons with style_arrow:",
                size=11
            )

            with Column(spacing=5):
                add_button(label="Down", style_arrow=Arrow.ArrowDown)
                add_button(label="Up", style_arrow=Arrow.ArrowUp)
                add_button(label="Left", style_arrow=Arrow.ArrowLeft)
                add_button(label="Right", style_arrow=Arrow.ArrowRight)

            add_space(height=20)

            # ======== SECTION 6: Arrow Variants ========
            add_text(content="=== AVAILABLE ARROW VARIANTS ===", size=16)
            add_text(
                content=f"Total arrow variants: {len(all_arrow_variants)}",
                size=12
            )

            # Display first 8 arrow variants as examples
            with Column(spacing=5):
                for i, arrow in enumerate(all_arrow_variants[:8]):
                    arrow_str = arrow_to_str(arrow)
                    add_text(
                        content=f"Arrow {i}: {arrow_str}",
                        size=11
                    )

                add_text(
                    content=f"... and {len(all_arrow_variants) - 8} more",
                    size=10
                )

start_session()
