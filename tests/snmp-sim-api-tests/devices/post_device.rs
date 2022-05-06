use demonstrate::demonstrate;

demonstrate! {
    #[actix_rt::test]
    describe "post_devices" {
        use crate::helpers::{spawn_app, route_snmp_v1_attributes};
        use actix_web::http::StatusCode;
        use snmp_sim::data_access::entity::managed_devices::{Entity, Column};
        use snmp_sim::routes::managed_devices::response;
        use sea_orm::EntityTrait;
        use uuid_dev::Uuid;
        use sea_orm::entity::prelude::*;
        use std::str::FromStr;
        use snmp_sim::data_access::helpers::*;

        before {
            let app = spawn_app().await;
            let client = reqwest::Client::new();
            let db_conn = app.db_conn.as_ref().unwrap();
            let agent = create_agent(db_conn, &Uuid::new_v4(), &Uuid::new_v4().to_string(), &Some(Uuid::new_v4().to_string()), &Uuid::new_v4().to_string())
                .await
                .unwrap()
                .unwrap_created();
            let agent_id = Uuid::from_str(&agent.id).unwrap();
        }

        context "generated_device_name" {
            before {
                let name = Uuid::new_v4().to_string();
                let description = Some(Uuid::new_v4().to_string());
            }

            context "seeded_database" {
                before {
                    #[allow(unused)]
                    let db_conn = app.db_conn.as_ref().unwrap();
                }

                context "create_device" {
                    before {
                        let response = client
                            .post(format!("{}/devices", app.address))
                            .json(&serde_json::json!({
                                "name": name,
                                "description": description,
                                "agent_id": agent_id,
                                "snmp_protocol_attributes": route_snmp_v1_attributes("public"),
                            }))
                            .send()
                            .await
                            .expect("Failed to execute request");
                    }

                    async it "returns_201" {
                        assert_eq!(response.status(), StatusCode::CREATED);
                    }

                    async it "returns_the_right_object" {
                        let device: response::Device = response.json().await.unwrap();
                        assert_eq!(device.name, name);
                        assert_eq!(device.description, description);
                        assert_eq!(device.agent.id, agent_id);
                        assert_eq!(device.snmp_protocol_attributes, route_snmp_v1_attributes("public"));
                    }

                    async it "creates_the_object_in_database" {
                        let device: response::Device = response.json().await.unwrap();
                        let db_conn = app.db_conn.as_ref().unwrap();
                        let db_obj = Entity::find()
                            .filter(Column::Name.eq(device.name))
                            .one(db_conn)
                            .await
                            .expect("Failed to find the inserted object")
                            .expect("No object is inserted in the database");

                        assert_eq!(Uuid::from_str(&db_obj.id).unwrap(), device.id);
                    }
                }
            }

            context "empty_database" {
                context "create_device" {
                    before {
                        #[allow(unused_variables)]
                        let response = client
                            .post(format!("{}/devices", app.address))
                            .json(&serde_json::json!({
                                "name": name,
                                "description": description,
                                "agent_id": agent_id,
                                "snmp_protocol_attributes": route_snmp_v1_attributes("public"),
                            }))
                            .send()
                            .await
                            .expect("Failed to execute request");
                    }

                    async it "returns_201" {
                        assert_eq!(response.status(), StatusCode::CREATED);
                    }

                    async it "returns_the_right_object" {
                        let device: response::Device = response.json().await.unwrap();
                        assert_eq!(device.name, name);
                        assert_eq!(device.description, description);
                        assert_eq!(device.agent.id, agent_id);
                        assert_eq!(device.snmp_protocol_attributes, route_snmp_v1_attributes("public"));
                    }

                    async it "creates_the_object_in_database" {
                        let device: response::Device = response.json().await.unwrap();
                        let db_conn = app.db_conn.as_ref().unwrap();
                        let db_obj = Entity::find()
                            .filter(Column::Name.eq(device.name))
                            .one(db_conn)
                            .await
                            .expect("Failed to find the inserted object")
                            .expect("No object is inserted in the database");

                        assert_eq!(Uuid::from_str(&db_obj.id).unwrap(), device.id);
                    }
                }
            }
        }
    }
}
