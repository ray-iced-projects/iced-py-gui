//! Minimal IPG app - supports only Window and Button for prototype
#![allow(unused)]
use std::collections::HashMap;

use iced::widget::Column;
use iced::window::{self, Position};
use iced::{font, Element, Point, Size, Subscription, Task, Theme};

use pyo3::{Py, PyAny};
type PyObject = Py<PyAny>;

use crate::state::{IpgContainers, IpgIds, IpgState, IpgWidgets, access_state, access_update_widgets, clone_state_to_runtime, set_state_of_widget_running_state};
use crate::widgets::ipg_button::{BTNMessage, button_callback, button_item_update, button_style_update_item, construct_button};
use crate::widgets::ipg_column::{column_item_update, construct_column};
use crate::widgets::ipg_container::{construct_container, container_item_update};
use crate::widgets::ipg_row::{construct_row, row_item_update};
use crate::widgets::ipg_window::{IpgWindow, IpgWindowMode, window_item_update};

#[derive(Debug, Clone)]
pub enum Message {
    Button(usize, BTNMessage),
    FontLoaded(Result<(), font::Error>),
    WindowOpened(window::Id, Option<Point>, Size),
    EventWindow((window::Id, iced::Event)),
}

#[derive(Debug, Clone)]
pub struct App {
    state: IpgState,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let mut state = IpgState::new();
        clone_state_to_runtime(&mut state);

        let open = add_windows(&mut state);

        (Self { state }, Task::batch(open))
    }

    pub fn title(&self, iced_window_id: window::Id) -> String {
        let ipg_window_id = match self.state.windows_iced_ipg_ids.get(&iced_window_id) {
            Some(id) => *id,
            None => panic!(
                "App: title, Unable to find ipg_window_id based on iced_window_id {:?}.",
                iced_window_id
            ),
        };

        let window_opt = self.state.containers.get(&ipg_window_id);
        let ipg_window = get_window_container(window_opt);

        ipg_window.title.clone()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FontLoaded(_) => Task::none(),
            Message::Button(id, message) => {
                button_callback(id, message);
                Task::none()
            }
            Message::WindowOpened(_, _, _) => Task::none(),
            Message::EventWindow((window_id, event)) => {
                // Handle window close
                if let iced::Event::Window(window::Event::CloseRequested) = event {
                    // Find if this is an exit_on_close window
                    if let Some(ipg_id) = self.state.windows_iced_ipg_ids.get(&window_id) {
                        if let Some(IpgContainers::IpgWindow(wnd)) = self.state.containers.get(ipg_id) {
                            if wnd.exit_on_close_request {
                                return iced::exit();
                            }
                        }
                    }
                    // Hide the window
                    self.state.windows_hidden.push(window_id);
                    if self.state.windows_opened.len() == self.state.windows_hidden.len() {
                        return iced::exit();
                    }
                }
                Task::none()
            }
        }
    }

    pub fn view(&self, window_id: window::Id) -> Element<'_, Message> {
        create_content(window_id, &self.state)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // Always listen to window events for close handling
        let w_event = window::events()
            .map(|(id, event)| Message::EventWindow((id, iced::Event::Window(event))));
        
        Subscription::batch([w_event])
    }

    pub fn theme(&self, iced_window_id: window::Id) -> Theme {
        let ipg_window_id_opt = self.state.windows_iced_ipg_ids.get(&iced_window_id);
        let ipg_window_id = match ipg_window_id_opt {
            Some(id) => *id,
            None => panic!(
                "App: theme, Unable to find ipg_window_id based on iced_window_id {:?}.",
                iced_window_id
            ),
        };

        let window_opt = self.state.containers.get(&ipg_window_id);
        let ipg_window = get_window_container(window_opt);

        ipg_window.theme.clone()
    }

    pub fn scale_factor(&self, iced_window_id: window::Id) -> f32 {
        let ipg_window_id_opt = self.state.windows_iced_ipg_ids.get(&iced_window_id);
        let ipg_window_id = match ipg_window_id_opt {
            Some(id) => *id,
            None => panic!(
                "App: scale_factor, Unable to find ipg_window_id based on iced_window_id {:?}.",
                iced_window_id
            ),
        };

        let window_opt = self.state.containers.get(&ipg_window_id);
        let ipg_window = get_window_container(window_opt);

        ipg_window.scale_factor as f32
    }
}

