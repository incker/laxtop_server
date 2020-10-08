use diesel::MysqlConnection;

use crate::model::{FirebaseUserInfo, OwnerType, Session, Spot, User, UserProfileInfo};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizedUserData {
    phone: String,
    token: String,
    #[serde(flatten)]
    user_info: UserProfileInfo,
    spots: Vec<Spot>,
}

impl AuthorizedUserData {
    pub fn new(firebase_user_info: FirebaseUserInfo, conn: &MysqlConnection) -> Self {
        let FirebaseUserInfo { uid: _, phone } = firebase_user_info;
        let user_id: u32 = User::create_if_not_exist(&phone, "", OwnerType::User, 0, conn);
        let session = Session::new_session_user(user_id, conn);

        AuthorizedUserData {
            phone,
            token: session.hash,
            user_info: UserProfileInfo::select_by_user(user_id, conn),
            spots: Spot::select_by_user(user_id, conn),
        }
    }
}
