CREATE TABLE artists(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL UNIQUE,
    sort_name TEXT NOT NULL,
    disambiguation TEXT NOT NULL,
    created_at timestamptz NOT NULL
);