fn add_windows(state: &mut IpgState) -> Vec<Task<Message>> {
    let mut spawn_window: Vec<Task<Message>> = vec![];

    for i in 0..state.windows.len() {
        let visible = match state.windows[i].mode {
            IpgWindowMode::Windowed => true,
            IpgWindowMode::FullScreen => true,
            IpgWindowMode::Closed => false,
        };

        let (iced_id, open) = window::open(window::Settings {
            size: state.windows[i].size,
            min_size: state.windows[i].min_size,
            max_size: state.windows[i].max_size,
            position: state.windows[i].position,
            visible,
            resizable: state.windows[i].resizable,
            decorations: state.windows[i].decorations,
            transparent: state.windows[i].transparent,
            level: get_level(&state.windows[i].level),
            exit_on_close_request: state.windows[i].exit_on_close_request,
            ..Default::default()
        });

        let id = state.windows[i].id;
        let debug = state.windows[i].debug;
        let theme = state.windows[i].theme.clone();
        let mode = state.windows[i].mode.clone();

        state.window_debug.insert(iced_id, (id, debug));
        state.window_theme.insert(iced_id, (id, theme));
        state.window_mode.insert(iced_id, (id, mode.to_iced()));
        state.windows_opened.push(iced_id);
        
        if !visible {
            state.windows_hidden.push(iced_id);
        }

        let ipg_id = state.windows[i].id;
        state.windows_iced_ipg_ids.insert(iced_id, ipg_id);
        let size = state.windows[i].size;
        spawn_window.push(open.map(move |_| Message::WindowOpened(iced_id, None, size)));
    }

    // Load fonts
    spawn_window.push(
        font::load(include_bytes!("./graphics/fonts/bootstrap-icons.ttf").as_slice())
            .map(Message::FontLoaded),
    );
    spawn_window.push(
        font::load(include_bytes!("./graphics/fonts/Roboto.ttf").as_slice())
            .map(Message::FontLoaded),
    );

    spawn_window
}

fn get_level(level: &crate::widgets::ipg_window::IpgWindowLevel) -> window::Level {
    use crate::widgets::ipg_window::IpgWindowLevel;
    match level {
        IpgWindowLevel::Normal => window::Level::Normal,
        IpgWindowLevel::AlwaysOnBottom => window::Level::AlwaysOnBottom,
        IpgWindowLevel::AlwaysOnTop => window::Level::AlwaysOnTop,
    }
}

fn get_window_container(container_opt: Option<&IpgContainers>) -> &IpgWindow {
    let container = match container_opt {
        Some(cnt) => cnt,
        None => panic!("App: get_window_container: Cannot find IpgContainer"),
    };

    match container {
        IpgContainers::IpgWindow(wnd) => wnd,
        _ => panic!("get_window: Not a Window"),
    }
}

// Create the content for a window
fn create_content<'a>(iced_id: window::Id, state: &'a IpgState) -> Element<'a, Message> {
    let ipg_window_id_opt = state.windows_iced_ipg_ids.get(&iced_id);

    let ipg_window_id = match ipg_window_id_opt {
        Some(id) => id,
        None => panic!(
            "App::create_content: Unable to find ipg_window_id with iced_id {:?}.",
            iced_id
        ),
    };

    // Get unique parent container IDs
    let unique_parent_ids = get_unique_parents(state.container_ids.get(ipg_window_id));

    // Combine parents with their children
    let all_parent_ids =
        get_combine_parents_and_children(&unique_parent_ids, state.ids.get(ipg_window_id));

    // Build the widget tree
    get_children(&all_parent_ids, &0, &unique_parent_ids, state)
}

