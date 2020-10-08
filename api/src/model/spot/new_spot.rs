use diesel::{Connection, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use rocket_contrib::json::Json;

use crate::base::{RespErrors, ValidateFormatter};
use crate::model::{Location, OwnerType, Spot, SpotAddress, SpotBaseInfo, SpotStatus, UserSpot};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewSpot {
    pub address: SpotAddress,
    pub location: Location,
}

impl NewSpot {
    pub fn insert(&self, creator_type: OwnerType, creator_id: u32, conn: &MysqlConnection) -> Spot {
        use crate::schema::spot::{self, dsl};

        let values = (
            dsl::address.eq(&self.address.address),
            dsl::spot_type.eq(&self.address.spot_type),
            dsl::spot_name.eq(&self.address.spot_name),
            dsl::about.eq(""),
            dsl::status.eq(SpotStatus::Active),
            dsl::lng.eq(self.location.lng),
            dsl::lat.eq(self.location.lat),
            dsl::creator_type.eq(&creator_type),
            dsl::creator_id.eq(creator_id),
        );

        let spot_id = conn
            .transaction::<u32, diesel::result::Error, _>(|| {
                diesel::insert_into(spot::table)
                    .values(&values)
                    .execute(conn)?;
                // select inserted id (important to make inside transaction)
                spot::table
                    .select(dsl::id)
                    .order(dsl::id.desc())
                    .first(conn)
            })
            .expect("could not insert in NewSpot::new_default");

        Spot::attach_to_suppliers(spot_id, &self.location, conn);

        if OwnerType::User == creator_type {
            UserSpot::insert_values(creator_id, spot_id, conn);
            Spot::select(creator_id, spot_id, conn)
        } else {
            Spot::select(0, spot_id, conn)
        }
    }

    pub fn create_test_spot(
        creator_type: OwnerType,
        creator_id: u32,
        location: &Location,
        conn: &MysqlConnection,
    ) -> Spot {
        use crate::schema::spot;
        use crate::schema::spot::dsl;

        let spot = SpotBaseInfo::get_default_test_spot();

        let values = (
            dsl::address.eq(&spot.address.address),
            dsl::spot_type.eq(&spot.address.spot_type),
            dsl::spot_name.eq(&spot.address.spot_name),
            dsl::image_id.eq(1),
            dsl::about.eq(""),
            dsl::status.eq(SpotStatus::Test),
            dsl::lng.eq(location.lng),
            dsl::lat.eq(location.lat),
            dsl::creator_type.eq(&creator_type),
            dsl::creator_id.eq(creator_id),
        );

        let spot_id = conn
            .transaction::<u32, diesel::result::Error, _>(|| {
                diesel::insert_into(spot::table)
                    .values(&values)
                    .execute(conn)?;
                // select inserted id (important to make inside transaction)
                spot::table
                    .select(dsl::id)
                    .order(dsl::id.desc())
                    .first(conn)
            })
            .expect("could not insert in NewSpot::create_test_spot");

        Spot::attach_to_suppliers(spot_id, &location, conn);

        if OwnerType::User == creator_type {
            UserSpot::insert_values(creator_id, spot_id, conn);
            Spot::select(creator_id, spot_id, conn)
        } else {
            Spot::select(0, spot_id, conn)
        }
    }

    pub fn rocket_validate(&self) -> Result<(), Json<RespErrors>> {
        self.address
            .run_validator()
            .map_err(|mut resp_errors| {
                if let Err(error) = self.location.validate() {
                    resp_errors.errors.push(error)
                }
                Json(resp_errors)
            })
            .and({
                self.location
                    .validate()
                    .map_err(|error| Json(RespErrors::new_error(error)))
            })
    }
}
