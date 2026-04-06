//! ipg_tool_tip

use std::collections::HashMap;

use iced::Element;
use iced::time::seconds;
use iced::widget::{Tooltip, container, text};
use iced::widget::tooltip::Position;
use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;

use crate::app::Message;
use crate::state::IpgWidgets;
use crate::widgets::ipg_container::IpgContainerStyleStd;
use crate::widgets::widget_param_update::{WidgetParamUpdate, 
    set_opt_bool, set_opt_f32, set_opt_string, set_opt_u32, 
    set_opt_u64, set_opt_usize};


#[derive(Debug, Clone)]
pub struct IpgToolTip {
    pub id: usize,
    pub position: Option<IpgToolTipPosition>,
    pub text: Option<String>,
    pub gap: Option<u32>,
    pub padding: Option<f32>,
    pub snap_within_viewport: Option<bool>,
    pub delay_sec: Option<u64>,
    pub style_id: Option<usize>,
    pub style_std: Option<IpgContainerStyleStd>,
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgToolTipPosition {
    FollowCursor,
    Top,
    Bottom,
    Left,
    Right,
}

impl IpgToolTip {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, IpgWidgets>, id: Option<usize>) -> Option<&'a IpgWidgets> {
        id.and_then(|id| widgets.get(&id))
    }
    
    pub fn construct<'a>(
        &'a self, 
        mut content: Vec<Element<'a, Message>>,
        widgets: &HashMap<usize, IpgWidgets>,
        ) -> Element<'a, Message>
    {
        
        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(IpgWidgets::as_container_style).cloned();

        let position: Position = match self.position {
            Some(IpgToolTipPosition::FollowCursor) => Position::FollowCursor,
            Some(IpgToolTipPosition::Top) => Position::Top,
            Some(IpgToolTipPosition::Bottom) => Position::Bottom,
            Some(IpgToolTipPosition::Left)   => Position::Left,
            Some(IpgToolTipPosition::Right)  => Position::Right,
            None => Position::Top,
        };

        let tooltip: Element<'a, Message> = 
            if let Some(txt) = &self.text {
                    text(txt).into()
                } else {
                    if content.len() < 2 {
                        text("If you are not using the text parameter,
                            \nyou must use two widgets/containers").into()
                    } else {
                        content.remove(0)
                    }
                };

        let tt: Element<'a, Message> = Tooltip::new(
                content.remove(0),
                tooltip,
                position,
                )
                .gap(self.gap.unwrap_or(0))
                .padding(self.padding.unwrap_or(0.0))
                .snap_within_viewport(self.snap_within_viewport.unwrap_or(false))
                .delay(seconds(self.delay_sec.unwrap_or(0)))
                .style(move|theme|
                    if let Some(st) = &style_opt {
                        st.to_iced(theme, &self.style_std)
                    } else {
                       match &self.style_std {
                            Some(std) => std.to_iced(theme),
                            None => container::transparent(theme),
                        }
                    }
                )
                .into();
        tt
    }
}



#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgToolTipParam {
    ContainerStyleId,
    ContentId,
    DelaySec,
    Gap,
    Padding,
    Position,
    SnapWithinViewport,
    Text,
}



pub fn try_extract_position(value: &PyObject, name: String) -> IpgToolTipPosition {
    Python::attach(|py| {
        let res = value.extract::<IpgToolTipPosition>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract tooltip position", name),
        }
    })  
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgToolTip {
    type Param = IpgToolTipParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgToolTipParam::ContainerStyleId => set_opt_usize(&mut self.style_id, value, "ContainerStyleId"),
            IpgToolTipParam::ContentId => set_opt_usize(&mut self.style_id, value, "ContentId"),
            IpgToolTipParam::DelaySec => set_opt_u64(&mut self.delay_sec, value, "DelaySec"),
            IpgToolTipParam::Gap => set_opt_u32(&mut self.gap, value, "Gap"),
            IpgToolTipParam::Padding => set_opt_f32(&mut self.padding, value, "Padding"),
            IpgToolTipParam::Position => {
                let pos = try_extract_position(value, "IpgToolTipParam::Position".to_string());
                self.position = Some(pos);
            },
            IpgToolTipParam::SnapWithinViewport => set_opt_bool(&mut self.snap_within_viewport, value, "SnapWithinViewport"),
            IpgToolTipParam::Text => set_opt_string(&mut self.text, value, "Text"),
        }
    }
}
