// std::time isn't supported on WASM platforms
use instant::{Duration, Instant};

use gloo_events::EventListener;
use log::info;
use sorting_algorithms::SortCommand;
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast, JsValue,
};

use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn setTimeout(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    fn clearTimeout(timeout_id: i32);
}

pub enum Msg {
    Resize,
}

#[derive(Properties, PartialEq)]
pub struct SortGraphProps {
    pub items: Vec<u32>,
    #[prop_or(vec![])]
    pub step: Vec<SortCommand<u32>>,
    #[prop_or(None)]
    pub prev_step: Option<Vec<SortCommand<u32>>>,
}

pub struct SortGraphConfig {
    color_changed: String,
    color_unchanged: String,
    update_rate: Duration,
}

pub struct SortGraph {
    canvas_ref: NodeRef,
    canvas: Option<HtmlCanvasElement>,
    ctx: Option<CanvasRenderingContext2d>,
    resize_listener: Option<EventListener>,
    /// Previous time when the graph was drawn. Used for limiting the drawing rate.
    prev_draw: Instant,
    config: SortGraphConfig,
    timeout_id: Option<i32>,
    _timeout_closure: Option<Closure<dyn FnMut()>>,
}

impl Component for SortGraph {
    type Message = Msg;
    type Properties = SortGraphProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            canvas: None,
            ctx: None,
            resize_listener: None,
            prev_draw: Instant::now(),
            config: SortGraphConfig {
                color_changed: "#00aaff".to_string(),
                color_unchanged: "#adff2f".to_string(),
                update_rate: Duration::from_millis(50),
            },
            timeout_id: None,
            _timeout_closure: None,
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
                draw(
                    self.canvas.as_ref().unwrap(),
                    self.ctx.as_ref().unwrap(),
                    self.config.color_unchanged.to_string(),
                    self.config.color_changed.to_string(),
                    &_ctx.props().items,
                    &_ctx.props().step,
                );

                let on_resize = _ctx.link().callback(|_e: Event| Msg::Resize);
                let window = window().expect("couldn't get window");
                let resize_listener =
                    EventListener::new(&window, "resize", move |e| on_resize.emit(e.clone()));
                self.resize_listener = Some(resize_listener);
            }
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Resize => {
                self.scale_canvas();
                draw(
                    self.canvas.as_ref().unwrap(),
                    self.ctx.as_ref().unwrap(),
                    self.config.color_unchanged.to_string(),
                    self.config.color_changed.to_string(),
                    &_ctx.props().items,
                    &_ctx.props().step,
                );
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        // Limit rate of redraws
        // TODO: This doesn't call itself after the time is elapsed.
        if self.prev_draw.elapsed() > self.config.update_rate {
            draw(
                self.canvas.as_ref().unwrap(),
                self.ctx.as_ref().unwrap(),
                self.config.color_unchanged.to_string(),
                self.config.color_changed.to_string(),
                &_ctx.props().items,
                &_ctx.props().step,
            );
            self.prev_draw = Instant::now();
        } else {
            match self.timeout_id {
                Some(id) => clearTimeout(id),
                None => (),
            };
            let cb = {
                let canvas = self.canvas.clone().unwrap();
                let ctx = self.ctx.clone().unwrap();
                let color_changed = self.config.color_changed.clone();
                let color_unchanged = self.config.color_unchanged.clone();
                let items = _ctx.props().items.clone();
                let step = _ctx.props().step.clone();
                Closure::wrap(Box::new(move || {
                    draw(
                        &canvas,
                        &ctx,
                        color_unchanged.clone(),
                        color_changed.clone(),
                        &items,
                        &step,
                    );
                }) as Box<dyn FnMut()>)
            };

            let interval_id = setTimeout(&cb, self.config.update_rate.as_millis() as u32);

            self.timeout_id = Some(interval_id);
            self._timeout_closure = Some(cb);
        }
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
    fn scale_canvas(&self) {
        let canvas = self.canvas.as_ref().unwrap();
        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);
    }
}

fn draw(
    canvas: &HtmlCanvasElement,
    ctx: &CanvasRenderingContext2d,
    color_unchanged: String,
    color_changed: String,
    items: &[u32],
    step: &[SortCommand<u32>],
) {
    let canvas_width = canvas.width() as f64;
    let canvas_height = canvas.height() as f64;
    let max_val = match items.iter().max() {
        Some(val) => *val,
        None => 0,
    };
    let width = canvas_width / items.len() as f64;
    let margin = width * 0.1;
    // Remove margin when it's small enough to avoid problem where some bars have a tiny margin and some don't.
    let margin = if margin < 0.5 { 0.0 } else { margin };

    let unchanged_indices = (0..items.len())
        .filter(|i| {
            !step.iter().any(|command| match command {
                SortCommand::Swap(from, to) => from == i || to == i,
                SortCommand::Set(index, _) => index == i,
            })
        })
        .to_owned()
        .collect::<Vec<usize>>();

    ctx.clear_rect(0.0, 0.0, canvas_width, canvas_height);
    ctx.set_line_width(width - margin);

    set_stroke_style(&ctx, color_unchanged);
    draw_bars(
        &ctx,
        &unchanged_indices,
        &items,
        max_val,
        width,
        canvas_height,
    );

    set_stroke_style(&ctx, color_changed);
    draw_bars(
        &ctx,
        &step
            .iter()
            .map(|command| match command.to_owned() {
                SortCommand::Swap(from, to) => vec![from, to],
                SortCommand::Set(index, _) => vec![index],
            })
            .collect::<Vec<Vec<usize>>>()
            .concat(),
        &items,
        max_val,
        width,
        canvas_height,
    );
}

fn draw_bars(
    ctx: &CanvasRenderingContext2d,
    indices: &[usize],
    values: &[u32],
    max_val: u32,
    width: f64,
    canvas_height: f64,
) {
    ctx.begin_path();
    for i in indices.iter() {
        let i = *i;
        let val = values[i] as f64;
        let x = (width * i as f64) + width * 0.5;
        let height = val / max_val as f64 * canvas_height;
        ctx.move_to(x, canvas_height);
        ctx.line_to(x, canvas_height - height);
    }
    ctx.stroke();
}

fn set_stroke_style(ctx: &CanvasRenderingContext2d, stroke_style: String) {
    ctx.set_stroke_style(&JsValue::from_str(&stroke_style));
}
