DROP TABLE IF EXISTS ddfs;

CREATE TABLE ddfs (
	id TEXT PRIMARY KEY DEFAULT gen_random_uuid(),
	type TEXT NOT NULL,
	sku_number TEXT,
	manufacturer TEXT NOT NULL,
	model TEXT,
	dce_serial TEXT NOT NULL,
	UNIQUE NULLS NOT DISTINCT (sku_number, manufacturer, model)
);