use crate::base::{MaxCleaner, RespErrors};

use crate::model::{
    AgentSpot, Base64Image, ImageSizeValidation, NewSpot, OwnerType, Spot, SpotUser, UserSpot,
};
use diesel::MysqlConnection;
use rocket_contrib::json::Json;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewAgentSpot {
    pub spot: NewSpot,
    pub user: SpotUser,
    #[serde(rename = "base64Image")]
    pub base64_image: Base64Image,
}

impl NewAgentSpot {
    pub fn clean(mut self) -> Self {
        let cleaner = MaxCleaner::default();
        self.user.clean(&cleaner);
        self.spot.address.clean(&cleaner);
        self
    }

    pub fn rocket_validate(&self) -> Result<(), Json<RespErrors>> {
        // required street, spot_type and image, valid location
        if self.base64_image.base64.is_empty() {
            Err(Json(RespErrors::new_error((
                "image".into(),
                "no image provided".into(),
            ))))
        } else {
            self.spot.rocket_validate()
        }
    }

    pub fn insert(
        &self,
        creator_type: OwnerType,
        creator_id: u32,
        conn: &MysqlConnection,
    ) -> Result<AgentSpot, (String, String)> {
        let spot_id = {
            let image_id = self
                .base64_image
                .save_image(conn, ImageSizeValidation::Vertical)
                .map_err(|err| ("image".into(), err))?;
            // coverage is setting while inserting spot
            let spot = self.spot.insert(creator_type, creator_id, conn);
            Spot::update_image_id(spot.id, image_id, conn);
            spot.id
        };

        let user_id = if !self.user.phone.is_empty() {
            let user_id = self.user.insert(creator_type, creator_id, conn);
            UserSpot::insert_values(user_id, spot_id, conn);
            user_id
        } else {
            0
        };

        Ok(AgentSpot::select(user_id, spot_id, conn))
    }
}
