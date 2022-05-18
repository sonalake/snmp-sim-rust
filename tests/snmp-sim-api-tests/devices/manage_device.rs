use demonstrate::demonstrate;

demonstrate! {
    #[actix_rt::test]
    describe "manage_devices" {
        use crate::helpers::{spawn_app, domain_snmp_v1_attributes_json};
        use actix_web::http::StatusCode;
        use uuid_dev::Uuid;
        use std::str::FromStr;
        use snmp_sim::data_access::helpers::*;
        use reqwest::Client;

        before {
            let app = spawn_app().await;
            let db_conn = app.db_conn.as_ref().unwrap();
            let agent = create_agent(db_conn, &Uuid::new_v4(), &Uuid::new_v4().to_string(), &Some(Uuid::new_v4().to_string()), &Uuid::new_v4().to_string())
                .await
                .unwrap()
                .unwrap_created();
            #[allow(unused)]
            let agent_id = Uuid::from_str(&agent.id).unwrap();
        }

        context "start_existing_device" {
            before {
                let (obj, _agent) = create_managed_device(db_conn,
                    &Uuid::new_v4(),
                    &Uuid::new_v4().to_string(),
                    &Some(Uuid::new_v4().to_string()),
                    &agent_id,
                    &domain_snmp_v1_attributes_json("public"),
                    "0.0.0.0",
                    30161)
                    .await
                    .unwrap()
                    .unwrap_created();

                #[allow(unused)]
                let response = Client::new()
                    .put(format!("{}/devices/{}/start", app.address, Uuid::from_str(&obj.id).unwrap()))
                    .send()
                    .await
                    .expect("Failed to execute request");
            }

            async it "returns_200" {
                assert_eq!(StatusCode::OK, response.status());
            }
        }

        context "start_not_existing_device" {
            before {
                #[allow(unused)]
                let response = Client::new()
                    .put(format!("{}/devices/{}/start", app.address, Uuid::new_v4()))
                    .send()
                    .await
                    .expect("Failed to execute request");
            }

            async it "returns_404" {
                assert_eq!(StatusCode::NOT_FOUND, response.status());
            }
        }

        context "stop_existing_device" {
            before {
                let (obj, _agent) = create_managed_device(db_conn,
                    &Uuid::new_v4(),
                    &Uuid::new_v4().to_string(),
                    &Some(Uuid::new_v4().to_string()),
                    &agent_id,
                    &domain_snmp_v1_attributes_json("public"),
                    "0.0.0.0",
                    30161)
                    .await
                    .unwrap()
                    .unwrap_created();

                #[allow(unused)]
                let response = Client::new()
                    .put(format!("{}/devices/{}/stop", app.address, Uuid::from_str(&obj.id).unwrap()))
                    .send()
                    .await
                    .expect("Failed to execute request");
            }

            async it "returns_200" {
                assert_eq!(StatusCode::OK, response.status());
            }
        }

        context "stop_not_existing_device" {
            before {
                #[allow(unused)]
                let response = Client::new()
                    .put(format!("{}/devices/{}/stop", app.address, Uuid::new_v4()))
                    .send()
                    .await
                    .expect("Failed to execute request");
            }

            async it "returns_404" {
                assert_eq!(StatusCode::NOT_FOUND, response.status());
            }
        }    }
}
