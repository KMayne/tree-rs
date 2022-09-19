use std::time::Instant;

use druid::*;
use druid::kurbo::Line;
use druid::piet::{StrokeStyle, Text, TextLayout, TextLayoutBuilder};
use uuid::Uuid;

use viewport::Viewport;

use crate::graph::Graph;
use crate::graph::node::Node;
use crate::graph_view::display_graph::DisplayGraph;
use crate::graph_view::drag_state::DragState;

mod viewport;
mod drag_state;
mod display_graph;

#[derive(Default)]
pub struct GraphView {
    viewport: Viewport,
    drag_state: Option<DragState>,
    display_graph: DisplayGraph,
    selection: Option<Rect>,
}

impl GraphView {
    pub(crate) fn new() -> Self {
        GraphView::default()
    }

    fn paint_dot_grid(&self, ctx: &mut PaintCtx) {
        const DOT_COLOUR: Color = Color::rgb8(0x61, 0x61, 0x61);
        const GRID_SPACING: f64 = 36.0;
        const DOT_SIZE: f64 = 2.0;
        // Used to ensure we're not looking at half dots when at (0, 0)
        let base_offset = GRID_SPACING / 2.0;
        // Includes dot size to ensure x & y are aligned
        let x_offset = base_offset - self.viewport.origin.x - DOT_SIZE / 2.0;
        let y_offset = (base_offset - self.viewport.origin.y) % GRID_SPACING;

        let scale = self.viewport.scale;
        let dotted_style = StrokeStyle::new().dash(vec![DOT_SIZE * scale, GRID_SPACING * scale], -x_offset / 2.0);
        let grid_area = ctx.size().to_rect();

        for y in (0..(grid_area.height() / (GRID_SPACING * scale)).ceil() as i64).map(|y_step| ((y_step as f64) * GRID_SPACING + y_offset) * scale) {
            ctx.stroke_styled(Line::new((grid_area.x0, y), (grid_area.x1, y)), &DOT_COLOUR, scale * DOT_SIZE, &dotted_style)
        }
    }

    fn paint_origin_marker(&self, ctx: &mut PaintCtx) {
        for line in vec![
            Line::new(
                self.viewport.scene_coord_to_screen(Point::new(-10.0, -10.0)),
                self.viewport.scene_coord_to_screen(Point::new(10.0, 10.0)),
            ),
            Line::new(self.viewport.scene_coord_to_screen(Point::new(-10.0, 10.0)),
                      self.viewport.scene_coord_to_screen(Point::new(10.0, -10.0))),
        ] { ctx.stroke(line, &Color::BLUE, 2.0 * self.viewport.scale); }
    }

    fn paint_edges(&self, _ctx: &mut PaintCtx) {}

    fn paint_nodes(&self, ctx: &mut PaintCtx) {
        const DEFAULT_FONT_SIZE: f64 = 16.0;
        for n in self.display_graph.nodes().into_iter() {
            let transformed_rect = &self.viewport.scene_rect_to_screen(n.rect);
            ctx.stroke(transformed_rect, &Color::BLACK, 2.0 * self.viewport.scale);
            ctx.fill(transformed_rect, &Color::WHITE);
            let text_layout = ctx.text().new_text_layout(n.text.clone())
                .font(FontFamily::default(), DEFAULT_FONT_SIZE * self.viewport.scale)
                .max_width(transformed_rect.width() - 8.0 * self.viewport.scale)
                .alignment(TextAlignment::Center)
                .build().unwrap();
            let vertical_align_offset = transformed_rect.height() / 2.0 - text_layout.size().height / 2.0;
            ctx.draw_text(&text_layout, Point::new(transformed_rect.x0, transformed_rect.y0 + vertical_align_offset))
        }
    }
}