fn get_unique_parents(ids: Option<&Vec<usize>>) -> Vec<usize> {
    let mut unique_ids: Vec<usize> = match ids {
        Some(ids) => ids.to_vec(),
        None => panic!("Container ids in unique_container_ids not found"),
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
    ids_opt: Option<&Vec<IpgIds>>,
) -> Vec<ParentChildIds> {
    let mut parent_child_ids: Vec<ParentChildIds> = vec![];

    let ids = match ids_opt {
        Some(ids) => ids,
        None => panic!("ids in get_and_combine_parents_and_children not found"),
    };

    for par_id in parent_ids {
        let mut child_ids: Vec<usize> = vec![];

        for id_info in ids {
            if par_id == &id_info.parent_uid {
                child_ids.push(id_info.id);
            }
        }

        parent_child_ids.push(ParentChildIds {
            parent_id: *par_id,
            child_ids,
        })
    }

    parent_child_ids
}

fn get_children<'a>(
    parents: &Vec<ParentChildIds>,
    index: &usize,
    parent_ids: &Vec<usize>,
    state: &'a IpgState,
) -> Element<'a, Message> {
    let mut content = vec![];

    for child in parents[*index].child_ids.iter() {
        if parent_ids.contains(child) {
            let idx = parents.iter().position(|r| &r.parent_id == child).unwrap();
            content.push(get_children(parents, &idx, parent_ids, state));
        } else if let Some(widget) = get_widget(state, child) {
            content.push(widget);
        }
    }

    let id = &parents[*index].parent_id;

    if id != &0 {
        get_container(state, id, content)
    } else {
        Column::with_children(content).into() // the final container
    }
}

fn get_container<'a>(
    state: &'a IpgState,
    id: &usize,
    content: Vec<Element<'a, Message>>,
) -> Element<'a, Message> {
    let container_opt: Option<&IpgContainers> = state.containers.get(id);

    match container_opt {
        Some(container) => match container {
            IpgContainers::IpgWindow(_wnd) => {
                // Window just wraps content in a column
                Column::with_children(content).into()
            }
            IpgContainers::IpgColumn(col) => {
                construct_column(col, content)
            },
            IpgContainers::IpgContainer(con) => {
                if content.len() > 1 {
                        panic!("A container can have only one widget, place your multiple widgets into a column or row")
                    }
                    let style_opt = 
                        match con.style_id {
                            Some(id) => {
                                state.widgets.get(&id)
                            },
                            None => None,
                        };

                    construct_container(con, content, style_opt)
            },
            IpgContainers::IpgRow(row) => {
                construct_row(row, content)
            },
        },
        None => panic!("Container not found in fn get_container id={}", id),
    }
}

fn get_widget<'a>(state: &'a IpgState, id: &usize) -> Option<Element<'a, Message>> {
    let widget_opt = state.widgets.get(id);

    match widget_opt {
        Some(widget) => match widget {
            IpgWidgets::IpgButton(btn) => {
                let style_opt = match btn.style_id {
                        Some(id) => {
                            state.widgets.get(&id)
                        },
                        None => None,
                    };
                construct_button(btn, style_opt)
            }
            // Add other widgets as needed
            _ => None,
        },
        None => panic!("App: Widget not found in fn get_widget id={}", id),
    }
}

