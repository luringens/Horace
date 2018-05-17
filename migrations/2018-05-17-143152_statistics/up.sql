CREATE TABLE statistics (
  guild_id  CHAR(20),
  user_id   CHAR(20),
  post_time DATE,
  messages  INTEGER NOT NULL,
  words     INTEGER NOT NULL,
  PRIMARY KEY (guild_id, user_id, post_time)
);
