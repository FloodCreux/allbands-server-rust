CREATE TABLE concerts (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    artist_id uuid NOT NULL,
    venue TEXT NOT NULL,
    city TEXT NOT NULL,
    state TEXT,
    date DATE NOT NULL,
    created_at TIMESTAMP NOT NULL,
    FOREIGN KEY (artist_id) REFERENCES artists (id)
);