fn process_updates(state: &mut IpgState) {
    
    let mut all_updates = access_update_widgets();

    // for deletes
    for (window_id, wid) in all_updates.deletes.iter() {
        let iced_id = match state.windows_str_ids.get(window_id) {
            Some(id) => *id,
            None => panic!("Window_id {} not found in delete_item", window_id)
        };

        let ipg_ids = match state.ids.get_mut(&iced_id) {
            Some(ids) => ids,
            None => panic!("Ids not found for window_id {} in delete_item", window_id)
        };

        let mut index: i32 = -1;

        for (i, ipg_id) in ipg_ids.iter().enumerate() {
            if ipg_id.id == *wid {
                index = i as i32;
                break;
            }
        }

        if index == -1 {
            panic!("item with id {wid} could not be found to delete")
        }

        ipg_ids.remove(index as usize);

        state.widgets.remove(wid);   
    }
    all_updates.deletes = vec![];

    // for moves
    for (window_id, 
        widget_id, 
        target_container_str_id, 
        move_after, 
        move_before) in all_updates.moves.iter() {

        let container_str_id_opt = state.container_str_ids.get(target_container_str_id);

        let container_usize_id = match container_str_id_opt {
            Some(id) => *id,
            None => panic!("move_widget: unable to find the target container id based on the id {}", target_container_str_id)
        };

        let window_id_usize_opt = state.windows_str_ids.get(window_id);

        let window_id_usize = match window_id_usize_opt {
            Some(id) => *id,
            None => panic!("move_widget: unable to find the window_id using the id {}", window_id)
        };

        let window_widget_ids_opt = state.ids.get_mut(&window_id_usize);

        let window_widget_ids = match window_widget_ids_opt {
            Some(ids) => ids,
            None => panic!("move_widget: unable to find widget using window_id {}", window_id)    
        };

        let mut before = false;
        let pos_id = if move_after.is_some() {
            move_after.unwrap()
        } else if move_before.is_some() { 
            before = true;
            move_before.unwrap()
        } else {
            1_000_000
        };

        //  set some large numbers to break early
        let mut found_index = 1_000_000;
        let mut target_index: usize = 1_000_000;

        for (i, ids) in window_widget_ids.iter_mut().enumerate() {
            if ids.id == *widget_id {
                ids.parent_uid = container_usize_id;
                ids.parent_id = target_container_str_id.clone();
                found_index = i;
            }
            if ids.id == pos_id {
                target_index = i
            }
            if found_index != 1_000_000 && (target_index != 1_000_000 || pos_id == 1_000_000) {
                break;
            }
        }
        
        let move_ids = window_widget_ids.remove(found_index);
        
        if pos_id == 1_000_000 {
            window_widget_ids.push(move_ids);
        } else if before {
            window_widget_ids.insert(target_index-1, move_ids);
        } else {
            window_widget_ids.insert(target_index, move_ids);
        }
    }  
    all_updates.moves = vec![];

    // for item updates
    for ((wid, item, value)) in all_updates.updates.iter() {
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
                                    state.last_id);
                    if last_id.is_some() {
                        state.last_id = last_id.unwrap();
                    }
                },
                None => panic!("Item_update: Widget, Container, or Window with id {wid} not found.")
            }
        }  
    }
    all_updates.updates = vec![];

    // updates for shows
    for (window_id, ids) in all_updates.shows.iter() {
        let iced_id = match state.windows_str_ids.get(window_id) {
            Some(id) => *id,
            None => panic!("Window_id {} not found in hide_item", window_id)
        };

        let ipg_ids = match state.ids.get_mut(&iced_id) {
            Some(ids) => ids,
            None => panic!("Ids not found for window_id {} in hide_item", window_id)
        };

        show_widget(state, ids);
    }
    
    all_updates.shows = vec![];

    // transfer any widgets or containers over
    let mut mutex_state = access_state();
    let widgets = mutex_state.widgets.to_owned();

    for key in widgets.keys() {
        let value = mutex_state.widgets.remove(key).unwrap();
        let parent_id = get_widget_parent_id(&value);
        set_state_of_widget_running_state(state, *key, parent_id);
        state.widgets.insert(*key, value);
    }

    drop(mutex_state);

}

fn match_container(
    container: &mut IpgContainers, 
    item: &PyObject, 
    value: &PyObject, 
    // canvas_state: &mut IpgCanvasState,
    last_id: usize,
    ) -> Option<usize>
{
    match container {
        // IpgContainers::IpgCanvas(_can) => {
        //     canvas_item_update(canvas_state, item, value, last_id)
        // },
        IpgContainers::IpgColumn(col) => {
            column_item_update(col, item, value);
            None
        },
        IpgContainers::IpgContainer(cont) => {
            container_item_update(cont, item, value);
            None
        },
        // IpgContainers::IpgMenu(menu) => {
        //     menu_item_update(menu, item, value);
        //     None
        // },
        // IpgContainers::IpgMouseArea(m_area) => {
        //     mousearea_item_update(m_area, item, value);
        //     None
        // },
        // IpgContainers::IpgOpaque(op) => {
        //     opaque_item_update(op, item, value);
        //     None
        // },
        IpgContainers::IpgRow(row) => {
            row_item_update(row, item, value);
            None
        },
        // IpgContainers::IpgStack(stack) => {
        //     stack_item_update(stack, item, value);
        //     None
        // },
        // IpgContainers::IpgTable(table) => {
        //     table_item_update(table, item, value);
        //     None
        // },
        // IpgContainers::IpgScrollable(scroll) => {
        //     scrollable_item_update(scroll, item, value);
        //     None
        // },
        // IpgContainers::IpgToolTip(tool) => {
        //     tooltip_item_update(tool, item, value);
        //     None
        // },
        IpgContainers::IpgWindow(wnd) => {
            window_item_update(wnd, item, value);
            None
        },
    }
}

