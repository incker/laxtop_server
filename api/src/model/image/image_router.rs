use radix_fmt::Radix;

pub struct ImageRouter(String);

impl ImageRouter {
    pub fn new(base_path: &str, id: u32) -> Self {
        // "/o/000/00/00.jpg"
        let mut res = String::with_capacity(base_path.len() + 16);
        res.push_str(base_path);
        res.push_str("/o/");
        let id_36_radix = format!("{:0>7}", Radix::new(id, 36).to_string());
        res.push_str(&id_36_radix[0..3]);
        res.push('/');
        res.push_str(&id_36_radix[3..5]);
        res.push('/');
        res.push_str(&id_36_radix[5..7]);
        res.push_str(".jpg");
        ImageRouter(res)
    }


    fn some_func(&mut self, ch: char) -> &str {
        let l = self.0.len() - 15;
        self.0.replace_range(l..l + 1, &String::from(ch));
        &self.0
    }


    pub fn dir_original(&mut self) -> &str {
        let l = self.0.len() - 6;
        &self.original()[..l]
    }


    pub fn dir_thumbnail(&mut self) -> &str {
        let l = self.0.len() - 6;
        &self.thumbnail()[..l]
    }


    pub fn original(&mut self) -> &str {
        self.some_func('o')
    }

    pub fn thumbnail(&mut self) -> &str {
        self.some_func('t')
    }
}


#[cfg(test)]
mod tests {
    use crate::model::ImageRouter;

    #[test]
    fn test_image_router() {
        let test_cases: Vec<(&str, u32, &str)> = vec![
            ("/test", u32::MAX, "/test/t/1z1/41/z3.jpg"),
            ("/test", 4, "/test/t/000/00/04.jpg"),
            ("", 4, "/t/000/00/04.jpg"),
        ];

        for (base_path, id, img_path) in test_cases {
            assert_eq!(ImageRouter::new(base_path, id).thumbnail(), img_path);
        }
    }

    #[test]
    fn test_image_router_dir() {
        let test_cases: Vec<(&str, u32, &str)> = vec![
            ("/test", u32::MAX, "/test/t/1z1/41/"),
            ("/test", 4, "/test/t/000/00/"),
            ("", 4, "/t/000/00/"),
        ];

        for (base_path, id, img_path) in test_cases {
            assert_eq!(ImageRouter::new(base_path, id).dir_thumbnail(), img_path);
        }
    }
}
