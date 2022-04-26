use demonstrate::demonstrate;

demonstrate! {
    #[actix_rt::test]
    describe "get_agents_id" {
        use crate::helpers::{seed_agents, spawn_app};
        use actix_web::http::StatusCode;
        use snmp_sim::data_access::helpers::*;
        use snmp_sim::routes::agents::response;
        use reqwest::Client;
        use uuid_dev::Uuid;
        use std::str::FromStr;

        before {
            let app = spawn_app().await;
        }

        context "empty_database" {
            context "endpoint_is_hit" {
                before {
                    let response = Client::new()
                        .get(format!("{}/agents/{}", app.address, Uuid::new_v4()))
                        .send()
                        .await
                        .expect("Failed to execute request");
                }

                async it "returns_404" {
                    assert_eq!(StatusCode::NOT_FOUND, response.status());
                }
            }
        }

        context "seeded_database" {
            before {
                let db_conn = app.db_conn.as_ref().unwrap();
                seed_agents(db_conn, 5).await;
            }

            context "nonexistent_requested" {
                before {
                    let response = Client::new()
                        .get(format!("{}/agents/{}", app.address, Uuid::new_v4()))
                        .send()
                        .await
                        .expect("Failed to execute request");
                }

                async it "returns_404" {
                    assert_eq!(StatusCode::NOT_FOUND, response.status());
                }

            }

            context "existent_requested" {
                before {
                    let obj = create_agent(db_conn, &Uuid::new_v4(), &Uuid::new_v4().to_string(), &Some(Uuid::new_v4().to_string()), &Uuid::new_v4().to_string())
                        .await
                        .unwrap()
                        .unwrap_created();
                    let response = Client::new()
                        .get(format!("{}/agents/{}", app.address, Uuid::from_str(&obj.id).unwrap()))
                        .send()
                        .await
                        .expect("Failed to execute request");
                }

                async it "returns_200" {
                    assert_eq!(StatusCode::OK, response.status());
                }

                async it "returns_the_object" {
                    let agent: response::Agent = response.json().await.unwrap();

                    assert_eq!(agent.id, Uuid::from_str(&obj.id).unwrap());
                    assert_eq!(agent.name, obj.name);
                }
            }
        }
    }
}
