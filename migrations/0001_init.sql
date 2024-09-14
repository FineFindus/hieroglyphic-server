CREATE TABLE IF NOT EXISTS labels (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS samples (
    id SERIAL PRIMARY KEY,
    label_id INT REFERENCES labels(id) ON DELETE CASCADE,
  -- strokes array, where each stroke is an array of points as json
    strokes JSONB NOT NULL
);
