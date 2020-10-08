use std::borrow::Cow;

use crate::base::{Cleaner, MaxCleaner};
use crate::model::{OwnerType, User};
use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

#[derive(Debug, Queryable, PartialEq, Serialize, Deserialize, Default)]
pub struct SpotUser {
    pub phone: String,
    pub name: String,
}

impl SpotUser {
    pub fn clean(&mut self, cleaner: &MaxCleaner) {
        let phone_cleaner = Cleaner::new_phone_cleaner();

        if let Some(new_name) = cleaner.clean_all(&self.name) {
            self.name = new_name;
        }

        // handle that number have to start with + and be 7 digits len? (not urgent)
        if let Cow::Owned(new_number) = phone_cleaner.clean(&self.phone) {
            self.phone = new_number;
        }
    }

    pub fn insert(&self, creator_type: OwnerType, creator_id: u32, conn: &MysqlConnection) -> u32 {
        User::create_if_not_exist(&self.phone, &self.name, creator_type, creator_id, conn)
    }

    pub fn select(user_id: u32, conn: &MysqlConnection) -> Option<Self> {
        if user_id == 0 {
            None
        } else {
            use crate::schema::user::{self, dsl};
            user::table
                .filter(dsl::id.eq(user_id))
                .select((dsl::number, dsl::name))
                .first::<SpotUser>(conn)
                .ok()
        }
    }
}
