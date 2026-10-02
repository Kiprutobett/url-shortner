-- Add migration script here
CREATE TABLE url_clicks(
    id UUID PRIMARY KEY,
    url_id  UUID NOT NULL REFERENCES urls(id) ON DELETE CASCADE,
    clicked_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_url_clicks_url_id
ON url_clicks(url_id);

CREATE INDEX idx_url_clicks_clicked_at
ON url_clicks(clicked_at);
