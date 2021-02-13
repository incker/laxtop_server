use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use diesel::{
    BoolExpressionMethods, Connection, ExpressionMethods, MysqlConnection, QueryDsl, QueryResult,
    RunQueryDsl,
};
use sha2::{Digest, Sha256};

use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::iter::Iterator;
use std::path::PathBuf;

use crate::model::image::get_image_format::get_image_format;
use crate::model::ImageRouter;
use crate::schema::image;

const IMAGE_STORAGE_PATH: &str = "./static/i";

#[derive(Debug, Queryable, PartialEq, Insertable)]
#[table_name = "image"]
pub struct Image {
    pub id: u32,
}

impl Image {
    pub fn insert(conn: &MysqlConnection) -> u32 {
        use crate::schema::image::dsl;

        let mut last_image_id: u32 = image::table
            .select(dsl::id)
            .order(dsl::id.desc())
            .first::<u32>(conn)
            .unwrap();

        last_image_id += 1;
        loop {
            // transaction needed to take correct next image_id
            let res: QueryResult<usize> =
                conn.transaction::<usize, diesel::result::Error, _>(|| {
                    diesel::insert_into(image::table)
                        .values(dsl::id.eq(&last_image_id))
                        .execute(conn)
                });

            match res {
                Ok(_) => break,
                Err(DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                    last_image_id += 1;
                }
                Err(error) => panic!("Some fail here: NewInvoice::insert, error: {:?} ", error),
            }
        }
        last_image_id
    }

    pub fn save_image_from_buffer(
        image_buffer: Vec<u8>,
        conn: &MysqlConnection,
    ) -> Result<u32, String> {
        if image_buffer.is_empty() {
            return Err("Image buffer is empty".to_string());
        }

        let ext = get_image_format(&image_buffer)?;
        if ext != "jpg" {
            panic!("ext is not jpg");
        }

        let id = Image::insert(conn);

        Image::save_file(id, image_buffer);

        Ok(id)
    }

    pub fn save_file(id: u32, image_buffer: Vec<u8>) {
        let mut image_router = ImageRouter::new(IMAGE_STORAGE_PATH, id);

        let dir_original = image_router.dir_original();
        fs::create_dir_all(dir_original).unwrap();

        let mut file = File::create(image_router.original()).unwrap();
        file.write_all(&image_buffer).unwrap();

        // todo thumbnail and so on...
    }

    // TODO is outdated image path
    pub fn default_test_spot_image() -> String {
        "aa/aa/test_spot.webp".into()
    }
}
