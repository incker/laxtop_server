use std::io::Read;
use std::ops::Deref;

use diesel::MysqlConnection;
use multipart::server::save::SavedData;
use rocket::Data;

use crate::guards::{Boundary, DbConn};
use crate::model::Image;

// multipart/form-data!
#[post("/upload-from-form", format = "multipart/form-data", data = "<data>")]
pub fn upload_from_form_post(
    boundary: Boundary,
    data: Data,
    db_conn: DbConn,
) -> Result<String, String> {
    save_image_files(boundary, data, db_conn.deref()).map(|_| "Ok".to_string())
}

fn save_image_files(boundary: Boundary, data: Data, conn: &MysqlConnection) -> Result<(), String> {
    let mut entries = boundary
        .entries_from_data(data)
        .ok_or_else(|| "Sorry, your request was not parsed by server".to_string())?;

    let _name: SavedData = entries
        .fields
        .remove("name")
        .ok_or_else(|| "Sorry, there are no field \"name\"".to_string())?
        .pop()
        .ok_or_else(|| "Sorry, there are no field \"name\"".to_string())?
        .data;

    let post_images = entries
        .fields
        .remove("photo")
        .ok_or_else(|| "Sorry, there are no field \"images\"".to_string())?;

    for post_image in post_images {
        let saved_data = &post_image.data;
        let mut data_reader = saved_data.readable().unwrap();
        let mut buffer = Vec::new();
        data_reader.read_to_end(&mut buffer).unwrap();
        Image::save_image_from_buffer(buffer, conn).unwrap();
    }

    Ok(())
}







pub struct Boundary<'a>(&'a str);

impl<'a> Boundary<'a> {
    pub fn entries_from_data(&self, data: Data) -> Option<Entries> {
        use multipart::server::Multipart;

        let mut multipart = Multipart::with_body(data.open(), self.0);
        multipart.save().temp().into_entries()
    }
}




// Guards


impl<'a, 'r> FromRequest<'a, 'r> for Boundary<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let content_type: &ContentType = request.guard::<&ContentType>().unwrap();

        if content_type.is_form_data() {
            match content_type.params().find(|&(k, _)| k == "boundary") {
                Some((_, boundary)) => Outcome::Success(Boundary(boundary)),
                None => Outcome::Failure((Status::from_code(401).unwrap(), ())),
            }
        } else {
            Outcome::Failure((Status::from_code(401).unwrap(), ()))
        }
    }
}






