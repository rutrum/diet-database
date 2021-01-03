-- This file should undo anything in `up.sql`
ALTER TABLE metric
ADD COLUMN weight FLOAT;