fn match_widget(
    widget: &mut IpgWidgets, 
    item: &PyObject, 
    value: &PyObject) 
{
    match widget {
        IpgWidgets::IpgButton(btn) => {
            button_item_update(btn, item, value);
        },
        IpgWidgets::IpgButtonStyle(style) => {
            button_style_update_item(style, item, value);
        },
        // IpgWidgets::IpgCard(card) => {
        //         card_item_update(card, item, value);
        //     },
        // IpgWidgets::IpgCardStyle(style) => {
        //         card_style_update(style, item, value);
        //     },
        // IpgWidgets::IpgCheckBox(chk) => {
        //         checkbox_item_update(chk, item, value);
        //     },
        // IpgWidgets::IpgCheckboxStyle(style) => {
        //         checkbox_style_update_item(style, item, value);
        //     },
        // IpgWidgets::IpgColorPicker(cp) => {
        //         color_picker_update(cp, item, value);
        //     },
        // IpgWidgets::IpgColorPickerStyle(style) => {
        //         color_picker_style_update_item(style, item, value);
        //     },
        // IpgWidgets::IpgContainerStyle(style) => {
        //         container_style_update_item(style, item, value);
        //     },
        // IpgWidgets::IpgDatePicker(dp) => {
        //         date_picker_item_update(dp, item, value);
        //     },
        // IpgWidgets::IpgDividerHorizontal(div) => {
        //         divider_horizontal_item_update(div, item, value);
        //     },
        // IpgWidgets::IpgDividerVertical(div) => {
        //         divider_vertical_item_update(div, item, value);
        //     },
        // IpgWidgets::IpgDividerStyle(style) => {
        //         divider_style_update_item(style, item, value);
        //     }
        // IpgWidgets::IpgImage(img) => {
        //         image_item_update(img, item, value);
        //     },
        // IpgWidgets::IpgMenuStyle(style) => {
        //         menu_style_update_item(style, item, value);
        //     },
        // IpgWidgets::IpgMenuBarStyle(style) => {
        //         menu_bar_style_update_item(style, item, value);
        //     },
        // IpgWidgets::IpgOpaqueStyle(style) => {
        //         opaque_style_update_item(style, item, value);
        //     }
        // IpgWidgets::IpgPickList(pl) => {
        //         pick_list_item_update(pl, item, value);
        //     },
        // IpgWidgets::IpgPickListStyle(style) => {
        //         pick_list_style_update_item(style, item, value);
        //     },
        // IpgWidgets::IpgProgressBar(pb) => {
        //         progress_bar_item_update(pb, item, value);
        //     },
        // IpgWidgets::IpgProgressBarStyle(style) => {
        //         progress_bar_style_update_item(style, item, value);
        //     },
        // IpgWidgets::IpgRadio(rd) => {
        //         radio_item_update(rd, item, value);
        //     },
        // IpgWidgets::IpgRadioStyle(style) => {
        //         radio_style_update_item(style, item, value);
        //     },
        // IpgWidgets::IpgRule(_) => (),
        // IpgWidgets::IpgRuleStyle(style) => {
        //         rule_style_update_item(style, item, value);
        //     },
        // IpgWidgets::IpgScrollableStyle(style) => {
        //         scroll_style_update_item(style, item, value)
        //     },
        // IpgWidgets::IpgSelectableText(st) => {
        //         selectable_text_item_update(st, item, value);
        //     },
        // IpgWidgets::IpgSeparator(sep) => {
        //         separator_item_update(sep, item, value);
        //     },
        // IpgWidgets::IpgSeparatorStyle(style) => {
        //         separator_style_update_item(style, item, value);
        //     },
        // IpgWidgets::IpgSlider(slider) => {
        //         slider_item_update(slider, item, value)
        //     },
        // IpgWidgets::IpgSliderStyle(style) => {
        //         slider_style_update_item(style, item, value)
        //     },
        // IpgWidgets::IpgSpace(_) => (),
        // IpgWidgets::IpgSvg(sg) => {
        //         svg_item_update(sg, item, value);
        //     },
        // IpgWidgets::IpgTableStyle(style) => {
        //         table_style_update_item(style, item, value);
        //     }
        // IpgWidgets::IpgText(txt) => {
        //         text_item_update(txt, item, value);
        //     },
        // IpgWidgets::IpgTextInput(ti) => {
        //         text_input_item_update(ti, item, value);
        //     },
        // IpgWidgets::IpgTextInputStyle(style) => {
        //         text_input_style_update_item(style, item, value);
        //     },
        // IpgWidgets::IpgTimer(tim) => {
        //         timer_item_update(tim, item, value);
        //     },
        // IpgWidgets::IpgTimerStyle(style) => {
        //         timer_style_update_item(style, item, value);
        //     },
        // IpgWidgets::IpgCanvasTimer(ctim) => {
        //         canvas_timer_item_update(ctim, item, value);
        //     },
        // IpgWidgets::IpgCanvasTimerStyle(style) => {
        //         canvas_timer_style_update_item(style, item, value);
        //     },
        // IpgWidgets::IpgToggler(tog) => {
        //         toggler_item_update(tog, item, value);
        //     },
        // IpgWidgets::IpgTogglerStyle(style) => {
        //         toggler_style_update_item(style, item, value);
        //     },
        // IpgWidgets::IpgToolTipStyle(style) => {
        //         tool_tip_style_update_item(style, item, value);
        //     },
        _ => ()
    }
}

