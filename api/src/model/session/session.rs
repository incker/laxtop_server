use chrono::NaiveDateTime;
use diesel::{update, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

use std::iter::Iterator;

use crate::guard;
use crate::model::OwnerType;
use crate::schema::session;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "session"]
pub struct Session {
    pub hash: String,
    pub owner_type: OwnerType,
    pub owner_id: u32,
    pub expired_at: NaiveDateTime,
}

impl Session {
    pub fn new_session_user(id: u32, conn: &MysqlConnection) -> Self {
        Session::new(id, OwnerType::User, conn)
    }
    pub fn new_session_supplier(id: u32, conn: &MysqlConnection) -> Self {
        Session::new(id, OwnerType::Supplier, conn)
    }
    pub fn new_session_admin(id: u32, conn: &MysqlConnection) -> Self {
        Session::new(id, OwnerType::Admin, conn)
    }
    pub fn new_session_telegram(id: u32, conn: &MysqlConnection) -> Self {
        Session::new(id, OwnerType::Telegram, conn)
    }

    pub fn new(id: u32, owner_type: OwnerType, conn: &MysqlConnection) -> Self {
        let expired_timestamp = Session::get_timestamp() + owner_type.session_duration();

        let mut new_session = Session {
            hash: Session::generate_session(40),
            owner_type,
            owner_id: id.to_owned(),
            expired_at: NaiveDateTime::from_timestamp(expired_timestamp as i64, 0),
        };

        loop {
            let saved = diesel::insert_into(session::table)
                .values(&new_session)
                .execute(conn);

            match saved {
                Ok(_) => break,
                Err(_) => {
                    new_session.hash = Session::generate_session(40);
                }
            }
        }

        new_session
    }

    pub fn select_session(api_key: &str, conn: &MysqlConnection) -> Option<Self> {
        use crate::schema::session::dsl;
        let s: Session = session::table
            .filter(dsl::hash.eq(api_key))
            .first(conn)
            .ok()?;
        s.update_expired_if_needed(conn);
        Some(s)
    }

    pub fn login_from_session(api_key: &str, conn: &MysqlConnection) -> Option<guard::ApiKeyLogin> {
        let s: Session = Session::select_session(api_key, conn)?;
        match s.owner_type {
            OwnerType::User => Some(guard::ApiKeyLogin::User(guard::UserId(s.owner_id))),
            OwnerType::Supplier => {
                Some(guard::ApiKeyLogin::Supplier(guard::SupplierId(s.owner_id)))
            }
            OwnerType::Admin => Some(guard::ApiKeyLogin::Admin(guard::AdminId(s.owner_id))),
            OwnerType::Agent => Some(guard::ApiKeyLogin::Agent(guard::AgentId(s.owner_id))),
            OwnerType::Telegram => {
                Session::delete_session(api_key, conn);
                None
            }
        }
    }

    pub fn get_telegram_session(
        api_key: &str,
        conn: &MysqlConnection,
    ) -> Option<guard::SupplierId> {
        let s: Session = Session::select_session(api_key, conn)?;
        // telegram session not need live (foreign especially)
        Session::delete_session(api_key, conn);

        match s.owner_type {
            OwnerType::Telegram => Some(guard::SupplierId(s.owner_id)),
            _ => None,
        }
    }

    pub fn update_expired_if_needed(&self, conn: &MysqlConnection) {
        if let Some(new_expired_at) =
            Session::check_need_prolongation(&self.expired_at, self.owner_type.session_duration())
        {
            use crate::schema::session::dsl;
            let target = dsl::hash.eq(&self.hash);
            let _res = update(session::table.filter(target))
                .set(dsl::expired_at.eq(new_expired_at))
                .execute(conn);
        }
    }

    pub fn check_need_prolongation(
        current_expired: &NaiveDateTime,
        duration: u64,
    ) -> Option<NaiveDateTime> {
        let timestamp = Session::get_timestamp() as i64;
        let dur = duration as i64;
        let half_expired_passed = (current_expired.timestamp() - timestamp) < (dur / 2);
        if half_expired_passed {
            Some(NaiveDateTime::from_timestamp(timestamp + dur, 0))
        } else {
            None
        }
    }

    pub fn generate_session(session_len: usize) -> String {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};
        use std::iter;

        // code from here:
        // https://rust-random.github.io/book/update-0.8.html#distributions
        let mut rng = thread_rng();
        iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(session_len)
            .collect()
    }

    pub fn get_timestamp() -> u64 {
        use std::time::SystemTime;

        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            _ => panic!("SystemTime before UNIX EPOCH!"),
        }
    }

    pub fn delete_old_sessions(conn: &MysqlConnection) {
        use crate::schema::session::dsl;
        let current_date_time = NaiveDateTime::from_timestamp(Session::get_timestamp() as i64, 0);
        let _res = diesel::delete(dsl::session.filter(dsl::expired_at.lt(current_date_time)))
            .execute(conn);
    }

    pub fn delete_session(hash: &str, conn: &MysqlConnection) {
        use crate::schema::session::dsl;
        let _res = diesel::delete(dsl::session.filter(dsl::hash.eq(hash))).execute(conn);
    }
}
