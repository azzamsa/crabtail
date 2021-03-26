#![
    allow(clippy::wildcard_imports) // importing all seed element make life harder
]

mod generated;
mod icon;
mod transform;

use seed::{prelude::*, *};

use crate::generated::css_classes::C;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        textarea_input: TextArea::default(TextAreaType::CSS),
        textarea_output: TextArea::default(TextAreaType::Typed),
        is_swapped: false,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    textarea_input: TextArea,
    textarea_output: TextArea,
    is_swapped: bool,
}

struct TextArea {
    label: Option<String>,
    placeholder: Option<String>,
    value: Option<String>,
}

#[derive(PartialEq)]
enum TextAreaType {
    CSS,
    Typed,
}

impl TextArea {
    fn default(textarea_type: TextAreaType) -> TextArea {
        Self::generate(textarea_type)
    }
    fn swapped(textarea_type: TextAreaType) -> TextArea {
        if textarea_type == TextAreaType::CSS {
            Self::generate(TextAreaType::Typed)
        } else {
            Self::generate(TextAreaType::Typed)
        }
    }
    fn generate(textarea_type: TextAreaType) -> TextArea {
        if textarea_type == TextAreaType::CSS {
            TextArea {
                label: Some("CSS".to_string()),
                placeholder: Some("py-2 text-white hover:bg-yellow-500".to_string()),
                value: Some("".to_string()),
            }
        } else {
            TextArea {
                label: Some("Typed".to_string()),
                placeholder: Some("C.py_2, C.text_white, C.hover__bg_yellow_500".to_string()),
                value: Some("".to_string()),
            }
        }
    }
}
// ------ ------
//    Update
// ------ ------

enum Msg {
    Transform,
    FirstTextAreaChanged(String),
    SecondTextAreaChanged(String),
    Swap,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    let is_swapped = &mut model.is_swapped;
    let textarea_input = &mut model.textarea_input;
    let textarea_output = &mut model.textarea_output;

    match msg {
        Msg::FirstTextAreaChanged(class_input) => {
            let default = if *is_swapped {
                TextArea::swapped(TextAreaType::CSS)
            } else {
                TextArea::default(TextAreaType::CSS)
            };
            *textarea_input = TextArea {
                value: Some(class_input),
                ..default
            };
        }
        Msg::SecondTextAreaChanged(class_input) => {
            let default = if *is_swapped {
                TextArea::swapped(TextAreaType::Typed)
            } else {
                TextArea::default(TextAreaType::Typed)
            };
            *textarea_output = TextArea {
                value: Some(class_input),
                ..default
            }
        }
        Msg::Swap => {
            let textarea_input_value_tmp = textarea_input.value.clone();
            let textarea_output_value_tmp = textarea_output.value.clone();

            let (default_input, default_output) = if *is_swapped {
                // keep the default
                *is_swapped = false;
                (
                    TextArea::default(TextAreaType::CSS),
                    TextArea::default(TextAreaType::Typed),
                )
            } else {
                // if not swapped yet
                // default_input (CSS) -> Typed
                // default_output (Typed) -> CSS
                *is_swapped = true;
                (
                    TextArea::default(TextAreaType::Typed),
                    TextArea::default(TextAreaType::CSS),
                )
            };
            *textarea_input = TextArea {
                value: Some(textarea_output_value_tmp.unwrap_or("".to_string())),
                ..default_input
            };
            *textarea_output = TextArea {
                value: Some(textarea_input_value_tmp.unwrap_or("".to_string())),
                ..default_output
            };
        }
        Msg::Transform => {
            let value = if *is_swapped {
                transform::to_css(&textarea_input.value.clone().unwrap_or("".to_string()))
            } else {
                // if not swapped yet
                transform::to_typed(&textarea_input.value.clone().unwrap_or("".to_string()))
            };
            *textarea_output = TextArea {
                value: Some(value),
                label: Some(textarea_output.label.clone().unwrap_or("".to_string())),
                placeholder: Some(
                    textarea_output
                        .placeholder
                        .clone()
                        .unwrap_or("".to_string()),
                ),
            };
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![section![
        C![
            C.bg_main,
            C.pt_12,
            C.md__pt_20,
            C.pb_6,
            C.px_2,
            C.md__px_5,
            C.min_h_screen
        ],
        header![
            C![C.max_w_lg, C.mx_auto],
            h1![C![C.font_bold, C.text_white C.text_center], "CrabTail"]
        ],
        main![
            C![
                C.bg_white,
                C.max_w_5xl,
                C.mx_auto,
                C.p_8,
                C.md__p_12,
                C.my_10,
                C.rounded_lg,
                C.shadow_2xl
            ],
            section![p![
                C![C.text_center, C.text_gray_600, C.pt_0],
                "Convert your TailwindCSS class to typed Rust",
            ]],
            section![
                C![C.flex, C.flex_col, C.mt_10],
                div![
                    C![C.mb_6, C.pt_3, C.rounded, C.bg_gray_200],
                    label![
                        C![C.input_label],
                        &model.textarea_input.label.as_deref().unwrap_or("")
                    ],
                    textarea![
                        id!["css"],
                        attrs! {
                            At::Type => "text",
                            At::Placeholder => &model.textarea_input.placeholder.as_deref().unwrap_or("");
                            At::Value => &model.textarea_input.value.as_deref().unwrap_or("")
                        },
                        C![C.input],
                        input_ev(Ev::Input, Msg::FirstTextAreaChanged),
                    ]
                ],
                div![
                    C![C.mb_6, C.pt_3, C.rounded, C.bg_gray_200],
                    label![C![C.input_label], &model.textarea_output.label],
                    textarea![
                        id!["typed"],
                        attrs! {
                            At::Type => "text";
                            At::Placeholder => &model.textarea_output.placeholder.as_deref().unwrap_or("");
                            At::Value => &model.textarea_output.value.as_deref().unwrap_or("");
                        },
                        C![C.input],
                        input_ev(Ev::Input, Msg::SecondTextAreaChanged),
                    ]
                ],
                div![
                    C![C.flex, C.justify_end],
                    button![
                        C![C.btn, C.mb_6, C.px_3, C.py_1, C.stroke_2],
                        raw_svg!(icon::get(icon::Name::SwitchVertical)),
                        ev(Ev::Click, |_| Msg::Swap),
                    ]
                ],
                button![
                    C![C.btn, C.inline_flex, C.justify_center, C.stroke_2],
                    raw_svg!(icon::get(icon::Name::Rocket)),
                    span!["Go"],
                    ev(Ev::Click, |_| Msg::Transform),
                ],
            ],
        ],
    ]]
}

// ------ ------
//     Start
// ------ ------

pub fn main() {
    App::start("app", init, update, view);
}
