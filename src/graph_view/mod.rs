use std::collections::HashSet;
use std::time::Instant;

use druid::*;
use druid::kurbo::Line;
use druid::piet::{StrokeStyle, Text, TextLayout, TextLayoutBuilder};

use viewport::Viewport;

use crate::graph::node::Node;
use crate::graph_view::display_graph::DisplayGraph;
use crate::graph_view::drag_state::DragState;
use crate::graph_view::element_ref::ElementRef;

mod viewport;
mod drag_state;
mod display_graph;
mod example_graphs;
mod element_ref;

#[derive(Default)]
pub struct GraphView {
    viewport: Viewport,
    drag_state: Option<DragState>,
    display_graph: DisplayGraph,
    selection: HashSet<ElementRef>,
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

    fn paint_edges(&self, ctx: &mut PaintCtx) {
        for e in self.display_graph.edges().into_iter() {
            let line = self.viewport.scene_line_to_screen(Line::new(e.start_point, e.end_point));
            ctx.stroke(line, &Color::BLACK, self.viewport.line_weight());
        }
    }

    fn paint_nodes(&self, ctx: &mut PaintCtx) {
        const DEFAULT_FONT_SIZE: f64 = 24.0;
        for n in self.display_graph.nodes().into_iter() {
            let transformed_rect = &self.viewport.scene_rect_to_screen(n.rect);
            ctx.stroke(transformed_rect, &Color::BLACK, self.viewport.line_weight());
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

impl Widget<()> for GraphView {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut (), _env: &Env) {
        match event {
            Event::WindowConnected => ctx.request_focus(),
            Event::MouseDown(me) => {
                let mut drag_state = DragState {
                    buttons: me.buttons,
                    last_mouse_pos: me.pos,
                    has_moved: false,
                    has_target: false,
                };
                if me.button.is_left() {
                    if me.count == 2 {
                        self.display_graph.add_node(Node::new(self.viewport.screen_coord_to_scene(me.pos), None));
                        ctx.request_paint();
                    } else {
                        let mouse_scene_pos = self.viewport.screen_coord_to_scene(me.pos);
                        if let Some(node) = self.display_graph.get_mut_node_at_point((mouse_scene_pos.x, mouse_scene_pos.y)) {
                            if !me.mods.ctrl() && !me.mods.shift() { self.selection.clear(); }
                            self.selection.insert(ElementRef::Node(node.id));
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
                        self.selection.clear();
                        ctx.request_paint();
                    }
                }
                self.drag_state = None
            }
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
            Event::KeyDown(ke) => {
                let maybe_graph =
                    if HotKey::new(Some(RawMods::Shift), KbKey::Escape).matches(ke) {
                        Some(DisplayGraph::default())
                    } else if HotKey::new(Some(RawMods::AltShift), "A").matches(ke) {
                        Some(example_graphs::arborealis_graph())
                    } else { None };
                if let Some(graph) = maybe_graph {
                    self.display_graph = graph;
                    self.selection.clear();
                    ctx.set_handled();
                    ctx.request_paint();
                }
            }
            _ => ()
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &(), _env: &Env) {
        ctx.register_for_focus();
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &(), _data: &(), _env: &Env) {}

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &(), _env: &Env) -> Size {
        Size {
            width: (if bc.is_width_bounded() { bc.max().width } else { 100.0 }),
            height: (if bc.is_height_bounded() { bc.max().height } else { 100.0 }),
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &(), _env: &Env) {
        let start_time = Instant::now();

        const BG_COLOR: Color = Color::grey8(0xf0);
        const HIGHLIGHT_COLOR: Color = Color::rgb8(0x75, 0xa7, 0xf8);
        let paint_area = ctx.size().to_rect();
        ctx.fill(paint_area, &BG_COLOR);
        self.paint_dot_grid(ctx);
        self.paint_origin_marker(ctx);
        self.paint_edges(ctx);
        self.paint_nodes(ctx);
        for elem_ref in &self.selection {
            match elem_ref {
                ElementRef::Node(node_id) => {
                    let selected_node = self.display_graph.get_node(&node_id).unwrap();
                    ctx.stroke(self.viewport.scene_rect_to_screen(selected_node.rect),
                               &HIGHLIGHT_COLOR, 3.0 * self.viewport.scale);
                }
                ElementRef::Edge(_edge_id) => {}
            }
        }

        let paint_time = Instant::now() - start_time;
        println!("Time to paint: {:.3}ms, equivalent FPS: {}", paint_time.as_secs_f64() * 1000.0, (1.0 / paint_time.as_secs_f64()).round());
    }
}