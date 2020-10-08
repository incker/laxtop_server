use std::fs::File;
use std::io;
use std::io::{Cursor, Read};
use std::iter::Iterator;

use image::{guess_format, ImageFormat};
use sha2::{Digest, Sha256};

pub struct ImageInfo;

impl ImageInfo {
    pub fn get_short_hash_from_file(file: &mut File) -> (String, String) {
        let sha256 = {
            let mut sha256 = Sha256::new();
            io::copy(file, &mut sha256).unwrap();
            sha256
        };

        ImageInfo::sha256_to_custom_short_hash(sha256)
    }

    pub fn get_short_hash_from_buff(file: &[u8]) -> (String, String) {
        let sha256 = {
            let mut sha256 = Sha256::new();
            let mut buff = Cursor::new(file);
            io::copy(&mut buff, &mut sha256).unwrap();
            sha256
        };

        ImageInfo::sha256_to_custom_short_hash(sha256)
    }

    fn sha256_to_custom_short_hash(sha256: Sha256) -> (String, String) {
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
        let hash = sha256.finalize();
        let f: Vec<u8> = hash[..30]
            .iter()
            .map(|&x| CHARSET[(x % 36) as usize])
            .collect();
        (
            String::from_utf8_lossy(&f[..10]).into_owned(),
            String::from_utf8_lossy(&f[10..30]).into_owned(),
        )
    }

    pub fn get_image_format(image_buffer: &[u8]) -> Result<String, String> {
        if image_buffer.is_empty() {
            return Err("Image buffer is empty".to_string());
        }

        let img_format = guess_format(&image_buffer).map_err(|img_err| format!("{:?}", img_err))?;

        match img_format {
            ImageFormat::WebP => Ok("webp".to_string()),
            _ => Err(format!(
                "Sorry, only jpg/png/webp formats supported, {:?} given",
                img_format
            )),
        }
    }

    pub fn check_if_equal(f1: &mut File, f2: &mut Cursor<&Vec<u8>>) -> std::io::Result<bool> {
        let buff1: &mut [u8] = &mut [0; 1024];
        let buff2: &mut [u8] = &mut [0; 1024];

        loop {
            let f1_read_len = f1.read(buff1)?;
            let f2_read_len = f2.read(buff2)?;

            if f1_read_len != f2_read_len {
                return Ok(false);
            }
            if f1_read_len == 0 {
                return Ok(true);
            }
            if buff1[0..f1_read_len] != buff2[0..f2_read_len] {
                return Ok(false);
            }
        }
    }
}
