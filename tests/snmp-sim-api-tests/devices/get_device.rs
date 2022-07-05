use demonstrate::demonstrate;

demonstrate! {
    #[actix_rt::test]
    describe "get_devices" {
        use crate::helpers::{seed_devices, spawn_app};
        use actix_web::http::StatusCode;
        use snmp_sim::routes::managed_devices::response;
        use reqwest::Client;
        use uuid_dev::Uuid;
        use snmp_sim::data_access::helpers::*;
        use std::str::FromStr;

        before {
            let app = spawn_app().await;
        }

        context "request_first_page" {
            before {
                let response = Client::new()
                    .get(format!("{}/devices/?page={}", app.address, 1))
                    .send()
                    .await
                    .expect("Failed to execute request");
            }

            async it "responds_with_200" {
                assert_eq!(StatusCode::OK, response.status());
            }
        }

        context "request_invalid_page" {
            before {
                let response = Client::new()
                    .get(format!("{}/devices/?page={}", app.address, -1))
                    .send()
                    .await
                    .expect("Failed to execute request");
            }

            async it "responds_with_400" {
                assert_eq!(StatusCode::BAD_REQUEST, response.status());
            }
        }

        context "request_invalid_page_size" {
            before {
                let response = Client::new()
                    .get(format!("{}/devices/?page_size={}", app.address, -1))
                    .send()
                    .await
                    .expect("Failed to execute request");
            }

            async it "responds_with_400" {
                assert_eq!(StatusCode::BAD_REQUEST, response.status());
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
                seed_devices(db_conn, &agent_id, 25, "locahost", 30161).await;
            }

            context "request_first_page" {
                before {
                    let response = Client::new()
                        .get(format!("{}/devices/?page={}", app.address, 1))
                        .send()
                        .await
                        .expect("Failed to execute request");
                }

                async it "responds_with_200" {
                    assert_eq!(StatusCode::OK, response.status());
                }

                async it "returns_20_devices" {
                    let json: Vec<response::Device> = response.json().await.unwrap();
                    assert_eq!(20, json.len());
                }
            }

            context "request_second_page" {
                before {
                    let response = Client::new()
                        .get(format!("{}/devices/?page={}", app.address, 2))
                        .send()
                        .await
                        .expect("Failed to execute request");
                }

                async it "responds_with_200" {
                    assert_eq!(StatusCode::OK, response.status());
                }
            }
        }
    }
}
