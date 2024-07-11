DROP TABLE IF EXISTS products;

CREATE TABLE products (
	id SERIAL PRIMARY KEY,
    sku_number TEXT NOT NULL,
    type TEXT NOT NULL,
    manufacturer TEXT NOT NULL,
    model TEXT,
    dce_serial_number TEXT NOT NULL,
    width FLOAT NOT NULL,
    height FLOAT NOT NULL,
    depth FLOAT NOT NULL,
    UNIQUE (sku_number, manufacturer, model)
);