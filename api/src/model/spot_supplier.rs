use diesel::{
    delete, insert_or_ignore_into, BoolExpressionMethods, ExpressionMethods, JoinOnDsl,
    MysqlConnection, QueryDsl, RunQueryDsl,
};

use crate::schema::spot_supplier;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "spot_supplier"]
pub struct SpotSupplier {
    pub spot_id: u32,
    pub supplier_id: u32,
}

impl SpotSupplier {
    pub fn set_spot_suppliers(spot_id: u32, supplier_ids: &[u32], conn: &MysqlConnection) {
        use crate::schema::spot_supplier::dsl;
        // delete not in
        let target = spot_supplier::table.filter(
            dsl::spot_id
                .eq(spot_id)
                .and(dsl::supplier_id.ne_all(supplier_ids)),
        );
        let _res = delete(target)
            .execute(conn)
            .expect("Some fail here: SpotSupplier::set_spot_suppliers :(");

        // insert ignore
        let mut values = Vec::with_capacity(supplier_ids.len());
        for supplier_id in supplier_ids {
            values.push((dsl::spot_id.eq(spot_id), dsl::supplier_id.eq(supplier_id)));
        }

        let _res = insert_or_ignore_into(spot_supplier::table)
            .values(&values)
            .execute(conn);
    }

    pub fn set_supplier_spots(supplier_id: u32, spot_ids: &[u32], conn: &MysqlConnection) {
        use crate::schema::spot_supplier::dsl;
        // delete not in
        let target = dsl::supplier_id
            .eq(supplier_id)
            .and(dsl::spot_id.ne_all(spot_ids));
        let _res = delete(spot_supplier::table.filter(target))
            .execute(conn)
            .expect("Some fail here: SpotSupplier::set_supplier_spots :(");

        // insert ignore
        let mut values = Vec::with_capacity(spot_ids.len());
        for spot_id in spot_ids {
            values.push((dsl::spot_id.eq(spot_id), dsl::supplier_id.eq(supplier_id)));
        }

        let _res = insert_or_ignore_into(spot_supplier::table)
            .values(&values)
            .execute(conn)
            .expect("Some fail here: insert_or_ignore_into(spot_supplier::table)");
    }

    pub fn select_spot_suppliers(spot_id: u32, conn: &MysqlConnection) -> Vec<u32> {
        use crate::schema::spot_supplier::dsl;

        let supplier_ids: Vec<u32> = spot_supplier::table
            .filter(dsl::spot_id.eq(spot_id))
            .select(dsl::supplier_id)
            .load(conn)
            .expect("Some fail here: SpotSupplierSequence::select_supplier_ids_by_user_id :(");
        supplier_ids
    }

    pub fn validate_ligament_exist(
        spot_id: u32,
        supplier_id: u32,
        conn: &MysqlConnection,
    ) -> Result<(), (String, String)> {
        use crate::schema::spot_supplier::dsl;

        spot_supplier::table
            .filter(
                dsl::spot_id
                    .eq(spot_id)
                    .and(dsl::supplier_id.eq(supplier_id)),
            )
            .select(dsl::supplier_id)
            .first::<u32>(conn)
            .map(|_| ())
            .map_err(|err| {
                dbg!(err);
                (
                    "supplier_id".to_string(),
                    format!(
                        "Sorry, you are not belonging to supplier (spot_id: {}, supplier: {})",
                        spot_id, supplier_id
                    ),
                )
            })
    }

    pub fn validate_user_ligament_exist(
        user_id: u32,
        supplier_id: u32,
        conn: &MysqlConnection,
    ) -> Result<(), (String, String)> {
        use crate::schema::user_spot;

        user_spot::table
            .left_join(
                spot_supplier::table.on(user_spot::dsl::spot_id.eq(spot_supplier::dsl::spot_id)),
            )
            .filter(
                user_spot::dsl::user_id
                    .eq(user_id)
                    .and(spot_supplier::dsl::supplier_id.eq(supplier_id)),
            )
            .select(user_spot::dsl::user_id)
            .first::<u32>(conn)
            .map(|_| ())
            .map_err(|err| {
                dbg!(err);
                (
                    "supplierId".to_string(),
                    format!("supplierId {} is not in your coverage", supplier_id),
                )
            })
    }
}
