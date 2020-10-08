use std::ops::Deref;

use rocket_contrib::json::Json;

use crate::base::RespErrors;
use crate::guard::{AgentId, DbConn};
use crate::model::{AgentSpot, NewAgentSpot, OwnerType};

// Agent can not add_existing_spot
// Agent can not set-user-license-accepted
// Agent can not set_spot_organization

#[post("/set-new-spot", format = "application/json", data = "<new_spot>")]
fn set_new_spot(
    agent_id: AgentId,
    new_spot: Json<NewAgentSpot>,
    db_conn: DbConn,
) -> Result<Json<AgentSpot>, Json<RespErrors>> {
    let new_agent_spot = new_spot.into_inner().clean();

    new_agent_spot.spot.rocket_validate()?;
    new_agent_spot
        .insert(OwnerType::Agent, agent_id.into(), db_conn.deref())
        .map_err(|err| Json(RespErrors::new_error(err)))
        .map(Json)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![set_new_spot,]
}
