use demonstrate::demonstrate;

demonstrate! {
    #[actix_rt::test]
    describe "start_device" {
        use crate::helpers::{spawn_app, seed_devices};
        use crate::snmp::DEVICE_PORT;
        use actix_web::http::StatusCode;
        use snmp_sim::data_access::helpers::*;
        use reqwest::Client;
        use uuid_dev::Uuid;
        use std::str::FromStr;

        before {
            let app = spawn_app().await;
        }

        context "seeded_database" {
            before {
                let db_conn = app.db_conn.as_ref().unwrap();

                let agent = create_agent(db_conn, &Uuid::new_v4(), &Uuid::new_v4().to_string(), &Some(Uuid::new_v4().to_string()), "os-linux-std.txt")
                    .await
                    .unwrap()
                    .unwrap_created();
                let agent_id = Uuid::from_str(&agent.id).unwrap();

                let host_ipaddr = "127.0.0.1";
                let device_port = DEVICE_PORT.write().get_next_value();

                #[allow(unused)]
                let device_ids = seed_devices(db_conn, &agent_id, 1, host_ipaddr, device_port).await;
            }

            async it "created_device" {
                assert_eq!(1, device_ids.len());
            }

            context "start_not_existing_device" {

                before {
                    let device_id = Uuid::new_v4();
                    let response = Client::new()
                        .put(format!("{}/devices/{}/start", app.address, device_id))
                        .send()
                        .await
                        .unwrap();
                }

                async it "returns_404" {
                    assert_eq!(StatusCode::NOT_FOUND, response.status());
                }
            }

            context "start_existing_device" {

                before {
                    let device_id = device_ids.first().unwrap();
                    #[allow(unused)]
                    let response = Client::new()
                        .put(format!("{}/devices/{}/start", app.address, device_id))
                        .send()
                        .await
                        .unwrap();
                }

                async it "returns_200" {
                    assert_eq!(StatusCode::OK, response.status());
                }

                context "start_already_running_device" {

                    before {
                        let response = Client::new()
                            .put(format!("{}/devices/{}/start", app.address, device_id))
                            .send()
                            .await
                            .unwrap();
                    }

                    async it "returns_409" {
                        assert_eq!(StatusCode::CONFLICT, response.status());
                    }
                }
            }
        }
    }
}
