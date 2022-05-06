-- Add migration script here
PRAGMA foreign_keys = ON;

-- Create managed devices table
CREATE TABLE managed_devices (
    id TEXT PRIMARY KEY NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    name TEXT NOT NULL,
    description TEXT NULL,
    agent_id TEXT NOT NULL REFERENCES agents(id),
    snmp_protocol_attributes TEXT NOT NULL
);

CREATE UNIQUE INDEX idx_managed_devices_on_id ON managed_devices(id);

CREATE TRIGGER [ManagedDevicesUpdateModifiedAt]  
    AFTER   
    UPDATE  
    ON managed_devices
    FOR EACH ROW   
    WHEN NEW.modified_at <= OLD.modified_at  
BEGIN  
    update managed_devices set modified_at=CURRENT_TIMESTAMP where id=OLD.id;  
END
