//! Minimal IPG app - supports only Window and Button for prototype
#![allow(unused)]

use std::collections::HashMap;

use iced::widget::Column;
use iced::window::{self, Position};
use iced::{font, Element, Point, Size, Subscription, Task, Theme};

use crate::state::{access_state, clone_state_to_runtime, IpgIds, IpgState, IpgContainers, IpgWidgets};
use crate::widgets::ipg_button::{button_callback, construct_button, BTNMessage};
use crate::widgets::ipg_column::construct_column;
use crate::widgets::ipg_container::construct_container;
use crate::widgets::ipg_row::construct_row;
use crate::widgets::ipg_window::{IpgWindow, IpgWindowMode};

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
