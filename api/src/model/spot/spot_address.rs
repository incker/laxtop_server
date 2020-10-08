use crate::base::MaxCleaner;
use validator::Validate;

#[derive(Debug, Queryable, PartialEq, Serialize, Deserialize, Validate)]
pub struct SpotAddress {
    #[validate(length(min = 1, max = 255, message = "Can not be empty"))]
    pub address: String,
    #[validate(length(min = 1, max = 255, message = "Can not be empty"))]
    #[serde(rename = "spotType")]
    pub spot_type: String,
    #[serde(rename = "spotName")]
    pub spot_name: String,
}

impl SpotAddress {
    pub fn clean(&mut self, cleaner: &MaxCleaner) {
        if let Some(new_address) = cleaner.clean_all(&self.address) {
            self.address = new_address;
        }

        if let Some(new_spot_type) = cleaner.clean_all(&self.spot_type) {
            self.spot_type = new_spot_type;
        }

        if let Some(new_spot_name) = cleaner.clean_all(&self.spot_name) {
            self.spot_name = new_spot_name;
        }
    }
}
