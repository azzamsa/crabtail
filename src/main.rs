mod generated;

use seed::{prelude::*, *};

use crate::generated::css_classes::C;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

type Model = i32;

// ------ ------
//    Update
// ------ ------

enum Msg {}

fn update(_: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {}

// ------ ------
//     View
// ------ ------

fn view(_: &Model) -> impl IntoNodes<Msg> {
    vec![
        header![
            C![C.max_w_lg, C.mx_auto],
            h1![C![C.font_bold, C.text_white C.text_center], "tail2rs"]
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
                "Convert your TailwindCSS class to typed Rust"
            ]],
            section![
                C![C.flex, C.flex_col, C.mt_10],
                div![
                    C![C.mb_6, C.pt_3, C.rounded, C.bg_gray_200],
                    label![
                        C![
                            C.block,
                            C.text_gray_700,
                            C.text_sm,
                            C.font_bold,
                            C.mb_2,
                            C.ml_3
                        ],
                        "CSS"
                    ],
                    input![
                        id!["css"],
                        attrs! {
                            At::Type => "text"
                        },
                        C!["input"],
                    ]
                ],
                div![
                    C![C.mb_6, C.pt_3, C.rounded, C.bg_gray_200],
                    label![
                        C![
                            C.block,
                            C.text_gray_700,
                            C.text_sm,
                            C.font_bold,
                            C.mb_2,
                            C.ml_3
                        ],
                        "Typed"
                    ],
                    input![
                        id!["typed"],
                        attrs! {
                            At::Type => "text"
                        },
                        C!["input"]
                    ]
                ],
                button![C!["btn"], "Go 🚀",]
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
