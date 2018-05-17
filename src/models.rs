use chrono::naive::NaiveDate;

#[derive(Queryable)]
pub struct Statistic {
    pub guild_id: String,
    pub user_id: String,
    pub post_time: NaiveDate,
    pub messages: i32,
    pub words: i32,
}