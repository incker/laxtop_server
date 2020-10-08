use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use geo::Coordinate;
use geo_types::{LineString, Point, Polygon};
use rocket_contrib::json::Json;

use crate::base::RespErrors;
use crate::model::{SpotSupplier, SupplierBounding};

pub struct SupplierCoverage {
    pub supplier_id: u32,
    pub coverage: Vec<(f32, f32)>,
}

impl SupplierCoverage {
    pub fn select(supplier_id: u32, conn: &MysqlConnection) -> Self {
        let coverage = SupplierCoverage::select_coverage(supplier_id, conn);
        SupplierCoverage {
            supplier_id,
            coverage,
        }
    }

    pub fn select_coverage(supplier_id: u32, conn: &MysqlConnection) -> Vec<(f32, f32)> {
        use crate::schema::supplier_polygon::{self, dsl};
        let target = dsl::supplier_id.eq(supplier_id);

        supplier_polygon::table
            .filter(target)
            .select((dsl::lng, dsl::lat))
            .load::<(f32, f32)>(conn)
            .expect("Error loading dsl::supplier_polygon.filter")
    }

    pub fn delete_supplier_coverage(supplier_id: u32, conn: &MysqlConnection) {
        use crate::schema::supplier_polygon::{self, dsl};

        let target = dsl::supplier_id.eq(supplier_id);
        diesel::delete(supplier_polygon::table.filter(target))
            .execute(conn)
            .expect("Some fail here: SupplierCoverage::delete_supplier_poly");
    }

    pub fn delete(&self, conn: &MysqlConnection) {
        SupplierCoverage::delete_supplier_coverage(self.supplier_id, conn);
    }

    pub fn insert(&self, conn: &MysqlConnection) {
        use geo::algorithm::contains::Contains;
        self.insert_polygon(conn);

        let supplier_bounding = SupplierBounding::new_from_poly(&self.coverage);
        supplier_bounding.update(self.supplier_id, conn);
        let need_cover_validation_spots = supplier_bounding.select_spots(conn);

        let polygon = {
            let ls: LineString<f32> = self.coverage.iter().map(|e| Point::new(e.0, e.1)).collect();
            Polygon::new(ls, vec![])
        };

        let covered_spots = {
            let mut covered_spots = Vec::new();

            for (spot_id, location) in need_cover_validation_spots {
                let coordinate: Coordinate<f32> = location.into();
                let point = Point::<f32>::from(coordinate);
                if polygon.contains(&point) {
                    covered_spots.push(spot_id);
                }
            }

            covered_spots
        };

        SpotSupplier::set_supplier_spots(self.supplier_id, &covered_spots, conn);
    }

    fn insert_polygon(&self, conn: &MysqlConnection) {
        use crate::schema::supplier_polygon::{self, dsl};

        self.delete(conn);

        let values = {
            let mut sequence: u16 = 1;
            let mut values = Vec::with_capacity(self.coverage.len() as usize);
            for (lng, lat) in &self.coverage {
                values.push((
                    dsl::supplier_id.eq(self.supplier_id),
                    dsl::lng.eq(lng),
                    dsl::lat.eq(lat),
                    dsl::sequence.eq(sequence),
                ));
                sequence += 1;
            }
            values
        };

        if !values.is_empty() {
            let _res = diesel::insert_or_ignore_into(supplier_polygon::table)
                .values(&values)
                .execute(conn)
                .expect("could not insert SupplierCoverage");
        }
    }

    pub fn rocket_validate(&self) -> Result<(), Json<RespErrors>> {
        let build_error: fn(String) -> Result<(), Json<RespErrors>> =
            |err: String| Err(Json(RespErrors::new_error(("coverage".to_string(), err))));

        if self.coverage.len() < 3 {
            return build_error(format!(
                "too few points provided to build polygon: {}",
                self.coverage.len()
            ));
        }

        for (lng, lat) in &self.coverage {
            if *lng < -180. || *lng > 180. {
                return build_error(format!("invalid longitude: {}", lng));
            }
            if *lat < -90. || *lat > 90. {
                return build_error(format!("invalid latitude: {}", lat));
            }
        }
        Ok(())
    }
}
