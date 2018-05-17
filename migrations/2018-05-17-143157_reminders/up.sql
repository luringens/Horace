CREATE TABLE reminders (
  id          SERIAL PRIMARY KEY,
  user_id     CHAR(20) NOT NULL,
  remind_time DATE NOT NULL,
  message     VARCHAR(256)
);
