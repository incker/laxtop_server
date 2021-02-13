use image::{guess_format, ImageFormat};

pub fn get_image_format(image_buffer: &[u8]) -> Result<String, String> {
    if image_buffer.is_empty() {
        return Err("Image buffer is empty".to_string());
    }

    let img_format = guess_format(&image_buffer).map_err(|img_err| format!("{:?}", img_err))?;

    match img_format {
        ImageFormat::Jpeg => Ok("jpg".to_string()),
        _ => Err(format!(
            "Sorry, only jpg format supported, {:?} given",
            img_format
        )),
    }
}
