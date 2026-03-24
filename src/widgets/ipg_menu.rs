//! ipg_menu
// #![allow(dead_code, unused_variables)]
#![allow(clippy::enum_variant_names)]

use std::collections::HashMap;

use iced::{Color, Element, Length, Padding, Renderer, Theme, Vector};
use iced_aw::{Menu, MenuBar, menu::{DrawPath, Item}};
use crate::{app, py_api::helpers::{get_padding, get_radius}, state::IpgWidgets};



use pyo3::{pyclass, Py, PyAny, Python};

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct IpgMenu {
    pub id: usize,
    pub bar_items: usize,
    pub menu_items: Vec<usize>,
    pub item_offset: Option<Vec<f32>>,
    pub item_padding: Option<Vec<f32>>,
    pub item_spacing: Option<Vec<f32>>,
    pub item_widths: Vec<Length>,
    pub bar_height: Length,
    pub bar_padding: Option<Vec<f32>>,
    pub bar_spacing: Option<f32>,
    pub bar_width: Length,
    pub close_on_item_click: Option<bool>,
    pub close_on_background_click: Option<bool>,
    pub bar_style_id: Option<usize>, // style_id of add_menu_bar_style()
    pub style_id: Option<usize>, // style_id of add_menu_style()
    pub show: bool,
    pub is_checked: bool,
    pub is_toggled: bool,
}

impl IpgMenu {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, IpgWidgets>, id: Option<usize>) -> Option<&'a IpgWidgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self, 
        mut content: Vec<Element<'a, app::Message>>,
        widgets: &HashMap<usize, IpgWidgets>,
        )-> Element<'a, app::Message, Theme, Renderer> 
    {
        
        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(IpgWidgets::as_menu_style).cloned();

        let bar_style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(IpgWidgets::as_menu_bar_style).cloned();

    
        let item_spacing = if let Some(sp) = &self.item_spacing {
            if sp.len() != self.bar_items {
                panic!("Menu spacings: The number of spacings {} must be 1 or match the number of bar items {}.", sp.len(), self.bar_items)
            } 
            if sp.len() == 1 {
                vec![sp[0]; self.bar_items]
            } else {
                sp.clone()
            }
        } else { vec![0.0; self.bar_items] };

        let item_widths = if self.item_widths.len() == 1 {
            vec![self.item_widths[0]; self.bar_items]
        } else if self.item_widths.len() != self.bar_items {
            panic!("Menu item widths: The number of widths {} must be 1 or match the number of bar items {}.", 
                    self.item_widths.len(), self.bar_items)
        } else {
            self.item_widths.clone()
        };
        
        let mut item_offsets = vec![0.0; self.bar_items];
        if self.item_offset.is_some() {
            let offsets = self.item_offset.clone().unwrap();
            if offsets.len() == 1 {
                item_offsets = vec![offsets[0]; self.bar_items]
            } else if offsets.len() != self.bar_items {
                panic!("Menu offsets: The number of offsets {} must be 1 or match the number of bar items {}.", 
                        item_offsets.len(), self.bar_items)
            } else {
                item_offsets = offsets;
            }
        }

        let mut bar_items: Vec<Item<app::Message, Theme, Renderer>> = vec![];

        for bar_index in 0..self.bar_items {

            let menu_bar = content.remove(0);

            let mut items = vec![];
            
            for _ in 0..self.menu_items[bar_index] {
                items.push(Item::new(content.remove(0)));
            }
            
            let menu_tpl = 
            |items| Menu::new(items)
                .max_width(100.0) // Don't see any effect
                .spacing(item_spacing[bar_index])
                .width(item_widths[bar_index])
                .offset(item_offsets[bar_index])
                .padding(get_padding(&self.item_padding))
                .close_on_item_click(self.close_on_item_click.unwrap_or_default())
                .close_on_background_click(self.close_on_background_click.unwrap_or_default());

            let bar_item = 
                Item::with_menu(
                    menu_bar, 
                    menu_tpl(items)
                    );

            bar_items.push(bar_item); 
        }
        
        let mb: MenuBar<'a, app::Message, Theme, Renderer> = 
            MenuBar::new(bar_items)
                .close_on_item_click(self.close_on_item_click.unwrap_or(false))
                .close_on_background_click(self.close_on_background_click.unwrap_or(false))
                .draw_path(DrawPath::Backdrop)
                .spacing(self.bar_spacing.unwrap_or(0.0))
                .padding(get_padding(&self.bar_padding))
                .width(self.bar_width)
                .height(self.bar_height);

        mb.into()
    }
}

#[derive(Debug, Clone, Default)]
pub struct IpgMenuBarStyle {
    pub id: usize,
    pub base_color: Option<Color>, // background
    pub border_color: Option<Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub shadow_color: Option<Color>,
    pub shadow_offset_xy: Option<[f32; 2]>,
    pub shadow_blur_radius: Option<f32>,
}


#[derive(Debug, Clone, Default)]
pub struct IpgMenuStyle {
    pub id: usize,
    pub base_color: Option<Color>, // background
    pub border_color: Option<Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub shadow_color: Option<Color>,
    pub shadow_offset_xy: Option<[f32; 2]>,
    pub shadow_blur_radius: Option<f32>,
    pub path_base_color: Option<Color>,
    pub path_border_color: Option<Color>,
    pub path_border_radius: Option<Vec<f32>>,
    pub path_border_width: Option<f32>,
}

