use std::borrow::Cow;

use regex::Regex;

pub struct Cleaner {
    re: Regex,
    replacer: String,
}

impl Cleaner {
    pub fn new_space_symbol_cleaner() -> Self {
        Cleaner {
            re: Regex::new("[\t\n\r]+").unwrap(),
            replacer: " ".to_string(),
        }
    }

    pub fn new_alarm_cleaner() -> Self {
        Cleaner {
            re: Regex::new("[!?]+").unwrap(),
            replacer: ".".to_string(),
        }
    }

    pub fn new_html_brackets_cleaner() -> Self {
        Cleaner {
            re: Regex::new("[<>]").unwrap(),
            replacer: " ".to_string(),
        }
    }

    pub fn new_multi_space_cleaner() -> Self {
        Cleaner {
            re: Regex::new(" {2,}").unwrap(),
            replacer: " ".to_string(),
        }
    }

    pub fn new_multi_dot_cleaner() -> Self {
        Cleaner {
            re: Regex::new("[.]{2,}").unwrap(),
            replacer: ".".to_string(),
        }
    }

    pub fn new_phone_cleaner() -> Self {
        Cleaner {
            re: Regex::new(r"[^+\d]").unwrap(),
            replacer: "".to_string(),
        }
    }

    pub fn clean<'a>(&self, text: &'a str) -> Cow<'a, str> {
        self.re.replace_all(text, self.replacer.as_str())
    }
}

pub struct MaxCleaner {
    cleaners: [Cleaner; 5],
}

impl Default for MaxCleaner {
    fn default() -> Self {
        MaxCleaner {
            cleaners: [
                Cleaner::new_space_symbol_cleaner(),
                Cleaner::new_alarm_cleaner(),
                Cleaner::new_html_brackets_cleaner(),
                Cleaner::new_multi_space_cleaner(),
                Cleaner::new_multi_dot_cleaner(),
            ],
        }
    }
}

impl MaxCleaner {
    pub fn clean_all(&self, text: &str) -> Option<String> {
        let t0 = &self.cleaners[0].clean(text.trim());
        let t1 = &self.cleaners[1].clean(t0);
        let t2 = &self.cleaners[2].clean(t1);
        let t3 = &self.cleaners[3].clean(t2);
        let t4 = &self.cleaners[4].clean(t3);

        let res: &str = t4;
        if res == text {
            None
        } else {
            Some(t4.to_string())
        }
    }

    pub fn clean_lines(&self, text: &str) -> Option<String> {
        let parts: Vec<&str> = text.split("\r\n").collect();
        let mut changed = false;

        let changed_parts: Vec<Option<String>> = {
            let mut changed_parts: Vec<Option<String>> = Vec::with_capacity(parts.len());
            for part in &parts {
                let res = self.clean_all(*part);
                if res.is_some() {
                    changed = true;
                }
                changed_parts.push(res);
            }
            changed_parts
        };

        if changed {
            let mut new_text = String::with_capacity(text.len());
            let last_index = changed_parts.len() - 1;
            for (i, item) in changed_parts.into_iter().enumerate() {
                if let Some(s) = item {
                    new_text.push_str(&s);
                } else {
                    new_text.push_str(parts[i]);
                }
                if last_index != i {
                    new_text.push_str("\r\n");
                }
            }
            Some(new_text)
        } else {
            None
        }
    }
}
