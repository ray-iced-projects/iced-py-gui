//! Image module - provides add_image pyfunction

use pyo3::{pyfunction, PyResult};

use crate::{access_state, 
    state::{Widgets, get_id, set_state_of_widget}, 
    widgets::{enums::{ContentFit, ColorFilter, Rotation}, ipg_image::Image}};


/// Add an image widget.
///
/// Displays an image from a file path with optional mouse interaction.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this image belongs to.
/// image_path : str
///     Sets the file path to the image.
/// content_fit : ContentFit, Optional
///     Sets the content fit strategy for the image.
/// filter_method : ColorFilter, Optional
///     Sets the color filter method applied to the image.
/// rotation_method : Rotation, Optional
///     Sets the rotation method for the image.
/// rotation_radians : float, Optional
///     Sets the rotation angle in radians.
/// opacity : float, Optional
///     Sets the opacity of the image (0.0 to 1.0).
/// show : bool, default True
///     Whether the image is visible.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// 
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created image.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    image_path,
    content_fit=None, 
    filter_method=None,
    rotation_method=None,
    rotation_radians=None, 
    opacity=None,
    show=true,
    gen_id=None, 
    ))]
pub fn add_image(
    parent_id: String,
    image_path: String,
    content_fit: Option<ContentFit>,
    filter_method: Option<ColorFilter>,
    rotation_method: Option<Rotation>,
    rotation_radians: Option<f32>,
    opacity: Option<f32>,
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
            parent_id,
            image_path,
            content_fit,
            filter_method,
            rotation_method,
            rotation_radians,
            opacity,
            show,
        }));

    drop(state);
    Ok(id)

}
