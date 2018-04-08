CREATE TABLE statistics (
    guild_id VARCHAR(20) NOT NULL,
    user_id VARCHAR(20) NOT NULL,
    date DATE NOT NULL,
    messages INTEGER NOT NULL,
    words INTEGER NOT NULL,
    PRIMARY KEY (guild_id, user_id, date)
);

CREATE TABLE reminders (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(20) NOT NULL,
    date TIMESTAMP NOT NULL,
    message VARCHAR(150)
);