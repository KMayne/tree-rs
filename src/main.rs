use druid_shell::{Application, WindowBuilder};

use tree_window::TreeWindow;

mod graph;
mod tree_window;
mod display_graph;

fn main() {
    let app = Application::new().unwrap();
    let mut builder = WindowBuilder::new(app.clone());
    let state = TreeWindow::new();
    builder.set_handler(Box::new(state));
    builder.set_title("Tree");
    // Show the window
    builder.build().unwrap().show();
    app.run(None);
}
