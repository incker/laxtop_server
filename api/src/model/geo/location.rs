use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use geo::Coordinate;
use rocket_contrib::json::Json;

use crate::base::RespErrors;
use crate::model::Spot;

// field sequence is equal to Coordinate struct
#[derive(Debug, Default, Queryable, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub lng: f32,
    pub lat: f32,
}

impl Into<(f32, f32)> for Location {
    fn into(self) -> (f32, f32) {
        let Location { lng, lat } = self;
        (lng, lat)
    }
}

impl Into<Coordinate<f32>> for Location {
    fn into(self) -> Coordinate<f32> {
        let Location { lng, lat } = self;
        Coordinate { x: lng, y: lat }
    }
}

impl Location {
    pub fn new_from_county_code(country_code: &str, conn: &MysqlConnection) -> Option<Self> {
        use crate::schema::country;
        country::table
            .select((country::dsl::lng, country::dsl::lat))
            .filter(country::dsl::code.eq(country_code))
            .first::<Location>(conn)
            .ok()
    }

    pub fn validate(&self) -> Result<(), (String, String)> {
        Location::validate_lng(self.lng).and(Location::validate_lat(self.lat))
    }

    pub fn validate_lng(lng: f32) -> Result<(), (String, String)> {
        if !(-180f32..=180f32).contains(&lng) {
            Err((
                "location".into(),
                format!("Wrong longitude provided: {}", lng),
            ))
        } else {
            Ok(())
        }
    }

    pub fn validate_lat(lat: f32) -> Result<(), (String, String)> {
        if !(-90f32..=90f32).contains(&lat) {
            Err((
                "location".into(),
                format!("Wrong latitude provided: {}", lat),
            ))
        } else {
            Ok(())
        }
    }

    pub fn rocket_validate(&self) -> Result<(), Json<RespErrors>> {
        self.validate()
            .map_err(|error| Json(RespErrors::new_error(error)))
    }

    pub fn rocket_validate_distance(
        &self,
        spot_id: u32,
        radius: f32,
        conn: &MysqlConnection,
    ) -> Result<(), Json<RespErrors>> {
        Spot::validate_distance(spot_id, &self, radius, conn)
            .map_err(|err| Json(RespErrors::new(vec![err])))
    }
}
