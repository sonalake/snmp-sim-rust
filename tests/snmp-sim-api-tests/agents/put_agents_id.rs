use demonstrate::demonstrate;

demonstrate! {
    #[actix_rt::test]
    describe "put_agents_id" {
        use crate::helpers::{spawn_app, seed_agents};
        use actix_web::http::StatusCode;
        use snmp_sim::data_access::entity::agents::Entity;
        use snmp_sim::data_access::helpers::*;
        use snmp_sim::routes::agents::response;
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

                seed_agents(db_conn, 5).await;
            }

            context "update_nonexisting" {

                before {
                    let agent_id = Uuid::new_v4();
                    let name = Uuid::new_v4().to_string();
                    let description = Some(Uuid::new_v4().to_string());
                    let snmp_data_url = Uuid::new_v4().to_string();
                    let response = Client::new()
                        .put(format!("{}/agents/{}", app.address, agent_id))
                        .json(&serde_json::json!(
                            {
                                "name": name,
                                "description": description,
                                "snmp_data_url": snmp_data_url
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
                    let json: response::Agent = response.json().await.unwrap();
                    assert_eq!(json.name, name);
                    assert_eq!(json.description, description);
                    assert_eq!(json.snmp_data_url, snmp_data_url);
                }

                async it "creates_the_object" {
                    let json: response::Agent = response.json().await.unwrap();
                    let updated_db = Entity::find_by_id(json.id.to_string())
                        .one(db_conn)
                        .await
                        .expect("Failed to find the inserted object")
                        .expect("No object is inserted in the database");
                    assert_eq!(updated_db.name, name);
                    assert_eq!(json.description, description);
                    assert_eq!(json.snmp_data_url, snmp_data_url);
                }
            }

            context "update_existing" {
                before {
                    let agent = create_agent(db_conn, &Uuid::new_v4(), &Uuid::new_v4().to_string(), &Some(Uuid::new_v4().to_string()), &Uuid::new_v4().to_string())
                        .await
                        .unwrap()
                        .unwrap_created();
                    let new_name = Uuid::new_v4().to_string();
                    let new_description: Option<String> = None;
                    let new_snmp_data_url = Uuid::new_v4().to_string();

                    #[allow(unused)]
                    let response = Client::new()
                        .put(format!("{}/agents/{}", app.address, Uuid::from_str(&agent.id).unwrap()))
                        .json(&serde_json::json!(
                            {
                                "name": new_name,
                                "description": new_description,
                                "snmp_data_url": new_snmp_data_url
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
                    let json: response::Agent = response.json().await.unwrap();
                    assert_eq!(json.id, Uuid::from_str(&agent.id).unwrap());
                    assert_eq!(json.name, new_name);
                    assert_eq!(json.description, new_description);
                    assert_eq!(json.snmp_data_url, new_snmp_data_url);
                }

                async it "updates_the_object" {
                    let updated_db = Entity::find_by_id(agent.id)
                        .one(db_conn)
                        .await
                        .expect("Failed to find the inserted object")
                        .expect("No object is inserted in the database");
                    assert_eq!(updated_db.name, new_name);
                    assert_eq!(updated_db.description, new_description);
                    assert_eq!(updated_db.snmp_data_url, new_snmp_data_url);
                }
            }
        }
    }
}
