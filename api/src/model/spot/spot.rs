use diesel::{
    update, BoolExpressionMethods, ExpressionMethods, JoinOnDsl, MysqlConnection, QueryDsl,
    RunQueryDsl,
};
use geo::algorithm::contains::Contains;
use geo::prelude::VincentyDistance;
use geo::{Coordinate, LineString, Point, Polygon};

use std::collections::HashMap;

use crate::model::{
    Image, ImageRouter, Location, OwnerType, SpotAddress, SpotOrg, SpotSupplier, UserSpot,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Spot {
    pub id: u32,
    pub address: SpotAddress,
    #[serde(rename = "spotOrg")]
    pub spot_org: SpotOrg,
    #[serde(rename = "imageId")]
    pub image_id: u32,
}

impl Spot {
    pub fn select_by_user(user_id: u32, conn: &MysqlConnection) -> Vec<Self> {
        use crate::schema::spot::{self, dsl};

        let spot_ids = UserSpot::select_by_user(user_id, conn);

        let rows: Vec<(u32, SpotAddress, u32)> = spot::table
            .select((
                dsl::id,
                (dsl::address, dsl::spot_type, dsl::spot_name),
                dsl::image_id,
            ))
            .filter(dsl::id.eq_any(&spot_ids))
            .load(conn)
            .expect("Error loading posts");

        let mut spot_org_hash_map: HashMap<u32, SpotOrg> =
            SpotOrg::select_by_ids(user_id, &spot_ids, conn);

        rows.into_iter()
            .map(|(id, spot_address, image_id)| Spot {
                id,
                address: spot_address,
                spot_org: spot_org_hash_map.remove(&id).unwrap_or_default(),
                image_id,
            })
            .collect()
    }

    pub fn select(user_id: u32, spot_id: u32, conn: &MysqlConnection) -> Self {
        use crate::schema::spot::{self, dsl};

        let (id, spot_address, image_id): (u32, SpotAddress, u32) = spot::table
            .select((
                dsl::id,
                (dsl::address, dsl::spot_type, dsl::spot_name),
                dsl::image_id,
            ))
            .filter(dsl::id.eq(spot_id))
            .first(conn)
            .expect("Error loading spot");

        Spot {
            id,
            address: spot_address,
            spot_org: SpotOrg::select(user_id, spot_id, conn),
            image_id,
        }
    }

    pub fn validate_distance(
        spot_id: u32,
        location: &Location,
        distance: f32,
        conn: &MysqlConnection,
    ) -> Result<(), (String, String)> {
        use crate::schema::spot::{self, dsl};

        let distance_in_meters: f32 = 100_000. * distance;
        let spot_location: (f32, f32) = spot::table
            .select((dsl::lng, dsl::lat))
            .filter(dsl::id.eq(spot_id))
            .first(conn)
            .map_err(|_| Spot::not_found(spot_id))?;

        let current_point = Point::new(location.lng, location.lat);
        let spot_point = Point::<f32>::from(spot_location);

        let real_distance = current_point.vincenty_distance(&spot_point).unwrap_or(0.0);

        if real_distance <= distance_in_meters {
            Ok(())
        } else {
            Err(("spotId".to_string(), "invalid distance".to_string()))
        }
    }

    pub fn validate_agent(
        agent_id: u32,
        spot_id: u32,
        conn: &MysqlConnection,
    ) -> Result<(), (String, String)> {
        use crate::schema::spot::{self, dsl};

        spot::table
            .select(dsl::id)
            .filter(
                dsl::id
                    .eq(spot_id)
                    .and(dsl::creator_type.eq(OwnerType::Agent))
                    .and(dsl::creator_id.eq(agent_id)),
            )
            .first::<u32>(conn)
            .map(|_| ())
            .map_err(|_| {
                (
                    "agent".to_string(),
                    format!(
                        "agent_id {} is not creator for spot_id {}",
                        agent_id, spot_id
                    ),
                )
            })
    }

    pub fn attach_to_suppliers(spot_id: u32, location: &Location, conn: &MysqlConnection) {
        let supplier_ids = Spot::force_count_covered_suppliers(location, conn);
        SpotSupplier::set_spot_suppliers(spot_id, &supplier_ids, conn);
    }

    fn force_count_covered_suppliers(location: &Location, conn: &MysqlConnection) -> Vec<u32> {
        use crate::schema::supplier;
        use crate::schema::supplier_polygon;

        let point: Point<f32> = Point::new(location.lng, location.lat);

        let mut covered_suppliers: Vec<u32> = Vec::new();

        let data: Vec<(u32, f32, f32)> = supplier_polygon::table
            .left_join(supplier::table.on(supplier::dsl::id.eq(supplier_polygon::dsl::supplier_id)))
            .filter(
                supplier::dsl::poly_lng_min
                    .le(location.lng)
                    .and(supplier::dsl::poly_lng_max.ge(location.lng))
                    .and(supplier::dsl::poly_lat_min.le(location.lat))
                    .and(supplier::dsl::poly_lat_max.ge(location.lat)),
            )
            .select((
                supplier_polygon::dsl::supplier_id,
                supplier_polygon::dsl::lng,
                supplier_polygon::dsl::lat,
            ))
            .load(conn)
            .unwrap();

        let mut hash_map: HashMap<u32, Vec<Coordinate<f32>>> = HashMap::new();

        for (supplier_id, lng, lat) in data {
            let coordinate = Coordinate { x: lng, y: lat };
            if let Some(vec) = hash_map.get_mut(&supplier_id) {
                vec.push(coordinate);
            } else {
                hash_map.insert(supplier_id, vec![coordinate]);
            };
        }

        for (supplier_id, points) in hash_map {
            let polygon = Polygon::new(LineString::from(points), vec![]);
            if polygon.contains(&point) {
                covered_suppliers.push(supplier_id);
            }
        }

        covered_suppliers.sort_unstable();
        covered_suppliers.dedup();

        covered_suppliers
    }

    pub fn update_image_id(spot_id: u32, image_id: u32, conn: &MysqlConnection) {
        use crate::schema::spot::{self, dsl};

        let _res = update(spot::table.filter(dsl::id.eq(spot_id)))
            .set(dsl::image_id.eq(image_id))
            .execute(conn)
            .expect("update_image_id fail");
    }

    pub fn not_found(spot_id: u32) -> (String, String) {
        (
            "spotId".to_string(),
            format!("spot_id {} not found", spot_id),
        )
    }
}
