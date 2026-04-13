//! Image module - provides add_image pyfunction

use pyo3::{pyfunction, PyResult};

use crate::{access_state, 
    state::{Widgets, get_id, set_state_of_widget}, 
    widgets::{enums::{ContentFit, FilterMethod, Rotation}, ipg_image::Image}};


/// """
/// add_image or Image parameters
/// 
/// Parameters
/// ----------
/// ImagePath: str
///     Sets the path to where the image is located.
/// BorderRadius: list[float, 4] | list[float]
///     Sets the border radius of the image either all corner same value [float] or independent [float,4].
/// ContentFit: ContentFit
///     Set how the image contents fits see ContentFit class.
/// CropHeight: float
///     Sets the height of the crop rectangle.
/// CropWidth: float
///     Sets the crop width of the crop rectangle.
/// CropX: float
///     Sets the origin x of the crop rectangle.
/// CropY: float
///     Sets the origin y of the crop rectangle.
/// Expand: float
///     Sets whether the image should try to fill as much space
///     available as possible while keeping aspect ratio and without
///     allocating extra space in any axis.
/// FilterMethod: FilterMethod
///     Sets the filter method, see FilterMethod
/// Fill: bool
///     Sets both width_fill and length_fill.
/// Height: float
///     Sets the height of the widget. 
/// HeightFill: bool
///     Sets the height to fill the available space, overrides height.
/// Opacity: float
///     Sets the opacity of the image.
/// Padding: list[float]
///     Sets the padding around the image.
/// RotationDegrees: float
///     Sets the roation of the image in degrees format.
/// RotationRadians: float
///     Sets the rotate the image in radians.
/// RotationMethod: Rotation
///     Set the roation method, see Rotation.
/// Scale: float
///     Sets the scale factor of the Image.
///     The region of the Image drawn will be scaled from the center by the given scale factor.
/// Show: bool
///     Whether to show or hide the image.
/// Width: float
///     Sets the width of the image.
/// WidthFill: bool
///     Whether to fill the width to the available container size.
/// 
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created image.
/// """
#[pyfunction]
#[pyo3(signature = (
    path,
    parent_id,
    width=None,
    width_fill=None,
    height=None,
    height_fill=None,
    fill=None,
    crop_x=None,
    crop_y=None,
    crop_width=None,
    crop_height=None,
    border_radius=None,
    content_fit=None,
    filter_method=None,
    rotation_method=None,
    rotation_radians=None,
    rotation_degrees=None,
    opacity=None,
    scale=None,
    expand=None,
    show=true,
    gen_id=None, 
    ))]
pub fn add_image(
    path: String,
    parent_id: String,
    width: Option<f32>,
    width_fill: Option<bool>,
    height: Option<f32>,
    height_fill: Option<bool>,
    fill: Option<bool>,
    crop_x: Option<u32>,
    crop_y: Option<u32>,
    crop_width: Option<u32>,
    crop_height: Option<u32>,
    border_radius: Option<Vec<f32>>,
    content_fit: Option<ContentFit>,
    filter_method: Option<FilterMethod>,
    rotation_method: Option<Rotation>,
    rotation_radians: Option<f32>,
    rotation_degrees: Option<f32>,
    opacity: Option<f32>,
    scale: Option<f32>,
    expand: Option<bool>,
    show: bool,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, Widgets::Image(
        Image {
            id,
            path,
            width,
            width_fill,
            height,
            height_fill,
            fill,
            crop_x,
            crop_y,
            crop_width,
            crop_height,
            border_radius,
            content_fit,
            filter_method,
            rotation_method,
            rotation_radians,
            rotation_degrees,
            opacity,
            scale,
            expand,
            show,
        }));

    drop(state);
    Ok(id)

}
