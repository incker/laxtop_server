use std::marker::PhantomData;

pub struct TextMonoRectangle<'a> {
    pub spaces: String,
    pub line_char_limit: usize,
    phantom: PhantomData<&'a str>,
}

impl<'a> TextMonoRectangle<'a> {
    pub fn new(line_char_limit: usize) -> Self {
        let spaces = unsafe { String::from_utf8_unchecked(vec![b' '; line_char_limit]) };
        TextMonoRectangle {
            spaces,
            line_char_limit,
            phantom: PhantomData,
        }
    }

    pub fn spaces_amount(&'a self, len: usize) -> &'a str {
        &self.spaces[..len]
    }

    pub fn chunk_text(&'a self, name: &'a str) -> Vec<(&'a str, &'a str)> {
        let mut str_res_res: Vec<(&str, &str)> = Vec::new();

        // count chars and bytes while iteration
        let mut char_num: usize = 0;
        let mut byte_len: usize = 0;

        // save best places where we can cut string
        let mut fit_char: usize = 0;
        let mut fit_byte: usize = 0;

        let mut rest_string = name;

        for name_char in name.chars() {
            char_num += 1;

            byte_len += name_char.len_utf8();

            if name_char == ' ' {
                // ignore all spaces in start
                if rest_string.starts_with(' ') {
                    byte_len -= 1;
                    char_num -= 1;
                    rest_string = &rest_string[1..];
                    continue;
                }
                fit_char = char_num;
                fit_byte = byte_len;
            }

            if char_num >= self.line_char_limit {
                if fit_char == 0 {
                    fit_byte = byte_len;
                    fit_char = char_num;
                }
                let (cut_string, rest) = rest_string.split_at(fit_byte);
                rest_string = rest;

                {
                    let spaces_needed: usize = self.line_char_limit + 1 - fit_char;
                    str_res_res.push((cut_string, self.spaces_amount(spaces_needed)));
                }

                byte_len -= fit_byte;
                char_num -= fit_char;

                fit_char = 0;
                fit_byte = 0;
            }
        }

        if !rest_string.is_empty() {
            let spaces_needed: usize = (self.line_char_limit + 1 - char_num) as usize;
            str_res_res.push((rest_string, self.spaces_amount(spaces_needed)));
        }

        str_res_res
    }
}

/*
        ("одсветксветксветксветксветксветкс", " ")
        ("ветксветксветксветксветксветксве", "  ")
        ("тксветксветкойВ ыкодсветкойВы ", "    ")
        ("кодсветко йВыкодсветкойВ ", "         ")
        ("ыкодсве ", "                          ")
        ("ткойВыкодсветкойВыкодсветкойВыко", "  ")
        ("дсветкойВык В ы кл юч ат ель ", "     ")
        ("проходной с подс", "                  ")
*/
