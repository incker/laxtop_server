use crate::schema::session;
use crate::schema::supplier;

joinable!(session -> supplier (owner_id));
