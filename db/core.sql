CREATE TABLE IF NOT EXISTS events (
    event_id VARCHAR(255) PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS email_templates (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    content TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS email_events_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

INSERT INTO email_events_types (name) VALUES ('order_created');

CREATE TABLE IF NOT EXISTS email_events (
    id SERIAL PRIMARY KEY,
    event_type INTEGER NOT NULL,
    template_id INTEGER NOT NULL,
    FOREIGN KEY (event_type) REFERENCES email_events_types(id),
    FOREIGN KEY (template_id) REFERENCES email_templates(id)
);
