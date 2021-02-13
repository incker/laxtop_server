use diesel::{
    update, BoolExpressionMethods, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl,
};

use crate::model::Location;

#[derive(Debug, Default)]
pub struct SupplierBounding {
    pub lng_min: f32,
    pub lng_max: f32,
    pub lat_min: f32,
    pub lat_max: f32,
}

impl SupplierBounding {
    pub fn new_from_poly(poly: &[(f32, f32)]) -> Self {
        let (mut lng_min, mut lng_max) = poly[0];
        let (mut lat_min, mut lat_max) = poly[0];

        for (lng, lat) in poly {
            lng_min = lng_min.min(*lng);
            lng_max = lng_max.max(*lng);
            lat_min = lat_min.min(*lat);
            lat_max = lat_max.max(*lat);
        }

        SupplierBounding {
            lng_min,
            lng_max,
            lat_min,
            lat_max,
        }
    }

    pub fn new_from_spot_radius(lng: f32, lat: f32, radius: f32) -> Self {
        SupplierBounding {
            lng_min: (lng - radius).min(180.),
            lng_max: (lng + radius).max(-180.),
            lat_min: (lat - radius).min(90.),
            lat_max: (lat + radius).max(-90.),
        }
    }

    pub fn delete(supplier_id: u32, conn: &MysqlConnection) {
        SupplierBounding::default().update(supplier_id, conn);
    }

    pub fn update(&self, supplier_id: u32, conn: &MysqlConnection) {
        use crate::schema::supplier::{self, dsl};

        update(supplier::table.filter(dsl::id.eq(supplier_id)))
            .set((
                dsl::poly_lng_min.eq(self.lng_min),
                dsl::poly_lng_max.eq(self.lng_max),
                dsl::poly_lat_min.eq(self.lat_min),
                dsl::poly_lat_max.eq(self.lat_max),
            ))
            .execute(conn)
            .unwrap();
    }

    pub fn select_spots(&self, conn: &MysqlConnection) -> Vec<(u32, Location)> {
        use crate::schema::spot::{self, dsl};
        spot::table
            .filter(
                dsl::lng
                    .ge(self.lng_min)
                    .and(dsl::lng.le(self.lng_max))
                    .and(dsl::lat.ge(self.lat_min))
                    .and(dsl::lat.le(self.lat_max)),
            )
            .select((dsl::id, (dsl::lng, dsl::lat)))
            .load(conn)
            .expect("Error loading select_spots")
    }
}
