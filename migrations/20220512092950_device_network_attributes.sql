-- Add migration script here
DELETE FROM managed_devices;

ALTER TABLE managed_devices
ADD COLUMN snmp_host TEXT NOT NULL;

ALTER TABLE managed_devices
ADD COLUMN snmp_port INTEGER NOT NULL;
