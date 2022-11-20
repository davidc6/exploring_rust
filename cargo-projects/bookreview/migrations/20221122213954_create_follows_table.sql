-- migrations/{timestamps}_create_follows_table.sql
-- Create Follows Table
CREATE TABLE follows(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    followed_at timestamptz NOT NULL
);
