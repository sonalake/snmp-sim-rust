use demonstrate::demonstrate;

demonstrate! {
    #[actix_rt::test]
    describe "delete_devices_id"{
        use reqwest::Client;
        use sea_orm::EntityTrait;
        use sea_orm::PaginatorTrait;
        use uuid_dev::Uuid;
        use crate::helpers::{seed_devices, spawn_app, domain_snmp_v1_attributes_json};
        use actix_web::http::StatusCode;
        use snmp_sim::data_access::entity::managed_devices::Entity;
        use snmp_sim::data_access::helpers::*;
        use snmp_sim::routes::managed_devices::response;
        use std::str::FromStr;

        before {
            let app = spawn_app().await;
        }

        context "empty_database" {
            context "nonexistent" {
                before {
                    let response = Client::new()
                        .delete(format!("{}/devices/{}", app.address, Uuid::new_v4()))
                        .send()
                        .await
                        .expect("Failed to execute request");
                }

                async it "returns_204" {
                    assert_eq!(StatusCode::NO_CONTENT, response.status());
                }
            }
        }

        context "seeded_database" {
            before {
                let db_conn = app.db_conn.as_ref().unwrap();
                let agent = create_agent(db_conn, &Uuid::new_v4(), &Uuid::new_v4().to_string(), &Some(Uuid::new_v4().to_string()), &Uuid::new_v4().to_string())
                    .await
                    .unwrap()
                    .unwrap_created();
                let agent_id = Uuid::from_str(&agent.id).unwrap();
                seed_devices(db_conn, &agent_id, 20).await;
            }

            context "nonexistent" {
                before {
                    let response = Client::new()
                        .delete(format!("{}/devices/{}", app.address, Uuid::new_v4()))
                        .send()
                        .await
                        .expect("Failed to execute request");
                }

                async it "returns_204" {
                    assert_eq!(StatusCode::NO_CONTENT, response.status());
                }
            }

            context "existent" {
                before {
                    let (obj, _agent) = create_managed_device(db_conn,
                        &Uuid::new_v4(),
                        &Uuid::new_v4().to_string(),
                        &Some(Uuid::new_v4().to_string()),
                        &agent_id,
                        &domain_snmp_v1_attributes_json("public"))
                        .await
                        .unwrap()
                        .unwrap_created();

                    #[allow(unused)]
                    let response = Client::new()
                        .delete(format!("{}/devices/{}", app.address, Uuid::from_str(&obj.id).unwrap()))
                        .send()
                        .await
                        .expect("Failed to execute request");
                }

                async it "returns_200" {
                    assert_eq!(StatusCode::OK, response.status());
                }

                async it "deletes_the_object" {
                    assert_eq!(
                        0,
                        Entity::find_by_id(obj.id)
                            .count(db_conn)
                            .await
                            .unwrap()
                    );
                }

                async it "returns_the_object" {
                    let returned: response::Device = response.json().await.unwrap();

                    assert_eq!(returned.id, Uuid::from_str(&obj.id).unwrap());
                    assert_eq!(returned.name, obj.name);
                }

                context "repeated_deletion" {
                    before {
                        let response = Client::new()
                            .delete(format!("{}/devices/{}", app.address, Uuid::from_str(&obj.id).unwrap()))
                            .send()
                            .await
                            .expect("Failed to execute request");
                    }

                    async it "returns_204" {
                        assert_eq!(StatusCode::NO_CONTENT, response.status());
                    }
                }
            }
        }
    }
}
