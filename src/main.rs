#![windows_subsystem = "windows"]

use std::sync::Arc;

use druid::widget::prelude::*;
use druid::widget::{Flex, Label, List, TextBox};
use druid::{AppLauncher, Data, Lens, WidgetExt, WindowDesc};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;

#[derive(Clone, Data, Lens)]
struct AppData {
    internal: (String, Arc<Vec<usize>>),
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title("List with filter")
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state: AppData = AppData {
        internal: ("odd".into(), Arc::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]))
    };

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppData> {
//    let textbox = TextBox::new()
//        .lens((String, Arc<Vec<usize>>)::0)
//        .lens(AppData::internal);

    let list = List::new(build_single_element)
        .lens(AppData::internal);

    Flex::column()
//        .with_child(textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(list)
}

fn build_single_element() -> impl Widget<(String, usize)> {
    Label::new(|(_, i): &(String, usize), _env: &Env| i.to_string())
}
