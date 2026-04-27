snake_case = [
    "background_color: Option<Color>",
    "background_color_alpha: Option<f32>",
    "background_rgba: Option<[f32; 4]>",

    "text_color: Option<Color>",
    "text_color_alpha: Option<f32>",
    "text_rgba: Option<[f32; 4]>",

    "text_color_active: Option<Color>",
    "text_color_alpha_active: Option<f32>",
    "text_rgba_active: Option<[f32; 4]>",

    "text_color_hovered: Option<Color>",
    "text_color_alpha_hovered: Option<f32>",
    "text_rgba_hovered: Option<[f32; 4]>",

    "text_color_pressed: Option<Color>",
    "text_color_alpha_pressed: Option<f32>",
    "text_rgba_pressed: Option<[f32; 4]>",

    "text_color_disabled: Option<Color>",
    "text_color_alpha_disabled: Option<f32>",
    "text_rgba_disabled: Option<[f32; 4]>",

    "background_gradient_color_stop: Option<Color>",
    "background_gradient_color_stop_alpha: Option<f32>",
    "background_gradient_rgba_stop: Option<[f32; 4]>",
    "background_gradient_degrees: Option<f32>",
    "background_gradient_radians: Option<f32>",

    "border_color_active: Option<Color>",
    "border_color_alpha_active: Option<f32>",
    "border_rgba_active: Option<[f32; 4]>",

    "border_color_hovered: Option<Color>",
    "border_color_alpha_hovered: Option<f32>",
    "border_rgba_hovered: Option<[f32; 4]>",

    "border_color_pressed: Option<Color>",
    "border_color_alpha_pressed: Option<f32>",
    "border_rgba_pressed: Option<[f32; 4]>",

    "border_color_disabled: Option<Color>",
    "border_color_alpha_disabled: Option<f32>",
    "border_rgba_disabled: Option<[f32; 4]>",

    "border_radius: Option<Vec<f32>>",
    "border_width: Option<f32>",

    "shadow_color: Option<Color>",
    "shadow_color_alpha: Option<f32>",
    "shadow_rgba: Option<[f32; 4]>",
    "shadow_offset_xy: Option<[f32; 2]>",
    "shadow_blur_radius: Option<f32>",

    "snap: Option<bool>",
]

param = "ButtonStyleParam::"

def to_pascal(snake: str) -> str:
    return "".join(word.capitalize() for word in snake.split("_"))



for entry in snake_case:
    split_entry = entry.split(":")[0].strip()
    wd = to_pascal(split_entry)
    print(f"{param}{wd} => set_t_value(&mut self.{split_entry}, value, \"{param}{wd}\"),")
