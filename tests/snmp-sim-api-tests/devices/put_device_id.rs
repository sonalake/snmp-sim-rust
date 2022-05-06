use demonstrate::demonstrate;

demonstrate! {
    #[actix_rt::test]
    describe "put_devices_id" {
        use crate::helpers::{spawn_app, seed_devices, route_snmp_v2c_attributes, route_snmp_v1_attributes, domain_snmp_v1_attributes_json, domain_snmp_v2c_attributes_json};
        use actix_web::http::StatusCode;
        use snmp_sim::data_access::entity::managed_devices::Entity;
        use snmp_sim::data_access::helpers::*;
        use snmp_sim::routes::managed_devices::response;
        use reqwest::Client;
        use sea_orm::EntityTrait;
        use uuid_dev::Uuid;
        use std::str::FromStr;

        before {
            let app = spawn_app().await;
        }

        context "seeded_database" {
            before {
                let db_conn = app.db_conn.as_ref().unwrap();

                let agent = create_agent(db_conn, &Uuid::new_v4(), &Uuid::new_v4().to_string(), &Some(Uuid::new_v4().to_string()), &Uuid::new_v4().to_string())
                    .await
                    .unwrap()
                    .unwrap_created();
                let agent_id = Uuid::from_str(&agent.id).unwrap();
                seed_devices(db_conn, &agent_id, 5).await;
            }

            context "update_nonexisting" {

                before {
                    let device_id = Uuid::new_v4();
                    let name = Uuid::new_v4().to_string();
                    let description = Some(Uuid::new_v4().to_string());
                    let response = Client::new()
                        .put(format!("{}/devices/{}", app.address, device_id))
                        .json(&serde_json::json!(
                            {
                                "name": name,
                                "description": description,
                                "agent_id": agent_id,
                                "snmp_protocol_attributes": route_snmp_v1_attributes("public"),
                            }
                        ))
                        .send()
                        .await
                        .unwrap();
                }

                async it "returns_201" {
                    assert_eq!(StatusCode::CREATED, response.status());
                }

                async it "returns_the_object" {
                    let json: response::Device = response.json().await.unwrap();
                    assert_eq!(json.name, name);
                    assert_eq!(json.description, description);
                    assert_eq!(json.agent.id, agent_id);
                    assert_eq!(json.description, description);
                    assert_eq!(json.snmp_protocol_attributes, route_snmp_v1_attributes("public"));
                }

                async it "creates_the_object" {
                    let json: response::Device = response.json().await.unwrap();
                    let updated_db = Entity::find_by_id(json.id.to_string())
                        .one(db_conn)
                        .await
                        .expect("Failed to find the inserted object")
                        .expect("No object is inserted in the database");
                    assert_eq!(updated_db.name, name);
                    assert_eq!(updated_db.description, description);
                    assert_eq!(updated_db.agent_id, agent_id.to_string());
                    assert_eq!(updated_db.description, description);
                    assert_eq!(updated_db.snmp_protocol_attributes, domain_snmp_v1_attributes_json("public"));
                }
            }

            context "update_existing" {
                before {
                    let (device, _agent) = create_managed_device(db_conn,
                        &Uuid::new_v4(),
                        &Uuid::new_v4().to_string(),
                        &Some(Uuid::new_v4().to_string()),
                        &agent_id,
                        &domain_snmp_v1_attributes_json("public"))
                        .await
                        .unwrap()
                        .unwrap_created();
                    let device_id = Uuid::from_str(&device.id).unwrap();
                    let new_name = Uuid::new_v4().to_string();
                    let new_description: Option<String> = None;

                    #[allow(unused)]
                    let response = Client::new()
                        .put(format!("{}/devices/{}", app.address, device_id))
                        .json(&serde_json::json!(
                            {
                                "name": new_name,
                                "description": new_description,
                                "agent_id": agent_id,
                                "snmp_protocol_attributes": route_snmp_v2c_attributes("public"),
                            }
                        ))
                        .send()
                        .await
                        .unwrap();
                }

                async it "returns_200" {
                    assert_eq!(StatusCode::OK, response.status());
                }

                async it "returns_updated_object" {
                    let json: response::Device = response.json().await.unwrap();
                    assert_eq!(json.id, device_id);
                    assert_eq!(json.name, new_name);
                    assert_eq!(json.description, new_description);
                    assert_eq!(json.agent.id, agent_id);
                    assert_eq!(json.snmp_protocol_attributes, route_snmp_v2c_attributes("public"));
                }

                async it "updates_the_object" {
                    let updated_db = Entity::find_by_id(device.id)
                        .one(db_conn)
                        .await
                        .expect("Failed to find the inserted object")
                        .expect("No object is inserted in the database");
                    assert_eq!(updated_db.name, new_name);
                    assert_eq!(updated_db.agent_id, agent_id.to_string());
                    assert_eq!(updated_db.description, new_description);
                    assert_eq!(updated_db.snmp_protocol_attributes, domain_snmp_v2c_attributes_json("public"));
                }
            }
        }
    }
}
