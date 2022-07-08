use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_interval;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct StepSliderProps {
    #[prop_or_default]
    pub label: String,
    pub active_step_index: usize,
    pub max: usize,
    #[prop_or(10.0)]
    // How long it should take to play all steps in seconds.
    pub playback_time: f32,
    #[prop_or(false)]
    pub disabled: bool,
    pub on_change: Callback<usize>,
}

#[function_component(StepSlider)]
pub fn step_slider(props: &StepSliderProps) -> Html {
    let StepSliderProps {
        label,
        active_step_index,
        max,
        playback_time,
        disabled,
        on_change,
    } = props.clone();

    // The interval is cancelled whenever the time is 0 ms.
    let interval_ms = use_state(|| 0);
    let playing = *interval_ms != 0;
    // How long one step should play for in milliseconds.
    let step_play_time_ms = use_state(|| playback_time / max as f32 * 1000.0);
    // Roughly 30 fps
    let max_refresh_rate_ms = 33.3333;

    {
        let step_play_time_ms = step_play_time_ms.clone();
        let interval_ms = interval_ms.clone();

        use_effect_with_deps(
            move |_| {
                let new_step_play_time_ms = playback_time / max as f32 * 1000.0;
                step_play_time_ms.set(new_step_play_time_ms);
                if playing && !disabled {
                    interval_ms.set(new_step_play_time_ms.max(max_refresh_rate_ms) as u32);
                }
                || ()
            },
            (playback_time, max),
        );
    }

    {
        let on_change = on_change.clone();
        let interval_ms_value = *interval_ms;
        let interval_ms = interval_ms.clone();
        let step_play_time_ms = *step_play_time_ms;

        use_interval(
            move || {
                if disabled {
                    return;
                }
                if active_step_index >= max {
                    // Clear interval when the end is reached.
                    interval_ms.set(0);
                } else {
                    let step_increment = (max_refresh_rate_ms / step_play_time_ms).ceil() as usize;
                    let new_step_index = active_step_index + step_increment;
                    if new_step_index >= max {
                        on_change.emit(max);
                    } else {
                        on_change.emit(new_step_index);
                    }
                }
            },
            interval_ms_value,
        )
    };

    let oninput = {
        let on_change = on_change.clone();

        Callback::from(move |event: InputEvent| {
            let el: HtmlInputElement = event.target_unchecked_into();
            let value = el.value().parse::<usize>().unwrap();
            on_change.emit(value);
        })
    };

    let play_step_player = {
        let interval_ms = interval_ms.clone();

        move || {
            if active_step_index >= max {
                on_change.emit(0);
            } else {
                on_change.emit(active_step_index + 1);
            }
            interval_ms.set(step_play_time_ms.max(max_refresh_rate_ms) as u32);
        }
    };
    let pause_step_player = move || interval_ms.set(0);

    let on_click_playback_button = Callback::from(move |_| {
        if playing || disabled {
            pause_step_player()
        } else {
            play_step_player()
        }
    });

    html! {
        <>
            {
                if !label.is_empty() {
                    html! {
                        <label class="step-slider-label" for={format!("stepSlider{}", label.clone())}>{{ label.clone() }}</label>
                    }
                } else {
                    html! {}
                }
            }
            <div class="step-slider">
                { playback_button(on_click_playback_button, playing) }
                <input
                    type="range"
                    id={format!("stepSlider{}", label.clone())}
                    min="0"
                    max={max.to_string()}
                    value={active_step_index.to_string()}
                    {disabled}
                    {oninput}
                />
            </div>
        </>
    }
}

fn playback_button(onclick: Callback<MouseEvent>, playing: bool) -> Html {
    html! {
        <button aria-label="Step playback button" {onclick}>
            {
                if playing {
                    html! {
                        <svg width="500" height="600" viewBox="0 0 500 600" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <g id="pause">
                                <path
                                    d="M455 15H340.742C324.173 15 310.742 28.4314 310.742 45V555C310.742 571.569 324.173 585 340.742 585H455C471.569 585 485 571.569 485 555V45C485 28.4315 471.569 15 455 15Z"
                                    fill="white" stroke="white" stroke-width="30" stroke-linejoin="round" />
                                <path
                                    d="M159.258 15H45C28.4315 15 15 28.4314 15 45V555C15 571.569 28.4315 585 45 585H159.258C175.827 585 189.258 571.569 189.258 555V45C189.258 28.4315 175.827 15 159.258 15Z"
                                    fill="white" stroke="white" stroke-width="30" stroke-linejoin="round" />
                            </g>
                        </svg>

                    }
                } else {
                    html! {
                        <svg width="502" height="586" viewBox="0 0 502 586" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path id="play" d="M472.125 267.275L60.4349 20.2609C40.4391 8.26348 15 22.6669 15 45.9857V540.014C15 563.333 40.4391 577.737 60.4348 565.739L472.125 318.725C491.546 307.073 491.546 278.927 472.125 267.275Z" fill="white" stroke="white" stroke-width="30" stroke-linejoin="round"/>
                        </svg>
                    }
                }
            }
        </button>
    }
}
