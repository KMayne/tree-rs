use std::env;

use druid::{AppLauncher, WindowDesc};
use druid_shell::{Application, WindowBuilder};

use crate::graph_view::GraphView;
use crate::tree_window::TreeWindow;

mod graph;
mod tree_window;
mod graph_view;

fn main() {
    if env::args().skip(1).next().map(|s| s.eq("--druid-shell")).unwrap_or(false) {
        let app = Application::new().unwrap();
        let state = TreeWindow::new();
        let mut builder = WindowBuilder::new(app.clone());
        builder.set_handler(Box::new(state));
        builder.set_title("Tree");
        // Show the window
        builder.build().unwrap().show();
        app.run(None);
    } else {
        let main_window = WindowDesc::new(|| GraphView::new()).title("Tree");
        // start the application. Here we pass in the application state.
        AppLauncher::with_window(main_window)
            .launch(String::new())
            .expect("Failed to launch application");
    }
}
