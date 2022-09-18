use druid::Point;
use druid_shell::MouseButtons;

pub struct DragState {
    pub(crate) buttons: MouseButtons,
    pub(crate) last_mouse_pos: Point,
}