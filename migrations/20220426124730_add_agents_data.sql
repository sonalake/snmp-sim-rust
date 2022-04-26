-- Add migration script here
DELETE FROM agents;

ALTER TABLE agents
ADD COLUMN snmp_data_url TEXT NOT NULL;

ALTER TABLE agents
ADD COLUMN description TEXT NULL;