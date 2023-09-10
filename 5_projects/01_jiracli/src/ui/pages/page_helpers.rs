use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {    
    let text_len = text.len();

    if text_len == width {
        return String::from(text);
    }
    else if text_len > width{
        match width{
            0 => String::from(""),
            1 => String::from("."),
            2 => String::from(".."),
            3 => String::from("..."),
            _ => String::from(text.truncate_ellipse(width - 3))
        }
    }
    else{
        return String::from(format!("{:width$}", text));
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    } 
}