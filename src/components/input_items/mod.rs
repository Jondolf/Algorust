mod button;
mod checkbox;
mod number_input;
mod select_input;

pub use button::Button;
pub use checkbox::Checkbox;
pub use number_input::{FloatInput, IntInput};
pub use select_input::SelectInput;

use regex::Regex;

pub fn input_title_to_id(title: &str) -> String {
    let reg = Regex::new(r"[^A-Za-z_]").unwrap();
    reg.replace_all(&title.replace(" ", "_"), "").to_lowercase()
}
