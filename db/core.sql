CREATE TABLE IF NOT EXISTS events (
    event_id VARCHAR(255) PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS email_templates (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    content TEXT NOT NULL
);

INSERT INTO email_templates (name, content) VALUES (
    'order_created_example',
    '<!DOCTYPE html>
    <html>
    <body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
        <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
            <h1 style="color: #2c3e50;">Thank You for Your Order!</h1>
            
            <p>Dear {{customer.first_name}} {{customer.last_name}},</p>
            
            <p style="font-style: italic;">note: This is not an order confirmation</p>
        </div>

        <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
            {{> signature}}
        </div>
    </body>
    </html>'
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

INSERT INTO email_events (event_type, template_id) VALUES (1, 1);

CREATE TABLE IF NOT EXISTS email_template_partials (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    content TEXT NOT NULL
);

INSERT INTO email_template_partials (name, content) VALUES (
    'signature',
    '<p>Sincerely **Shop_Name**</p>'
);
