

pub mod html {

    pub fn convert_to_html(input: &str) -> String {
        let mut s = String::new();
        for i in input.chars() {
            match i {
                '<' => {
                    s.push_str("&lt;");
                }
                '>' => {
                    s.push_str("&gt;");
                }
                ' ' => {
                    s.push_str("&nbsp;");
                }
                '&' => {
                    s.push_str("&amp;");
                }
                '\"' => {
                    s.push_str("&quot;");
                }
                _ => {
                    s.push(i);
                }
            }
        }
        s
    }


}