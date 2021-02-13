use diesel::MysqlConnection;

use std::io::Cursor;

use crate::model::{Image, ImageSizeValidation};

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
        let buffer =
            base64::decode(&self.base64).map_err(|decode_err| format!("{:?}", decode_err))?;

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
}
