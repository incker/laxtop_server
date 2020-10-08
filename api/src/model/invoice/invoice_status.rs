use std::io;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::{Tinyint, Unsigned};
use serde::de::{self, Unexpected};
use serde::{Deserializer, Serializer};

#[derive(Debug, Copy, Clone, AsExpression, FromSqlRow)]
#[sql_type = "Unsigned<Tinyint>"]
pub enum InvoiceStatus {
    NotDelivered,
    Delivered,
}

impl InvoiceStatus {
    pub fn from_code(code: u8) -> Result<Self, String> {
        Ok(match code {
            1 => InvoiceStatus::NotDelivered,
            2 => InvoiceStatus::Delivered,
            _ => return Err(format!("Unknown match in InvoiceStatus: {}", code)),
        })
    }

    pub fn code(self) -> u8 {
        match self {
            InvoiceStatus::NotDelivered => 1,
            InvoiceStatus::Delivered => 2,
        }
    }
}

impl<DB: Backend> ToSql<Unsigned<Tinyint>, DB> for InvoiceStatus
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

impl<DB: Backend> FromSql<Unsigned<Tinyint>, DB> for InvoiceStatus
where
    u8: FromSql<Unsigned<Tinyint>, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let code = u8::from_sql(bytes)?;
        InvoiceStatus::from_code(code).map_err(|e| e.into())
    }
}

impl serde::Serialize for InvoiceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.code())
    }
}

impl<'de> serde::Deserialize<'de> for InvoiceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let code: u8 = u8::deserialize(deserializer)?;
        InvoiceStatus::from_code(code)
            .map_err(|_| de::Error::invalid_value(Unexpected::Unsigned(code as u64), &"0"))
    }
}

// Deserialize tutorial
// https://damad.be/joost/blog/rust-serde-deserialization-of-an-enum-variant.html
