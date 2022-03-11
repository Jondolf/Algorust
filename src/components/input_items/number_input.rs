use crate::components::input_items::input_title_to_id;

use std::{fmt::Display, str::FromStr};

use num_traits::{Float, PrimInt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct IntInputProps<T: 'static + Clone + Copy + Display + PartialEq + PrimInt + FromStr> {
    pub title: String,
    pub value: T,
    #[prop_or(None)]
    pub min: Option<T>,
    #[prop_or(None)]
    pub max: Option<T>,
    #[prop_or(T::from(1).unwrap())]
    pub step: T,
    pub oninput: Callback<T>,
}

#[function_component(IntInput)]
pub fn int_input<T: 'static + Clone + Copy + Display + PartialEq + PrimInt + FromStr>(
    props: &IntInputProps<T>,
) -> Html {
    let IntInputProps {
        title,
        value,
        min,
        max,
        step,
        oninput,
    } = props.clone();
    let id = input_title_to_id(&title);
    let oninput = move |event: InputEvent| {
        let el: HtmlInputElement = event.target_unchecked_into();
        if let Ok(value) = el.value().parse::<T>() {
            oninput.emit(value);
        }
    };

    html! {
        <div class="input number-input">
            <label for={id.clone()}>{title.to_string()}</label>
            <input {id}
                type="number"
                placeholder={title.to_string()}
                value={value.to_string()}
                min={min.unwrap_or_else(T::min_value).to_string()}
                max={max.unwrap_or_else(T::max_value).to_string()}
                step={step.to_string()}
                oninput={oninput}
            />
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct FloatInputProps<T: 'static + Clone + Copy + Display + PartialEq + Float + FromStr> {
    pub title: String,
    pub value: T,
    #[prop_or(None)]
    pub min: Option<T>,
    #[prop_or(None)]
    pub max: Option<T>,
    #[prop_or(T::from(0.1).unwrap())]
    pub step: T,
    pub oninput: Callback<T>,
}

#[function_component(FloatInput)]
pub fn float_input<T: 'static + Clone + Copy + Display + PartialEq + Float + FromStr>(
    props: &FloatInputProps<T>,
) -> Html {
    let FloatInputProps {
        title,
        value,
        min,
        max,
        step,
        oninput,
    } = props.clone();
    let id = input_title_to_id(&title);
    let oninput = move |event: InputEvent| {
        let el: HtmlInputElement = event.target_unchecked_into();
        if let Ok(value) = el.value().parse::<T>() {
            oninput.emit(value);
        }
    };

    html! {
        <div class="input number-input">
            <label for={id.clone()}>{title.to_string()}</label>
            <input {id}
                type="number"
                placeholder={title.to_string()}
                value={value.to_string()}
                min={min.unwrap_or_else(T::min_value).to_string()}
                max={max.unwrap_or_else(T::max_value).to_string()}
                step={step.to_string()}
                oninput={oninput}
            />
        </div>
    }
}
