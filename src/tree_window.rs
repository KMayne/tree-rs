use std::any::Any;
use std::cell::RefCell;

use druid_shell::{Application, Cursor, KeyEvent, MouseButtons, MouseEvent, Region, TimerToken, WindowHandle, WinHandler};
use druid_shell::keyboard_types::Key;
use druid_shell::kurbo::{Line, Point, Rect, Size, Vec2};
use druid_shell::piet::{Color, FontFamily, Piet, PietText, RenderContext, StrokeStyle, Text, TextAlignment, TextLayout, TextLayoutBuilder};
use uuid::Uuid;

use crate::graph::Graph;
use crate::graph::node::Node;

struct DragState {
    buttons: MouseButtons,
    last_mouse_pos: Point,
}

pub(crate) struct TreeWindow {
    scene: RefCell<Graph>,
    view_origin: Point,
    scale: f64,
    size: Size,
    handle: WindowHandle,
    drag_state: Option<DragState>,
    font: FontFamily,
    text: PietText,
}

impl TreeWindow {
    pub(crate) fn new() -> Self {
        let win_handle = WindowHandle::default();
        let mut win_text = win_handle.text();
        let font = win_text.font_family("Segoe UI").or_else(|| win_text.font_family("Arial")).unwrap().clone();
        TreeWindow {
            scene: Default::default(),
            view_origin: Default::default(),
            scale: 1.0,
            size: Default::default(),
            handle: win_handle,
            drag_state: Default::default(),
            text: win_text,
            font,
        }
    }
}


fn scene_coord_to_screen(point: Point, view_origin: Point, scale: f64) -> Point {
    ((point - view_origin) * scale).to_point()
}

fn screen_coord_to_scene(point: Point, view_origin: Point, scale: f64) -> Point {
    ((point.to_vec2() / scale) + view_origin.to_vec2()).to_point()
}

fn paint_dot_grid(piet: &mut Piet, area: Rect, grid_spacing: f64, dot_size: f64, grid_offset: Vec2, scale: f64) {
    const DOT_COLOUR: Color = Color::rgb8(0x61, 0x61, 0x61);
    let base_offset = grid_spacing / 2.0;
    // Includes dot size to ensure x & y are aligned
    let x_offset = base_offset - grid_offset.x - dot_size / 2.0;
    let y_offset = (base_offset - grid_offset.y) % grid_spacing;
    let dotted_style = StrokeStyle::new().dash(vec![dot_size * scale, grid_spacing * scale], -x_offset / 2.0);
    // Used to ensure we're not looking at half dots when at (0, 0)
    for y in (0..(area.height() / (grid_spacing * scale)).ceil() as i64).map(|y_step| ((y_step as f64) * grid_spacing + y_offset) * scale) {
        piet.stroke_styled(Line::new((area.x0, y), (area.x1, y)), &DOT_COLOUR, scale * dot_size, &dotted_style)
    }
}

