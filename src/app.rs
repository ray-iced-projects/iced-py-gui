#![allow(unused)]
use std::collections::HashMap;
use std::time::Instant;

use iced::time::milliseconds;
use iced::widget::{Column, scrollable};
use iced::advanced::widget::operation::scrollable::scroll_to;
use iced::window::{self, Position};
use iced::{Color, Element, Event, Point, Size, Subscription, Task, Theme, font, message, time};

use palette::chromatic_adaptation::AdaptInto;
use pyo3::{Py, PyAny};
type PyObject = Py<PyAny>;

use crate::py_api::helpers::find_key_for_value;
use crate::state::{IpgContainers, IpgWidgetNode, IpgState, IpgWidgets, access_state, access_update_widgets, access_window_actions, clone_state_to_runtime, set_state_of_widget_running_state};
use crate::widgets::ipg_button::{BtnMessage, button_callback};
use crate::widgets::ipg_card::{CardMessage, card_callback};
use crate::widgets::ipg_checkbox::{ChkMessage, checkbox_callback};
use crate::widgets::ipg_color_picker::{ColPikMessage, color_picker_callback};
use crate::widgets::ipg_date_picker::{DPMessage, date_picker_update};
use crate::widgets::ipg_divider::{DivMessage, divider_callback};
use crate::widgets::ipg_events::{process_keyboard_events, process_mouse_events, process_touch_events, process_window_event};
use crate::widgets::ipg_image::{ImageMessage, construct_image, image_callback};
use crate::widgets::ipg_mouse_area::{construct_mousearea, mousearea_callback, mousearea_callback_point};
use crate::widgets::ipg_opaque::{construct_opaque, opaque_callback};
use crate::widgets::ipg_pick_list::{PLMessage, construct_picklist, pick_list_callback};
use crate::widgets::ipg_progress_bar::construct_progress_bar;
use crate::widgets::ipg_radio::{RDMessage, construct_radio, radio_callback};
use crate::widgets::ipg_rule::construct_rule;
use crate::widgets::ipg_scrollable::scrollable_callback;
use crate::widgets::ipg_selectable_text::{SLTXTMessage, construct_selectable_text, selectable_text_callback};
use crate::widgets::ipg_separator::construct_separator;
use crate::widgets::ipg_slider::{SLMessage, construct_slider, slider_callback};
use crate::widgets::ipg_space::construct_space;
use crate::widgets::ipg_svg::{SvgMessage, construct_svg, svg_callback};
use crate::widgets::ipg_table::{TableMessage, table_callback};
use crate::widgets::ipg_text_input::{TIMessage, text_input_callback};
use crate::widgets::ipg_timer::{TimerState, timer_callback};
use crate::widgets::ipg_toggle::{TOGMessage, construct_toggler, toggle_callback};
use crate::widgets::ipg_window::{IpgWindow, IpgWindowLevel, IpgWindowMode, add_windows, construct_window};
use crate::widgets::widget_param_update::{param_update, container_param_update};


#[derive(Debug, Clone)]
pub enum Message {
    Button(usize, BtnMessage),
//     Canvas(CanvasMessage),
    Card(usize, CardMessage),
    CheckBox(usize, ChkMessage),
    ColorPicker(usize, ColPikMessage),
    DatePicker(usize, DPMessage),
    Divider(usize, DivMessage),
    EventKeyboard(Event),
    EventMouse(Event),
    EventWindow((window::Id, Event)),
    EventTouch(Event),
    Image(usize, ImageMessage),
//     // Modal(usize, ModalMessage),
    PickList(usize, PLMessage),
    Radio(usize, RDMessage),
    Scrolled(scrollable::Viewport, usize),
    SelectableText(usize, SLTXTMessage),
    Slider(usize, SLMessage),
    Svg(usize, SvgMessage),

    TableScrolled(scrollable::Viewport, usize),
    TableDividerChanged((usize, usize, f32)),
    TableDividerReleased(usize),

    TextInput(usize, TIMessage),
    Toggler(usize, TOGMessage),
//     CanvasTextBlink,
    Tick(usize, Instant),
//     CanvasTick,
//     CanvasTimer(usize, CanvasTimerMessage),
    FontLoaded(Result<(), font::Error>),
    WindowOpened(window::Id, Option<Point>, Size),

    MouseAreaOnPress(usize),
    MouseAreaOnRelease(usize),
    MouseAreaOnRightPress(usize),
    MouseAreaOnRightRelease(usize),
    MouseAreaOnMiddlePress(usize),
    MouseAreaOnMiddleRelease(usize),
    MouseAreaOnEnter(usize),
    MouseAreaOnMove(Point, usize),
    MouseAreaOnExit(usize),

    OpaqueOnPress(usize),
}

#[derive(Debug, Clone)]
pub struct App {
    state: IpgState,
    // canvas_state: IpgCanvasState,
}

impl App {
    
