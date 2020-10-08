use std::io;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::{Tinyint, Unsigned};
use serde::{Deserializer, Serializer};

#[derive(Debug, Copy, Clone, AsExpression, FromSqlRow, Eq, PartialEq)]
#[sql_type = "Unsigned<Tinyint>"]
pub enum Unit {
    Unknown,
    Piece,
    Kilogram,
    Liter,
}

impl Unit {
    pub fn from_code(code: u8) -> Self {
        match code {
            1 => Unit::Piece,
            2 => Unit::Kilogram,
            3 => Unit::Liter,
            _ => Unit::Unknown,
        }
    }

    pub fn code(self) -> u8 {
        match self {
            Unit::Unknown => 0,
            Unit::Piece => 1,
            Unit::Kilogram => 2,
            Unit::Liter => 3,
        }
    }
}

impl serde::Serialize for Unit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.code())
    }
}

impl<'de> serde::Deserialize<'de> for Unit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let unit_code: u8 = u8::deserialize(deserializer)?;
        Ok(Unit::from_code(unit_code))
    }
}

impl<DB: Backend> ToSql<Unsigned<Tinyint>, DB> for Unit
where
    u8: ToSql<Unsigned<Tinyint>, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: io::Write,
    {
        self.code().to_sql(out)
    }
}

impl<DB: Backend> FromSql<Unsigned<Tinyint>, DB> for Unit
where
    u8: FromSql<Unsigned<Tinyint>, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let unit_code = u8::from_sql(bytes)?;
        Ok(Unit::from_code(unit_code))
    }
}

impl Default for Unit {
    fn default() -> Self {
        Unit::Unknown
    }
}
