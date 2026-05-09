//! PickList module - provides add_pick_list pyfunction

use pyo3::{Py, PyAny, PyResult, Python, pyfunction};
use pyo3::types::{PyAnyMethods, PyListMethods};
type PyObject = Py<PyAny>;

use iced::widget::combo_box;
use crate::widgets::ipg_combo_box::ComboBox;
use crate::{access_state, add_callback_to_mutex, 
    add_user_data_to_mutex};

use crate::state::{Widgets, get_id, set_state_of_widget}; 



/// Add a combobox widget.
///
/// A dropdown list that lets the user select one option
/// from a list of choices.  The selctions are filter based
/// on what is typed.  The combobox is good for long lists.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this pick list belongs to.
/// options : list of str
///     Sets the list of selectable options.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, 
///     used for the gen_id parameter.
/// on_select : callable, Optional
///     Sets the Callback method to invoke when an option is selected.
/// on_open : callable, Optional
///     Sets the Callback method to invoke when the list is opened.
/// on_close : callable, Optional
///     Sets the Callback method to invoke when the list is closed.
/// on_input : callable, Optional
///     Sets the Callback method to invoke when the letters are typed in.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.  This is required  
///     if the container has no defined width.
/// width_fill : bool, default False
///     Whether the pick list fills available width.
/// menu_height : float, Optional
///     Sets the Fixed height of the dropdown menu in logical pixels.
/// menu_height_fill : bool, default False
///     Whether the dropdown menu fills available height.
/// padding : list of float, Optional
///     Sets the Padding as [all], [vertical, horizontal], or
///     [top, right, bottom, left].
/// placeholder : str, Optional
///     Sets the placeholder text shown when no option is selected.
/// selected : str, Optional
///     Sets the currently selected option.
/// text_size : float, Optional
///     Sets the Font size for the text.
/// text_line_height : float, Optional
///     Sets the Line height for the text.
/// user_data : Any, Optional
///     Sets the Arbitrary data forwarded to callbacks.
/// show : bool, default True
///     Whether the pick list is visible.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created pick list.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    options, 
    on_select=None,
    on_open=None,
    on_close=None,
    on_input=None,
    width=None, 
    width_fill=None,
    menu_height=None,
    menu_height_fill=None, 
    padding=None,  
    placeholder=None, 
    selected=None, 
    text_size=None, 
    text_line_height=None,
    text_ellipsis_start=None,
    text_ellipsis_middle=None,
    text_ellipsis_end=true,
    user_data=None, 
    show=true,
    gen_id=None, 
    ))]
pub fn add_combobox(
    parent_id: String,
    options: PyObject,
    on_select: Option<PyObject>,
    on_open: Option<PyObject>,
    on_close: Option<PyObject>,
    on_input: Option<PyObject>,
    width: Option<f32>,
    width_fill: Option<bool>,
    menu_height: Option<f32>,
    menu_height_fill: Option<bool>,
    padding: Option<Vec<f32>>,
    placeholder: Option<String>,
    selected: Option<String>,
    text_size: Option<f32>,
    text_line_height: Option<f32>,
    text_ellipsis_start: Option<bool>,
    text_ellipsis_middle: Option<bool>,
    text_ellipsis_end: bool,
    user_data: Option<PyObject>,
    show: bool,
    gen_id: Option<usize>,
) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let text_ellipsis = if text_ellipsis_start == Some(true) {
        iced::widget::text::Ellipsis::Start
    } else if text_ellipsis_middle == Some(true) {
        iced::widget::text::Ellipsis::Middle
    } else if text_ellipsis_end {
        iced::widget::text::Ellipsis::End
    } else {
        iced::widget::text::Ellipsis::None
    };

    if let Some(py) = on_select {
        add_callback_to_mutex(id, "on_select".to_string(), py);
    }

    if let Some(py) = on_open {
        add_callback_to_mutex(id, "on_open".to_string(), py);
    }

    if let Some(py) = on_close{
        add_callback_to_mutex(id, "on_close".to_string(), py);
    }

    if let Some(py) = on_input{
        add_callback_to_mutex(id, "on_input".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    let options: Vec<String> = Python::attach(|py| {
        let list = options.bind(py);
        let py_list = list.downcast::<pyo3::types::PyList>()
            .expect("options must be a list");
        py_list.iter()
            .map(|item| item.str()
                .expect("option item must be convertible to str")
                .to_string())
            .collect()
    });

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, Widgets::ComboBox(
        ComboBox { 
            id,
            cb_state: combo_box::State::new(options.clone()),
            placeholder,
            selected,
            width,
            width_fill,
            menu_height,
            menu_height_fill,
            padding,
            text_size,
            text_line_height,
            text_ellipsis,
            show,
        }));


    drop(state);
    Ok(id)
}