    pub fn new() -> (Self, Task<Message>) {
        let mut state = IpgState::new();
        clone_state(&mut state);

        // let mut canvas_state = IpgCanvasState::default();
        // clone_canvas_state(&mut canvas_state);

        let mut open = add_windows(&mut state);
        open.push(font::load(include_bytes!("./graphics/fonts/bootstrap-icons.ttf").as_slice()).map(Message::FontLoaded));
        open.push(font::load(include_bytes!("./graphics/fonts/Roboto.ttf").as_slice()).map(Message::FontLoaded));
        open.push(font::load(include_bytes!("./graphics/fonts/FiraSans-Regular.ttf").as_slice()).map(Message::FontLoaded));
        open.push(font::load(include_bytes!("./graphics/fonts/Newsreader.ttf").as_slice()).map(Message::FontLoaded));
        
        (
            Self {
                state,
                // canvas_state,
            },
            
            Task::batch(open),
        )
    }

    pub fn title(&self, iced_window_id: window::Id) -> String {
        
        let ipg_window_id = match self.state.windows_iced_ipg_ids.get(&iced_window_id) {
            Some(id) => *id,
            None => panic!("App: title, Unable to find ipg_window_id based on iced_window_id {:?}.", iced_window_id)
        };
        
        let window_opt = self.state.containers.get(&ipg_window_id);
        let ipg_window = get_window_container(window_opt);

        ipg_window.title.clone().unwrap_or("".to_string())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FontLoaded(_) => {
                Task::none()
            },
            Message::Button(id, message) => {
                button_callback(id, message);
                process_widget_updates(&mut self.state);
                // process_updates(&mut self.state, &mut self.canvas_state);
                get_tasks(&mut self.state)
            },
            // Message::Canvas(canvas_message) => {
            //     canvas_callback(canvas_message, &mut self.state, &mut self.canvas_state);
            //     process_updates(&mut self.state, &mut self.canvas_state);
            //     get_tasks(&mut self.state)
            // },
            Message::Card(id, message) => {
                card_callback(id, message);
                process_widget_updates(&mut self.state); //, &mut self.canvas_state);
                Task::none()
            },
            Message::CheckBox(id, message) => {
                checkbox_callback(&mut self.state, id, message);
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                get_tasks(&mut self.state)
            },
            Message::ColorPicker(id, message ) => {
                color_picker_callback(&mut self.state, id, message);
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::DatePicker(id, message) => {
                date_picker_update(&mut self.state, id, message);
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::Divider(id, message) => {
                divider_callback(&mut self.state, id, message);
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::EventKeyboard(event) => {
                process_keyboard_events(event, self.state.keyboard_event_id_enabled.0);
                process_widget_updates(&mut self.state); //, &mut self.canvas_state);
                Task::none()
            },
            Message::EventMouse(event) => {
                process_mouse_events(event, self.state.mouse_event_id_enabled.0);
                process_widget_updates(&mut self.state); //, &mut self.canvas_state);
                Task::none()
            },
            Message::EventWindow((window_id, event)) => {
                process_window_event(&mut self.state, event, window_id);
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                if self.state.windows_opened.len() == self.state.windows_hidden.len() {
                    iced::exit()
                } else {
                    // check for any other window changes
                    get_tasks(&mut self.state)
                }
            },
            Message::WindowOpened(_, _, _) => {
                Task::none()
            },
            Message::EventTouch(event) => {
                process_touch_events(event, self.state.touch_event_id_enabled.0);
                process_widget_updates(&mut self.state); //, &mut self.canvas_state);
                Task::none()
            },
            Message::Image(id, message) => {
                image_callback(id, message);
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::MouseAreaOnPress(id) => {
                mousearea_callback(&mut self.state, id, "on_press".to_string());
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::MouseAreaOnRelease(id) => {
                mousearea_callback(&mut self.state, id, "on_release".to_string());
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::MouseAreaOnRightPress(id) => {
                mousearea_callback(&mut self.state, id, "on_right_press".to_string());
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::MouseAreaOnRightRelease(id) => {
                mousearea_callback(&mut self.state, id, "on_right_release".to_string());
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::MouseAreaOnMiddlePress(id) => {
                mousearea_callback(&mut self.state, id, "on_middle_press".to_string());
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::MouseAreaOnMiddleRelease(id) => {
                mousearea_callback(&mut self.state, id, "on_middle_release".to_string());
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::MouseAreaOnEnter(id) => {
                mousearea_callback(&mut self.state, id, "on_enter".to_string());
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::MouseAreaOnMove(point, id) => {
                mousearea_callback_point(&mut self.state, id, point, "on_move".to_string());
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::MouseAreaOnExit(id) => {
                mousearea_callback(&mut self.state, id, "on_exit".to_string());
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::OpaqueOnPress(id) => {
                opaque_callback(&mut self.state, id, "on_press".to_string());
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::PickList(id, message) => {
                pick_list_callback(&mut self.state, id, message);
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::Radio(id, message) => {
                radio_callback(&mut self.state, id, message);
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::Scrolled(vp, id) => {
                scrollable_callback(&mut self.state, id, vp);
                process_widget_updates(&mut self.state); //, &mut self.canvas_state);
                Task::none()
            },
            Message::SelectableText(id, message) => {
                selectable_text_callback(id, message);
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::Slider(id, message) => {
                slider_callback(&mut self.state, id, message);
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::Svg(id, message) => {
                svg_callback(&mut self.state, id, message);
                process_widget_updates(&mut self.state); //, &mut self.canvas_state);
                Task::none()
            },
            Message::TableScrolled(vp, id) => {
                scrollable_callback(&mut self.state, id, vp);
                process_widget_updates(&mut self.state); //, &mut self.canvas_state);
                Task::none()
            },
            Message::TableDividerChanged((id, index, value)) => {
                let message = TableMessage::DivDragging((index, value));
                table_callback(&mut self.state, id, message);
                process_widget_updates(&mut self.state); //, &mut self.canvas_state);
                Task::none()
            },
            Message::TableDividerReleased(id) => {
                let message = TableMessage::DivOnRelease;
                table_callback(&mut self.state, id, message);
                process_widget_updates(&mut self.state); //, &mut self.canvas_state);
                Task::none()
            },
            Message::TextInput(id, message) => {
                text_input_callback(&mut self.state, id, message);
                process_widget_updates(&mut self.state); //, &mut self.canvas_state);
                Task::none()
            },
            Message::Tick(id, instant) => {
                dbg!("message tick");
                timer_callback(&mut self.state, id, instant);
                process_widget_updates(&mut self.state); //, &mut self.canvas_state);
                Task::none()
            },
            // Message::CanvasTextBlink => {
            //     self.canvas_state.elapsed_time += self.canvas_state.timer_duration;
            //     self.canvas_state.blink = !self.canvas_state.blink;
            //     self.canvas_state.request_text_redraw();
            //     Task::none()
            // },
            // Message::CanvasTick => {
            //     canvas_tick_callback(&mut self.state);
            //     process_canvas_updates(&mut self.canvas_state);
            //     process_updates(&mut self.state, &mut self.canvas_state); 
            //     self.canvas_state.request_image_redraw();
            //     Task::none()
            // },
            // Message::CanvasTimer(id, message) => {
            //     self.state.canvas_timer_event_id_enabled.1 = !self.state.canvas_timer_event_id_enabled.1;
            //     self.state.canvas_timer_event_id_enabled.0 = id;
            //     let started = self.state.canvas_timer_event_id_enabled.1;
            //     self.state.canvas_timer_duration = canvas_timer_callback(&mut self.state, id, started);
            //     process_updates(&mut self.state, &mut self.canvas_state);    
            //     Task::none()
            // },
            Message::Toggler(id, message) => {
                toggle_callback(&mut self.state, id, message);
                // process_updates(&mut self.state, &mut self.canvas_state);
                process_widget_updates(&mut self.state);
                get_tasks(&mut self.state)
            },
        }
        
    }

    pub fn view(&self, window_id: window::Id) -> Element<'_, Message> {

        let (debug, theme) = get_window_values(window_id, &self.state);
 
        let content = 
            create_content(window_id, &self.state);
            // create_content(window_id, &self.state, &self.canvas_state);
        
        if debug {
            let color = match_theme_with_debug_color(theme);
                content.explain(color)  
        } else {
            content
        }

    }

    pub fn subscription(&self) -> Subscription<Message> {

        let mut subscriptions = vec![];
        
        for (id, ts) in self.state.timer_state.iter() {
            if ts.enable {
                let id = id.clone();
                subscriptions.push(time::every(milliseconds(ts.duration_ms))
                    .with(id)
                    .map(|(id, last_tick)| Message::Tick(id, last_tick)));
                }
        }
        
        // if self.state.canvas_timer_event_id_enabled.1 {
        //     subscriptions
        //     .push(time::every(iced::time::Duration::from_millis(
        //         self.state.canvas_timer_duration)).map(|_| Message::CanvasTick));
        // }
        
        if self.state.keyboard_event_id_enabled.1 {
            subscriptions.push(iced::event::listen().map(Message::EventKeyboard));
        }

        if self.state.mouse_event_id_enabled.1 {
            subscriptions.push(iced::event::listen().map(Message::EventMouse));
        }
        // if self.canvas_state.timer_event_enabled {
        //     subscriptions.push(time::every(
        //         iced::time::Duration::from_millis(
        //             self.canvas_state.timer_duration))
        //             .map(|_| Message::CanvasTextBlink));
        // }

        // window event is always enabled, since we are using iced::daemon, the windows
        // closing need to be followed and iced exited when the last window is closed.
        // The closing is the only event monitored unless the user enables the window events.
        let w_event = window::events()
            .map(|(id, event)| Message::EventWindow((id, iced::Event::Window(event))));

        subscriptions.push(w_event);

        if !subscriptions.is_empty() {
            Subscription::batch(subscriptions)
        }
        else {
            Subscription::none()
        }
    }

    pub fn theme(&self, iced_window_id: window::Id) -> Theme {

        let ipg_window_id_opt = self.state.windows_iced_ipg_ids.get(&iced_window_id);
        let ipg_window_id = match ipg_window_id_opt {
            Some(id) => *id,
            None => panic!("App: theme, Unable to find ipg_window_id based on iced_window_id {:?}.", iced_window_id)
        };
        
        let window_opt = self.state.containers.get(&ipg_window_id);
        let ipg_window = get_window_container(window_opt);

        if let Some(theme) = &ipg_window.theme {
            theme.to_iced()
        } else {
            Theme::TokyoNight
        }
    }

    pub fn scale_factor(&self, iced_window_id: window::Id) -> f32 {

        let ipg_window_id_opt = self.state.windows_iced_ipg_ids.get(&iced_window_id);
        let ipg_window_id = match ipg_window_id_opt {
            Some(id) => *id,
            None => panic!("App: title, Unable to find ipg_window_id based on iced_window_id {:?}.", iced_window_id)
        };
        
        let window_opt = self.state.containers.get(&ipg_window_id);
        let ipg_window = get_window_container(window_opt);

        ipg_window.scale_factor.unwrap_or(1.0)
    }

}


fn get_window_values(iced_window_id: window::Id, state: &IpgState) -> (bool, Theme) {

    let ipg_window_id_opt = state.windows_iced_ipg_ids.get(&iced_window_id);
    let ipg_window_id = match ipg_window_id_opt {
        Some(id) => *id,
        None => panic!("App: get_window_values, Unable to find ipg_window_id based on iced_window_id {:?}.", iced_window_id)
    };
    
    let window_opt = state.containers.get(&ipg_window_id);
    let ipg_window = get_window_container(window_opt);

    let debug = ipg_window.debug;
    let theme = if let Some(wt) = &ipg_window.theme {
        wt.to_iced()
    } else {
        Theme::TokyoNight
    };
    
    (debug.unwrap_or(false), theme)
}

fn get_tasks(ipg_state: &mut IpgState) -> Task<Message> {
    
    let mut state = access_window_actions();

    let mut actions = vec![];

    for (ipg_id, mode) in state.mode.iter() {
        let iced_id = find_key_for_value(ipg_state.windows_iced_ipg_ids.clone(), *ipg_id);
        actions.push(window::set_mode(iced_id, *mode));
    }
    state.mode = vec![];

    for id in state.decorations.iter() {
        let iced_id = find_key_for_value(ipg_state.windows_iced_ipg_ids.clone(), *id);
        actions.push(window::toggle_decorations(iced_id))
    }
    state.decorations = vec![];

    for (id, width, height) in state.resize.iter() {
        let iced_id = find_key_for_value(ipg_state.windows_iced_ipg_ids.clone(), *id);
        let size = Size::new(*width, *height);
        actions.push(window::resize(iced_id, size))
    }
    state.resize = vec![];

    for (id, x, y) in state.position.iter() {
        let iced_id = find_key_for_value(ipg_state.windows_iced_ipg_ids.clone(), *id);
        let point = Point::new(*x, *y);
        actions.push(window::move_to(iced_id, point))
    }
    state.position = vec![];

    for (ipg_id, level) in state.level.iter() {
        let iced_id = find_key_for_value(ipg_state.windows_iced_ipg_ids.clone(), *ipg_id);
        actions.push(window::set_level(iced_id, *level));
    }
    state.level = vec![];

    drop(state);

    if actions.is_empty() {
        actions.push(Task::none());
    }
    Task::batch(actions)
}


// Central method to get the structures stored in the mutex and then the children 
fn create_content<'a>(
    iced_id: window::Id, 
    state: &'a IpgState, 
    // canvas_state: &'a IpgCanvasState
    ) -> Element<'a, Message> {
    
    let ipg_window_id_opt = state.windows_iced_ipg_ids.get(&iced_id);

    let ipg_window_id = match ipg_window_id_opt {
        Some(id) => id,
        None => panic!("App::create_content: Unable to find ipg_window_id with iced_id {:?}.", iced_id),
    };

    // First we find the unique containers in the window
    let unique_parent_ids = get_unique_parents(state.container_ids.get(ipg_window_id));

    // The unique parent containers are combined with all the children ids held in a vec.
    let all_parent_ids = get_combine_parents_and_children(
                            &unique_parent_ids, state.ids.get(ipg_window_id));

    let content = get_children(&all_parent_ids,
                                                                &0, 
                                                                &unique_parent_ids,
                                                                state,
                                                                // canvas_state
                                                            );
    content
}

fn get_unique_parents(ids: Option<&Vec<usize>>) -> Vec<usize> {
    // The unique ids are the ids sorted and duplicates removed
    let mut unique_ids: Vec<usize> = match ids {
        Some(ids) => ids.to_vec(),
        None => panic!("Container ids in unique_container_ids not found")
    }; 

    unique_ids.sort();
    unique_ids.dedup();

    unique_ids
}

#[derive(Debug, Clone, PartialEq)]
struct ParentChildIds {
    parent_id: usize,
    child_ids: Vec<usize>,
}

fn get_combine_parents_and_children(
    parent_ids: &Vec<usize>, 
    ids_opt: Option<&Vec<IpgWidgetNode>>) 
    -> Vec<ParentChildIds> {

    let mut parent_child_ids: Vec<ParentChildIds> = vec![];

    let ids = match ids_opt {
        Some(ids) => ids,
        None => panic!("ids in get_and_combine_parents_and_children not found")
    };

    for par_id in parent_ids {

        let mut child_ids: Vec<usize> = vec![];

        for ids in ids {
            if par_id == &ids.parent_uid {
                child_ids.push(ids.id);
            }  
        }
        
        parent_child_ids.push(ParentChildIds { parent_id: *par_id, child_ids })
    }

    parent_child_ids
}

fn get_children<'a>(parents: &Vec<ParentChildIds>, 
                index: &usize, 
                parent_ids: &Vec<usize>, 
                state: &'a IpgState,
                // canvas_state: &'a IpgCanvasState,
                ) -> Element<'a, Message> 
{

    let mut content= vec![];

    for child in parents[*index].child_ids.iter() {
        if parent_ids.contains(child) {
            let index = parents.iter().position(|r| &r.parent_id == child).unwrap();
            // content.push(get_children(parents, &index, parent_ids, state, canvas_state));
            content.push(get_children(parents, &index, parent_ids, state));
        } else if get_widget(state, child).is_some() {
                content.push(get_widget(state, child).unwrap());
        }
    }
    let id = &parents[*index].parent_id;

    if id != &0 {
        // get_container(state, id, content, canvas_state)
        get_container(state, id, content)
    } else {
        Column::with_children(content).into()  // the final container
    }
}


fn get_container<'a>(state: &'a IpgState, 
                    id: &usize, 
                    content: Vec<Element<'a, Message>>,
                    // canvas_state: &'a IpgCanvasState,
                    ) -> Element<'a, Message> {

    let container_opt: Option<&IpgContainers> = state.containers.get(id);

    match container_opt 
    {
        Some(container) => 
            match container {
                // IpgContainers::IpgCanvas(canvas) => {
                //     construct_canvas(canvas_state)
                // },
                IpgContainers::IpgColumn(col) => {
                    col.construct(content) 
                },
                IpgContainers::IpgContainer(cont) => {
                    if content.len() > 1 {
                        panic!("A container can have only one widget, place your multiple widgets into a column or row")
                    }
                    
                    cont.construct(content, &state.widgets)
                },
                // IpgContainers::IpgMenu(menu) => {
                //     let bar_style: Option<&IpgWidgets> = 
                //         match menu.menu_bar_style_id {
                //             Some(id) => {
                //                 state.widgets.get(&id)
                //             },
                //             None => None,
                //         };
                //     let menu_style: Option<&IpgWidgets> = 
                //         match menu.menu_style_id {
                //             Some(id) => {
                //                 state.widgets.get(&id)
                //             },
                //             None => None,
                //         };

                //     construct_menu(menu, content, bar_style, menu_style)
                // },
                // IpgContainers::IpgModal(modal) => {
                //     construct_modal(modal, content)
                // },
                IpgContainers::IpgMouseArea(m_area) => {
                    construct_mousearea(m_area, content)
                },
                IpgContainers::IpgOpaque(op) => {
                    let style_opt = 
                        if let Some(id)  = op.style_id {
                            state.widgets.get(&id)
                        } else { None };

                    construct_opaque(op, content, style_opt)
                },
                IpgContainers::IpgTable(table) => {
                    table.construct(content, &state.widgets)
                },
                IpgContainers::IpgRow(row) => {
                    row.construct(content)
                },
                IpgContainers::IpgScrollable(scroll) => {
                    scroll.construct(content, &state.widgets)
                },
                IpgContainers::IpgStack(stk) => {
                    stk.construct(content)
                }
                // IpgContainers::IpgToolTip(tool) => {
                //     let style_opt = 
                //         match tool.style_id {
                //             Some(id) => {
                //                 state.widgets.get(&id)
                //             },
                //             None => None,
                //         };
                //     construct_tool_tip(tool, content, style_opt)
                // },
                IpgContainers::IpgWindow(_wnd) => {
                    construct_window(content)
                },
            },
        
        None => panic!("Container not found in fn get_container id={}", id),        
    }
    
}

fn get_widget<'a>(state: &'a IpgState, id: &usize) -> Option<Element<'a, Message>> {

    let widget_opt = state.widgets.get(id);

    match widget_opt 
    {
        Some(widget) => 
            match widget {      
                IpgWidgets::IpgButton(btn) => {
                    btn.construct(&state.widgets)
                },
                IpgWidgets::IpgCard(crd) => {
                    crd.construct(&state.widgets)
                },
                IpgWidgets::IpgCheckBox(chk) => {
                    chk.construct(&state.widgets)
                },
                IpgWidgets::IpgColorPicker(cp) => {
                    cp.construct(&state.widgets)
                },
                IpgWidgets::IpgDivider(div) => {
                    div.construct(&state.widgets)
                },
                IpgWidgets::IpgImage(image) => {
                    construct_image(image)
                },
                // IpgWidgets::IpgMenu(menu) => {
                //     Some(construct_menu(menu.clone(), state))
                // },
                IpgWidgets::IpgDatePicker(dp) => {
                    dp.construct(&state.widgets)
                },
                IpgWidgets::IpgPickList(pick) => {
                    let style_opt = 
                        if let Some(id) = pick.style_id {
                            state.widgets.get(&id)
                        } else { None };
                    
                    construct_picklist(pick, style_opt)
                },
                IpgWidgets::IpgProgressBar(bar) => {
                    let style_opt = 
                        if let Some(id) = bar.style_id {
                            state.widgets.get(&id)
                        } else { None };
                    construct_progress_bar(bar, style_opt)
                },
                IpgWidgets::IpgRadio(rad) => {
                    let style_opt = 
                        if let Some(id) = rad.style_id {
                            state.widgets.get(&id)
                        } else { None };
                    let font_opt = 
                        if let Some(id) = rad.font_id {
                            state.widgets.get(&id)
                        } else { None };
                    construct_radio(rad, style_opt, font_opt)
                },
                IpgWidgets::IpgRule(rule) => {
                    let style_opt = 
                        if let Some(id) = rule.style_id {
                            state.widgets.get(&id)
                        } else { None };
                    construct_rule(rule, style_opt)
                },
                IpgWidgets::IpgSelectableText(sltxt) => {
                    let font_opt = 
                        if let Some(id) = sltxt.font_id {
                            state.widgets.get(&id)
                        } else { None };
                    construct_selectable_text(sltxt, font_opt)
                },
                IpgWidgets::IpgSeparator(sep) => {
                    let style_opt = 
                        if let Some(id) = sep.style_id {
                            state.widgets.get(&id)
                        } else { None };
                    construct_separator(sep, style_opt)
                }
                IpgWidgets::IpgSlider(slider) => {
                    let style_opt = 
                        if let Some(id) = slider.style_id {
                            state.widgets.get(&id)
                        } else { None };
                    construct_slider(slider, style_opt)
                },
                IpgWidgets::IpgSpace(sp) => {
                    construct_space(sp)
                },
                IpgWidgets::IpgSvg(svg) => {
                    construct_svg(svg)
                },
                IpgWidgets::IpgText(txt) => {
                    txt.construct(&state.widgets)
                },
                IpgWidgets::IpgTextInput(input) => {
                    input.construct(&state.widgets)       
                },
                // IpgWidgets::IpgCanvasTimer(ctimer) => {
                //     let style_opt = match ctimer.style_id {
                //         Some(id) => {
                //             state.widgets.get(&id)
                //         },
                //         None => None,
                //     };
                //     construct_canvas_timer(ctimer, style_opt)
                // },
                IpgWidgets::IpgToggler(tog) => {
                    let style_opt = 
                        if let Some(id) = tog.style_id {
                            state.widgets.get(&id)
                        } else { None };
                    let font_opt = 
                        if let Some(id) = tog.font_id {
                            state.widgets.get(&id)
                        } else { None };
                    construct_toggler(tog, style_opt, font_opt)   
                },
                _ => None,

            },
        None => panic!("App: Widgets not found in fn get_widget id={}", id)
    }
}

fn match_theme_with_debug_color(theme: Theme) -> Color {

    match theme {
        Theme::Light => Color::BLACK,
        Theme::Dark => Color::WHITE,
        Theme::Dracula => Color::WHITE,
        Theme::Ferra => Color::WHITE,
        Theme::Nord => Color::WHITE,
        Theme::SolarizedLight => Color::BLACK,
        Theme::SolarizedDark => Color::WHITE,
        Theme::GruvboxLight => Color::BLACK,
        Theme::GruvboxDark => Color::WHITE,
        Theme::CatppuccinLatte => Color::WHITE,
        Theme::CatppuccinFrappe => Color::WHITE,
        Theme::CatppuccinMacchiato => Color::WHITE,
        Theme::CatppuccinMocha => Color::WHITE,
        Theme::TokyoNight => Color::WHITE,
        Theme::TokyoNightStorm => Color::WHITE,
        Theme::TokyoNightLight => Color::BLACK,
        Theme::KanagawaWave => Color::WHITE,
        Theme::KanagawaDragon => Color::WHITE,
        Theme::KanagawaLotus => Color::BLACK,
        Theme::Moonfly => Color::WHITE,
        Theme::Nightfly => Color::WHITE,
        Theme::Oxocarbon => Color::WHITE,
        Theme::Custom(_) => Color::WHITE,
    }
}

fn get_window_container(container_opt: Option<&IpgContainers>) -> &IpgWindow {
    
    let container = match container_opt {
        Some(cnt) => cnt,
        None => panic!("App: get_window_container: Cannot find IpgContainer"),
    };

    match container {
        IpgContainers::IpgWindow(wnd) => {
            wnd
        },
        _ => panic!("get_window: Not a Window")
    }
}

fn process_widget_updates(
    state: &mut IpgState, 
    // canvas_state: &mut IpgCanvasState
) {
    
    let mut all_updates = access_update_widgets();

    process_deletes(state, &all_updates.deletes);
    all_updates.deletes = vec![];

    process_moves(state, &all_updates.moves);
    all_updates.moves = vec![];

    process_updates(state, &all_updates.updates);
    all_updates.updates = vec![];

    process_shows(state, &all_updates.shows);
    all_updates.shows = vec![];

    drop(all_updates);

    process_new_widgets(state);
}

fn process_deletes(
    state: &mut IpgState,
    deletes: &[usize],
) {
    for wid in deletes.iter() {
        for (_, nodes) in state.ids.iter_mut() {
            if let Some(index) = nodes.iter().position(|node| node.id == *wid) {
                nodes.remove(index);
                state.widgets.remove(wid);
                break;
            }
        }   
    }
}

fn process_moves(
    state: &mut IpgState,
    moves: &[(usize, Option<usize>, Option<usize>, Option<usize>)],
) {
    for (widget_id, move_after, move_before, target_parent_id) in moves.iter() {

        // Find the nodes list containing this widget
        let nodes = state.ids.values_mut()
            .find(|nodes| nodes.iter().any(|n| n.id == *widget_id))
            .expect("move_widget: widget not found in any window");

        // Derive the target parent from the sibling, or use explicit target_parent_id
        let (parent_uid, parent_id) = if let Some(after_id) = move_after {
            let sibling = nodes.iter().find(|n| n.id == *after_id)
                .expect("move_widget: move_after target not found");
            (sibling.parent_uid, sibling.parent_id.clone())
        } else if let Some(before_id) = move_before {
            let sibling = nodes.iter().find(|n| n.id == *before_id)
                .expect("move_widget: move_before target not found");
            (sibling.parent_uid, sibling.parent_id.clone())
        } else if let Some(parent_id) = target_parent_id {
            let parent_node = nodes.iter().find(|n| n.id == *parent_id)
                .expect("move_widget: target_parent_id not found");
            let pid = parent_node.container_id.clone()
                .unwrap_or_else(|| parent_node.parent_id.clone());
            (*parent_id, pid)
        } else {
            panic!("move_widget: at least one of move_after, move_before, or target_parent_id must be provided")
        };

        // Update the widget's parent references
        if let Some(node) = nodes.iter_mut().find(|n| n.id == *widget_id) {
            node.parent_uid = parent_uid;
            node.parent_id = parent_id;
        }

        // Remove the widget from its current position
        let found_index = nodes.iter().position(|n| n.id == *widget_id).unwrap();
        let moved_node = nodes.remove(found_index);

        // Re-insert at the requested position
        if let Some(after_id) = move_after {
            let target = nodes.iter().position(|n| n.id == *after_id)
                .expect("move_widget: move_after target not found");
            nodes.insert(target + 1, moved_node);
        } else if let Some(before_id) = move_before {
            let target = nodes.iter().position(|n| n.id == *before_id)
                .expect("move_widget: move_before target not found");
            nodes.insert(target, moved_node);
        } else {
            nodes.push(moved_node);
        }
    }  
}

fn process_updates(
    state: &mut IpgState,
    updates: &[(usize, PyObject, PyObject)],
) {
    for ((wid, item, value)) in updates.iter() {
        let widget = state.widgets.get_mut(wid);
        if let Some(w) = widget {
            match_widget(w, item, value);
        } else {
            match state.containers.get_mut(wid) {
                Some(cnt) => {
                    let last_id = match_container(
                                                cnt, 
                                                item, 
                                                value, 
                                                // canvas_state, 
                                                state.last_id);
                    if last_id.is_some() {
                        state.last_id = last_id.unwrap();
                    }
                },
                None => panic!("Process_updates: No ids could be found to update with id {wid}.")
            }
        }  
    }
}

fn process_new_widgets(state: &mut IpgState) {
    let mut mutex_state = access_state();
    if !mutex_state.widgets.is_empty() {
        // Move new widgets into the runtime state
        let new_widgets: HashMap<usize, IpgWidgets> = mutex_state.widgets.drain().collect();
        let new_parent_ids: HashMap<usize, String> = mutex_state.widget_container_ids.drain().collect();
        // Sync the last_id so future IDs don't collide
        state.last_id = mutex_state.last_id;
        drop(mutex_state);

        for (id, widget) in new_widgets {
            if let Some(parent_id) = new_parent_ids.get(&id) {
                set_state_of_widget_running_state(state, id, parent_id.clone());
            }
            state.widgets.insert(id, widget);
        }
    } else {
        drop(mutex_state);
    }
}

// fn process_canvas_updates(cs: &mut IpgCanvasState) {
//     let mut canvas_items = access_canvas_update_items();

//     for ((wid, item, value)) in canvas_items.updates.iter() {
//         let mut canvas_widget = if cs.curves.get_mut(wid).is_some(){
//             cs.curves.get_mut(wid).unwrap()
//         } else if cs.image_curves.get_mut(wid).is_some() {
//             cs.image_curves.get_mut(wid).unwrap()
//         } else if cs.text_curves.get_mut(wid).is_some() {
//             cs.text_curves.get_mut(wid).unwrap()
//         } else {
//            panic!("canvas_item_update: canvas item with id, {} not found", wid);
//         };
//         match_canvas_widget(canvas_widget, item, value);
//     }
//     canvas_items.updates = vec![];

// }

fn process_shows(
    state: &mut IpgState,
    shows: &[(usize, bool)],
) {
    for (id, val) in shows.iter() {

        let mut wid = state.widgets.get_mut(id);
        let widget = if wid.is_some() {
            wid.take().unwrap()
        } else {
            panic!("Process shows method- unable to find id {}", id)
        };
        match widget {
            IpgWidgets::IpgButton(bt) => bt.show= *val,
            IpgWidgets::IpgCard(cd) => cd.show= *val,
            IpgWidgets::IpgCheckBox(cb) => cb.show= *val,
            IpgWidgets::IpgColorPicker(cp) => cp.show= *val,
            IpgWidgets::IpgDatePicker(dp) => dp.show= *val,
            IpgWidgets::IpgDivider(d) => d.show = *val,
            IpgWidgets::IpgImage(im) => im.show= *val,
            IpgWidgets::IpgPickList(pl) => pl.show= *val,
            IpgWidgets::IpgProgressBar(pb) => pb.show= *val,
            IpgWidgets::IpgRadio(rd) => rd.show= *val,
            IpgWidgets::IpgRule(ru) => ru.show  = *val,
            IpgWidgets::IpgSelectableText(st) => st.show= *val,
            IpgWidgets::IpgSeparator(sp) => sp.show= *val,
            IpgWidgets::IpgSlider(sl) => sl.show= *val,
            IpgWidgets::IpgSpace(sp) => sp.show= *val,
            IpgWidgets::IpgSvg(svg) => svg.show= *val,
            IpgWidgets::IpgTextInput(ti) => ti.show= *val,
            IpgWidgets::IpgToggler(tog) => tog.show= *val,
            _ => (),
        }
    }
}


use once_cell::sync::Lazy;
fn clone_state(state: &mut IpgState) {
    let mut mutex_state = access_state();
    state.ids = mutex_state.ids.to_owned();
    state.last_id = mutex_state.last_id.to_owned();
    state.containers = mutex_state.containers.to_owned();
    state.container_ids = mutex_state.container_ids.to_owned();
    state.container_wnd_str_ids = mutex_state.container_wnd_str_ids.to_owned();
    state.container_str_ids = mutex_state.container_str_ids.to_owned();
    state.container_window_usize_ids = mutex_state.container_window_usize_ids.to_owned();
    state.widgets = mutex_state.widgets.to_owned();
    state.widget_container_ids = mutex_state.widget_container_ids.to_owned();
    state.windows_iced_ipg_ids = mutex_state.windows_iced_ipg_ids.to_owned();
    state.windows_str_ids = mutex_state.windows_str_ids.to_owned();
    state.windows = mutex_state.windows.to_owned();
    state.window_debug = mutex_state.window_debug.to_owned();
    state.window_theme = mutex_state.window_theme.to_owned();
    state.window_mode = mutex_state.window_mode.to_owned();
    
    state.keyboard_event_id_enabled = mutex_state.keyboard_event_id_enabled.to_owned();
    state.mouse_event_id_enabled = mutex_state.mouse_event_id_enabled.to_owned();
    state.canvas_timer_event_id_enabled = mutex_state.canvas_timer_event_id_enabled.to_owned();
    state.window_event_id_enabled = mutex_state.window_event_id_enabled.to_owned();
    state.touch_event_id_enabled = mutex_state.touch_event_id_enabled.to_owned();
    state.timer_state = mutex_state.timer_state.to_owned();
    state.canvas_timer_duration = mutex_state.canvas_timer_duration.to_owned();

    // zeroing out any unneeded vecs and hashmaps
    mutex_state.widgets = Lazy::new(||HashMap::new());
    mutex_state.widget_container_ids = Lazy::new(||HashMap::new());
    mutex_state.windows = vec![];
    mutex_state.windows_iced_ipg_ids = Lazy::new(||HashMap::new());
    mutex_state.window_debug = Lazy::new(||HashMap::new());
    mutex_state.window_theme = Lazy::new(||HashMap::new());
    mutex_state.window_mode = Lazy::new(||HashMap::new());
    
    drop(mutex_state);
}

fn match_widget(
    widget: &mut IpgWidgets, 
    item: &PyObject, 
    value: &PyObject) 
{
    param_update(widget, item, value);
}

fn match_container(
    container: &mut IpgContainers, 
    item: &PyObject, 
    value: &PyObject, 
    // canvas_state: &mut IpgCanvasState,
    last_id: usize,
    ) -> Option<usize>
{
    container_param_update(container, item, value);
    None
}
