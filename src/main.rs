mod generated;
mod transform;

use seed::{attrs, button, div, h1, header, id, label, main, p, prelude::*, section, textarea, C};

use crate::generated::css_classes::C;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        first_textarea: TextArea {
            label: "CSS".to_string(),
            placeholder: "py-2 text-white hover:bg-yellow-500".to_string(),
            value: "".to_string(),
        },
        second_textarea: TextArea {
            label: "Typed".to_string(),
            placeholder: "C.py_2, C.text_white, C.hover__bg_yellow_500".to_string(),
            value: "".to_string(),
        },
        swapped_label: false,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    first_textarea: TextArea,
    second_textarea: TextArea,
    swapped_label: bool,
}

struct TextArea {
    label: String,
    placeholder: String,
    value: String,
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
    let swapped_label = &mut model.swapped_label;
    let first_textarea = &mut model.first_textarea;
    let second_textarea = &mut model.second_textarea;

    match msg {
        Msg::FirstTextAreaChanged(class_input) => {
            *first_textarea = TextArea {
                label: first_textarea.label.clone(),
                placeholder: first_textarea.placeholder.clone(),
                value: class_input,
            };
        }
        Msg::SecondTextAreaChanged(class_input) => {
            *second_textarea = TextArea {
                label: second_textarea.label.clone(),
                placeholder: second_textarea.placeholder.clone(),
                value: class_input,
            };
        }
        Msg::Transform => {
            if *swapped_label {
                *second_textarea = TextArea {
                    label: second_textarea.label.clone(),
                    placeholder: second_textarea.placeholder.clone(),
                    value: transform::to_css(&first_textarea.value),
                };
            } else {
                *second_textarea = TextArea {
                    label: second_textarea.label.clone(),
                    placeholder: second_textarea.placeholder.clone(),
                    value: transform::to_typed(&first_textarea.value),
                };
            }
        }
        Msg::Swap => {
            if *swapped_label {
                let first_textarea_value_tmp = first_textarea.value.clone();
                let second_textarea_value_tmp = second_textarea.value.clone();
                *first_textarea = TextArea {
                    label: "CSS".to_string(),
                    placeholder: "py-2 text-white hover:bg-yellow-500".to_string(),
                    value: second_textarea_value_tmp,
                };
                *second_textarea = TextArea {
                    label: "Typed".to_string(),
                    placeholder: "C.py_2, C.text_white, C.hover__bg_yellow_500".to_string(),
                    value: first_textarea_value_tmp,
                };
                *swapped_label = false;
            } else {
                let first_textarea_value_tmp = first_textarea.value.clone();
                let second_textarea_value_tmp = second_textarea.value.clone();
                *first_textarea = TextArea {
                    label: "Typed".to_string(),
                    placeholder: "C.py_2, C.text_white, C.hover__bg_yellow_500".to_string(),
                    value: second_textarea_value_tmp,
                };
                *second_textarea = TextArea {
                    label: "CSS".to_string(),
                    placeholder: "py-2 text-white hover:bg-yellow-500".to_string(),
                    value: first_textarea_value_tmp,
                };
                *swapped_label = true;
            }
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
                "Convert your TailwindCSS \u{1f4a8} class to typed Rust \u{1f980}",
            ]],
            section![
                C![C.flex, C.flex_col, C.mt_10],
                div![
                    C![C.mb_6, C.pt_3, C.rounded, C.bg_gray_200],
                    label![C!["input-label"], &model.first_textarea.label],
                    textarea![
                        id!["css"],
                        attrs! {
                            At::Type => "text",
                            At::Placeholder => &model.first_textarea.placeholder;
                            At::Value => model.first_textarea.value;
                        },
                        C!["input"],
                        input_ev(Ev::Input, Msg::FirstTextAreaChanged),
                    ]
                ],
                div![
                    C![C.mb_6, C.pt_3, C.rounded, C.bg_gray_200],
                    label![C!["input-label"], &model.second_textarea.label],
                    textarea![
                        id!["typed"],
                        attrs! {
                            At::Type => "text";
                            At::Placeholder => &model.second_textarea.placeholder;
                            At::Value => model.second_textarea.value;
                        },
                        C!["input"],
                        input_ev(Ev::Input, Msg::SecondTextAreaChanged),
                    ]
                ],
                div![
                    C![C.flex, C.justify_end],
                    button![
                        C![
                            "btn",
                            C.mb_6,
                            C.px_3,
                            C.py_1,
                            C.hover__text_yellow_700,
                            C.hover__underline
                        ],
                        ev(Ev::Click, |_| Msg::Swap),
                        "\u{1f500}"
                    ]
                ],
                button![C!["btn"], ev(Ev::Click, |_| Msg::Transform), "Go \u{1f680}"]
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
