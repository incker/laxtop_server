use std::io;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::{Tinyint, Unsigned};

#[derive(Debug, Copy, Clone, PartialEq, AsExpression, FromSqlRow)]
#[sql_type = "Unsigned<Tinyint>"]
pub enum OwnerType {
    User,
    Supplier,
    Admin,
    Agent,
    Telegram,
}

impl OwnerType {
    pub fn session_duration(self) -> u64 {
        match self {
            OwnerType::User => 60 * 86_400,
            OwnerType::Supplier => 14 * 86_400,
            OwnerType::Admin => 14 * 86_400,
            OwnerType::Agent => 14 * 86_400,
            OwnerType::Telegram => 60 * 30,
        }
    }
}

impl<DB: Backend> ToSql<Unsigned<Tinyint>, DB> for OwnerType
where
    u8: ToSql<Unsigned<Tinyint>, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: io::Write,
    {
        let v: u8 = match *self {
            OwnerType::User => 1,
            OwnerType::Supplier => 2,
            OwnerType::Admin => 3,
            OwnerType::Agent => 4,
            OwnerType::Telegram => 5,
        };
        v.to_sql(out)
    }
}

impl<DB: Backend> FromSql<Unsigned<Tinyint>, DB> for OwnerType
where
    u8: FromSql<Unsigned<Tinyint>, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let v = u8::from_sql(bytes)?;
        Ok(match v {
            1 => OwnerType::User,
            2 => OwnerType::Supplier,
            3 => OwnerType::Admin,
            4 => OwnerType::Agent,
            5 => OwnerType::Telegram,
            _ => return Err(format!("Unknown match in OwnerType: {}", v).into()),
        })
    }
}
