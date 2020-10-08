use chrono::NaiveDateTime;
use diesel::{
    update, BoolExpressionMethods, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl,
};

use crate::base::MaxCleaner;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileInfo {
    #[serde(rename = "licenseAccepted")]
    pub license_accepted: bool,
    pub name: String,
}

impl UserProfileInfo {
    pub fn select_by_user(user_id: u32, conn: &MysqlConnection) -> Self {
        use crate::schema::user::{self, dsl};
        let (name, license_accepted_date): (String, NaiveDateTime) = user::table
            .select((dsl::name, dsl::license_accepted))
            .filter(dsl::id.eq(user_id))
            .first(conn)
            .unwrap();

        // пока что и так сойдет
        let fresh_license_date = NaiveDateTime::from_timestamp(1_579_949_850, 0);

        UserProfileInfo {
            license_accepted: fresh_license_date < license_accepted_date,
            name,
        }
    }

    pub fn update_name_if_not_set(user_id: u32, user_name: &str, conn: &MysqlConnection) {
        use crate::schema::user::{self, dsl};
        let clean_name = MaxCleaner::default()
            .clean_all(user_name)
            .unwrap_or_else(|| user_name.to_string());
        // update only if in db name is "" (I think it is more secure)
        let target = dsl::id.eq(user_id).and(dsl::name.eq(""));
        update(user::table.filter(target))
            .set(dsl::name.eq(&clean_name))
            .execute(conn)
            .unwrap();
    }

    pub fn update_license_accepted(user_id: u32, accepted: bool, conn: &MysqlConnection) {
        use crate::schema::user::{self, dsl};
        if accepted {
            update(user::table.filter(dsl::id.eq(user_id)))
                .set(dsl::license_accepted.eq(diesel::dsl::now))
                .execute(conn)
                .unwrap();
        }
    }
}
