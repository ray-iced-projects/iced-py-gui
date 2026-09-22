
from icedpygui import WidgetStatus, StateVariant, PaletteKey, StylePart

statuses = [
((WidgetStatus.Active, StateVariant.NoVariant),
[(StylePart.Background, PaletteKey.Base, 1.0),
(StylePart.Border, PaletteKey.Base, 1.0),
(StylePart.Text, PaletteKey.BaseText, 1.0)
]),
((WidgetStatus.Hovered, StateVariant.NoVariant),
[(StylePart.Background, PaletteKey.Strong, 1.0),
(StylePart.Border, PaletteKey.Strong, 1.0),
(StylePart.Text, PaletteKey.StrongText, 0.8)
]),
((WidgetStatus.Pressed, StateVariant.NoVariant),
[(StylePart.Background, PaletteKey.Base, 1.0),
(StylePart.Border, PaletteKey.Base, 1.0),
(StylePart.Text, PaletteKey.BaseText, 1.0)
]),
((WidgetStatus.Disabled, StateVariant.NoVariant),
[(StylePart.Background, PaletteKey.Base, 0.5),
(StylePart.Border, PaletteKey.Strong, 0.5),
(StylePart.Text, PaletteKey.BaseText, 0.8)])]


status_parts = {}
for (status, variant), parts in statuses:
    status_key = str(status).rsplit('.', maxsplit=1)[-1].lower() # "active"
    status_parts[status_key] = {}
    for part, key, alpha in parts:
        part_key = str(part).rsplit('.', maxsplit=1)[-1].lower()  # "background"
        status_parts[status_key][part_key] = (key, alpha)

# Usage
key, alpha = status_parts["active"]["background"]
print(key, alpha)
