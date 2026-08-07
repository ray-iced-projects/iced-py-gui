
use std::collections::HashMap;
use std::time::Instant;

use iced::time::milliseconds;
use iced::widget::{Column, scrollable};
use iced::window;
use iced::clipboard;
use iced::{Element, Event, Point, Size, Subscription, Task, Theme, font, time};

use pyo3::{Py, PyAny};
type PyObject = Py<PyAny>;

use crate::ipg_widgets::ipg_canvas_draw::canvas_draw::CanvasWidget;
use crate::py_api::helpers::find_key_for_value;
use crate::state::{Containers, IpgState, WidgetNode, Widgets, access_clipboard_actions, access_file_dialog_actions, access_state, access_update_widgets, access_window_actions, set_state_of_widget_running_state};
use crate::widgets::callbacks::invoke_callback_with_args;

use crate::widgets::ipg_button::{BtnMessage, button_callback};
use crate::widgets::ipg_card::{CardMessage, card_callback};
use crate::widgets::ipg_color_picker::{ColorPikMessage, color_picker_callback};
use crate::widgets::ipg_checkbox::{ChkMessage, checkbox_callback};
use crate::widgets::ipg_combo_box::{CBMessage, combo_box_callback};
use crate::widgets::ipg_date_picker::{DatePikMessage, date_picker_callback};
use crate::widgets::ipg_file_system::{FileSystemMessage, fsd_callback};
use crate::widgets::ipg_sash::{sash_callback, SashMessage};
use crate::widgets::ipg_draw::{draw_callback, process_draw_updates};
use crate::widgets::ipg_events::{process_keyboard_events, process_mouse_events, process_touch_events, process_window_event};
use crate::widgets::ipg_mouse_area::{MaMessage, mousearea_callback};
use crate::widgets::ipg_pick_list::{PLMessage, pick_list_callback};
use crate::widgets::ipg_popup::{PopUpMessage, popup_callback};
use crate::widgets::ipg_radio::{RDMessage, radio_callback};
use crate::widgets::ipg_scrollable::scrollable_callback;
use crate::widgets::ipg_slider::{SldMessage, slider_callback};
use crate::widgets::ipg_table::{TableBasicMessage, TableSections, table_callback};
use crate::widgets::ipg_text_rich::rich_text_callback;
use crate::widgets::ipg_text_editor::{TxtEdMessage, text_ed_callback};
use crate::widgets::ipg_text_input::{TIMessage, text_input_callback};
use crate::widgets::ipg_timer::timer_callback;
use crate::widgets::ipg_toggle::{TOGMessage, toggle_callback};
use crate::widgets::ipg_window::{Window, add_windows, construct_window};
use crate::widgets::ipg_menu::GroupedItem;
use crate::widgets::widget_param_update::{param_update, container_param_update};


#[derive(Debug, Clone)]
pub enum Message {
    Button(usize, BtnMessage),
    CanvasDraw(usize, CanvasWidget),
    Card(usize, CardMessage),
    CheckBox(usize, ChkMessage),
    ColorPicker(usize, ColorPikMessage),
    ComboBox(usize, CBMessage),
    DatePicker(usize, DatePikMessage),
    EventKeyboard(Event),
    EventMouse(Event),
    EventWindow((window::Id, Event)),
    EventTouch(Event),
    FileSystemWindow(usize, FileSystemMessage),
    MouseArea(usize, MaMessage),
    PickList(usize, PLMessage),
    PopUp(usize, PopUpMessage),
    Radio(usize, RDMessage),
    RichTextLinkClicked(usize, usize),
    Sash(usize, SashMessage),
    Scrolled(scrollable::Viewport, usize),
    Slider(usize, SldMessage),
    Table(usize, TableBasicMessage),
    TextEditor(usize, TxtEdMessage),
    TextInput(usize, TIMessage),
    Toggler(usize, TOGMessage),
    Tick(usize, Instant),
    ClipboardReadResult(usize, Option<String>),
    FontLoaded(Result<(), font::Error>),
    WindowOpened(window::Id, Option<Point>, Size),

}

#[derive(Debug, Clone)]
pub struct App {
    state: IpgState,
}

impl App {
    