impl Widget<String> for GraphView {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut String, _env: &Env) {
        match event {
            Event::WindowConnected => ctx.request_focus(),
            Event::MouseDown(me) => {
                let mut drag_state = DragState {
                    buttons: me.buttons,
                    last_mouse_pos: me.pos,
                    has_moved: false,
                    has_target: false
                };
                if me.button.is_left() {
                    if me.count == 2 {
                        self.display_graph.add_node(Node::new(self.viewport.screen_coord_to_scene(me.pos), None));
                        ctx.request_paint();
                    } else {
                        let mouse_scene_pos = self.viewport.screen_coord_to_scene(me.pos);
                        if let Some(node) = self.display_graph.get_mut_node_at_point((mouse_scene_pos.x, mouse_scene_pos.y)) {
                            self.selection = Some(node.rect.clone());
                            node.selected = true;
                            drag_state.has_target = true;
                        }
                        ctx.request_paint();
                    }
                }
                self.drag_state = Some(drag_state);
            }
            Event::MouseUp(_me) => {
                if let Some(drag) = &self.drag_state {
                    if !drag.has_target && !drag.has_moved {
                        self.selection = None;
                        ctx.request_paint();
                    }
                }
                self.drag_state = None
            },
            Event::MouseMove(me) => {
                match &mut self.drag_state {
                    Some(drag_state) => {
                        if drag_state.buttons.has_left() {
                            self.viewport.apply_mouse_move(drag_state.last_mouse_pos - me.pos);
                            ctx.request_paint();
                        }
                        drag_state.has_moved = drag_state.last_mouse_pos != me.pos;
                        drag_state.last_mouse_pos = me.pos;
                        drag_state.buttons = me.buttons;
                    }
                    None => {}
                }
            }
            Event::Wheel(me) => {
                self.viewport.apply_scale(me.pos, -me.wheel_delta.y / 1600.0);
                ctx.request_paint();
            }
            Event::Zoom(scale_amount) => {
                self.viewport.apply_scale((ctx.size() / 2.0).to_vec2().to_point(), scale_amount.clone());
            }
            Event::KeyDown(ke) =>
                if HotKey::new(None, KbKey::Escape).matches(ke) {
                    self.display_graph = DisplayGraph::from(Graph {
                        nodes: vec![],
                        edges: vec![],
                    });
                    ctx.set_handled();
                    ctx.request_paint();
                } else if HotKey::new(None, "a").matches(ke) {
                    println!("Replacing graph with arborealis graph");
                    self.display_graph = DisplayGraph::from(Graph {
                        nodes: vec![
                            Node {
                                id: Uuid::new_v4(),
                                text: String::from("ARBOREALIS"),
                                rect: Rect::from_origin_size(Point::new(866.0, 184.0), Size::new(197.0, 75.0)),
                            },
                            Node {
                                id: Uuid::new_v4(),
                                text: String::from("sapling (based on druid-shell)"),
                                rect: Rect::from_origin_size(Point::new(1592.5, 338.5), Size::new(179.0, 100.0)),
                            },
                            Node {
                                id: Uuid::new_v4(),
                                text: String::from("selection"),
                                rect: Rect::from_origin_size(Point::new(1100.0, 300.5), Size::new(100.0, 40.0)),
                            },
                        ],
                        edges: vec![],
                    });
                    self.selection = None;

                    ctx.set_handled();
                    ctx.request_paint();
                }
            _ => ()
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &String, _env: &Env) {
        ctx.register_for_focus();
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &String, _data: &String, _env: &Env) {}

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &String, _env: &Env) -> Size {
        Size {
            width: (if bc.is_width_bounded() { bc.max().width } else { 100.0 }),
            height: (if bc.is_height_bounded() { bc.max().height } else { 100.0 }),
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &String, _env: &Env) {
        let start_time = Instant::now();

        const BG_COLOR: Color = Color::grey8(0xf0);
        const HIGHLIGHT_COLOR: Color = Color::rgb8(0x75, 0xa7, 0xf8);
        let paint_area = ctx.size().to_rect();
        ctx.fill(paint_area, &BG_COLOR);
        self.paint_dot_grid(ctx);
        self.paint_origin_marker(ctx);
        self.paint_nodes(ctx);
        if let Some(selection) = self.selection {
            ctx.stroke(self.viewport.scene_rect_to_screen(selection),
                       &HIGHLIGHT_COLOR, 3.0 * self.viewport.scale);
        }

        let paint_time = Instant::now() - start_time;
        println!("Time to paint: {}, equivalent FPS: {}", paint_time.as_micros(), (1.0 / paint_time.as_secs_f64()).round());
    }
}