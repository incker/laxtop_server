use diesel::{insert_into, update, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

use crate::model::{OwnerType, RespApiKey, Session, Supplier};
use crate::schema::telegram_user;

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name = "telegram_user"]
pub struct TelegramUser {
    pub id: i64,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub photo_url: String,
}

impl TelegramUser {
    pub fn insert(&self, conn: &MysqlConnection) -> Result<usize, diesel::result::Error> {
        insert_into(telegram_user::table).values(self).execute(conn)
    }

    pub fn update(&self, conn: &MysqlConnection) -> Result<usize, diesel::result::Error> {
        use crate::schema::telegram_user::dsl;

        let query = update(telegram_user::table.filter(dsl::id.eq(self.id))).set(self);

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

    pub fn login_as_supplier(&self, conn: &MysqlConnection) -> Option<(u32, RespApiKey)> {
        Supplier::get_supplier_id(self.id, conn).map(|supplier_id| {
            let session = Session::new(supplier_id, OwnerType::Supplier, conn);
            (supplier_id, RespApiKey::new_logged_in(session.hash))
        })
    }
}
