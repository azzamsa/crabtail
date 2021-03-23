#![allow(clippy::wildcard_imports)]

mod generated;
mod transform;

use seed::{prelude::*, *};

use crate::generated::css_classes::C;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        css_input: "".to_string(),
        css_typed: "".to_string(),
    }
}

// ------ ------
//     Model
// ------ ------

//#[derive(Copy, Clone)]
struct Model {
    css_input: String,
    css_typed: String,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    Transform,
    CSSInputChanged(String),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    let css_input = &mut model.css_input;
    let css_typed = &mut model.css_typed;
    match msg {
        Msg::CSSInputChanged(input) => {
            *css_input = input;
        }
        Msg::Transform => *css_typed = transform::transform(css_input),
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![
        header![
            C![C.max_w_lg, C.mx_auto],
            h1![C![C.font_bold, C.text_white C.text_center], "CrabTail"]
        ],
        main![
            C![
                C.bg_white,
                C.max_w_lg,
                C.mx_auto,
                C.p_8,
                C.md__p_12,
                C.my_10,
                C.rounded_lg,
                C.shadow_2xl
            ],
            section![p![
                C![C.text_center, C.text_gray_600, C.pt_0],
                "Convert your TailwindCSS 💨 class to typed Rust 🦀",
            ]],
            section![
                C![C.flex, C.flex_col, C.mt_10],
                div![
                    C![C.mb_6, C.pt_3, C.rounded, C.bg_gray_200],
                    label![C!["input-label"], "CSS"],
                    input![
                        id!["css"],
                        attrs! {
                            At::Type => "text",
                            At::Placeholder => "py-2 text-white hover:bg-blue-light";

                        },
                        C!["input"],
                        input_ev(Ev::Input, Msg::CSSInputChanged),
                    ]
                ],
                div![
                    C![C.mb_6, C.pt_3, C.rounded, C.bg_gray_200],
                    label![C!["input-label"], "Typed"],
                    input![
                        id!["typed"],
                        attrs! {
                            At::Type => "text"
                            At::Value => model.css_typed;
                        },
                        C!["input"],
                    ]
                ],
                button![C!["btn"], ev(Ev::Click, |_| Msg::Transform), "Go 🚀",]
            ],
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

pub fn main() {
    App::start("app", init, update, view);
}
