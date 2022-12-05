-- migrations/{timestamps}_create_books_table.sql
-- Create Books Table
CREATE TABLE books(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    author TEXT NOT NULL,
    title TEXT NOT NULL,
    added_at timestamptz NOT NULL
);
