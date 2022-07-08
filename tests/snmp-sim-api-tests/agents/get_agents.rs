use demonstrate::demonstrate;

demonstrate! {
    #[actix_rt::test]
    describe "get_agents" {
        use crate::helpers::{seed_agents, spawn_app};
        use actix_web::http::StatusCode;
        use snmp_sim::routes::agents::response;
        use reqwest::Client;

        before {
            let app = spawn_app().await;
        }

        context "request_first_page" {
            before {
                let response = Client::new()
                    .get(format!("{}/agents/?page={}", app.address, 1))
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
                    .get(format!("{}/agents/?page={}", app.address, -1))
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
                    .get(format!("{}/agents/?page_size={}", app.address, -1))
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
                seed_agents(db_conn, 25).await;
            }

            context "request_first_page" {
                before {
                    let response = Client::new()
                        .get(format!("{}/agents/?page={}", app.address, 1))
                        .send()
                        .await
                        .expect("Failed to execute request");
                }

                async it "responds_with_200" {
                    assert_eq!(StatusCode::OK, response.status());
                }

                async it "returns_20_agents" {
                    let json: response::Agents = response.json().await.unwrap();
                    assert_eq!(20, json.agents.len());
                }
            }

            context "request_second_page" {
                before {
                    let response = Client::new()
                        .get(format!("{}/agents/?page={}", app.address, 2))
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
