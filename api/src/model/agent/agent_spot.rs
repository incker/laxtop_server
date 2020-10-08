use crate::model::{SpotBaseInfo, SpotUser};
use diesel::MysqlConnection;

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentSpot {
    pub spot: SpotBaseInfo,
    pub user: SpotUser,
}

impl AgentSpot {
    pub fn select(user_id: u32, spot_id: u32, conn: &MysqlConnection) -> Self {
        AgentSpot {
            spot: SpotBaseInfo::select(spot_id, conn),
            user: SpotUser::select(user_id, conn).unwrap_or_default(),
        }
    }
}
