use log::info;
use wasm_bindgen::{JsCast, JsValue};

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

pub enum Msg {
    Resize,
}

#[derive(Properties, PartialEq)]
pub struct SortGraphProps {
    pub values: Vec<i32>,
}

pub struct SortGraph {
    canvas_ref: NodeRef,
    canvas: Option<HtmlCanvasElement>,
    ctx: Option<CanvasRenderingContext2d>,
}

impl Component for SortGraph {
    type Message = Msg;
    type Properties = SortGraphProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            canvas: None,
            ctx: None,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(canvas) = self.canvas_ref.cast::<HtmlCanvasElement>() {
                self.canvas = Some(canvas);
                let canvas = self.canvas.as_ref().unwrap();
                self.ctx = Some(
                    canvas
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into()
                        .unwrap(),
                );
                self.scale_canvas();
                self.set_stroke_style("#adff2f");
                self.draw_values(_ctx.props().values.to_owned());
            }
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Resize => {
                info!("Hi");
                let canvas = self.canvas.as_ref().unwrap();
                canvas.set_width(canvas.client_width() as u32);
                canvas.set_height(canvas.client_height() as u32);
                self.ctx
                    .as_ref()
                    .unwrap()
                    .set_stroke_style(&JsValue::from_str("#adff2f"));
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        let values = _ctx.props().values.clone();
        self.draw_values(values);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let onresize = _ctx.link().callback(|_| {
            info!("resize");
            Msg::Resize
        });

        html! {
            <div class="sort-visualization">
                <canvas class="sort-visualization" {onresize} ref={self.canvas_ref.clone()}></canvas>
            </div>
        }
    }
}
impl SortGraph {
    fn draw_values(&self, values: Vec<i32>) {
        let canvas = self.canvas.as_ref().unwrap();
        let ctx = self.ctx.as_ref().unwrap();

        let canvas_width = canvas.width() as f64;
        let canvas_height = canvas.height() as f64;
        let max_height = match values.iter().max() {
            Some(val) => *val,
            None => 0,
        };
        let width = canvas_width / values.len() as f64;
        let margin = width / 10.0;
        // Remove margin when it's small enough to avoid problem where some bars have a tiny margin and some don't.
        let margin = if margin < 0.5 { 0.0 } else { margin };

        ctx.clear_rect(0.0, 0.0, canvas_width, canvas_height);

        ctx.begin_path();
        ctx.set_line_width(width - margin);
        for (i, val) in values.iter().enumerate() {
            let val = *val as f64;
            let x = (width * i as f64) + width * 0.5;
            let height = (val / max_height as f64) * canvas_height;
            ctx.move_to(x, canvas_height);
            ctx.line_to(x, canvas_height - height);
        }
        ctx.stroke();
    }
    fn scale_canvas(&self) {
        let canvas = self.canvas.as_ref().unwrap();
        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);
    }
    fn set_stroke_style(&self, stroke_style: &str) {
        self.ctx
            .as_ref()
            .unwrap()
            .set_stroke_style(&JsValue::from_str(stroke_style));
    }
}
