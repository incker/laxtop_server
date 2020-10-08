use diesel::{
    insert_or_ignore_into, update, BoolExpressionMethods, ExpressionMethods, JoinOnDsl,
    MysqlConnection, QueryDsl, RunQueryDsl,
};

use crate::model::{SpotSupplier, SupplierStatus};

/*
Данные записываются в таблицу supplier_sequence только только когда юзер сделал ручную сортировку (ф-ция set_sequence)
Выбираются данные из supplier_sequence только джоинами для того чтоб получить инфо о том что пользователь сортировал (если сортировал)
Из таблицы supplier_sequence никогда ничего не удаляется
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotSupplierSequence {
    pub id: u32,
    pub sequence: Vec<u32>,
}

impl SpotSupplierSequence {
    pub fn select(user_id: u32, spot_id: u32, conn: &MysqlConnection) -> Self {
        SpotSupplierSequence {
            id: spot_id,
            sequence: SpotSupplierSequence::get_sequence(user_id, spot_id, conn),
        }
    }

    pub fn get_sequence(user_id: u32, spot_id: u32, conn: &MysqlConnection) -> Vec<u32> {
        use crate::schema::{spot_supplier, supplier, supplier_sequence};

        // sort active suppliers:
        // new suppliers first (that have sequence is null)
        // than sorted suppliers by sequence

        // do not select inactive suppliers, but keep there sequence info anyway

        let supplier_ids_sequence: Vec<u32> = spot_supplier::table
            .left_join(
                supplier_sequence::table.on(spot_supplier::dsl::supplier_id
                    .eq(supplier_sequence::dsl::supplier_id)
                    .and(spot_supplier::dsl::spot_id.eq(supplier_sequence::dsl::spot_id))
                    .and(supplier_sequence::dsl::user_id.eq(user_id))),
            )
            .left_join(supplier::table.on(spot_supplier::dsl::supplier_id.eq(supplier::dsl::id)))
            .filter(
                spot_supplier::dsl::spot_id
                    .eq(spot_id)
                    .and(supplier::dsl::status.eq(SupplierStatus::Active)),
            )
            .order((
                supplier_sequence::dsl::sequence.is_not_null(),
                supplier_sequence::dsl::sequence.asc(),
            ))
            .select(spot_supplier::dsl::supplier_id)
            .load(conn)
            .unwrap();

        /*
        SELECT `spot_supplier`.`supplier_id`
        FROM ((`spot_supplier` LEFT OUTER JOIN `supplier_sequence` ON
                `spot_supplier`.`supplier_id` = `supplier_sequence`.`supplier_id` AND `spot_supplier`.`spot_id`
                = `supplier_sequence`.`spot_id` AND `supplier_sequence`.`user_id` = 1)
                 LEFT OUTER JOIN `supplier` ON `spot_supplier`.`supplier_id` = `supplier`.`id`)
        WHERE `spot_supplier`.`spot_id` = 1
          AND `supplier`.`status` = 1
        ORDER BY `supplier_sequence`.`sequence` IS NOT NULL, `supplier_sequence`.`sequence` ASC
        */

        supplier_ids_sequence
    }

    pub fn set_sequence(user_id: u32, spot_id: u32, supplier_ids: &[u32], conn: &MysqlConnection) {
        use crate::schema::supplier_sequence::{self, dsl};

        // to sync data form spot_supplier table to supplier_sequence table
        let _active_supplier_ids: Vec<u32> =
            SpotSupplierSequence::get_active_suppliers(user_id, spot_id, conn);

        let mut sequence: u16 = 1;

        for supplier_id in supplier_ids {
            let target = supplier_sequence::table.filter(
                dsl::user_id
                    .eq(user_id)
                    .and(dsl::supplier_id.eq(supplier_id)),
            );
            let _res = update(target).set(dsl::sequence.eq(sequence)).execute(conn);
            sequence += 1;
        }
    }

    fn get_active_suppliers(user_id: u32, spot_id: u32, conn: &MysqlConnection) -> Vec<u32> {
        use crate::schema::supplier_sequence::{self, dsl};

        let supplier_ids: Vec<u32> = SpotSupplier::select_spot_suppliers(spot_id, conn);

        // 2 insert ignore
        let mut values = Vec::with_capacity(supplier_ids.len());
        for supplier_id in &supplier_ids {
            values.push((
                dsl::user_id.eq(user_id),
                dsl::spot_id.eq(spot_id),
                dsl::supplier_id.eq(supplier_id),
            ));
        }

        let _res = insert_or_ignore_into(supplier_sequence::table)
            .values(&values)
            .execute(conn);

        supplier_ids
    }
}
