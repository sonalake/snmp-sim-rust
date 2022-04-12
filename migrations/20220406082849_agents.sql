-- Add migration script here
CREATE TABLE agents (
    id BLOB PRIMARY KEY NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    name TEXT NOT NULL
);

CREATE UNIQUE INDEX idx_agents_on_id ON agents(id);

CREATE TRIGGER [UpdateModifiedAt]  
    AFTER   
    UPDATE  
    ON agents
    FOR EACH ROW   
    WHEN NEW.modified_at <= OLD.modified_at  
BEGIN  
    update agents set modified_at=CURRENT_TIMESTAMP where id=OLD.id;  
END  
