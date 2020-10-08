use diesel::{
    insert_into, update, BoolExpressionMethods, ExpressionMethods, MysqlConnection, QueryDsl,
    RunQueryDsl,
};

use crate::model::Unit;
use crate::schema::product;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "product"]
pub struct NewProduct {
    pub name: String,
    pub supplier_id: u32,
    pub key: String,
    pub unit: Unit,
    pub is_deleted: bool,
}

impl NewProduct {
    pub fn insert(&self, conn: &MysqlConnection) -> Result<usize, diesel::result::Error> {
        insert_into(product::table).values(self).execute(conn)
    }

    pub fn update(&self, conn: &MysqlConnection) -> Result<usize, diesel::result::Error> {
        use crate::schema::product::dsl;

        let target = dsl::name
            .eq(&self.name)
            .and(dsl::supplier_id.eq(self.supplier_id))
            .and(dsl::key.eq(&self.key));

        let query = update(product::table.filter(target))
            .set((dsl::unit.eq(self.unit), dsl::is_deleted.eq(self.is_deleted)));

        // let debug = debug_query::<Mysql, _>(&query);
        // println!("{:?}", debug);
        query.execute(conn)
    }

    pub fn insert_or_update(&self, conn: &MysqlConnection) {
        use diesel::result::DatabaseErrorKind;
        use diesel::result::Error::DatabaseError;

        match self.insert(conn) {
            Err(DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                self.update(conn).unwrap();
            }
            Err(error) => panic!("{:?}", error),
            _ => (),
        };
    }

    pub fn get_unit_by_name(name: &str, supplier_id: u32, conn: &MysqlConnection) -> Unit {
        use crate::schema::product::dsl;

        let target = dsl::name
            .eq(name)
            .and(dsl::supplier_id.eq(supplier_id))
            .and(dsl::key.eq(""));

        let unit: Unit = dsl::product
            .select(dsl::unit)
            .filter(target)
            .first(conn)
            .unwrap_or_default();

        // let debug = diesel::debug_query::<Mysql, _>(&query);
        // println!("{:?}", debug);

        unit
    }
}
