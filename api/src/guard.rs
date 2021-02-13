use std::ops::Deref;

use diesel::MysqlConnection;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request};
use rocket_contrib::json::Json;

use crate::base::RespErrors;
use crate::model::{OwnerType, RespApiKey, Session, Spot, SpotSupplier, UserSpot};

#[database("laxtop_db")]
pub struct DbConn(MysqlConnection);

pub struct ApiKey<'a>(pub &'a str);

impl<'a> ApiKey<'a> {
    pub fn logout(&self, conn: &MysqlConnection) {
        Session::delete_session(self.0, conn)
    }

    pub fn security_logout(request: &Request) {
        let db_conn: DbConn = request.guard::<DbConn>().unwrap();
        if let Outcome::Success(api_key) = request.guard::<ApiKey>() {
            api_key.logout(db_conn.deref());
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let mut keys: Vec<_> = request.headers().get("x-api-key").collect();
        match keys.pop() {
            Some(k) => Outcome::Success(ApiKey(k)),
            None => Outcome::Failure((Status::from_code(401).unwrap(), ())),
        }
    }
}

pub struct SupplierId(pub u32);

impl<'a, 'r> FromRequest<'a, 'r> for SupplierId {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.guard::<ApiKeyLogin>()? {
            ApiKeyLogin::Supplier(supplier_id) => Outcome::Success(supplier_id),
            _ => {
                ApiKey::security_logout(request);
                Outcome::Failure((Status::from_code(401).unwrap(), ()))
            }
        }
    }
}

impl Into<u32> for SupplierId {
    fn into(self) -> u32 {
        self.0
    }
}

pub struct UserId(pub u32);

impl UserId {
    pub fn rocket_validate_spot_access(
        &self,
        spot_id: u32,
        conn: &MysqlConnection,
    ) -> Result<(), Json<RespErrors>> {
        UserSpot::validate_ligament(self.0, spot_id, conn)
            .map_err(|error| Json(RespErrors::new_error(error)))
    }

    pub fn rocket_validate_supplier_access(
        &self,
        supplier_id: u32,
        conn: &MysqlConnection,
    ) -> Result<(), Json<RespErrors>> {
        SpotSupplier::validate_user_ligament_exist(self.0, supplier_id, conn)
            .map_err(|error| Json(RespErrors::new_error(error)))
    }
}

impl Into<u32> for UserId {
    fn into(self) -> u32 {
        self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserId {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.guard::<ApiKeyLogin>()? {
            ApiKeyLogin::User(user_id) => Outcome::Success(user_id),
            _ => {
                ApiKey::security_logout(request);
                Outcome::Failure((Status::from_code(401).unwrap(), ()))
            }
        }
    }
}

pub struct AdminId(pub u32);

impl<'a, 'r> FromRequest<'a, 'r> for AdminId {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.guard::<ApiKeyLogin>()? {
            ApiKeyLogin::Admin(admin_id) => Outcome::Success(admin_id),
            _ => {
                ApiKey::security_logout(request);
                Outcome::Failure((Status::from_code(401).unwrap(), ()))
            }
        }
    }
}

pub struct AgentId(pub u32);

impl AgentId {
    pub fn rocket_validate_spot_access(
        &self,
        spot_id: u32,
        conn: &MysqlConnection,
    ) -> Result<(), Json<RespErrors>> {
        Spot::validate_agent(self.0, spot_id, conn)
            .map_err(|error| Json(RespErrors::new_error(error)))
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AgentId {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.guard::<ApiKeyLogin>()? {
            ApiKeyLogin::Agent(agent_id) => Outcome::Success(agent_id),
            _ => {
                ApiKey::security_logout(request);
                Outcome::Failure((Status::from_code(401).unwrap(), ()))
            }
        }
    }
}

impl Into<u32> for AgentId {
    fn into(self) -> u32 {
        self.0
    }
}

pub enum ApiKeyLogin {
    User(UserId),
    Supplier(SupplierId),
    Admin(AdminId),
    Agent(AgentId),
}

impl ApiKeyLogin {
    pub fn new(api_key: &str, conn: &MysqlConnection) -> Option<Self> {
        Session::login_from_session(api_key, conn)
    }

    pub fn owner_type(&self) -> OwnerType {
        match self {
            ApiKeyLogin::User(_) => OwnerType::User,
            ApiKeyLogin::Supplier(_) => OwnerType::Supplier,
            ApiKeyLogin::Admin(_) => OwnerType::Admin,
            ApiKeyLogin::Agent(_) => OwnerType::Agent,
        }
    }

    pub fn owner_id(&self) -> u32 {
        match self {
            ApiKeyLogin::User(UserId(id)) => *id,
            ApiKeyLogin::Supplier(SupplierId(id)) => *id,
            ApiKeyLogin::Admin(AdminId(id)) => *id,
            ApiKeyLogin::Agent(AgentId(id)) => *id,
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKeyLogin {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let api_key: ApiKey = request.guard::<ApiKey>()?;
        let db_conn: DbConn = request.guard::<DbConn>()?;

        match ApiKeyLogin::new(api_key.0, db_conn.deref()) {
            Some(api_key_login) => Outcome::Success(api_key_login),
            None => Outcome::Failure((Status::from_code(401).unwrap(), ())),
        }
    }
}

pub struct Logout(RespApiKey);

impl Logout {
    pub fn json(self) -> Json<RespApiKey> {
        Json(RespApiKey::default())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Logout {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        ApiKey::security_logout(request);
        Outcome::Success(Logout(RespApiKey::default()))
    }
}
