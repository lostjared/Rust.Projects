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

    /*
    std::string urldecode(std::string text) {
        std::ostringstream stream;
        for(unsigned int i = 0; i < text.length(); ++i) {
            if(text[i] == '+') {
                stream << " ";
                continue;
            }
            if(text[i] == '%') {
                if(i+2 < text.length()) {
                    ++i;
                    std::string test;
                    test += text[i];
                    ++i;
                    test += text[i];
                    int char_value;
                    sscanf(test.c_str(), "%x", &char_value);
                    stream << (char)char_value;
                    continue;
                }
            }
            stream << text[i];
        }
        return stream.str();
    } */

    pub fn url_decode(input: &str) -> String {
        let mut s = String::new();
        let mut i = 0;

        while i < input.len() {
            if input.chars().nth(i).unwrap() == '+' {
                s.push(' ');
                i += 1;
                continue;
            }
            if input.chars().nth(i).unwrap() == '%' {
                if i + 2 < input.len() {
                    i += 1;
                    let mut sval = String::new();
                    sval.push(input.chars().nth(i).unwrap());
                    i += 1;
                    sval.push(input.chars().nth(i).unwrap());
                    let ch_val = u8::from_str_radix(&sval, 16).unwrap();
                    s.push(ch_val as char);
                    i += 1;
                    continue;
                }
                i += 1;
                continue;
            }
            s.push(input.chars().nth(i).unwrap());
            i += 1;
        }
        s
    }
}
