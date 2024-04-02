BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS "users" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "username" TEXT NOT NULL UNIQUE,
    "password" TEXT,
    "enabled" INTEGER NOT NULL DEFAULT 1,
    "permissions" INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS "sessions" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "token" TEXT NOT NULL UNIQUE,
    "user_id" BLOB NOT NULL REFERENCES "users" ("id"),
    "timestamp" TEXT NOT NULL,
    "valid" INTEGER NOT NULL DEFAULT 1
);


CREATE TABLE IF NOT EXISTS "seasons" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "name" TEXT NOT NULL UNIQUE,
    "enabled" INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS "groups" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "name" TEXT NOT NULL UNIQUE,
    "enabled" INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS "group_season_participation" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "group_id" BLOB NOT NULL REFERENCES "groups" ("id"),
    "season_id" BLOB NOT NULL REFERENCES "seasons" ("id"),
    "enabled" INTEGER NOT NULL DEFAULT 1,
    UNIQUE("season_id", "group_id")
);

CREATE TABLE IF NOT EXISTS "competitions" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "name" TEXT NOT NULL UNIQUE,
    "enabled" INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS "season_competitions" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "season_id" BLOB NOT NULL REFERENCES "seasons" ("id"),
    "competition_id" BLOB NOT NULL REFERENCES "competitions" ("id"),
    "enabled" INTEGER NOT NULL DEFAULT 1,
    UNIQUE("season_id", "competition_id")
);

CREATE TABLE IF NOT EXISTS "teams" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "group_season_participation_id" BLOB NOT NULL REFERENCES "group_season_participation" ("id"),
    "name" TEXT NOT NULL,
    "enabled" INTEGER NOT NULL DEFAULT 1,
    UNIQUE("group_season_participation_id", "name")
);

CREATE TABLE IF NOT EXISTS "events" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "competition_id" BLOB NOT NULL REFERENCES "competitions" ("id"),
    "name" TEXT NOT NULL,
    "enabled" INTEGER NOT NULL DEFAULT 1,
    UNIQUE("competition_id", "name")
);

CREATE TABLE IF NOT EXISTS "season_competition_events" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "season_competition_id" BLOB NOT NULL REFERENCES "season_competitions" ("id"),
    "event_id" BLOB NOT NULL REFERENCES "events" ("id"),
    "enabled" INTEGER NOT NULL DEFAULT 1,
    "score_type" TEXT NOT NULL DEFAULT 'team',
    "score_config" TEXT NOT NULL,
    UNIQUE("season_competition_id", "event_id")
);

CREATE TABLE IF NOT EXISTS "group_scores" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "season_competition_event_id" BLOB NOT NULL REFERENCES "season_competition_events" ("id"),
    "group_id" BLOB NOT NULL REFERENCES "groups" ("id"),
    "score_data" TEXT NOT NULL,
    "timestamp" TEXT NOT NULL,
    "valid" INTEGER NOT NULL DEFAULT 1,
    "disqualified" INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS "team_scores" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "season_competition_event_id" BLOB NOT NULL REFERENCES "season_competition_events" ("id"),
    "team_id" BLOB NOT NULL REFERENCES "teams" ("id"),
    "score_data" TEXT NOT NULL,
    "timestamp" TEXT NOT NULL,
    "valid" INTEGER NOT NULL DEFAULT 1,
    "disqualified" INTEGER NOT NULL DEFAULT 0
);

COMMIT;