fn show_widget(state: &mut IpgState, ids: &[(usize, bool)]) {
    
    for (id, value) in ids.iter() {
        
        let mut wid = state.widgets.get_mut(id);
        let widget = if wid.is_some() {
            wid.take().unwrap()
        } else {
            panic!("Show_items - unable to find id {}", id)
        };
        match widget {
            IpgWidgets::IpgButton(ipg_button) => ipg_button.show= *value,
            // IpgWidgets::IpgCard(ipg_card) => ipg_card.show= *value,
            // IpgWidgets::IpgCheckBox(ipg_check_box) => ipg_check_box.show= *value,
            // IpgWidgets::IpgColorPicker(ipg_color_picker) => ipg_color_picker.show= *value,
            // IpgWidgets::IpgDatePicker(ipg_date_picker) => ipg_date_picker.show= *value,
            // IpgWidgets::IpgImage(ipg_image) => ipg_image.show= *value,
            // IpgWidgets::IpgPickList(ipg_pick_list) => ipg_pick_list.show= *value,
            // IpgWidgets::IpgProgressBar(ipg_progress_bar) => ipg_progress_bar.show= *value,
            // IpgWidgets::IpgRadio(ipg_radio) => ipg_radio.show= *value,
            // IpgWidgets::IpgRule(ipg_rule) => ipg_rule.show  = * value,
            // IpgWidgets::IpgSelectableText(ipg_selectable_text) => ipg_selectable_text.show= *value,
            // IpgWidgets::IpgSeparator(ipg_separator) => ipg_separator.show= *value,
            // IpgWidgets::IpgSlider(ipg_slider) => ipg_slider.show= *value,
            // IpgWidgets::IpgSpace(ipg_space) => ipg_space.show= *value,
            // IpgWidgets::IpgSvg(ipg_svg) => ipg_svg.show= *value,
            // IpgWidgets::IpgText(ipg_text) => ipg_text.show= *value,
            // IpgWidgets::IpgTextInput(ipg_text_input) => ipg_text_input.show= *value,
            // IpgWidgets::IpgTimer(ipg_timer) => ipg_timer.show= *value,
            // IpgWidgets::IpgToggler(ipg_toggler) => ipg_toggler.show= *value,
            _ => (),
        }
    }
    
}

