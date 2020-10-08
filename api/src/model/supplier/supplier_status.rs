use std::io;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::{Tinyint, Unsigned};
use serde::de::{self, Unexpected};
use serde::{Deserializer, Serializer};

#[derive(Debug, Copy, Clone, PartialEq, AsExpression, FromSqlRow)]
#[sql_type = "Unsigned<Tinyint>"]
pub enum SupplierStatus {
    // for now active is when telegram is connected (can receive invoices)
    Active,
    // lack telegram
    Pending,
    Deleted,
}

impl SupplierStatus {
    pub fn from_code(code: u8) -> Result<Self, String> {
        Ok(match code {
            1 => SupplierStatus::Active,
            2 => SupplierStatus::Pending,
            3 => SupplierStatus::Deleted,
            _ => return Err(format!("Unknown match in SpotStatus: {}", code)),
        })
    }

    pub fn code(self) -> u8 {
        match self {
            SupplierStatus::Active => 1,
            SupplierStatus::Pending => 2,
            SupplierStatus::Deleted => 3,
        }
    }
}

impl<DB: Backend> ToSql<Unsigned<Tinyint>, DB> for SupplierStatus
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

impl<DB: Backend> FromSql<Unsigned<Tinyint>, DB> for SupplierStatus
where
    u8: FromSql<Unsigned<Tinyint>, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let code = u8::from_sql(bytes)?;
        SupplierStatus::from_code(code).map_err(|e| e.into())
    }
}

impl serde::Serialize for SupplierStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.code())
    }
}

impl<'de> serde::Deserialize<'de> for SupplierStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let code: u8 = u8::deserialize(deserializer)?;
        SupplierStatus::from_code(code)
            .map_err(|_| de::Error::invalid_value(Unexpected::Unsigned(code as u64), &"0"))
    }
}

// Deserialize tutorial
// https://damad.be/joost/blog/rust-serde-deserialization-of-an-enum-variant.html
