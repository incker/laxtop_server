use std::ops::Deref;

use geo::Coordinate;
use rocket_contrib::json::Json;

use crate::base::RespErrors;
use crate::guard::{DbConn, UserId};
use crate::model::{
    Base64Image, DataWrapper, ImageSizeValidation, Location, NewSpot, OwnerType, Spot,
    SpotBaseInfo, SpotOrg, UserProfileInfo, UserSpot,
};

// radius for spots nearby
const RADIUS: f32 = 0.002; // 220 meters (https://habr.com/ru/post/228023/)

#[post("/set-user-name", format = "application/json", data = "<user_name>")]
fn set_user_name(
    user_id: UserId,
    user_name: Json<DataWrapper<String>>,
    db_conn: DbConn,
) -> Result<Json<DataWrapper<String>>, Json<RespErrors>> {
    let user_name = user_name.into_inner().data();
    if user_name == "" {
        Err(Json(RespErrors::new_error((
            "name".into(),
            "Name can not be empty".into(),
        ))))
    } else {
        UserProfileInfo::update_name_if_not_set(user_id.0, &user_name, db_conn.deref());
        // name can not be set twice, so read it from db
        let immutable_name = UserProfileInfo::select_by_user(user_id.into(), db_conn.deref()).name;
        Ok(Json(DataWrapper::new(immutable_name)))
    }
}

#[post(
    "/set-user-license-accepted",
    format = "application/json",
    data = "<accepted>"
)]
fn set_user_license_accepted(
    user_id: UserId,
    accepted: Json<DataWrapper<bool>>,
    db_conn: DbConn,
) -> Json<DataWrapper<bool>> {
    let accepted = accepted.into_inner().data();
    UserProfileInfo::update_license_accepted(user_id.0, accepted, db_conn.deref());
    // license can not be unaccepted
    let accepted = accepted
        || UserProfileInfo::select_by_user(user_id.into(), db_conn.deref()).license_accepted;
    Json(DataWrapper::new(accepted))
}

#[post("/get-spots-nearby", format = "application/json", data = "<location>")]
fn get_spots_nearby(
    _user_id: UserId,
    location: Json<Location>,
    db_conn: DbConn,
) -> Result<Json<DataWrapper<Vec<SpotBaseInfo>>>, Json<RespErrors>> {
    location.rocket_validate()?;

    let coordinate: Coordinate<f32> = location.into_inner().into();
    let spots = SpotBaseInfo::get_nearby_spots(coordinate, RADIUS, db_conn.deref());

    Ok(Json(DataWrapper::new(spots)))
}

#[post("/set-new-spot", format = "application/json", data = "<new_spot>")]
fn set_new_spot(
    user_id: UserId,
    new_spot: Json<NewSpot>,
    db_conn: DbConn,
) -> Result<Json<Spot>, Json<RespErrors>> {
    new_spot.rocket_validate()?;

    let spot = new_spot
        .into_inner()
        .insert(OwnerType::User, user_id.into(), db_conn.deref());
    Ok(Json(spot))
}

#[post(
    "/add-existing-spot/<spot_id>",
    format = "application/json",
    data = "<location>"
)]
fn add_existing_spot(
    user_id: UserId,
    spot_id: u32,
    location: Json<Location>,
    db_conn: DbConn,
) -> Result<Json<Spot>, Json<RespErrors>> {
    location.rocket_validate()?;
    let spot = if spot_id != 1 {
        location.rocket_validate_distance(spot_id, RADIUS * 2.0, db_conn.deref())?;
        UserSpot::insert_values(user_id.0, spot_id, db_conn.deref());
        Spot::select(user_id.into(), spot_id, db_conn.deref())
    } else {
        // if spot id is 1 - create test spot
        NewSpot::create_test_spot(OwnerType::User, user_id.into(), &location, db_conn.deref())
    };

    Ok(Json(spot))
}

#[post(
    "/set-spot-organization/<spot_id>",
    format = "application/json",
    data = "<spot_org>"
)]
fn set_spot_organization(
    user_id: UserId,
    spot_id: u32,
    spot_org: Json<SpotOrg>,
    db_conn: DbConn,
) -> Result<Json<Spot>, Json<RespErrors>> {
    user_id.rocket_validate_spot_access(spot_id, db_conn.deref())?;
    spot_org.rocket_validate()?;

    spot_org.insert_or_update(user_id.0, spot_id, db_conn.deref());
    let spot = Spot::select(user_id.into(), spot_id, db_conn.deref());
    Ok(Json(spot))
}

#[post(
    "/set-spot-image/<spot_id>",
    format = "application/json",
    data = "<spot_img>"
)]
fn set_spot_image(
    user_id: UserId,
    spot_id: u32,
    spot_img: Json<Base64Image>,
    db_conn: DbConn,
) -> Result<Json<Spot>, Json<RespErrors>> {
    user_id.rocket_validate_spot_access(spot_id, db_conn.deref())?;

    let image_id: u32 = spot_img
        .into_inner()
        .save_image(db_conn.deref(), ImageSizeValidation::Vertical)
        .map_err(|err| Json(RespErrors::new(vec![("image".to_string(), err)])))?;

    Spot::update_image_id(spot_id, image_id, db_conn.deref());
    let spot = Spot::select(user_id.into(), spot_id, db_conn.deref());
    Ok(Json(spot))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        set_user_name,
        set_user_license_accepted,
        get_spots_nearby,
        set_new_spot,
        add_existing_spot,
        set_spot_organization,
        set_spot_image,
    ]
}
