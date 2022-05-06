use demonstrate::demonstrate;

demonstrate! {
    #[actix_rt::test]
    describe "post_agents" {
        use crate::helpers::spawn_app;
        use actix_web::http::StatusCode;
        use snmp_sim::data_access::entity::agents::{Entity, Column};
        use snmp_sim::routes::agents::response;
        use sea_orm::EntityTrait;
        use uuid_dev::Uuid;
        use sea_orm::entity::prelude::*;
        use std::str::FromStr;

        before {
            let app = spawn_app().await;
            let client = reqwest::Client::new();
        }

        context "generated_agent_name" {
            before {
                let name = Uuid::new_v4().to_string();
                let snmp_data_url = Uuid::new_v4().to_string();
            }

            context "seeded_database" {
                before {
                    #[allow(unused)]
                    let db_conn = app.db_conn.as_ref().unwrap();
                }

                context "create_agent" {
                    before {
                        let response = client
                            .post(format!("{}/agents", app.address))
                            .json(&serde_json::json!({
                                "name": name,
                                "snmp_data_url": snmp_data_url
                            }))
                            .send()
                            .await
                            .expect("Failed to execute request");
                    }

                    async it "returns_201" {
                        assert_eq!(response.status(), StatusCode::CREATED);
                    }

                    async it "returns_the_right_object" {
                        let agent: response::Agent = response.json().await.unwrap();
                        assert_eq!(agent.name, name);
                        assert_eq!(agent.snmp_data_url, snmp_data_url);
                    }

                    async it "creates_the_object_in_database" {
                        let agent: response::Agent = response.json().await.unwrap();
                        let db_conn = app.db_conn.as_ref().unwrap();
                        let db_obj = Entity::find()
                            .filter(Column::Name.eq(agent.name))
                            .one(db_conn)
                            .await
                            .expect("Failed to find the inserted object")
                            .expect("No object is inserted in the database");

                        assert_eq!(Uuid::from_str(&db_obj.id).unwrap(), agent.id);
                    }
                }
            }

            context "empty_database" {
                context "create_agent" {
                    before {
                        #[allow(unused_variables)]
                        let response = client
                            .post(format!("{}/agents", app.address))
                            .json(&serde_json::json!({
                                "name": name,
                                "snmp_data_url": snmp_data_url
                            }))
                            .send()
                            .await
                            .expect("Failed to execute request");
                    }

                    async it "returns_201" {
                        assert_eq!(response.status(), StatusCode::CREATED);
                    }

                    async it "returns_the_right_object" {
                        let agent: response::Agent = response.json().await.unwrap();
                        assert_eq!(agent.name, name);
                        assert_eq!(agent.snmp_data_url, snmp_data_url);
                    }

                    async it "creates_the_object_in_database" {
                        let agent: response::Agent = response.json().await.unwrap();
                        let db_conn = app.db_conn.as_ref().unwrap();
                        let db_obj = Entity::find()
                            .filter(Column::Name.eq(agent.name))
                            .one(db_conn)
                            .await
                            .expect("Failed to find the inserted object")
                            .expect("No object is inserted in the database");

                        assert_eq!(Uuid::from_str(&db_obj.id).unwrap(), agent.id);
                    }
                }
            }
        }
    }
}
