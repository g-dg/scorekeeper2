BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS "users" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "username" TEXT NOT NULL UNIQUE,
    "password" TEXT,
    "description" TEXT NOT NULL DEFAULT "",
    "enabled" INTEGER NOT NULL DEFAULT 1,
    "permissions" INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS "sessions" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "token" TEXT NOT NULL UNIQUE,
    "user_id" BLOB NOT NULL,
    "timestamp" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f+00:00')),
    "valid" INTEGER NOT NULL DEFAULT 1
);
CREATE INDEX IF NOT EXISTS "index__sessions__user_id" ON "sessions" ("user_id");


CREATE TABLE IF NOT EXISTS "score_calculators" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "name" TEXT NOT NULL UNIQUE,
    "description" TEXT NOT NULL DEFAULT "",
    "script" TEXT NOT NULL,
    "default_config" TEXT NOT NULL DEFAULT '{}',
    "supports_seasons" INTEGER NOT NULL DEFAULT 0,
    "supports_competitions" INTEGER NOT NULL DEFAULT 0,
    "supports_events" INTEGER NOT NULL DEFAULT 0,
    "score_fields" TEXT
);

CREATE TABLE IF NOT EXISTS "seasons" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "name" TEXT NOT NULL UNIQUE,
    "description" TEXT NOT NULL DEFAULT "",
    "score_calculator" BLOB REFERENCES "score_calculators" ("id"),
    "calculator_config" TEXT NOT NULL DEFAULT '{}',
    "enabled" INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS "competitions" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "name" TEXT NOT NULL UNIQUE,
    "description" TEXT NOT NULL DEFAULT "",
    "enabled" INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS "season_competitions" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "season_id" BLOB NOT NULL REFERENCES "seasons" ("id"),
    "competition_id" BLOB NOT NULL REFERENCES "competitions" ("id"),
    "description" TEXT NOT NULL DEFAULT "",
    "score_calculator" BLOB REFERENCES "score_calculators" ("id"),
    "calculator_config" TEXT NOT NULL DEFAULT '{}',
    "enabled" INTEGER NOT NULL DEFAULT 1,
    UNIQUE("season_id", "competition_id")
);

CREATE TABLE IF NOT EXISTS "events" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "competition_id" BLOB NOT NULL REFERENCES "competitions" ("id"),
    "name" TEXT NOT NULL,
    "description" TEXT NOT NULL DEFAULT "",
    "enabled" INTEGER NOT NULL DEFAULT 1,
    UNIQUE("competition_id", "name")
);

CREATE TABLE IF NOT EXISTS "competition_events" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "season_competition_id" BLOB NOT NULL REFERENCES "season_competitions" ("id"),
    "event_id" BLOB NOT NULL REFERENCES "events" ("id"),
    "description" TEXT NOT NULL DEFAULT "",
    "score_calculator" BLOB REFERENCES "score_calculators" ("id"),
    "calculator_config" TEXT NOT NULL DEFAULT '{}',
    "enabled" INTEGER NOT NULL DEFAULT 1,
    "score_type" TEXT NOT NULL DEFAULT 'team',
    UNIQUE("season_competition_id", "event_id")
);

CREATE TABLE IF NOT EXISTS "groups" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "name" TEXT NOT NULL UNIQUE,
    "description" TEXT NOT NULL DEFAULT "",
    "enabled" INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS "group_participation" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "group_id" BLOB NOT NULL REFERENCES "groups" ("id"),
    "season_id" BLOB NOT NULL REFERENCES "seasons" ("id"),
    "description" TEXT NOT NULL DEFAULT "",
    "enabled" INTEGER NOT NULL DEFAULT 1,
    UNIQUE("season_id", "group_id")
);

CREATE TABLE IF NOT EXISTS "teams" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "group_participation_id" BLOB NOT NULL REFERENCES "group_participation" ("id"),
    "name" TEXT NOT NULL,
    "description" TEXT NOT NULL DEFAULT "",
    "enabled" INTEGER NOT NULL DEFAULT 1,
    UNIQUE("group_participation_id", "name")
);

CREATE TABLE IF NOT EXISTS "group_scores" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "competition_event_id" BLOB NOT NULL REFERENCES "competition_events" ("id"),
    "group_participation_id" BLOB NOT NULL REFERENCES "group_participation" ("id"),
    "score_data" TEXT NOT NULL DEFAULT '{}',
    "timestamp" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f+00:00')),
    "valid" INTEGER NOT NULL DEFAULT 1,
    "disqualified" INTEGER NOT NULL DEFAULT 0,
    "notes" TEXT
);
CREATE INDEX IF NOT EXISTS "index__group_scores__competition_event_id__group_participation_id" ON "group_scores" ("competition_event_id", "group_participation_id");

CREATE TABLE IF NOT EXISTS "team_scores" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "competition_event_id" BLOB NOT NULL REFERENCES "competition_events" ("id"),
    "team_id" BLOB NOT NULL REFERENCES "teams" ("id"),
    "score_data" TEXT NOT NULL DEFAULT '{}',
    "timestamp" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f+00:00')),
    "valid" INTEGER NOT NULL DEFAULT 1,
    "disqualified" INTEGER NOT NULL DEFAULT 0,
    "notes" TEXT
);
CREATE INDEX IF NOT EXISTS "index__team_scores__competition_event_id__team_id" ON "team_scores" ("competition_event_id", "team_id");


CREATE TABLE IF NOT EXISTS "log" (
    "id" BLOB PRIMARY KEY NOT NULL DEFAULT (randomblob(16)),
    "user_id" BLOB,
    "timestamp" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f+00:00')),
    "action" TEXT NOT NULL,
    "data" TEXT
);

COMMIT;
