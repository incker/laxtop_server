use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

use crate::model::Location;
use crate::schema::country_phone_code;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "country_phone_code"]
pub struct CountryPhoneCode {
    pub country_code: String,
    pub phone_code: u32,
}

impl CountryPhoneCode {
    pub fn new_from_phone(phone: u64, conn: &MysqlConnection) -> Option<Self> {
        use crate::schema::country_phone_code::dsl;
        let possible_codes = CountryPhoneCode::get_phone_possible_codes(phone)?;
        let target = dsl::phone_code.eq_any(&possible_codes);

        dsl::country_phone_code // .select(dsl::country_code)
            .filter(target)
            .order(dsl::phone_code.desc())
            .first(conn)
            .ok()
    }

    pub fn get_phone_possible_codes(phone: u64) -> Option<[u32; 7]> {
        if phone < 1_000_000_000 {
            return None;
        }

        let possible_code: u32 = {
            let mut first_part = phone.to_owned();
            while first_part > 9_999_999 {
                first_part /= 10;
            }
            first_part as u32
        };

        Some([
            possible_code,
            possible_code / 10,
            possible_code / 100,
            possible_code / 1_000,
            possible_code / 10_000,
            possible_code / 100_000,
            possible_code / 1_000_000,
        ])
    }

    pub fn capital_location(&self, conn: &MysqlConnection) -> Option<Location> {
        Location::new_from_county_code(&self.country_code, conn)
    }
}
