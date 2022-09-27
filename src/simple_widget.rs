use druid::{BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget};

trait SimpleWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event);
    fn layout(&mut self, _ctx: &mut LayoutCtx, _bc: &BoxConstraints) -> Size;
    fn paint(&mut self, _ctx: &mut PaintCtx);
}

impl Widget<()> for dyn SimpleWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut (), _env: &Env) {
        self.event(ctx, event);
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &(), _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &(), _data: &(), _env: &Env) {}

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &(), _env: &Env) -> Size {
        self.layout(ctx, bc)
    }
    fn paint(&mut self, ctx: &mut PaintCtx, _data: &(), _env: &Env) {
        self.paint(ctx);
    }
}
