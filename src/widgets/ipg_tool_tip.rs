//! ipg_tool_tip

use std::collections::HashMap;

use iced::Element;
use iced::time::seconds;
use iced::widget::{self, container, text};
use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app::Message;
use crate::state::Widgets;
use crate::widgets::ipg_container::ContainerStyleStd;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};


#[derive(Debug, Clone)]
pub struct ToolTip {
    pub id: usize,
    pub position_follow_cursor: Option<bool>,
    pub position_bottom: Option<bool>,
    pub position_left: Option<bool>,
    pub position_top: Option<bool>,
    pub position_right: Option<bool>,
    pub text: Option<String>,
    pub gap: Option<u32>,
    pub padding: Option<f32>,
    pub snap_within_viewport: Option<bool>,
    pub delay_sec: Option<u64>,
    pub style_id: Option<usize>,
    pub style_std: Option<ContainerStyleStd>,
}

impl ToolTip {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }
    
    pub fn construct<'a>(
        &'a self, 
        mut content: Vec<Element<'a, Message>>,
        widgets: &HashMap<usize, Widgets>,
        ) -> Element<'a, Message>
    {
        
        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_container_style).cloned();

        let position: widget::tooltip::Position = 
        if self.position_follow_cursor == Some(true) {
            widget::tooltip::Position::FollowCursor
        } else if self.position_bottom == Some(true) {
            widget::tooltip::Position::Bottom
        } else if self.position_left == Some(true) {
            widget::tooltip::Position::Left
        } else if self.position_right == Some(true) {
            widget::tooltip::Position::Right
        } else {
            widget::tooltip::Position::Top
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

        let tt: Element<'a, Message> = widget::tooltip::Tooltip::new(
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



#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ToolTipParam {
    ContainerStyleId,
    ContentId,
    DelaySec,
    Gap,
    Padding,
    PositionFollowCursor,
    PositionBottom,
    PositionLeft,
    PositionTop,
    PositionRight,
    SnapWithinViewport,
    Text,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for ToolTip {
    type Param = ToolTipParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ToolTipParam::ContainerStyleId => set_t_value(&mut self.style_id, value, "ToolTipParam::ContainerStyleId"),
            ToolTipParam::ContentId => set_t_value(&mut self.style_id, value, "ToolTipParam::ContentId"),
            ToolTipParam::DelaySec => set_t_value(&mut self.delay_sec, value, "ToolTipParam::DelaySec"),
            ToolTipParam::Gap => set_t_value(&mut self.gap, value, "ToolTipParam::Gap"),
            ToolTipParam::Padding => set_t_value(&mut self.padding, value, "ToolTipParam::Padding"),
            ToolTipParam::PositionBottom => set_t_value(&mut self.position_bottom, value, "ToolTipParam::PositionBottom"),
            ToolTipParam::PositionFollowCursor => set_t_value(&mut self.position_follow_cursor, value, "ToolTipParam::PositionFollowCursor"),
            ToolTipParam::PositionLeft => set_t_value(&mut self.position_left, value, "ToolTipParam::PositionBottom"),
            ToolTipParam::PositionRight => set_t_value(&mut self.position_right, value, "ToolTipParam::PositionRight"),
            ToolTipParam::PositionTop => set_t_value(&mut self.position_top, value, "ToolTipParam::PositionLeft"),
            ToolTipParam::SnapWithinViewport => set_t_value(&mut self.snap_within_viewport, value, "ToolTipParam::SnapWithinViewport"),
            ToolTipParam::Text => set_t_value(&mut self.text, value, "ToolTipParamText"),
        }
    }
}
