#![windows_subsystem = "windows"]

use std::ops::Deref;
use std::sync::Arc;

use druid::widget::prelude::*;
use druid::widget::{Flex, Label, List, ListIter, TextBox};
use druid::{AppLauncher, Data, Lens, WidgetExt, WindowDesc};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;

#[derive(Clone, Data, Lens)]
struct AppData {
    filter: String,
    list: Arc<Vec<usize>>,
}

impl ListIter<(String, usize)> for AppData {
    fn for_each(&self, mut cb: impl FnMut(&(String, usize), usize)) {
        for (i, item) in self.list.iter().enumerate() {
            let d = (self.filter.clone(), item.to_owned());
            cb(&d, i);
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut (String, usize), usize)) {
        let mut new_data: Option<Vec<usize>> = None;

        for (i, item) in self.list.iter().enumerate() {
            let mut d = (self.filter.clone(), item.to_owned());
            cb(&mut d, i);

            self.filter = d.0;

            if !item.same(&d.1) {
                match &mut new_data {
                    Some(vec) => {
                        vec[i] = d.1;
                    }
                    None => {
                        let mut new = self.list.deref().clone();
                        new[i] = d.1;
                        new_data = Some(new);
                    }
                }
            }
        }
        if let Some(vec) = new_data {
            self.list = Arc::new(vec);
        }
    }

    fn data_len(&self) -> usize {
        self.list.len()
    }
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title("List with filter")
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state: AppData = AppData {
        filter: "odd".into(),
        list: Arc::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
    };

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppData> {
    let textbox = TextBox::new().lens(AppData::filter);

    let list = List::new(build_single_element);

    Flex::column()
        .with_child(textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(list)
}

fn build_single_element() -> impl Widget<(String, usize)> {
    Label::new(|(filter, i): &(String, usize), _env: &Env| {
        if filter.is_empty()
            || filter.to_lowercase().eq("all")
            || (filter.to_lowercase().eq("odd") && i % 2 == 1)
            || (filter.to_lowercase().eq("even") && i % 2 == 0)
            || filter.contains(&i.to_string())
        {
            i.to_string()
        } else {
            String::from("")
        }
    })
}
