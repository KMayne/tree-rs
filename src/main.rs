use druid::{AppLauncher, WindowDesc};

use crate::graph_view::GraphView;

mod graph;
mod graph_view;

fn main() {
    let main_window = WindowDesc::new(|| GraphView::new()).title("Tree");
    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .launch(String::new())
        .expect("Failed to launch application");
}
