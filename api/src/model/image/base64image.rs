use std::fs::File;
use std::io::{Cursor, Read};
use std::process::Command;

use diesel::MysqlConnection;
use files_to_sub_dirs::FolderSwitcher;

use crate::model::{new_file_sort_handler, Image, ImageSizeValidation};

#[derive(Debug, Serialize, Deserialize)]
pub struct Base64Image {
    pub base64: String,
}

impl Base64Image {
    pub fn save_image(
        &self,
        conn: &MysqlConnection,
        validation: ImageSizeValidation,
    ) -> Result<u32, String> {
        let webp_image_path: String = self.to_webp()?;

        let buffer = {
            let mut f = File::open(webp_image_path).unwrap();
            let mut buf = Vec::new();
            // read the whole file
            f.read_to_end(&mut buf).unwrap();
            buf
        };

        if validation != ImageSizeValidation::None {
            let res = image::io::Reader::new(Cursor::new(&buffer))
                // для размера формат в любом случае нужен
                .with_guessed_format()
                .unwrap();
            let (width, height) = res.into_dimensions().unwrap();

            if validation == ImageSizeValidation::Vertical && width > height {
                return Err("image is not vertical".to_string());
            }

            if let ImageSizeValidation::Size((valid_width, valid_height)) = validation {
                if width != valid_width || height != valid_height {
                    return Err(format!(
                        "image need to be {}х {} px, but {}x {} provided",
                        valid_width, valid_height, width, height
                    ));
                }
            }
        }

        Image::save_image_from_buffer(buffer, conn)
    }

    pub fn to_webp(&self) -> Result<String, String> {
        let random_name: String = {
            use rand::distributions::Alphanumeric;
            use rand::{thread_rng, Rng};
            use std::iter;

            let mut rng = thread_rng();
            iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .take(20)
                .collect()
        };

        let image_buffer =
            base64::decode(&self.base64).map_err(|decode_err| format!("{:?}", decode_err))?;

        // let ext: String = ImageInfo::get_image_format(&image_buffer)?;
        let file_name = random_name + ".tmp";

        let dirs: FolderSwitcher = {
            let mut file_sort_handler = new_file_sort_handler();
            file_sort_handler
                .save_file_from_buffer(&file_name, &image_buffer)
                .unwrap()
        };

        let mut storage_file_path: String = format!("./static/i/{}/{}", &dirs.0, &file_name);
        // node src/to_webp.js --file ./static/i/aa/aa/ygrx6a8kbe.jpg

        let node_file = {
            let mut node_file = dotenv::var("NODE_PATH").expect("NODE_PATH is not set");
            node_file.push_str("/to_webp.js");
            node_file
        };

        let output = Command::new("node")
            .arg(&node_file)
            .arg("--file")
            .arg(&storage_file_path)
            .output()
            .expect("failed to execute process");

        let node_resp = std::str::from_utf8(&output.stdout).unwrap().trim();

        if node_resp == "done" {
            storage_file_path.push_str(".webp");
            Ok(storage_file_path)
        } else {
            Err(if node_resp.is_empty() {
                "server error: node convert to webp has no response".to_string()
            } else {
                node_resp.to_string()
            })
        }
    }
}
