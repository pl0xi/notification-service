CREATE TABLE IF NOT EXISTS events (
    event_id VARCHAR(255) PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS templates (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    content TEXT NOT NULL
);


INSERT INTO templates (name, content) VALUES (
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

INSERT INTO templates (name, content) VALUES (
    'order_cancelled_example',
    '<!DOCTYPE html>
    <html>
    <body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
        <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
            <h1 style="color: #2c3e50;">Your order has been cancelled</h1>
        </div>

        <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
            {{> signature}}
        </div>
    </body>
    </html>'
);

INSERT INTO templates (name, content) VALUES (
    'order_fulfilled_example',
    '<!DOCTYPE html>
    <html>
    <body>
        <h1>Invoice</h1>
        <div>  
            {{ > invoice_details}}
        </div>

        <div>
            {{ > signature}}
        </div>
    </body>
    </html>'
);

INSERT INTO templates (name, content) VALUES (
    'invoice_example',
    '<html>
        <body>
            <div>
                <p>Invoice</p>
            </div>
        </body>
    </html>'
);

CREATE TABLE IF NOT EXISTS template_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

INSERT INTO template_types (name) VALUES ('order_created');
INSERT INTO template_types (name) VALUES ('order_cancelled');
INSERT INTO template_types (name) VALUES ('order_fulfilled');
INSERT INTO template_types (name) VALUES ('invoice');

CREATE TABLE IF NOT EXISTS active_templates (
    id SERIAL PRIMARY KEY,
    template_type_id INTEGER NOT NULL UNIQUE,
    template_id INTEGER NOT NULL,
    FOREIGN KEY (template_type_id) REFERENCES template_types(id),
    FOREIGN KEY (template_id) REFERENCES templates(id)
);

INSERT INTO active_templates (template_type_id, template_id) VALUES (1, 1);
INSERT INTO active_templates (template_type_id, template_id) VALUES (2, 2);
INSERT INTO active_templates (template_type_id, template_id) VALUES (3, 3);
INSERT INTO active_templates (template_type_id, template_id) VALUES (4, 4);

CREATE TABLE IF NOT EXISTS template_partials (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    content TEXT NOT NULL
);

INSERT INTO template_partials (name, content) VALUES (
    'signature',
    '<p>Sincerely **Shop_Name**</p>'
);

INSERT INTO template_partials (name, content) VALUES (
    'invoice_details',
    '<p>Invoice details</p>'
);
