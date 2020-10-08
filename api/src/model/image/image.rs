use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Seek, SeekFrom};
use std::path::PathBuf;

use diesel::{
    BoolExpressionMethods, Connection, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl,
};
use files_to_sub_dirs::{FileSortHandler, FolderSwitcher};

use crate::model::ImageInfo;
use crate::schema::image;

const IMAGE_STORAGE_PATH: &str = "./static/i/";

pub fn new_file_sort_handler() -> FileSortHandler {
    FileSortHandler::new(IMAGE_STORAGE_PATH, 1000)
        .unwrap_or_else(|_| panic!("wrong dir: {}", IMAGE_STORAGE_PATH))
}

#[derive(Debug, Queryable, PartialEq, Insertable)]
#[table_name = "image"]
pub struct Image {
    pub id: u32,
    pub dir: String,
    pub dir2: String,
    pub hash: String,
    pub hash2: String,
}

impl Image {
    pub fn get_by_id(image_id: u32, conn: &MysqlConnection) -> String {
        use crate::schema::image::dsl;
        if image_id == 0 {
            "".to_string()
        } else {
            image::table
                .filter(dsl::id.eq(image_id))
                .first::<Image>(conn)
                .map(|image: Image| image.get_src())
                .unwrap_or_default()
        }
    }

    pub fn get_urls_by_ids(image_ids: &[u32], conn: &MysqlConnection) -> HashMap<u32, String> {
        use crate::schema::image::dsl;
        let images: Vec<Image> = dsl::image
            .filter(dsl::id.eq_any(image_ids))
            .load::<Image>(conn)
            .expect("Image::get_by_ids");
        let mut hash_map = HashMap::with_capacity(images.len());
        for image in images {
            let src = image.get_url();
            hash_map.insert(image.id, src);
        }
        hash_map
    }

    pub fn save_image_from_buffer(
        image_buffer: Vec<u8>,
        conn: &MysqlConnection,
    ) -> Result<u32, String> {
        if image_buffer.is_empty() {
            return Err("Image buffer is empty".to_string());
        }

        let ext = ImageInfo::get_image_format(&image_buffer)?;
        if ext != "webp" {
            panic!("ext is not webp");
        }

        let (hash, hash2) = ImageInfo::get_short_hash_from_buff(&image_buffer);

        {
            let mut cursor = Cursor::new(&image_buffer);
            let images = Image::select_by_hash(&hash, &hash2, conn);
            for image in images {
                cursor.seek(SeekFrom::Start(0)).unwrap();
                if image.check_if_equal(&mut cursor) {
                    return Ok(image.id);
                }
            }
        }

        let dirs: FolderSwitcher = {
            let file_name = format!("{}.webp", &hash);
            let mut file_sort_handler = new_file_sort_handler();
            file_sort_handler
                .save_file_from_buffer(&file_name, &image_buffer)
                .unwrap()
        };

        let id = Image::values_insert(dirs.dir(), dirs.dir2(), &hash, &hash2, conn);

        Ok(id)
    }

    pub fn get_src(&self) -> String {
        format!(
            "{}{}/{}/{}.webp",
            IMAGE_STORAGE_PATH, &self.dir, &self.dir2, &self.hash
        )
    }

    pub fn build_url(dir: &str, dir2: &str, hash: &str) -> String {
        format!("{}/{}/{}.webp", dir, dir2, hash)
    }

    pub fn get_url(&self) -> String {
        Image::build_url(&self.dir, &self.dir2, &self.hash)
    }

    pub fn select_by_hash(hash: &str, hash2: &str, conn: &MysqlConnection) -> Vec<Self> {
        use crate::schema::image::dsl;
        let target = dsl::hash.eq(hash).and(dsl::hash2.eq(hash2));

        dsl::image
            .filter(target)
            .load::<Image>(conn)
            .expect("Error loading images")
    }

    pub fn file_server_path(&self) -> PathBuf {
        let mut path = PathBuf::with_capacity(14);
        path.push(IMAGE_STORAGE_PATH);
        path.push(&self.dir);
        path.push(&self.dir2);
        path.push(&self.hash);
        path.set_extension("webp");
        path
    }

    pub fn open_file(&self) -> File {
        let path = self.file_server_path();
        File::open(&path).unwrap_or_else(|_| panic!("unable to open path: {:?}", &path))
    }

    pub fn check_if_equal(&self, cursor: &mut Cursor<&Vec<u8>>) -> bool {
        ImageInfo::check_if_equal(&mut self.open_file(), cursor).unwrap()
    }

    pub fn values_insert(
        dir: &str,
        dir2: &str,
        hash: &str,
        hash2: &str,
        conn: &MysqlConnection,
    ) -> u32 {
        use crate::schema::image::dsl;

        let values = (
            dsl::dir.eq(dir),
            dsl::dir2.eq(dir2),
            dsl::hash.eq(hash),
            dsl::hash2.eq(hash2),
        );

        conn.transaction::<u32, diesel::result::Error, _>(|| {
            diesel::insert_into(image::table)
                .values(&values)
                .execute(conn)?;
            // select inserted id (important to make inside transaction)
            image::table
                .select(dsl::id)
                .order(dsl::id.desc())
                .first(conn)
        })
            .expect("could not insert in Image::values_insert")
    }

    pub fn default_test_spot_image() -> String {
        "aa/aa/test_spot.webp".into()
    }
}
