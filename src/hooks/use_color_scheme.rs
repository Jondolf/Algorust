use std::fmt::Display;

use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorScheme {
    Light,
    Dark,
}

impl Display for ColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ColorScheme::Light => "Light",
                ColorScheme::Dark => "Dark",
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ColorSchemeMode {
    #[default]
    Auto,
    Light,
    Dark,
}

pub fn use_color_scheme() -> ColorScheme {
    let app_color_scheme = use_state_eq(|| ColorScheme::Light);

    use_effect_with_deps(
        move |()| {
            let mode: Result<ColorScheme, StorageError> =
                LocalStorage::get("app-color-scheme-mode");
            if mode.is_err() {
                LocalStorage::set("app-color-scheme-mode", ColorSchemeMode::default()).unwrap();
            }
            || ()
        },
        (),
    );

    {
        let app_color_scheme = app_color_scheme.clone();

        use_effect_with_deps(
            move |(mode, preferred)| {
                app_color_scheme.set(match mode {
                    ColorSchemeMode::Auto => *preferred,
                    ColorSchemeMode::Light => ColorScheme::Light,
                    ColorSchemeMode::Dark => ColorScheme::Dark,
                });

                || ()
            },
            (
                LocalStorage::get("app-color-scheme-mode").unwrap_or_default(),
                LocalStorage::get("preferred-color-scheme").unwrap_or(ColorScheme::Light),
            ),
        );
    }

    *app_color_scheme
}