    pub fn new() -> (Self, Task<Message>) {
        let mut state = IpgState::new();
        clone_state(&mut state);

        let mut open = add_windows(&mut state);
        open.push(font::load(include_bytes!("./graphics/fonts/bootstrap-icons.ttf").as_slice()).map(Message::FontLoaded));
        open.push(font::load(include_bytes!("./graphics/fonts/Roboto/static/Roboto-Black.ttf").as_slice()).map(Message::FontLoaded));
        open.push(font::load(include_bytes!("./graphics/fonts/Roboto/static/Roboto-Bold.ttf").as_slice()).map(Message::FontLoaded));
        open.push(font::load(include_bytes!("./graphics/fonts/Roboto/static/Roboto-ExtraBold.ttf").as_slice()).map(Message::FontLoaded));
        open.push(font::load(include_bytes!("./graphics/fonts/Roboto/static/Roboto-ExtraLight.ttf").as_slice()).map(Message::FontLoaded));
        open.push(font::load(include_bytes!("./graphics/fonts/Roboto/static/Roboto-Light.ttf").as_slice()).map(Message::FontLoaded));
        open.push(font::load(include_bytes!("./graphics/fonts/Roboto/static/Roboto-Medium.ttf").as_slice()).map(Message::FontLoaded));
        open.push(font::load(include_bytes!("./graphics/fonts/Roboto/static/Roboto-Regular.ttf").as_slice()).map(Message::FontLoaded));
        open.push(font::load(include_bytes!("./graphics/fonts/Roboto/static/Roboto-SemiBold.ttf").as_slice()).map(Message::FontLoaded));
        open.push(font::load(include_bytes!("./graphics/fonts/Roboto/static/Roboto-Thin.ttf").as_slice()).map(Message::FontLoaded));
        open.push(font::load(include_bytes!("./graphics/fonts/FiraSans-Regular.ttf").as_slice()).map(Message::FontLoaded));
        open.push(font::load(include_bytes!("./graphics/fonts/Newsreader.ttf").as_slice()).map(Message::FontLoaded));

        // Load user-provided fonts
        let user_fonts = {
            let mut mutex_state = access_state();
            std::mem::take(&mut mutex_state.user_fonts)
        };
        for font_bytes in user_fonts {
            open.push(font::load(font_bytes).map(Message::FontLoaded));
        }
        
        (
            Self {
                state,
                // draw_state,
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
                process_draw_updates(&mut self.state);
                get_tasks(&mut self.state)
            },
            Message::CanvasDraw(id, message) => {
                draw_callback(&mut self.state, id, message);
                process_draw_updates(&mut self.state);
                get_tasks(&mut self.state)
            },
            Message::Card(id, message) => {
                card_callback(id, message);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::CheckBox(id, message) => {
                checkbox_callback(&mut self.state, id, message);
                process_widget_updates(&mut self.state);
                process_draw_updates(&mut self.state);
                get_tasks(&mut self.state)
            },
            Message::ClipboardReadResult(id, text) => {
                invoke_callback_with_args(
                    id,
                    "callback",
                    "Clipboard",
                    text,
                    "def callback(req_id: int, text: str | None)",
                );
                process_widget_updates(&mut self.state);
                get_tasks(&mut self.state)
            },
            Message::ColorPicker(id, message) => {
                let task = 
                    color_picker_callback(&mut self.state, id, message);
                process_widget_updates(&mut self.state);
                process_draw_updates(&mut self.state);
                match task {
                    Some(t) => t,
                    None => Task::none()
                }
            },
            Message::ComboBox(id, message) => {
                combo_box_callback(&mut self.state, id, message);
                process_widget_updates(&mut self.state);
                process_draw_updates(&mut self.state);
                Task::none()
            }
            Message::DatePicker(id, message) => {
                let task = 
                    date_picker_callback(&mut self.state, id, message);
                process_widget_updates(&mut self.state);
                process_draw_updates(&mut self.state);
                match task {
                    Some(t) => t,
                    None => Task::none()
                }
            },
            Message::EventKeyboard(event) => {
                process_keyboard_events(event, self.state.keyboard_event_id_enabled.0);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::EventMouse(event) => {
                process_mouse_events(event, self.state.mouse_event_id_enabled.0);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::EventWindow((window_id, event)) => {
                process_window_event(&mut self.state, event, window_id);
                process_widget_updates(&mut self.state);
                if self.state.windows_opened.len() == self.state.windows_hidden.len() {
                    iced::exit()
                } else {
                    // check for any other window changes
                    get_tasks(&mut self.state)
                }
            },
            Message::FileSystemWindow(id, message) => {
                fsd_callback(&mut self.state, id, message);
                process_widget_updates(&mut self.state);
                get_tasks(&mut self.state)
            },
            Message::WindowOpened(_, _, _) => {
                Task::none()
            },
            Message::EventTouch(event) => {
                process_touch_events(event, self.state.touch_event_id_enabled.0);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::MouseArea(id, message) => {
                mousearea_callback(id, message);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::PickList(id, message) => {
                pick_list_callback(&mut self.state, id, message);
                process_widget_updates(&mut self.state);
                process_draw_updates(&mut self.state);
                Task::none()
            },
            Message::PopUp(id, message) => {
                popup_callback(id, message);
                process_widget_updates(&mut self.state);
                process_draw_updates(&mut self.state);
                Task::none()
            }
            Message::Radio(id, message) => {
                radio_callback(&mut self.state, id, message);
                process_widget_updates(&mut self.state);
                process_draw_updates(&mut self.state);
                Task::none()
            },
            Message::RichTextLinkClicked(id, link_id) => {
                rich_text_callback(id, link_id);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::Sash(widget_id, message) => {
                sash_callback(&mut self.state, widget_id, message);
                Task::none()
            },
            Message::Scrolled(vp, id) => {
                scrollable_callback(id, vp);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::Slider(id, message) => {
                slider_callback(&mut self.state, id, message);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::Table(id, message) => {
                table_callback(&mut self.state, id, message);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::TextEditor(id, message) => {
                text_ed_callback(id, message, &mut self.state);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::TextInput(id, message) => {
                text_input_callback(&mut self.state, id, message);
                process_widget_updates(&mut self.state);
                process_draw_updates(&mut self.state);
                Task::none()
            },
            Message::Tick(id, instant) => {
                timer_callback(&mut self.state, id, instant);
                process_widget_updates(&mut self.state);
                Task::none()
            },
            Message::Toggler(id, message) => {
                toggle_callback(&mut self.state, id, message);
                process_widget_updates(&mut self.state);
                process_draw_updates(&mut self.state);
                get_tasks(&mut self.state)
            },
        }
        
    }

    pub fn view(&self, window_id: window::Id) -> Element<'_, Message> {

        let (debug, theme) = get_window_values(window_id, &self.state);
 
        let content = 
            create_content(window_id, &self.state);
        
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

        if self.state.keyboard_event_id_enabled.1 {
            subscriptions.push(iced::event::listen().map(Message::EventKeyboard));
        }

        if self.state.mouse_event_id_enabled.1 {
            subscriptions.push(iced::event::listen().map(Message::EventMouse));
        }

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
        } else if let Some(name) = &ipg_window.custom_theme_name {
            crate::widgets::ipg_window::get_custom_theme(name)
                .unwrap_or(Theme::TokyoNight)
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


/// Retrieves window-specific configuration values from the application state.
///
/// This function looks up a window's debug flag and theme based on its Iced window ID.
/// It performs two-level ID mapping: Iced ID → IPG ID → Window container.
///
/// # Arguments
/// * `iced_window_id` - The Iced framework window ID to look up
/// * `state` - Reference to the application state containing window mappings
///
/// # Returns
/// A tuple of `(debug_enabled, theme)` where:
/// - `debug_enabled` - Whether debug mode is active for this window
/// - `theme` - The Iced Theme for rendering (custom, standard, or default TokyoNight)
///
/// # Panics
/// - If the Iced ID cannot be mapped to an IPG window ID
/// - If the IPG window ID is not found in the containers
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
    } else if let Some(name) = &ipg_window.custom_theme_name {
        crate::widgets::ipg_window::get_custom_theme(name)
            .unwrap_or(Theme::TokyoNight)
    } else {
        Theme::TokyoNight
    };
    
    (debug.unwrap_or(false), theme)
}

/// Processes pending window and clipboard actions and converts them into Iced tasks.
///
/// This function drains action queues from the shared window and clipboard state,
/// converting each action into corresponding Iced task commands. It handles:
/// - Window mode changes (fullscreen, windowed, etc.)
/// - Decoration toggling (title bar, borders)
/// - Window resizing and positioning
/// - Window stacking level (always-on-top, etc.)
/// - Clipboard read/write operations
///
/// # Arguments
/// * `ipg_state` - Mutable reference to the application state (for ID mapping)
///
/// # Returns
/// A batched `Task<Message>` containing all pending actions. Returns `Task::none()`
/// wrapped in a batch if no actions are pending.
///
/// # Side Effects
/// - Clears all action queues from window_actions and clipboard_actions mutexes
/// - May trigger native window manager operations
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

    let mut clipboard_actions = access_clipboard_actions();

    for text in clipboard_actions.writes.iter() {
        actions.push(clipboard::write(text.clone()).discard());
    }
    clipboard_actions.writes = vec![];

    for req_id in clipboard_actions.reads.iter() {
        let rid = *req_id;
        actions.push(
            clipboard::read_text().map(move |result| {
                Message::ClipboardReadResult(rid, result.ok().map(|arc| (*arc).clone()))
            }),
        );
    }
    clipboard_actions.reads = vec![];

    drop(clipboard_actions);

    let mut file_dialog = access_file_dialog_actions();

    let tasks = std::mem::take(&mut file_dialog.tasks);

    for task in tasks {
        actions.push(task);
    }

    if actions.is_empty() {
        actions.push(Task::none());
    }
    Task::batch(actions)
}


/// Constructs the complete widget tree for a window from the application state.
///
/// This is the central rendering function that orchestrates the entire UI hierarchy:
/// 1. Maps Iced window ID to IPG window ID
/// 2. Retrieves unique parent container IDs for this window
/// 3. Combines parent IDs with their child widget relationships
/// 4. Recursively constructs the widget tree starting from the root
///
/// # Arguments
/// * `iced_id` - The Iced window ID to render content for
/// * `state` - Reference to the application state containing all widgets and containers
///
/// # Returns
/// An `Element<Message>` representing the complete widget tree for the window.
/// This element is ready for Iced's renderer.
///
/// # Panics
/// - If the Iced ID cannot be found in the window ID mapping
/// - If container IDs for the window are not found
/// - If widget IDs for the window are not found
/// - If no root container is returned from `get_children()`
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

    let content = 
        get_children(
            &all_parent_ids,
            &0, 
            &unique_parent_ids,
            state,
        );
    content.expect("Root container should always produce an element")
}

/// Extracts unique parent container IDs while preserving creation order.
///
/// This function deduplicates container IDs from the input vector, maintaining
/// the original creation order. Order preservation is critical because widgets
/// created with pre-reserved IDs may have lower numeric IDs than later
/// auto-generated container IDs, and sorting would break the layout hierarchy.
///
/// # Arguments
/// * `ids` - Optional reference to a vector of container IDs (may contain duplicates)
///
/// # Returns
/// A vector of unique container IDs in creation order.
///
/// # Panics
/// - If `ids` is `None` (container IDs must be present for this window)
fn get_unique_parents(ids: Option<&Vec<usize>>) -> Vec<usize> {
    // Preserve creation order; sorting by numeric id breaks layout when
    // widgets are created with a pre-reserved gen_id that is lower than
    // later auto-generated container ids.
    let ids = match ids {
        Some(ids) => ids,
        None => panic!("Container ids in unique_container_ids not found")
    };

    let mut unique_ids: Vec<usize> = vec![];
    for id in ids {
        if !unique_ids.contains(id) {
            unique_ids.push(*id);
        }
    }

    unique_ids
}

#[derive(Debug, Clone, PartialEq)]
struct ParentChildIds {
    parent_id: usize,
    child_ids: Vec<usize>,
}

/// Combines parent container IDs with their child widget relationships.
///
/// Iterates through each parent container ID and collects all child widget IDs
/// that are directly parented to it. Creates a flattened structure mapping
/// each parent to its immediate children for use in hierarchical rendering.
///
/// # Arguments
/// * `parent_ids` - Slice of unique parent container IDs
/// * `ids_opt` - Optional reference to vector of WidgetNode entries (parent-child relationships)
///
/// # Returns
/// Vector of `ParentChildIds` structs, one for each parent, containing its child IDs.
///
/// # Panics
/// - If `ids_opt` is `None` (widget node mapping must exist)
fn get_combine_parents_and_children(
    parent_ids: &Vec<usize>, 
    ids_opt: Option<&Vec<WidgetNode>>) 
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

/// Recursively constructs the widget tree for a container and its children.
///
/// This is the core recursive function for building the Iced element tree.
/// It handles special cases for containers with custom construction logic
/// (Menu, RichText, Table) and delegates to their `construct()` methods.
/// For standard containers, it processes children recursively.
///
/// # Arguments
/// * `parents` - Vector of parent-child ID mappings for this window
/// * `index` - Current position in the parents vector (0 for root)
/// * `parent_ids` - List of unique parent IDs for validation
/// * `state` - Reference to the application state with all containers and widgets
///
/// # Returns
/// `Some(Element)` with the constructed widget tree for this container,
/// or `None` if the container ID is not found or construction fails.
///
/// # Special Handling
/// - Menu containers: Builds grouped content from MenuBarItem children
/// - RichText containers: Delegates to RichText's construct method
/// - Table containers: Groups widgets into table sections before construction
fn get_children<'a>(parents: &Vec<ParentChildIds>, 
                index: &usize, 
                parent_ids: &Vec<usize>, 
                state: &'a IpgState,
                ) -> Option<Element<'a, Message>> 
{

    let mut content= vec![];

    let id = &parents[*index].parent_id;

    // Special handling for Menu: build grouped content from MenuBarItem children
    if id != &0 {
        if let Some(Containers::Menu(menu)) = state.containers.get(id) {
            let grouped = get_menu_children(parents, index, parent_ids, state);
            return menu.construct(grouped, &state.widgets, &state.containers);
        }

        if let Some(Containers::RichText(rt)) = state.containers.get(id) {
            return rt.construct(&parents[*index].child_ids, &state.widgets);
        }

        if let Some(Containers::Table(table)) = state.containers.get(id) {
            let sections = get_table_sections(parents, index, parent_ids, state);
            return table.construct(sections, &state.widgets);
        }
    }

    for child in parents[*index].child_ids.iter() {
        if parent_ids.contains(child) {
            let index = parents.iter().position(|r| &r.parent_id == child).unwrap();
            if let Some(el) = get_children(parents, &index, parent_ids, state) {
                content.push(el);
            }
        } else if get_widget(state, child).is_some() {
                content.push(get_widget(state, child).unwrap());
        }
    }

    if id != &0 {
        get_container(state, id, content)
    } else {
        Some(Column::with_children(content).into())  // the final container
    }
}

/// Build grouped content for a Menu container.
/// Each child of the Menu should be an MenuBarItem.
/// For each MenuBarItem, its first child becomes the bar widget and
/// the remaining children become dropdown menu items.
/// Returns (MenuBarItem id, elements) tuples so construct() can
/// look up per-item parameters.
fn get_menu_children<'a>(
    parents: &Vec<ParentChildIds>,
    menu_index: &usize,
    parent_ids: &Vec<usize>,
    state: &'a IpgState,
) -> Vec<(usize, Vec<GroupedItem<'a>>)> {
    let mut grouped: Vec<(usize, Vec<GroupedItem<'a>>)> = vec![];

    for child_id in parents[*menu_index].child_ids.iter() {
        // Each child should be a MenuBarItem container
        if parent_ids.contains(child_id) {
            let bar_item_index = parents.iter().position(|r| &r.parent_id == child_id).unwrap();

            let mut group: Vec<GroupedItem<'a>> = vec![];

            for grandchild in parents[bar_item_index].child_ids.iter() {
                if parent_ids.contains(grandchild) {
                    // Check if this child container is a MenuSubItem
                    if matches!(state.containers.get(grandchild), Some(Containers::MenuSubItem(_))) {
                        let sub_index = parents.iter().position(|r| &r.parent_id == grandchild).unwrap();
                        let sub_items = collect_sub_items(parents, &sub_index, parent_ids, state);
                        if let Some((trigger, children)) = sub_items {
                            group.push(GroupedItem::Sub {
                                trigger,
                                children,
                                sub_item_id: *grandchild,
                            });
                        }
                    } else {
                        let idx = parents.iter().position(|r| &r.parent_id == grandchild).unwrap();
                        if let Some(el) = get_children(parents, &idx, parent_ids, state) {
                            group.push(GroupedItem::Plain(el));
                        }
                    }
                } else if let Some(widget_el) = get_widget(state, grandchild) {
                    group.push(GroupedItem::Plain(widget_el));
                }
            }

            grouped.push((*child_id, group));
        }
    }

    grouped
}

/// Collect the trigger element and child GroupedItems for a MenuSubItem container.
/// Returns `Some((trigger, children))` where `trigger` is the first child and
/// `children` are the remaining items (which may themselves be further MenuSubItems).
fn collect_sub_items<'a>(
    parents: &Vec<ParentChildIds>,
    sub_index: &usize,
    parent_ids: &Vec<usize>,
    state: &'a IpgState,
) -> Option<(Element<'a, Message>, Vec<GroupedItem<'a>>)> {
    let child_ids = &parents[*sub_index].child_ids;
    if child_ids.is_empty() {
        return None;
    }

    let mut iter = child_ids.iter();

    // First child is always the trigger element
    let first_id = iter.next().unwrap();
    let trigger = if parent_ids.contains(first_id) {
        let idx = parents.iter().position(|r| &r.parent_id == first_id).unwrap();
        get_children(parents, &idx, parent_ids, state)?
    } else {
        get_widget(state, first_id)?
    };

    // Remaining children become the sub-menu items (recursively)
    let mut children: Vec<GroupedItem<'a>> = vec![];
    for child_id in iter {
        if parent_ids.contains(child_id) {
            if matches!(state.containers.get(child_id), Some(Containers::MenuSubItem(_))) {
                let sub_idx = parents.iter().position(|r| &r.parent_id == child_id).unwrap();
                if let Some((trigger, sub_children)) = collect_sub_items(parents, &sub_idx, parent_ids, state) {
                    children.push(GroupedItem::Sub {
                        trigger,
                        children: sub_children,
                        sub_item_id: *child_id,
                    });
                }
            } else {
                let idx = parents.iter().position(|r| &r.parent_id == child_id).unwrap();
                if let Some(el) = get_children(parents, &idx, parent_ids, state) {
                    children.push(GroupedItem::Plain(el));
                }
            }
        } else if let Some(widget_el) = get_widget(state, child_id) {
            children.push(GroupedItem::Plain(widget_el));
        }
    }

    Some((trigger, children))
}

/// Build the three sections (header, body, footer) for a Table container.
/// Each direct child of Table should be a TableHeader, TableBody, or TableFooter.
/// The widgets nested inside each section are collected into the matching vec.
fn get_table_sections<'a>(
    parents: &Vec<ParentChildIds>,
    table_index: &usize,
    parent_ids: &Vec<usize>,
    state: &'a IpgState,
) -> TableSections<'a> {
    let mut header = vec![];
    let mut body   = vec![];
    let mut footer = vec![];

    for child_id in parents[*table_index].child_ids.iter() {
        if !parent_ids.contains(child_id) {
            continue; // widgets directly in Table are ignored — use a section
        }
        let section_index = parents.iter().position(|r| &r.parent_id == child_id).unwrap();
        let target = match state.containers.get(child_id) {
            Some(Containers::TableHeader(_)) => &mut header,
            Some(Containers::TableBody(_))   => &mut body,
            Some(Containers::TableFooter(_)) => &mut footer,
            _ => continue,
        };

        for gc in parents[section_index].child_ids.iter() {
            if parent_ids.contains(gc) {
                let idx = parents.iter().position(|r| &r.parent_id == gc).unwrap();
                if let Some(el) = get_children(parents, &idx, parent_ids, state) {
                    target.push(el);
                }
            } else if let Some(el) = get_widget(state, gc) {
                target.push(el);
            }
        }
    }

    TableSections { header, body, footer }
}


fn get_container<'a>(state: &'a IpgState, 
                    id: &usize, 
                    content: Vec<Element<'a, Message>>,
                    ) -> Option<Element<'a, Message>> {

    let container_opt: Option<&Containers> = state.containers.get(id);

    match container_opt 
    {
        Some(container) => 
            match container {
                Containers::CanvasDraw(draw) => {
                    let draw_id = draw.id;
                    if let Some(cs) = state.canvas_states.get(&draw_id) {
                        draw.construct(cs)
                    } else {
                        None
                    }
                },
                Containers::CardClass(crd) => {
                    crd.construct(content, &state.widgets)
                },
                Containers::ColorPicker(cp) => {
                    if content.len() > 1 {
                        eprintln!("[WARNING] A color picker can have only 1 trigger widget, others are ignored")
                    }
                    cp.construct(content)
                },
                Containers::Column(col) => {
                    col.construct(content)
                },
                Containers::Container(cont) => {
                    if content.len() > 1 {
                        eprintln!("[WARNING] A container can have only one widget, place your multiple widgets into a column or row, others ignored")
                    }
                    cont.construct(content, &state.widgets)
                },
                Containers::DatePicker(dp) => {
                    if content.len() > 1 {
                        eprintln!("[WARNING] A date picker can have only 1 trigger widget, others ignored")
                    }
                    dp.construct(content)
                },
                Containers::Float(float) => {
                    if content.len() > 1 {
                        eprintln!("[WARNING] A float can have only one widget, place your multiple widgets into a column or row, others ignored")
                    }
                    float.construct(content)
                },
                Containers::Grid(grid) => {
                    grid.construct(content)
                },
                Containers::Menu(_) => {
                    // Menu children are consumed by get_menu_children;
                    // it should never reach get_container.
                    panic!("Menu should not reach get_container directly")
                },
                Containers::MenuBarItem(_) => {
                    // MenuBarItem children are consumed by get_menu_children;
                    // it should never reach get_container.
                    panic!("MenuBarItem should not reach get_container directly")
                },
                Containers::MenuSubItem(_) => {
                    // MenuSubItem children are consumed by collect_sub_items;
                    // it should never reach get_container.
                    panic!("MenuSubItem should not reach get_container directly")
                },
                Containers::MouseArea(m_area) => {
                    m_area.construct(content)
                },
                Containers::Opaque(op) => {
                    op.construct(content)
                },
                Containers::PopUp(pu) => {
                    if content.len() > 2 {
                        eprintln!("[WARNING] A PopUp can have only 1 or 2 widgets, If 1 widget, the PopUp is hidden until shown by updating the Opened parameter to true, if 2 widgets, then the first one is a widget that allows a callback to update the PopUp like a Button, etc.  The second should be a Container containing all of the other widgets to be displayed. All other added widgets ignored.")
                    }
                    pu.construct(content)
                },
                Containers::Sash(sh) => {
                    sh.construct(content, &state.widgets)
                },
                Containers::RichText(rt) => {
                    rt.construct(&[], &state.widgets)
                },
                Containers::Row(row) => {
                    row.construct(content)
                },
                Containers::Scrollable(scroll) => {
                    scroll.construct(content, &state.widgets)
                },
                Containers::Stack(stk) => {
                    stk.construct(content)
                }
                Containers::TableBasic(tbl_basic) => {
                    tbl_basic.construct(content, &state.widgets)
                },
                Containers::Table(_) => {
                    // Table children are consumed by get_table_sections;
                    // it should never reach get_container.
                    panic!("Table should not reach get_container directly")
                },
                Containers::TableHeader(_) => {
                    // TableHeader is consumed by get_table_sections;
                    // it should never reach get_container.
                    panic!("TableHeader should not reach get_container directly")
                },
                Containers::TableBody(_) => {
                    // TableBody is consumed by get_table_sections;
                    // it should never reach get_container.
                    panic!("TableBody should not reach get_container directly")
                },
                Containers::TableFooter(_) => {
                    // TableFooter is consumed by get_table_sections;
                    // it should never reach get_container.
                    panic!("TableFooter should not reach get_container directly")
                },
                Containers::ToolTip(tool) => {
                    if content.len() > 2 {
                        eprintln!("[WARNING] A tooltip can have only 2 containersor widgets, place your multiple widgets into a column or row, otehr are ignored.")
                    }
                    tool.construct(content, &state.widgets)
                },
                Containers::Window(_wnd) => {
                    Some(construct_window(content))
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
                Widgets::Button(btn) => {
                    btn.construct(&state.widgets)
                },
                Widgets::Card(card) => {
                    card.construct(&state.widgets)
                },
                Widgets::CheckBox(chk) => {
                    chk.construct(&state.widgets)
                },
                Widgets::ComboBox(cb) => {
                    cb.construct(&state.widgets)
                },
                Widgets::Image(image) => {
                    image.construct()
                },
                Widgets::PickList(pick) => {
                    pick.construct(&state.widgets)
                },
                Widgets::ProgressBar(bar) => {
                    bar.construct(&state.widgets)
                },
                Widgets::Radio(rad) => {
                    rad.construct(&state.widgets)
                },
                Widgets::Rule(rule) => {
                    rule.construct(&state.widgets)
                },
                Widgets::Separator(sep) => {
                    sep.construct(&state.widgets)
                },
                Widgets::Slider(slider) => {
                    slider.construct(&state.widgets)
                },
                Widgets::Space(sp) => {
                    sp.construct()
                },
                Widgets::Svg(svg) => {
                    svg.construct()
                },
                Widgets::Text(txt) => {
                    txt.construct(&state.widgets)
                },
                Widgets::TextEditor(txt) => {
                    txt.construct(&state.widgets)
                },
                Widgets::TextInput(input) => {
                    input.construct(&state.widgets)       
                },
                Widgets::Toggler(tog) => {
                    tog.construct(&state.widgets)   
                },
                _ => None,

            },
        None => panic!("App: Widgets not found in fn get_widget id={}", id)
    }
}

fn match_theme_with_debug_color(theme: Theme) -> iced::Color {

    match theme {
        Theme::Light => iced::Color::BLACK,
        Theme::Dark => iced::Color::WHITE,
        Theme::Dracula => iced::Color::WHITE,
        Theme::Ferra => iced::Color::WHITE,
        Theme::Nord => iced::Color::WHITE,
        Theme::SolarizedLight => iced::Color::BLACK,
        Theme::SolarizedDark => iced::Color::WHITE,
        Theme::GruvboxLight => iced::Color::BLACK,
        Theme::GruvboxDark => iced::Color::WHITE,
        Theme::CatppuccinLatte => iced::Color::WHITE,
        Theme::CatppuccinFrappe => iced::Color::WHITE,
        Theme::CatppuccinMacchiato => iced::Color::WHITE,
        Theme::CatppuccinMocha => iced::Color::WHITE,
        Theme::TokyoNight => iced::Color::WHITE,
        Theme::TokyoNightStorm => iced::Color::WHITE,
        Theme::TokyoNightLight => iced::Color::BLACK,
        Theme::KanagawaWave => iced::Color::WHITE,
        Theme::KanagawaDragon => iced::Color::WHITE,
        Theme::KanagawaLotus => iced::Color::BLACK,
        Theme::Moonfly => iced::Color::WHITE,
        Theme::Nightfly => iced::Color::WHITE,
        Theme::Oxocarbon => iced::Color::WHITE,
        Theme::Custom(_) => iced::Color::WHITE,
    }
}

/// Extracts and validates a Window container from a generic Containers reference.
///
/// This is a type-safe helper that retrieves a Window from the Containers enum,
/// panicking if the container is not found or is not a Window variant.
/// Used internally to ensure type safety when accessing window-specific data.
///
/// # Arguments
/// * `container_opt` - Optional reference to a Containers enum variant
///
/// # Returns
/// A reference to the Window container.
///
/// # Panics
/// - If `container_opt` is `None` (window container must exist)
/// - If the container is not a `Containers::Window` variant
fn get_window_container(container_opt: Option<&Containers>) -> &Window {
    
    let container = match container_opt {
        Some(cnt) => cnt,
        None => panic!("App: get_window_container: Cannot find Container"),
    };

    match container {
        Containers::Window(wnd) => {
            wnd
        },
        _ => panic!("get_window: Not a Window")
    }
}


/// Orchestrates the complete widget state synchronization cycle.
///
/// This is the main update dispatcher that batches all pending widget modifications
/// in a single update pass. It processes updates in sequence: deletes → moves →
/// updates → shows, then syncs new widgets and timer state from shared mutexes
/// to the runtime state.
///
/// # Arguments
/// * `state` - Mutable reference to the runtime application state
///
/// # Processing Order
/// 1. Process widget deletions (remove from state)
/// 2. Process widget moves (reparent and reorder)
/// 3. Process parameter updates (change widget properties)
/// 4. Process show/hide state changes
/// 5. Inject newly created widgets into state
/// 6. Synchronize timer state from shared state
///
/// # Side Effects
/// - Modifies the widget state, container hierarchy, and timer state
/// - Clears all update queues from shared mutexes
fn process_widget_updates(
    state: &mut IpgState, 
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

    // Sync timer state from static mutex to runtime state
    let mutex_state = access_state();
    state.timer_state = mutex_state.timer_state.to_owned();
    drop(mutex_state);
}

/// Removes deleted widgets from all state structures.
///
/// Iterates through each window's widget nodes and removes any widgets marked
/// for deletion, then cleans up the widget from the global widgets map.
/// This ensures both the hierarchy and widget storage stay synchronized.
///
/// # Arguments
/// * `state` - Mutable reference to the runtime state
/// * `deletes` - Slice of widget IDs to remove
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

/// Reparents and reorders widgets within the widget hierarchy.
///
/// Moves a widget to a new parent container and/or position, supporting three
/// position modes: after a sibling, before a sibling, or append to parent.
/// Updates both parent references and the ordering in the nodes vector.
///
/// # Arguments
/// * `state` - Mutable reference to the runtime state
/// * `moves` - Slice of tuples: `(widget_id, move_after, move_before, target_parent_id)`
///   - `move_after`: Optional sibling ID to insert after
///   - `move_before`: Optional sibling ID to insert before
///   - `target_parent_id`: Optional explicit parent ID (derived from sibling if not provided)
///
/// # Panics
/// - If widget is not found in any window's nodes
/// - If sibling references don't exist
/// - If neither move position nor target parent is provided
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

/// Applies parameter value changes to widgets and containers.
///
/// Routes each update to either a widget's `match_widget()` or a container's
/// `container_param_update()` based on whether the ID is found in widgets
/// or containers. This dispatches property changes like text, color, size, etc.
///
/// # Arguments
/// * `state` - Mutable reference to the runtime state
/// * `updates` - Slice of tuples: `(id, param_name, new_value)`
///   - `id` - Widget or container ID to update
///   - `param_name` - Python object identifying the parameter (converted to Rust enum)
///   - `new_value` - Python object with the new value
///
/// # Panics
/// - If the ID is not found in either widgets or containers
fn process_updates(
    state: &mut IpgState,
    updates: &[(usize, PyObject, PyObject)],
) {
    for (wid, item, value) in updates.iter() {
        let widget = state.widgets.get_mut(wid);
        if let Some(w) = widget {
            match_widget(w, item, value);
        } else {
            match state.containers.get_mut(wid) {
                Some(cnt) => {
                    container_param_update(cnt, item, value);
                },
                None => panic!("Process_updates: No ids could be found to update with id {wid}.")
            }
        }  
    }
}

/// Routes a parameter update to the appropriate widget parameter handler.
///
/// This is a thin wrapper that dispatches widget parameter updates to
/// `param_update()`, which contains the match logic for converting Python
/// parameter names and values into Rust widget property changes.
///
/// # Arguments
/// * `widget` - Mutable reference to the widget to update
/// * `item` - Python object representing the parameter name/identifier
/// * `value` - Python object with the new parameter value
///
/// # Side Effects
/// - Modifies the widget's properties based on the parameter update
fn match_widget(
    widget: &mut Widgets, 
    item: &PyObject, 
    value: &PyObject) 
{
    param_update(widget, item, value);
}

/// Syncs newly created widgets from shared state into the runtime state.
///
/// Drains the shared mutex's widget queue, associates each widget with its parent
/// container via `set_state_of_widget_running_state()`, and inserts into the
/// runtime widgets map. Also updates the last generated widget ID to prevent
/// future ID collisions.
///
/// # Arguments
/// * `state` - Mutable reference to the runtime state
///
/// # Side Effects
/// - Clears the widgets queue from the shared mutex
/// - Sets up parent-child relationships in the runtime state
/// - Updates `state.last_id` for ID generation
fn process_new_widgets(state: &mut IpgState) {
    let mut mutex_state = access_state();
    if !mutex_state.widgets.is_empty() {
        // Move new widgets into the runtime state
        let new_widgets: HashMap<usize, Widgets> = mutex_state.widgets.drain().collect();
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

/// Updates the visibility state of widgets.
///
/// Processes a batch of widget show/hide operations by setting the `show`
/// property on each widget. Only certain widgets support this property
/// (Button, CheckBox, ComboBox, Image, PickList, etc.).
///
/// # Arguments
/// * `state` - Mutable reference to the runtime state
/// * `shows` - Slice of tuples: `(widget_id, visible)`
///   - `widget_id` - ID of the widget to show/hide
///   - `visible` - True to show, false to hide
///
/// # Panics
/// - If a widget ID is not found in the widgets map
///
/// # Logging
/// - Prints warning for widgets that don't support the show property
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
            // Widgets that support the show parameter
            Widgets::Button(w) => w.show = *val,
            Widgets::Card(w) => w.show = *val,
            Widgets::CheckBox(w) => w.show = *val,
            Widgets::ComboBox(w) => w.show = *val,
            Widgets::Image(w) => w.show = *val,
            Widgets::PickList(w) => w.show = *val,
            Widgets::ProgressBar(w) => w.show = *val,
            Widgets::Radio(w) => w.show = *val,
            Widgets::Rule(w) => w.show = *val,
            Widgets::Separator(w) => w.show = *val,
            Widgets::Slider(w) => w.show = *val,
            Widgets::Space(w) => w.show = *val,
            Widgets::Svg(w) => w.show = *val,
            Widgets::Text(w) => w.show = *val,
            Widgets::TextInput(w) => w.show = *val,
            Widgets::Toggler(w) => w.show = *val,
            
            // Widgets that do NOT support show parameter
              Widgets::AutoScrollStyle(_)
            | Widgets::ButtonStyle(_)
            | Widgets::CardStyle(_)
            | Widgets::CheckboxStyle(_)
            | Widgets::ComboBoxInputStyle(_)
            | Widgets::ComboBoxMenuStyle(_)
            | Widgets::ContainerStyle(_)
            | Widgets::FileSystemDialog(_)
            | Widgets::Font(_)
            | Widgets::Icon(_)
            | Widgets::MenuStyle(_)
            | Widgets::Palette(_)
            | Widgets::PickListStyle(_)
            | Widgets::ProgressBarStyle(_)
            | Widgets::RadioStyle(_)
            | Widgets::RailStyle(_)
            | Widgets::RuleStyle(_)
            | Widgets::SashStyle(_)
            | Widgets::ScrollableStyle(_)
            | Widgets::Scroller(_)
            | Widgets::SeparatorStyle(_)
            | Widgets::SliderStyle(_)
            | Widgets::Span(_)
            | Widgets::TableStyle(_)
            | Widgets::TextEditor(_)
            | Widgets::TextEditorStyle(_)
            | Widgets::TextInputStyle(_)
            | Widgets::TogglerStyle(_) => {
                eprintln!("Widget {:?} does not support the show parameter", widget)
            }
            
        }
    }
}


use once_cell::sync::Lazy;
/// Synchronizes runtime state with data from the shared thread-safe state.
///
/// This is the primary state initialization function that copies all widget
/// hierarchies, containers, window metadata, and event state from the shared
/// mutex into the runtime state. Also initializes CanvasState for each
/// CanvasDraw container and clears the shared state to prevent duplicate
/// processing in subsequent updates.
///
/// # Arguments
/// * `state` - Mutable reference to the runtime state to populate
///
/// # Processing
/// 1. Copy all structural data (IDs, widgets, containers, windows)
/// 2. Copy all configuration (debug flags, themes, modes)
/// 3. Copy event state (keyboard, mouse, canvas, window, touch events)
/// 4. Initialize DrawState for each CanvasDraw container
/// 5. Clear the shared mutex state to reset for next update cycle
///
/// # Side Effects
/// - Clears all data from the shared mutex state
/// - Populates `state.canvas_states` with new DrawState instances
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

    // Initialize CanvasState for each CanvasDraw container (runtime-only, not in mutex_state)
    use crate::ipg_widgets::ipg_canvas_draw::canvas_draw::DrawState;
    for (id, container) in state.containers.iter() {
        if let Containers::CanvasDraw(draw) = container {
            let mut cs = DrawState::default();
            cs.curves = draw.curves.clone();
            cs.text_curves = draw.text_curves.clone();
            cs.init_text_caches();
            state.canvas_states.insert(*id, cs);
        }
    }

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


