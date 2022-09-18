use druid::Point;
use druid::MouseButtons;

pub struct DragState {
    pub(crate) buttons: MouseButtons,
    pub(crate) last_mouse_pos: Point,
    pub(crate) has_moved: bool,
    pub(crate) has_target: bool
}