fn get_mb_styling(theme: &Theme, 
                    status: Status,
                    br_style: Option<IpgMenuBarStyle>,
                    mn_style: Option<IpgMenuStyle>,
                ) -> Style {

    let mut menu_style = primary(theme, status);

    if br_style.is_none() && mn_style.is_none() {
        return menu_style
    }

    if br_style.is_some() {

        let b_style = br_style.unwrap();

        if b_style.base_color.is_some() {
            menu_style.bar_background = b_style.base_color.unwrap().into();
        }

        if b_style.border_width.is_some() {
            menu_style.bar_border.width = b_style.border_width.unwrap();
        }

        if b_style.border_color.is_some() {
            // just in case the user forget to set width, then something shows
            if menu_style.bar_border.width == 0.0 {
                menu_style.bar_border.width = 1.0;
            }
            menu_style.bar_border.color = b_style.border_color.unwrap();
        }

        if b_style.border_radius.is_some() {
            menu_style.bar_border.radius = get_radius(b_style.border_radius.clone().unwrap(),
                                                "Menu".to_string());
        }

        if b_style.shadow_color.is_some() {
            menu_style.bar_shadow.color = b_style.shadow_color.unwrap();
        }

        if b_style.shadow_offset_xy.is_some() {
            let offset = b_style.shadow_offset_xy.unwrap();
            menu_style.bar_shadow.offset = 
                Vector{ x: offset[0], y: offset[1] };
        }

        if b_style.shadow_blur_radius.is_some() {
            menu_style.bar_shadow.blur_radius = b_style.shadow_blur_radius.unwrap();
        }
    }

    if mn_style.is_some() {

        let m_style = mn_style.unwrap();

        if m_style.base_color.is_some() {
            menu_style.menu_background = m_style.base_color.unwrap().into();
        }

        if m_style.border_width.is_some() {
            menu_style.menu_border.width = m_style.border_width.unwrap();
        }

        if m_style.border_color.is_some() {
            // just in case the user forget to set width, then something shows
            if menu_style.menu_border.width == 0.0 {
                menu_style.menu_border.width = 1.0;
            }
            menu_style.menu_border.color = m_style.border_color.unwrap();
        }

        if m_style.border_radius.is_some() {
            menu_style.menu_border.radius = 
                get_radius(
                    m_style.border_radius.clone().unwrap(),
                    "Menu".to_string()
                );
        }

        if m_style.shadow_color.is_some() {
            menu_style.menu_shadow.color = m_style.shadow_color.unwrap();
        }

        if m_style.shadow_offset_xy.is_some() {
            let offset = m_style.shadow_offset_xy.unwrap();
            menu_style.menu_shadow.offset = 
                Vector{ x: offset[0], y:offset[1] };
        }

        if m_style.shadow_blur_radius.is_some() {
            menu_style.menu_shadow.blur_radius = 
                m_style.shadow_blur_radius.unwrap();
        }

        if m_style.path_base_color.is_some() {
            menu_style.path = m_style.path_base_color.unwrap().into();
        }

        if m_style.path_border_width.is_some() {
            menu_style.path_border.width = m_style.path_border_width.unwrap();
        }

        if m_style.path_border_color.is_some() {
            // just in case the user forget to set width, then something shows
            if menu_style.path_border.width == 0.0 {
                menu_style.path_border.width = 1.0;
            }
            menu_style.path_border.color = m_style.path_border_color.unwrap();
        }

        if m_style.path_border_radius.is_some() {
            menu_style.path_border.radius = 
                get_radius(
                    m_style.path_border_radius.clone().unwrap(),
                    "Menu".to_string()
                );
        }
    }

    menu_style

}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgMenuParam {
    BarHeight,
    BarHeightFill,
    BarPadding,
    BarSpacing,
    BarWidth,
    CheckBoundsWidth,
    Show,
}


pub fn try_extract_menu_update(update_obj: &PyObject) -> IpgMenuParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgMenuParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Menu update extraction failed"),
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgMenuBarStyleParam {
    BaseIpgColor,
    BaseRgbaColor,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    ShadowIpgColor,
    ShadowRgbaColor,
    ShadowOffsetXY,
    ShadowBlurRadius,
}

pub fn menu_bar_style_update_item(style: &mut IpgMenuBarStyle,
                            item: &PyObject,
                            value: &PyObject,) 
{
    
}

fn get_menu_bar_style(style: Option<&IpgWidgets>) -> Option<IpgMenuBarStyle>{
    match style {
        Some(IpgWidgets::IpgMenuBarStyle(style)) => {
            Some(style.clone())
        }
        _ => None,
    }
}

pub fn try_extract_menu_bar_style_update(update_obj: &PyObject) -> IpgMenuBarStyleParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgMenuBarStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Menu Bar style parameter update extraction failed"),
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgMenuStyleParam {
    BaseIpgColor,
    BaseRgbaColor,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    ShadowIpgColor,
    ShadowRgbaColor,
    ShadowOffsetXY,
    ShadowBlurRadius,
    PathBaseIpgColor,
    PathBaseRgbaColor,
    PathBorderIpgColor,
    PathBorderRgbaColor,
    PathBorderRadius,
    PathBorderWidth,
}

pub fn menu_style_update_item(style: &mut IpgMenuStyle,
                            item: &PyObject,
                            value: &PyObject,) 
{
    
}

fn get_menu_style(style: Option<&IpgWidgets>) -> Option<IpgMenuStyle>{
    match style {
        Some(IpgWidgets::IpgMenuStyle(style)) => {
            Some(style.clone())
        }
        _ => None,
    }
}

pub fn try_extract_menu_style_update(update_obj: &PyObject) -> IpgMenuStyleParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgMenuStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Menu style parameter update extraction failed"),
        }
    })
}

