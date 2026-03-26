from icedpygui import Window, Container, Column, Row, start_session, \
    add_container, add_container_style, add_text, Scrollable, IpgColor, \
    get_color_palette, get_rgba_color


"""
Styling color notes:

Of the three containers, Container, Column, and Row, the only one that has styling is the container. If you want styling for the other two, place them within a Container.

If no styling is supplied, then the default theme styling will be used.  However, you can used the style_standard option for each widget to select a limited number of styles such as primary, success, and danger.  However, you can use any of the IpgColors to style your widgets.  There are 155 colors (https://www.w3schools.com/cssref/css_colors.php) to choose from or make up you own using the rgba option. 

Internally to Iced, there are 4 special color parameters used in add_styling_colors, base, strong, weak, and text. These colors are used to color multiple parts of the widget to show mouse hover and drag effects, etc.  These colors are calculated mostly using the background base color.  There are some where the theme primary color is used which is a predefined color for each theme color.

To keep things simple, when you want to add colors to your widgets, IPG allows you to define the colors you want by using terms like background_color or background_rgba_color.  In some cases if you define, for example, a new background and nothing more, your background color may be incorporated and used to define other colors for the widget using a background strong or weak color.

This program generates the base, strong and weak colors so that you can see how the colors are used. If you have another color you would like to add, just use get_color_palette(base_rgba=[your colors]).
"""

colors = [IpgColor.PRIMARY, IpgColor.SECONDARY, IpgColor.SUCCESS, IpgColor.DANGER, IpgColor.WARNING, 
          IpgColor.INFO, IpgColor.LIGHT, IpgColor.DARK, IpgColor.ALICE_BLUE, IpgColor.ANTIQUE_WHITE, 
          IpgColor.AQUA, IpgColor.AQUAMARINE, IpgColor.AZURE, IpgColor.BEIGE, IpgColor.BISQUE, 
          IpgColor.BLACK, IpgColor.BLANCHED_ALMOND, IpgColor.BLUE, IpgColor.BLUE_VIOLET, IpgColor.BROWN, 
          IpgColor.BURLY_WOOD, IpgColor.CADET_BLUE, IpgColor.CHARTREUSE, IpgColor.CHOCOLATE, IpgColor.CORAL, 
          IpgColor.CORNFLOWER_BLUE, IpgColor.CORNSILK, IpgColor.CRIMSON, IpgColor.CYAN, IpgColor.DARK_BLUE, 
          IpgColor.DARK_CYAN, IpgColor.DARK_GOLDEN_ROD, IpgColor.DARK_GRAY, IpgColor.DARK_GREY, 
          IpgColor.DARK_GREEN, IpgColor.DARK_KHAKI, IpgColor.DARK_MAGENTA, IpgColor.DARK_OLIVE_GREEN, 
          IpgColor.DARK_ORANGE, IpgColor.DARK_ORCHID, IpgColor.DARK_RED, IpgColor.DARK_SALMON, 
          IpgColor.DARK_SEA_GREEN, IpgColor.DARK_SLATE_BLUE, IpgColor.DARK_SLATE_GRAY, IpgColor.DARK_SLATE_GREY, 
          IpgColor.DARK_TURQUOISE, IpgColor.DARK_VIOLET, IpgColor.DEEP_PINK, IpgColor.DEEP_SKY_BLUE, 
          IpgColor.DIM_GRAY, IpgColor.DIM_GREY, IpgColor.DODGER_BLUE, IpgColor.FIRE_BRICK, IpgColor.FLORAL_WHITE, 
          IpgColor.FOREST_GREEN, IpgColor.FUCHSIA, IpgColor.GAINSBORO, IpgColor.GHOST_WHITE, IpgColor.GOLD, 
          IpgColor.GOLDEN_ROD, IpgColor.GRAY, IpgColor.GREY, IpgColor.GREEN, IpgColor.GREEN_YELLOW, IpgColor.HONEY_DEW, 
          IpgColor.HOT_PINK, IpgColor.INDIAN_RED, IpgColor.INDIGO, IpgColor.IVORY, IpgColor.KHAKI, IpgColor.LAVENDER, 
          IpgColor.LAVENDER_BLUSH, IpgColor.LAWN_GREEN, IpgColor.LEMON_CHIFFON, IpgColor.LIGHT_BLUE, IpgColor.LIGHT_CORAL, 
          IpgColor.LIGHT_CYAN, IpgColor.LIGHT_GOLDEN_ROD_YELLOW, IpgColor.LIGHT_GRAY, IpgColor.LIGHT_GREY, 
          IpgColor.LIGHT_GREEN, IpgColor.LIGHT_PINK, IpgColor.LIGHT_SALMON, IpgColor.LIGHT_SEA_GREEN, 
          IpgColor.LIGHT_SKY_BLUE, IpgColor.LIGHT_SLATE_GRAY, IpgColor.LIGHT_SLATE_GREY, IpgColor.LIGHT_STEEL_BLUE, 
          IpgColor.LIGHT_YELLOW, IpgColor.LIME, IpgColor.LIME_GREEN, IpgColor.LINEN, IpgColor.MAGENTA, IpgColor.MAROON, 
          IpgColor.MEDIUM_AQUA_MARINE, IpgColor.MEDIUM_BLUE, IpgColor.MEDIUM_ORCHID, IpgColor.MEDIUM_PURPLE, 
          IpgColor.MEDIUM_SEA_GREEN, IpgColor.MEDIUM_SLATE_BLUE, IpgColor.MEDIUM_SPRING_GREEN, IpgColor.MEDIUM_TURQUOISE, 
          IpgColor.MEDIUM_VIOLET_RED, IpgColor.MIDNIGHT_BLUE, IpgColor.MINT_CREAM, IpgColor.MISTY_ROSE, IpgColor.MOCCASIN, 
          IpgColor.NAVAJO_WHITE, IpgColor.NAVY, IpgColor.OLD_LACE, IpgColor.OLIVE, IpgColor.OLIVE_DRAB, IpgColor.ORANGE, 
          IpgColor.ORANGE_RED, IpgColor.ORCHID, IpgColor.PALE_GOLDEN_ROD, IpgColor.PALE_GREEN, IpgColor.PALE_TURQUOISE, 
          IpgColor.PALE_VIOLET_RED, IpgColor.PAPAYA_WHIP, IpgColor.PEACH_PUFF, IpgColor.PERU, IpgColor.PINK, IpgColor.PLUM,
          IpgColor.POWDER_BLUE, IpgColor.PURPLE, IpgColor.REBECCA_PURPLE, IpgColor.RED, IpgColor.ROSY_BROWN, 
          IpgColor.ROYAL_BLUE, IpgColor.SADDLE_BROWN, IpgColor.SALMON, IpgColor.SANDY_BROWN, IpgColor.SEA_GREEN, 
          IpgColor.SEA_SHELL, IpgColor.SIENNA, IpgColor.SILVER, IpgColor.SKY_BLUE, IpgColor.SLATE_BLUE, 
          IpgColor.SLATE_GRAY, IpgColor.SLATE_GREY, IpgColor.SNOW, IpgColor.SPRING_GREEN, IpgColor.STEEL_BLUE, 
          IpgColor.TAN, IpgColor.TEAL, IpgColor.THISTLE, IpgColor.TOMATO, IpgColor.TRANSPARENT, IpgColor.TURQUOISE, 
          IpgColor.VIOLET, IpgColor.WHEAT, IpgColor.WHITE, IpgColor.WHITE_SMOKE, IpgColor.YELLOW, IpgColor.YELLOW_GREEN]

