from icedpygui import Window, Container, Column, Row, start_session, \
    add_container, add_container_style, add_text, Scrollable, Color, \
    get_color_palette, get_rgba_color


"""
Styling color notes:

Of the three containers, Container, Column, and Row, the only one that has styling is the container. If you want styling for the other two, place them within a Container.

If no styling is supplied, then the default theme styling will be used.  However, you can used the style_standard option for each widget to select a limited number of styles such as primary, success, and danger.  However, you can use any of the IpgColors to style your widgets.  There are 155 colors (https://www.w3schools.com/cssref/css_colors.php) to choose from or make up you own using the rgba option. 

Internally to Iced, there are 4 special color parameters used in add_styling_colors, base, strong, weak, and text. These colors are used to color multiple parts of the widget to show mouse hover and drag effects, etc.  These colors are calculated mostly using the background base color.  There are some where the theme primary color is used which is a predefined color for each theme color.

To keep things simple, when you want to add colors to your widgets, IPG allows you to define the colors you want by using terms like background_color or background_rgba_color.  In some cases if you define, for example, a new background and nothing more, your background color may be incorporated and used to define other colors for the widget using a background strong or weak color.

This program generates the base, strong and weak colors so that you can see how the colors are used. If you have another color you would like to add, just use get_color_palette(base_rgba=[your color]) or if you want to change the alpha on an Color by just changing the alpha, use the same function with the alpha parameter, get_color_palette(ipg_color=your ipg color, alpha= 0.5).  The alpha parameter is in most widgets also.
"""

colors = [Color.PRIMARY, Color.SECONDARY, Color.SUCCESS, Color.DANGER, Color.WARNING, 
          Color.INFO, Color.LIGHT, Color.DARK, Color.ALICE_BLUE, Color.ANTIQUE_WHITE, 
          Color.AQUA, Color.AQUAMARINE, Color.AZURE, Color.BEIGE, Color.BISQUE, 
          Color.BLACK, Color.BLANCHED_ALMOND, Color.BLUE, Color.BLUE_VIOLET, Color.BROWN, 
          Color.BURLY_WOOD, Color.CADET_BLUE, Color.CHARTREUSE, Color.CHOCOLATE, Color.CORAL, 
          Color.CORNFLOWER_BLUE, Color.CORNSILK, Color.CRIMSON, Color.CYAN, Color.DARK_BLUE, 
          Color.DARK_CYAN, Color.DARK_GOLDEN_ROD, Color.DARK_GRAY, Color.DARK_GREY, 
          Color.DARK_GREEN, Color.DARK_KHAKI, Color.DARK_MAGENTA, Color.DARK_OLIVE_GREEN, 
          Color.DARK_ORANGE, Color.DARK_ORCHID, Color.DARK_RED, Color.DARK_SALMON, 
          Color.DARK_SEA_GREEN, Color.DARK_SLATE_BLUE, Color.DARK_SLATE_GRAY, Color.DARK_SLATE_GREY, 
          Color.DARK_TURQUOISE, Color.DARK_VIOLET, Color.DEEP_PINK, Color.DEEP_SKY_BLUE, 
          Color.DIM_GRAY, Color.DIM_GREY, Color.DODGER_BLUE, Color.FIRE_BRICK, Color.FLORAL_WHITE, 
          Color.FOREST_GREEN, Color.FUCHSIA, Color.GAINSBORO, Color.GHOST_WHITE, Color.GOLD, 
          Color.GOLDEN_ROD, Color.GRAY, Color.GREY, Color.GREEN, Color.GREEN_YELLOW, Color.HONEY_DEW, 
          Color.HOT_PINK, Color.INDIAN_RED, Color.INDIGO, Color.IVORY, Color.KHAKI, Color.LAVENDER, 
          Color.LAVENDER_BLUSH, Color.LAWN_GREEN, Color.LEMON_CHIFFON, Color.LIGHT_BLUE, Color.LIGHT_CORAL, 
          Color.LIGHT_CYAN, Color.LIGHT_GOLDEN_ROD_YELLOW, Color.LIGHT_GRAY, Color.LIGHT_GREY, 
          Color.LIGHT_GREEN, Color.LIGHT_PINK, Color.LIGHT_SALMON, Color.LIGHT_SEA_GREEN, 
          Color.LIGHT_SKY_BLUE, Color.LIGHT_SLATE_GRAY, Color.LIGHT_SLATE_GREY, Color.LIGHT_STEEL_BLUE, 
          Color.LIGHT_YELLOW, Color.LIME, Color.LIME_GREEN, Color.LINEN, Color.MAGENTA, Color.MAROON, 
          Color.MEDIUM_AQUA_MARINE, Color.MEDIUM_BLUE, Color.MEDIUM_ORCHID, Color.MEDIUM_PURPLE, 
          Color.MEDIUM_SEA_GREEN, Color.MEDIUM_SLATE_BLUE, Color.MEDIUM_SPRING_GREEN, Color.MEDIUM_TURQUOISE, 
          Color.MEDIUM_VIOLET_RED, Color.MIDNIGHT_BLUE, Color.MINT_CREAM, Color.MISTY_ROSE, Color.MOCCASIN, 
          Color.NAVAJO_WHITE, Color.NAVY, Color.OLD_LACE, Color.OLIVE, Color.OLIVE_DRAB, Color.ORANGE, 
          Color.ORANGE_RED, Color.ORCHID, Color.PALE_GOLDEN_ROD, Color.PALE_GREEN, Color.PALE_TURQUOISE, 
          Color.PALE_VIOLET_RED, Color.PAPAYA_WHIP, Color.PEACH_PUFF, Color.PERU, Color.PINK, Color.PLUM,
          Color.POWDER_BLUE, Color.PURPLE, Color.REBECCA_PURPLE, Color.RED, Color.ROSY_BROWN, 
          Color.ROYAL_BLUE, Color.SADDLE_BROWN, Color.SALMON, Color.SANDY_BROWN, Color.SEA_GREEN, 
          Color.SEA_SHELL, Color.SIENNA, Color.SILVER, Color.SKY_BLUE, Color.SLATE_BLUE, 
          Color.SLATE_GRAY, Color.SLATE_GREY, Color.SNOW, Color.SPRING_GREEN, Color.STEEL_BLUE, 
          Color.TAN, Color.TEAL, Color.THISTLE, Color.TOMATO, Color.TRANSPARENT, Color.TURQUOISE, 
          Color.VIOLET, Color.WHEAT, Color.WHITE, Color.WHITE_SMOKE, Color.YELLOW, Color.YELLOW_GREEN]

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
                    color_name = str(color)[6:]

                    # Get the 3 colors based on the given Color
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
