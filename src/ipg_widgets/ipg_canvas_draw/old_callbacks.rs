

// Message::WidgetDraw(mut widget) => {
//     // Since the text widget may have a blinking cursor, the only way to use a timer
//     // is to use the main subscription one at this time, canvas lacks a time event.
//     // Therefore, the pending has to return the curve also at each change so that
//     // the curves can be updated.  The subscription clears the text cache at each tick.
//     match widget {
//         CanvasWidget::Text(_) => {
//             let (draw_mode, draw_status) = get_draw_mode_and_status(&widget);
//             let id = get_widget_id(&widget);
//             match draw_status {
//                 DrawStatus::Completed => {
//                     widget = set_widget_mode_or_status(widget, Some(DrawMode::DrawAll), None);
//                     self.canvas_state.text_curves.entry(id).and_modify(|k| *k= widget.clone());
//                     self.canvas_state.timer_event_enabled = false;
//                     self.canvas_state.draw_mode = DrawMode::DrawAll;
//                 },
//                 DrawStatus::Delete => {
//                     self.canvas_state.text_curves.remove(&id);
//                     self.canvas_state.timer_event_enabled = false;
//                 },
//                 DrawStatus::Inprogress => {
//                     // Since the text always returns a new curve or updated curve,
//                     // a check for the first return is need to see if a text is present. 
//                     let present = self.canvas_state.text_curves.get(&id);
//                     if present.is_none() {
//                         self.canvas_state.text_curves.insert(id, widget.clone());
//                     } else {
//                         self.canvas_state.text_curves.entry(id).and_modify(|k| *k= widget.clone());
//                     }
//                 },
//             }
//             match draw_mode {
//                 DrawMode::Edit | DrawMode::Rotate => {
//                     let id = get_widget_id(&widget);
//                     self.canvas_state.edit_widget_id = Some(id.clone());
//                     self.canvas_state.text_curves.entry(id).and_modify(|k| *k= widget);
//                 },
//                 _ => (),
//             }
//             self.canvas_state.request_text_redraw();
//         },
//         _ => {
//             let (draw_mode, draw_status) = get_draw_mode_and_status(&widget);
//             match draw_status {
//                 DrawStatus::Completed => {
//                     widget = set_widget_mode_or_status(widget, Some(DrawMode::DrawAll), None);
//                 },
//                 DrawStatus::Delete => {
//                     let id = get_widget_id(&widget);
//                     self.canvas_state.curves.remove(&id);
//                 },  
//                 _ => (),
//             }
//             if draw_mode == DrawMode::New {
//                 let id = get_widget_id(&widget);
//                 let widget = set_widget_mode_or_status(widget.clone(), Some(DrawMode::DrawAll), Some(DrawStatus::Completed));
//                 self.canvas_state.curves.insert(id, widget);
//             } else {
//                 // if not new must be in edit or rotate mode so modify.
//                 let id = get_widget_id(&widget);
//                 self.canvas_state.edit_widget_id = Some(id.clone());
//                 self.canvas_state.curves.entry(id).and_modify(|k| *k= widget);
//             }
            
//             self.canvas_state.request_redraw();
//         },
//     }

    
// }
// Message::Clear => {
//     self.canvas_state.curves.clear();
//     self.canvas_state = draw_canvas::CanvasState::default();
// }
// Message::ModeSelected(mode) => {
//     let mode = DrawMode::to_enum(mode.clone());
//     match mode {
//         DrawMode::DrawAll => {
//             self.canvas_state.draw_mode = DrawMode::DrawAll;
//         },
//         DrawMode::Edit => {
//             if self.canvas_state.curves.is_empty() && 
//                 self.canvas_state.text_curves.is_empty() {
//                 return
//             }
//             self.canvas_state.draw_mode = DrawMode::Edit;
//         },
//         DrawMode::New => {
//             self.canvas_state.draw_mode = DrawMode::New;
//             // When both the draw_mode is new and widget is text
//             // then we cut on the timer
//             if self.canvas_state.selected_radio_widget == Some(Widget::Text) {
//                 self.canvas_state.timer_event_enabled = true;
//             }
//         },
//         DrawMode::Rotate => {
//             self.canvas_state.draw_mode = DrawMode::Rotate;
//         },
//     }
//     self.canvas_state.request_redraw();
// },
