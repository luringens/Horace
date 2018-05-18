use chrono::naive::NaiveDate;
use diesel;
use diesel::pg::upsert::excluded;
use super::schema::statistics;

#[derive(Insertable)]
#[table_name="statistics"]
pub struct NewStatistic<'a> {
    pub guild_id: &'a str,
    pub user_id: &'a str,
    pub post_time: NaiveDate,
    pub messages: i32,
    pub words: i32,
}

pub fn create_statistic<'a>(conn: &PgConnection, 
    new_guild_id: &'a str,
    new_user_id: &'a str,
    new_post_time: NaiveDate,
    new_messages: i32,
    new_words: i32,
    ) -> Statistic {
    use statistics::dsl::*;
    let new_statistic = NewStatistic {
        guild_id: new_guild_id,
        user_id: new_user_id,
        post_time: new_post_time,
        messages: new_messages,
        words: new_words,
    };

    diesel::insert_into(statistics::table)
        .values(&new_statistic)
        .on_conflict((guild_id, user_id, post_time))
        .do_update()
        .set(words.eq(words + newwords))
        .get_result(conn)
        .expect("Error saving new post")
}

#[derive(Queryable)]
pub struct Statistic {
    pub guild_id: String,
    pub user_id: String,
    pub post_time: NaiveDate,
    pub messages: i32,
    pub words: i32,
}