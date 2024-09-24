CREATE TABLE IF NOT EXISTS graph (
    source TEXT PRIMARY KEY,
    target REFERENCES graph,
    data TEXT NOT NULL DEFAULT '{}',
    CHECK(json_valid(data))
);

CREATE INDEX IF NOT EXISTS target ON graph(target);

