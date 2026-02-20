//! Widget parameter update function
#![allow(unused)]
use crate::py_api::helpers::{get_height, get_width, try_extract_boolean, try_extract_f32, try_extract_string, try_extract_vec_f32};
use crate::state::IpgWidgets;
use crate::widgets::enums::{IpgHorizontalAlignment, IpgVerticalAlignment};
use crate::widgets::ipg_button::{try_extract_button_arrow, try_extract_button_style_standard};
use super::ipg_button::{IpgButtonParam, IpgButtonStyleParam};
use super::ipg_checkbox::IpgCheckboxParam;

use pyo3::{Py, PyAny, Python};
type PyObject = Py<PyAny>;



pub fn param_update(
        widget: &mut IpgWidgets,
        item: &PyObject,
        value: &PyObject,
        name: String,
    )
{
    match widget {
        IpgWidgets::IpgButton(btn) => {
            let param = extract_param::<IpgButtonParam>(item, "Button");
            match param {
                IpgButtonParam::ArrowStyle => {
            btn.style_arrow = Some(try_extract_button_arrow(value));
        },
        IpgButtonParam::Label => {
            btn.label = Some(try_extract_string(value, name));
        },
        IpgButtonParam::Height => {
            let val = try_extract_f32(value, name);
            btn.height = get_height(Some(val), false);
        },
        IpgButtonParam::HeightFill => {
            let val = try_extract_boolean(value, name);
            btn.height = get_height(None, val);
        },
        IpgButtonParam::Padding => {
            btn.padding =  Some(try_extract_vec_f32(value, name));
        },
        IpgButtonParam::Clip => {
            btn.clip = Some(try_extract_boolean(value, name));
        }
        IpgButtonParam::Show => {
            btn.show = try_extract_boolean(value, name);
        },
        IpgButtonParam::StyleId => {
            btn.style_id = Some(try_extract_f32(value, name) as usize);
        },
        IpgButtonParam::StyleStandard => {
            btn.style_standard = Some(try_extract_button_style_standard(value, name));
        },
        IpgButtonParam::TextAlignX => {
            btn.text_align_x = IpgHorizontalAlignment::extract(value);
        },
        IpgButtonParam::TextAlignY => {
            btn.text_align_y= IpgVerticalAlignment::extract(value);
        },
        IpgButtonParam::TextSize => {
            btn.text_size = Some(try_extract_f32(value, name));
        },
        IpgButtonParam::Width => {
            let val = try_extract_f32(value, name);
            btn.width = get_width(Some(val), false);
        },
        IpgButtonParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            btn.width = get_width(None, val);
        },
            }
        },
        IpgWidgets::IpgButtonStyle(style) => {
            let param = extract_param::<IpgButtonStyleParam>(item, "ButtonStyle");
            match param {
                IpgButtonStyleParam::BackgroundIpgColor => todo!(),
                IpgButtonStyleParam::BackgroundRbgaColor => todo!(),
                IpgButtonStyleParam::BackgroundIpgColorHovered => todo!(),
                IpgButtonStyleParam::BackgroundIpgRgbaHovered => todo!(),
                IpgButtonStyleParam::BorderIpgColor => todo!(),
                IpgButtonStyleParam::BorderRgbaColor => todo!(),
                IpgButtonStyleParam::BorderRadius => todo!(),
                IpgButtonStyleParam::BorderWidth => todo!(),
                IpgButtonStyleParam::ShadowIpgColor => todo!(),
                IpgButtonStyleParam::ShadowRgbaColor => todo!(),
                IpgButtonStyleParam::ShadowOffsetX => todo!(),
                IpgButtonStyleParam::ShadowOffsetY => todo!(),
                IpgButtonStyleParam::ShadowBlurRadius => todo!(),
                IpgButtonStyleParam::TextIpgColor => todo!(),
                IpgButtonStyleParam::TextRgbaColor => todo!(),
            }
            
        },
        IpgWidgets::IpgCheckBox(chk) => {
            let param = extract_param::<IpgCheckboxParam>(item, "Checkbox");
            match param {
                IpgCheckboxParam::Icon => todo!(),
                IpgCheckboxParam::IconFont => todo!(),
                IpgCheckboxParam::IconLineHeight => todo!(),
                IpgCheckboxParam::IconSize => todo!(),
                IpgCheckboxParam::IconShaping => todo!(),
                IpgCheckboxParam::IsChecked => todo!(),
                IpgCheckboxParam::Label => todo!(),
                IpgCheckboxParam::Spacing => todo!(),
                IpgCheckboxParam::Style => todo!(),
                IpgCheckboxParam::StyleStandard => todo!(),
                IpgCheckboxParam::TextFont => todo!(),
                IpgCheckboxParam::TextLineHeight => todo!(),
                IpgCheckboxParam::TextShaping => todo!(),
                IpgCheckboxParam::TextSize => todo!(),
                IpgCheckboxParam::Width => todo!(),
                IpgCheckboxParam::WidthFill => todo!(),
                IpgCheckboxParam::Show => todo!(),
            }
        },
        _ => ()
    }
    

}

pub fn extract_param<T>(update_obj: &PyObject, widget_name: &str) -> T
where
    T: for<'py> pyo3::FromPyObject<'py>,
{
    Python::attach(|py| {
        let res = update_obj.extract::<T>(py);
        match res {
            Ok(update) => update,
            Err(err) => panic!("{} param extraction failed: {}", widget_name, err),
        }
    })
}