fn get_widget_parent_id(widget: &IpgWidgets) -> String {
    match widget {
        IpgWidgets::IpgButton(ipg_button) => ipg_button.parent_id.clone(),
        // IpgWidgets::IpgButtonStyle(ipg_button_style) => todo!(),
        // IpgWidgets::IpgCard(ipg_card) => ipg_card.parent_id.clone(),
        // IpgWidgets::IpgCardStyle(ipg_card_style) => todo!(),
        // IpgWidgets::IpgCheckBox(ipg_check_box) => todo!(),
        // IpgWidgets::IpgCheckboxStyle(ipg_checkbox_style) => todo!(),
        // IpgWidgets::IpgColorPicker(ipg_color_picker) => todo!(),
        // IpgWidgets::IpgColorPickerStyle(ipg_color_picker_style) => todo!(),
        // IpgWidgets::IpgContainerStyle(ipg_container_style) => todo!(),
        // IpgWidgets::IpgDatePicker(ipg_date_picker) => todo!(),
        // IpgWidgets::IpgDividerHorizontal(_) => todo!(),
        // IpgWidgets::IpgDividerVertical(_) => todo!(),
        // IpgWidgets::IpgDividerStyle(_) => todo!(),
        // IpgWidgets::IpgImage(ipg_image) => todo!(),
        // IpgWidgets::IpgMenuStyle(ipg_menu_style) => todo!(),
        // IpgWidgets::IpgMenuBarStyle(ipg_menu_bar_style) => todo!(),
        // IpgWidgets::IpgOpaqueStyle(ipg_opaque_style) => todo!(),
        // IpgWidgets::IpgPickList(ipg_pick_list) => todo!(),
        // IpgWidgets::IpgPickListStyle(ipg_pick_list_style) => todo!(),
        // IpgWidgets::IpgProgressBar(ipg_progress_bar) => todo!(),
        // IpgWidgets::IpgProgressBarStyle(ipg_progress_bar_style) => todo!(),
        // IpgWidgets::IpgRadio(ipg_radio) => todo!(),
        // IpgWidgets::IpgRadioStyle(ipg_radio_style) => todo!(),
        // IpgWidgets::IpgRule(ipg_rule) => todo!(),
        // IpgWidgets::IpgRuleStyle(ipg_rule_style) => todo!(),
        // IpgWidgets::IpgScrollableStyle(ipg_scrollable_style) => todo!(),
        // IpgWidgets::IpgSelectableText(ipg_selectable_text) => todo!(),
        // IpgWidgets::IpgSeparator(ipg_separator) => todo!(),
        // IpgWidgets::IpgSeparatorStyle(ipg_separator_style) => todo!(),
        // IpgWidgets::IpgSlider(ipg_slider) => todo!(),
        // IpgWidgets::IpgSliderStyle(ipg_slider_style) => todo!(),
        // IpgWidgets::IpgSpace(ipg_space) => todo!(),
        // IpgWidgets::IpgSvg(ipg_svg) => todo!(),
        // IpgWidgets::IpgTableStyle(ipg_table_style) => todo!(),
        // IpgWidgets::IpgText(text) => text.parent_id.clone(),
        // IpgWidgets::IpgTextInput(ipg_text_input) => todo!(),
        // IpgWidgets::IpgTextInputStyle(ipg_text_input_style) => todo!(),
        // IpgWidgets::IpgTimer(ipg_timer) => todo!(),
        // IpgWidgets::IpgTimerStyle(ipg_timer_style) => todo!(),
        // IpgWidgets::IpgCanvasTimer(ipg_canvas_timer) => todo!(),
        // IpgWidgets::IpgCanvasTimerStyle(ipg_canvas_timer_style) => todo!(),
        // IpgWidgets::IpgToggler(ipg_toggler) => todo!(),
        // IpgWidgets::IpgTogglerStyle(ipg_toggler_style) => todo!(),
        // IpgWidgets::IpgToolTipStyle(ipg_tool_tip) => todo!(),
        _ => String::new()
    }
}
