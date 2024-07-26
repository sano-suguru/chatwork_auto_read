use serde::Deserialize;

#[derive(Deserialize)]
pub struct ReadStatus {
    pub unread_num: i32,
    pub mention_num: i32,
}
