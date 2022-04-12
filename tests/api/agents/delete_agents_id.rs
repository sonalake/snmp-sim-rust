use demonstrate::demonstrate;

demonstrate! {
    #[actix_rt::test]
    describe "delete_agents_id"{
        use reqwest::Client;
        use sea_orm::EntityTrait;
        use sea_orm::PaginatorTrait;
        use uuid_dev::Uuid;
        use crate::helpers::{seed_agents, spawn_app};
        use actix_web::http::StatusCode;
        use snmp_sim::data_access::entity::agents::Entity;
        use snmp_sim::data_access::helpers::*;
        use snmp_sim::routes::agents::response;
        use std::str::FromStr;

        before {
            let app = spawn_app().await;
        }

        context "empty_database" {
            context "nonexistent" {
                before {
                    let response = Client::new()
                        .delete(format!("{}/agents/{}", app.address, Uuid::new_v4().to_string()))
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
                seed_agents(db_conn, 20).await;
            }

            context "nonexistent" {
                before {
                    let response = Client::new()
                        .delete(format!("{}/agents/{}", app.address, Uuid::new_v4().to_string()))
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
                    let obj = create_agent(db_conn, &Uuid::new_v4(), &Uuid::new_v4().to_string())
                        .await
                        .unwrap()
                        .unwrap_created();

                    #[allow(unused)]
                    let response = Client::new()
                        .delete(format!("{}/agents/{}", app.address, Uuid::from_str(&obj.id).unwrap()))
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
                    let returned: response::Agent = response.json().await.unwrap();

                    assert_eq!(returned.id, Uuid::from_str(&obj.id).unwrap());
                    assert_eq!(returned.name, obj.name);
                }

                context "repeated_deletion" {
                    before {
                        let response = Client::new()
                            .delete(format!("{}/agents/{}", app.address, Uuid::from_str(&obj.id).unwrap()))
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