impl WinHandler for TreeWindow {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
    }

    fn size(&mut self, size: Size) {
        self.size = size;
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, piet: &mut Piet, _: &Region) {
        const BG_COLOR: Color = Color::grey8(0xf0);
        let paint_area = self.size.to_rect();
        // Clear screen
        piet.fill(paint_area, &BG_COLOR);
        paint_dot_grid(piet, paint_area, 36.0, 2.0, self.view_origin.to_vec2(), self.scale);
        piet.stroke(Line::new(scene_coord_to_screen(Point::new(-10.0, -10.0), self.view_origin, self.scale),
                              scene_coord_to_screen(Point::new(10.0, 10.0), self.view_origin, self.scale)), &Color::BLUE, 2.0 * self.scale);
        piet.stroke(Line::new(scene_coord_to_screen(Point::new(-10.0, 10.0), self.view_origin, self.scale),
                              scene_coord_to_screen(Point::new(10.0, -10.0), self.view_origin, self.scale)), &Color::BLUE, 2.0 * self.scale);
        for n in &self.scene.borrow().nodes {
            let transformed_rect = &n.rect.with_origin(scene_coord_to_screen(Point::new(n.rect.x0, n.rect.y0), self.view_origin, self.scale)).with_size(n.rect.size() * self.scale);
            piet.stroke(transformed_rect, &Color::BLACK, 2.0 * self.scale);
            piet.fill(transformed_rect, &Color::WHITE);
            let text_layout = self.text.new_text_layout(n.text.clone())
                .font(self.font.clone(), 16.0 * self.scale)
                .max_width(transformed_rect.width() - 8.0 * self.scale)
                .alignment(TextAlignment::Center)
                .build().unwrap();
            piet.draw_text(&text_layout, (transformed_rect.x0, transformed_rect.y0 + transformed_rect.height() / 2.0 - text_layout.size().height / 2.0))
        }
    }

    fn command(&mut self, id: u32) {
        match id {
            0x100 => {
                self.handle.close();
                Application::global().quit()
            }
            _ => println!("unexpected id {}", id),
        }
    }

    fn key_down(&mut self, event: KeyEvent) -> bool {
        match event.key {
            Key::Escape => {
                self.scene.replace(Graph {
                    nodes: vec![],
                    edges: vec![],
                });
                self.handle.invalidate();
                true
            }
            Key::Character(c) => {
                if c.eq("a") {
                    self.scene.replace(Graph {
                        nodes: vec![
                            Box::new(Node {
                                id: Uuid::new_v4(),
                                text: String::from("ARBOREALIS"),
                                rect: Rect::from_origin_size(Point::new(866.0, 184.0), Size::new(197.0, 75.0)),
                            }),
                            Box::new(Node {
                                id: Uuid::new_v4(),
                                text: String::from("sapling (based on druid-shell)"),
                                rect: Rect::from_origin_size(Point::new(1592.5, 338.5), Size::new(179.0, 100.0)),
                            })
                        ],
                        edges: vec![],
                    });
                }
                self.handle.invalidate();
                true
            }
            _ => {
                println!("keydown: {:?}", event);
                false
            }
        }
    }

    fn key_up(&mut self, event: KeyEvent) {
        println!("keyup: {:?}", event);
    }

    fn wheel(&mut self, event: &MouseEvent) {
        let original_scale = self.scale;
        self.scale = (self.scale - (event.wheel_delta.y / 1600.0 * self.scale)).clamp(0.20, 10.0);
        // We want to zoom based on the mouse position
        self.view_origin += event.pos.to_vec2() * (1.0 / original_scale - 1.0 / self.scale);
        self.handle.invalidate();
    }

    fn mouse_move(&mut self, event: &MouseEvent) {
        self.handle.set_cursor(&Cursor::Arrow);
        match &mut self.drag_state {
            Some(drag_state) => {
                if drag_state.buttons.has_left() {
                    let movement = (drag_state.last_mouse_pos - event.pos) / self.scale;
                    self.view_origin.x += movement.x;
                    self.view_origin.y += movement.y;
                    self.handle.invalidate();
                }
                drag_state.last_mouse_pos = event.pos;
            }
            None => {}
        }
    }
    fn mouse_down(&mut self, event: &MouseEvent) {
        self.drag_state = Some(DragState {
            buttons: event.buttons,
            last_mouse_pos: event.pos,
        });
        if event.count >= 2 && event.button.is_left() {
            self.scene.get_mut().nodes.push(Box::new(Node::new(screen_coord_to_scene(event.pos, self.view_origin, self.scale), None)));
            self.handle.invalidate();
        }
    }
    fn mouse_up(&mut self, _event: &MouseEvent) {
        self.drag_state = None;
    }

    fn timer(&mut self, id: TimerToken) {
        println!("timer fired: {:?}", id);
    }

    fn request_close(&mut self) {
        self.handle.close();
    }

    fn destroy(&mut self) {
        Application::global().quit()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}