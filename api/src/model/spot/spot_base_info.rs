use diesel::{BoolExpressionMethods, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use geo::prelude::VincentyDistance;
use geo::{Coordinate, Point};

use crate::model::{Image, Spot, SpotAddress, SpotStatus, SupplierBounding};

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotBaseInfo {
    pub id: u32,
    pub address: SpotAddress,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
}

impl SpotBaseInfo {
    pub fn get_nearby_spots(
        coordinate: Coordinate<f32>,
        radius: f32,
        conn: &MysqlConnection,
    ) -> Vec<SpotBaseInfo> {
        use crate::schema::spot::{self, dsl};
        let radius_in_meters: f32 = 100_000. * radius;
        let supplier_bounding =
            SupplierBounding::new_from_spot_radius(coordinate.x, coordinate.y, radius);

        let current_point = Point::from(coordinate);
        // no need to join images as lots of selected spots will be cleaned up by distance
        let rows: Vec<(u32, SpotAddress, u32, u8, f32, f32)> = spot::table
            .filter(
                dsl::lng
                    .ge(supplier_bounding.lng_min)
                    .and(dsl::lng.le(supplier_bounding.lng_max))
                    .and(dsl::lat.ge(supplier_bounding.lng_min))
                    .and(dsl::lat.le(supplier_bounding.lat_max))
                    .and(dsl::image_id.ne(0u32))
                    .and(dsl::status.eq(SpotStatus::Active)),
            )
            .select((
                dsl::id,
                (dsl::address, dsl::spot_type, dsl::spot_name),
                dsl::image_id,
                dsl::status,
                dsl::lng,
                dsl::lat,
            ))
            .load(conn)
            .unwrap();

        let mut image_ids = Vec::new();

        let spots_with_images: Vec<(u32, SpotAddress, u32)> = {
            let mut distance_spots = Vec::<(f32, (u32, SpotAddress, u32))>::new();
            for (id, spot_address, image_id, _status, lng, lat) in rows {
                let distance = current_point
                    .vincenty_distance(&Point::new(lng, lat))
                    .unwrap_or_default();
                #[cfg(debug_assertions)]
                println!("spot id: {}, distance: {}", &id, &distance);
                if distance < radius_in_meters {
                    distance_spots.push((distance, (id, spot_address, image_id)));
                    image_ids.push(image_id);
                }
            }
            distance_spots.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            distance_spots.into_iter().map(|v| v.1).collect()
        };

        image_ids.sort_unstable();
        let hash_map_src = Image::get_urls_by_ids(&image_ids, conn);

        let mut res: Vec<SpotBaseInfo> = spots_with_images
            .into_iter()
            .map(|(id, address, image_id)| {
                // it is possible that image_id same for different spots,
                // even if it is not desirable behavior to have same image_id,
                // so we copy
                let image_url = hash_map_src
                    .get(&image_id)
                    .map(ToString::to_string)
                    .unwrap_or_default();
                SpotBaseInfo {
                    id,
                    address,
                    image_url,
                }
            })
            .collect();

        // Add test spot
        res.push(SpotBaseInfo::get_default_test_spot());

        res
    }

    pub fn select(spot_id: u32, conn: &MysqlConnection) -> Self {
        let Spot {
            id: spot_id,
            address,
            image_url,
            ..
        } = Spot::select(0, spot_id, conn);

        SpotBaseInfo {
            id: spot_id,
            address,
            image_url,
        }
    }

    pub fn get_default_test_spot() -> Self {
        SpotBaseInfo {
            id: 1,
            address: SpotAddress {
                address: "ул Неизвестная 0".into(),
                spot_type: "Киоск".into(),
                spot_name: "Тест".into(),
            },
            image_url: Image::default_test_spot_image(),
        }
    }
}
