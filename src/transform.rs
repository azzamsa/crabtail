use voca_rs::*;

pub fn transform(input: &str) -> String {
    let classes = split::split(input, " ");
    let classes_underscored = classes
        .iter()
        .map(|&x| manipulate::replace_all(x, "-", "_"))
        .map(|x| manipulate::replace_all(&x, ":", "__"))
        .collect::<Vec<_>>();
    let classes_with_c = classes_underscored
        .iter()
        .map(|x| manipulate::insert(x, "C.", 0))
        .collect::<Vec<_>>();
    classes_with_c.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        let input_string = "text-white font-bold py-2";
        let expected_result = "C.text_white, C.font_bold, C.py_2".to_string();
        assert_eq!(transform(input_string), expected_result);
    }

    #[test]
    fn test_transform_double_dash() {
        let input_string = "bg-gray-100 text-white font-bold py-2";
        let expected_result = "C.bg_gray_100, C.text_white, C.font_bold, C.py_2".to_string();
        assert_eq!(transform(input_string), expected_result);
    }

    #[test]
    fn test_transform_one_word() {
        let input_string = "flex";
        let expected_result = "C.flex".to_string();
        assert_eq!(transform(input_string), expected_result);
    }

    #[test]
    fn test_transform_two_word() {
        let input_string = "flex flex-col";
        let expected_result = "C.flex, C.flex_col".to_string();
        assert_eq!(transform(input_string), expected_result);
    }

    #[test]
    fn test_transform_with_semicolon() {
        let input_string = "text-white font-bold py-2 hover:bg-blue-light rounded";
        let expected_result =
            "C.text_white, C.font_bold, C.py_2, C.hover__bg_blue_light, C.rounded".to_string();
        assert_eq!(transform(input_string), expected_result);
    }
}
