use chrono::{NaiveDate, NaiveDateTime};
use env;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use serenity::model::guild;
use serenity::model::id::{GuildId, UserId};
use std::str::FromStr;
use typemap::Key;

use command_error::CommandError;

#[derive(Clone)]
pub struct ConnectionPool {
    pub pool: Pool<PostgresConnectionManager>,
}

impl ConnectionPool {
    pub fn new() -> ConnectionPool {
        let connstring = env::var("POSTGRES_CONNSTRING")
            .expect("Expected a PostgreSQL connection string in the environment");
        let manager = PostgresConnectionManager::new(connstring, TlsMode::None)
            .expect("Failed to set up Postgres connection manager.");
        let pool = Pool::new(manager).expect("Failed to set up R2D2 connection pool.");

        ConnectionPool { pool }
    }

    pub fn get_conn(&mut self) -> PooledConnection<PostgresConnectionManager> {
        self.pool.get().expect("Failed to get postgres connection.")
    }

    pub fn update_statistics(
        &mut self,
        guild_id: GuildId,
        user_id: UserId,
        date: NaiveDate,
        messages: i32,
        words: i32,
    ) -> Result<(), CommandError> {
        self.get_conn().execute(
            "INSERT INTO statistics (guild_id, user_id, date, messages, words)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT(guild_id, user_id, date) DO UPDATE SET
                messages = statistics.messages + $4,
                words = statistics.words + $5",
            &[
                &format!("{}", guild_id.0),
                &format!("{}", user_id.0),
                &date,
                &messages,
                &words,
            ],
        )?;

        Ok(())
    }

    pub fn get_statistics(
        &mut self,
        guild_id: GuildId,
        days: u32,
    ) -> Result<Vec<Statistics>, CommandError> {
        // Don't reply to PM's as the command is only valid for guilds.
        let rows = self.get_conn().query(
            "SELECT user_id, SUM(messages) as messages, SUM(words) as words FROM statistics
             WHERE guild_id = $1 AND date > current_date - CAST ($2 AS INTEGER)
             GROUP BY user_id
             ORDER BY words DESC
             fetch first 10 rows only",
            &[&format!("{}", guild_id.0), &days],
        )?;

        let mut results = vec![];
        for row in rows.into_iter() {
            let user_id: String = row.get(0);
            let user_id = match u64::from_str(&user_id) {
                Ok(id) => id,
                Err(_) => {
                    error!("Failed to parse id {} to int.", user_id);
                    continue;
                }
            };

            let user = match guild_id.member(user_id) {
                Ok(user) => user,
                Err(_) => {
                    error!("Failed to look up member from id {}.", user_id);
                    continue;
                }
            };

            let messages: i64 = row.get(1);
            let chars: i64 = row.get(2);
            results.push(Statistics {
                user_id: user,
                messages: messages,
                words: chars,
            });
        }
        results.sort_by(|a, b| b.words.cmp(&a.words));

        Ok(results)
    }

    pub fn add_reminder(
        &mut self,
        user_id: &UserId,
        date: NaiveDateTime,
        message: &str,
        bookmark: &str,
    ) -> Result<(), CommandError> {
        self.get_conn().execute(
            "INSERT INTO reminders (user_id, date, message, bookmark) VALUES ($1, $2, $3, $4)",
            &[&format!("{}", user_id.0), &date, &message, &bookmark],
        )?;

        Ok(())
    }

    pub fn get_expired_reminders(&mut self) -> Result<Vec<Reminder>, CommandError> {
        let rows = self.get_conn().query(
            "SELECT id, user_id, message FROM reminders WHERE date < current_timestamp",
            &[],
        )?;

        let result: Vec<Reminder> = rows.into_iter()
            .map(|row| Reminder {
                id: row.get(0),
                user_id: row.get(1),
                message: row.get(2),
                bookmark: row.get(3),
            })
            .collect();

        Ok(result)
    }

    pub fn delete_reminder(&mut self, id: i32) -> Result<(), CommandError> {
        self.get_conn()
            .execute("DELETE FROM reminders WHERE id = $1", &[&id])?;
        Ok(())
    }
}

impl Default for ConnectionPool {
    fn default() -> Self {
        ConnectionPool::new()
    }
}

impl Key for ConnectionPool {
    type Value = ConnectionPool;
}

#[derive(Debug)]
pub struct Statistics {
    pub user_id: guild::Member,
    pub messages: i64,
    pub words: i64,
}

#[derive(Debug)]
pub struct Reminder {
    pub id: i32,
    pub user_id: String,
    pub message: String,
    pub bookmark: String,
}