# global to define the 3 column widths
widths = [250, 150, 150]
headers = ["Base Color", "Weak Color", "Strong Color"]

# Add the window first
with Window(title="Colors", center=True, size=(550, 600)):

    # add row with some padding on top
    with Row(width_fill=True):

        for i in range(0, 3):
            # Add the 3 containers for the header text
            with Container(width=widths[i], align_center=True):
                add_text(content=headers[i])

    # Add a scrollable container for all of the colors
    with Scrollable(height=550.0, width_fill=True):

        # Add a column container to hold everything
        with Column(width_fill=True):

            for (i, color) in enumerate(colors):
                # Add a row for each color set
                with Row():

                    # get the name by cutting off the first 9 characters
                    color_name = str(color)[9:]

                    # Get the 3 colors based on the given IpgColor
                    # These return colors are rgba format
                    # You can also supply the color in rgba format base_rgba=[]
                    (strong, weak, text) = get_color_palette(base_color=color)

                    # create styling for the 3 containers
                    style_base = add_container_style(
                                    background_color=color,
                                    text_rgba=text)
                    
                    style_strong = add_container_style(
                                    background_rgba=strong,
                                    text_rgba=text)
                    
                    style_weak = add_container_style(
                                    background_rgba=weak,
                                    text_rgba=text)

                    # add the 3 containers
                    with Container(
                            style_id= style_base,
                            width=widths[0], height=30.0,
                            align_center=True):
                        add_text(content=f"{color_name}")
                        
                    add_container(
                        style_id= style_strong,
                        width=widths[1], 
                        height=30.0)
                    
                    add_container(
                        style_id= style_weak,
                        width=widths[2], height=30.0)

                    
# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
