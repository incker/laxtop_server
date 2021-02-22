use std::io;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::{Integer, Unsigned};
use diesel::Queryable;

#[derive(Debug, Serialize, Deserialize, QueryId)]
pub struct SpotId(pub u32);

impl From<SpotId> for u32 {
    fn from(spot_id: SpotId) -> u32 {
        spot_id.0
    }
}

impl<DB: Backend> ToSql<Unsigned<Integer>, DB> for SpotId
where
    u32: ToSql<Unsigned<Integer>, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: io::Write,
    {
        self.0.to_sql(out)
    }
}

impl<DB: Backend> FromSql<Unsigned<Integer>, DB> for SpotId
where
    u32: FromSql<Unsigned<Integer>, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        u32::from_sql(bytes).map(SpotId)
    }
}

impl<DB, ST> Queryable<ST, DB> for SpotId
where
    DB: Backend,
    u32: Queryable<ST, DB>,
{
    type Row = <u32 as Queryable<ST, DB>>::Row;

    fn build(row: Self::Row) -> Self {
        SpotId(u32::build(row))
    }
}

/*
// проверил что правильно в json превращается SpotId(3) -> 3
#[get("/test")]
fn test() -> Json<SpotSupplierSequence> {
    Json(SpotSupplierSequence{
        id: SpotId(3),
        sequence: vec![]
    })
}
*/
