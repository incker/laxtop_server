use diesel::{BoolExpressionMethods, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

use crate::guard::{AdminId, AgentId, SupplierId};
use crate::model::{OwnerType, RespApiKey, Session};

#[derive(Deserialize)]
pub struct SignInFields {
    lg: String,
    ps: String,
}

impl SignInFields {
    pub fn supplier_login(
        &self,
        conn: &MysqlConnection,
    ) -> Result<(SupplierId, RespApiKey), (String, String)> {
        self.login(OwnerType::Supplier, conn)
            .map(|(id, resp_api_key)| (SupplierId(id), resp_api_key))
    }

    pub fn agent_login(
        &self,
        conn: &MysqlConnection,
    ) -> Result<(AgentId, RespApiKey), (String, String)> {
        self.login(OwnerType::Agent, conn)
            .map(|(id, resp_api_key)| (AgentId(id), resp_api_key))
    }

    pub fn admin_login(
        &self,
        conn: &MysqlConnection,
    ) -> Result<(AdminId, RespApiKey), (String, String)> {
        self.login(OwnerType::Admin, conn)
            .map(|(id, resp_api_key)| (AdminId(id), resp_api_key))
    }

    fn login(
        &self,
        owner_type: OwnerType,
        conn: &MysqlConnection,
    ) -> Result<(u32, RespApiKey), (String, String)> {
        use crate::schema::login::{self, dsl};
        login::table
            .select(dsl::owner_id)
            .filter(
                dsl::lg
                    .eq(&self.lg)
                    .and(dsl::ps.eq(&self.ps))
                    .and(dsl::owner_type.eq(owner_type)),
            )
            .first::<u32>(conn)
            .map(|id| {
                let session = Session::new(id, owner_type, conn);
                (id, RespApiKey::new_logged_in(session.hash))
            })
            .map_err(|_| ("login".into(), "Can not login".into()))
    }
}
