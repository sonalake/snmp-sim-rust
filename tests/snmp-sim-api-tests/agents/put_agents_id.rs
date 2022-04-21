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
                    let authserver_id = Uuid::new_v4();
                    let name = Uuid::new_v4().to_string();
                    let response = Client::new()
                        .put(format!("{}/agents/{}", app.address, authserver_id))
                        .json(&serde_json::json!(
                            {
                                "name": name,
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
                }

                async it "creates_the_object" {
                    let json: response::Agent = response.json().await.unwrap();
                    let updated_db = Entity::find_by_id(json.id.to_string())
                        .one(db_conn)
                        .await
                        .expect("Failed to find the inserted object")
                        .expect("No object is inserted in the database");
                    assert_eq!(updated_db.name, name);
                }
            }

            context "update_existing" {
                before {
                    let agent = create_agent(db_conn, &Uuid::new_v4(), &Uuid::new_v4().to_string())
                        .await
                        .unwrap()
                        .unwrap_created();
                    let name = Uuid::new_v4().to_string();

                    #[allow(unused)]
                    let response = Client::new()
                        .put(format!("{}/agents/{}", app.address, Uuid::from_str(&agent.id).unwrap()))
                        .json(&serde_json::json!(
                            {
                                "name": name,
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
                    assert_eq!(json.name, name);
                }

                async it "updates_the_object" {
                    let updated_db = Entity::find_by_id(agent.id)
                        .one(db_conn)
                        .await
                        .expect("Failed to find the inserted object")
                        .expect("No object is inserted in the database");
                    assert_eq!(updated_db.name, name);
                }
            }
        }
    }
}
