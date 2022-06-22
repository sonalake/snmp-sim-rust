use demonstrate::demonstrate;

demonstrate! {
    #[actix_rt::test]
    describe "snmp_v1" {
        use crate::helpers::{spawn_app, seed_devices};
        use actix_web::http::StatusCode;
        use snmp_sim::data_access::helpers::*;
        use reqwest::Client as reqwestClient;
        use uuid_dev::Uuid;
        use std::str::FromStr;
        use snmp_sim::snmp::codec::GenericSnmpMessage;
        use crate::snmp::DEVICE_PORT;
        use num_traits::ToPrimitive;
        use rasn_smi::v1::*;
        use rasn_snmp::v1::*;
        use snmp_sim::udp_client::Client;
        use snmp_data_parser::parser::snmp_data::component::string_to_oid;
        use crate::snmp::{get_request_v1, get_next_request_v1};

        before {
            let app = spawn_app().await;
        }

        describe "agent_and_device" {
            before {
                let db_conn = app.db_conn.as_ref().unwrap();
                let host_ipaddr = "127.0.0.1";
                let device_port = DEVICE_PORT.write().get_next_value();

                let agent = create_agent(db_conn, &Uuid::new_v4(), &Uuid::new_v4().to_string(), &Some(Uuid::new_v4().to_string()), "os-linux-std.txt")
                    .await
                    .unwrap()
                    .unwrap_created();
                let agent_id = Uuid::from_str(&agent.id).unwrap();
                let device_ids = seed_devices(db_conn, &agent_id, 1, host_ipaddr, device_port).await;
                #[allow(unused)]
                let device_id = *device_ids.first().unwrap();
            }

            async it "created" {
                assert_eq!(1, device_ids.len());
            }

            describe "with_device" {

                before {
                    #[allow(unused)]
                    let response = reqwestClient::new()
                        .put(format!("{}/devices/{}/start", app.address, device_id))
                        .send()
                        .await
                        .unwrap();
                }

                async it "started" {
                    assert_eq!(StatusCode::OK, response.status());
                }

                describe "get_request" {
                    describe "with_known_oid" {

                        before {
                            let remote_addr = format!("{host_ipaddr}:{device_port}");
                            let oid = string_to_oid(".1.3.6.1.2.1.1.1.0");
                            let response = Client::new(remote_addr.parse().unwrap()).unwrap()
                                .send_request(get_request_v1(1, "public", vec![oid.clone()]))
                                .await;
                        }

                        async it "returns_valid_string" {
                            if let Ok(GenericSnmpMessage::V1Message(msg)) = &response {
                                if let Pdus::GetResponse(resp) = &msg.data {
                                    assert_eq!(0, resp.0.error_index.to_u32().unwrap());
                                    assert_eq!(0, resp.0.error_status.to_u32().unwrap());
                                    assert_eq!(1, resp.0.variable_bindings.len());
                                    let var_bind = resp.0.variable_bindings.first().unwrap();
                                    assert_eq!(oid, var_bind.name);
                                    let expected_value = ObjectSyntax::Simple(
                                        SimpleSyntax::String(
                                            "Linux nmsworker-devel 2.6.18-164.el5 #1 SMP Thu Sep 3 03:28:30 EDT 2009 x86_64".into()));
                                    assert_eq!(expected_value, var_bind.value);
                                }
                                else {
                                    println!("{:?}", response);
                                    debug_assert!(false, "Not a valid response");
                                }
                            }
                            else {
                                println!("{:?}", response);
                                debug_assert!(false, "Not a valid response");
                            }
                        }
                    }

                    describe "with_unknown_oid" {

                        before {
                            let remote_addr = format!("{host_ipaddr}:{device_port}");
                            let oid = string_to_oid(".1.3.6.1.2.1.1.1.1");
                            let response = Client::new(remote_addr.parse().unwrap()).unwrap()
                                .send_request(get_request_v1(1, "public", vec![oid.clone()]))
                                .await;
                        }

                        async it "returns_error" {
                            if let Ok(GenericSnmpMessage::V1Message(msg)) = &response {
                                if let Pdus::GetResponse(resp) = &msg.data {
                                    assert_eq!(1, resp.0.error_index.to_u32().unwrap());
                                    assert_eq!(2, resp.0.error_status.to_u32().unwrap());
                                    assert_eq!(1, resp.0.variable_bindings.len());
                                    assert_eq!(oid, resp.0.variable_bindings.first().unwrap().name);
                                }
                                else {
                                    println!("{:?}", response);
                                    debug_assert!(false, "Not a valid response");
                                }
                            }
                            else {
                                println!("{:?}", response);
                                debug_assert!(false, "Not a valid response");
                            }
                        }
                    }

                    describe "with_known_oids_array" {

                        before {
                            let remote_addr = format!("{host_ipaddr}:{device_port}");
                            let oids = vec![
                                string_to_oid(".1.3.6.1.2.1.1.1.0"),
                                string_to_oid(".1.3.6.1.2.1.1.2.0"),
                                string_to_oid(".1.3.6.1.2.1.1.6.0")
                                ];
                            let response = Client::new(remote_addr.parse().unwrap()).unwrap()
                                .send_request(get_request_v1(1, "public", oids.clone()))
                                .await;
                        }

                        async it "returns_valid_objects" {
                            if let Ok(GenericSnmpMessage::V1Message(msg)) = &response {
                                if let Pdus::GetResponse(resp) = &msg.data {
                                    assert_eq!(0, resp.0.error_index.to_u32().unwrap());
                                    assert_eq!(0, resp.0.error_status.to_u32().unwrap());
                                    assert_eq!(3, resp.0.variable_bindings.len());

                                    let expected_values = vec![
                                            ObjectSyntax::Simple(
                                                SimpleSyntax::String(
                                                    "Linux nmsworker-devel 2.6.18-164.el5 #1 SMP Thu Sep 3 03:28:30 EDT 2009 x86_64".into())),
                                            ObjectSyntax::Simple(
                                                SimpleSyntax::Object(
                                                    string_to_oid(".1.3.6.1.4.1.8072.3.2.10"))),
                                            ObjectSyntax::Simple(
                                                SimpleSyntax::String(
                                                    "Unknown (edit /etc/snmp/snmpd.conf)".into()))];

                                    resp.0.variable_bindings
                                        .iter()
                                        .enumerate()
                                        .for_each(|(idx, var_bind)|{
                                            assert_eq!(oids[idx], var_bind.name);
                                            assert_eq!(expected_values[idx], var_bind.value);
                                    });
                                }
                                else {
                                    println!("{:?}", response);
                                    debug_assert!(false, "Not a valid response");
                                }
                            }
                            else {
                                println!("{:?}", response);
                                debug_assert!(false, "Not a valid response");
                            }
                        }
                    }

                    describe "with_unknown_oids_array" {

                        before {
                            let remote_addr = format!("{host_ipaddr}:{device_port}");
                            let oids = vec![
                                string_to_oid(".1.3.6.1.2.1.1.1.0"),
                                string_to_oid(".1.3.6.1.2.1.1.2.0"),
                                string_to_oid(".1.3.6.1.2.1.1.6.1")
                                ];
                            let response = Client::new(remote_addr.parse().unwrap()).unwrap()
                                .send_request(get_request_v1(1, "public", oids.clone()))
                                .await;
                        }

                        async it "returns_error" {
                            if let Ok(GenericSnmpMessage::V1Message(msg)) = &response {
                                if let Pdus::GetResponse(resp) = &msg.data {
                                    assert_eq!(3, resp.0.error_index.to_u32().unwrap());
                                    assert_eq!(Pdu::ERROR_STATUS_NO_SUCH_NAME, resp.0.error_status.to_u64().unwrap());
                                    assert_eq!(1, resp.0.variable_bindings.len());

                                    let var_bind = resp.0.variable_bindings.first().unwrap();
                                    assert_eq!(var_bind.name, string_to_oid(".1.3.6.1.2.1.1.6.1"));
                                    assert_eq!(var_bind.value, ObjectSyntax::Simple(SimpleSyntax::Empty));
                                }
                                else {
                                    println!("{:?}", response);
                                    debug_assert!(false, "Not a valid response");
                                }
                            }
                            else {
                                println!("{:?}", response);
                                debug_assert!(false, "Not a valid response");
                            }
                        }
                    }

                    describe "with_oid_returning_number" {

                        before {
                            let remote_addr = format!("{host_ipaddr}:{device_port}");
                            let oid = string_to_oid("1.3.6.1.2.1.2.1.0");
                            let response = Client::new(remote_addr.parse().unwrap()).unwrap()
                                .send_request(get_request_v1(1, "public", vec![oid.clone()]))
                                .await;
                        }

                        async it "returns_valid_number" {
                            if let Ok(GenericSnmpMessage::V1Message(msg)) = &response {
                                if let Pdus::GetResponse(resp) = &msg.data {
                                    assert_eq!(0, resp.0.error_index.to_u32().unwrap());
                                    assert_eq!(0, resp.0.error_status.to_u32().unwrap());
                                    assert_eq!(1, resp.0.variable_bindings.len());
                                    let var_bind = resp.0.variable_bindings.first().unwrap();
                                    assert_eq!(oid, var_bind.name);
                                    let expected_value = ObjectSyntax::Simple(
                                        SimpleSyntax::Number(3.into()));
                                    assert_eq!(expected_value, var_bind.value);
                                }
                                else {
                                    println!("{:?}", response);
                                    debug_assert!(false, "Not a valid response");
                                }
                            }
                            else {
                                println!("{:?}", response);
                                debug_assert!(false, "Not a valid response");
                            }
                        }
                    }

                    describe "with_oid_returning_counter" {

                        before {
                            let remote_addr = format!("{host_ipaddr}:{device_port}");
                            let oid = string_to_oid(".1.3.6.1.2.1.2.2.1.10.1");
                            let response = Client::new(remote_addr.parse().unwrap()).unwrap()
                                .send_request(get_request_v1(1, "public", vec![oid.clone()]))
                                .await;
                        }

                        async it "returns_valid_counter" {
                            if let Ok(GenericSnmpMessage::V1Message(msg)) = &response {
                                if let Pdus::GetResponse(resp) = &msg.data {
                                    assert_eq!(0, resp.0.error_index.to_u32().unwrap());
                                    assert_eq!(0, resp.0.error_status.to_u32().unwrap());
                                    assert_eq!(1, resp.0.variable_bindings.len());
                                    let var_bind = resp.0.variable_bindings.first().unwrap();
                                    assert_eq!(oid, var_bind.name);
                                    let expected_value = ObjectSyntax::ApplicationWide(
                                        ApplicationSyntax::Counter(Counter(914518245)));
                                    assert_eq!(expected_value, var_bind.value);
                                }
                                else {
                                    println!("{:?}", response);
                                    debug_assert!(false, "Not a valid response");
                                }
                            }
                            else {
                                println!("{:?}", response);
                                debug_assert!(false, "Not a valid response");
                            }
                        }
                    }

                    describe "with_oid_returning_gauge" {

                        before {
                            let remote_addr = format!("{host_ipaddr}:{device_port}");
                            let oid = string_to_oid(".1.3.6.1.2.1.4.24.6.0");
                            let response = Client::new(remote_addr.parse().unwrap()).unwrap()
                                .send_request(get_request_v1(1, "public", vec![oid.clone()]))
                                .await;
                        }

                        async it "returns_valid_gauge" {
                            if let Ok(GenericSnmpMessage::V1Message(msg)) = &response {
                                if let Pdus::GetResponse(resp) = &msg.data {
                                    assert_eq!(0, resp.0.error_index.to_u32().unwrap());
                                    assert_eq!(0, resp.0.error_status.to_u32().unwrap());
                                    assert_eq!(1, resp.0.variable_bindings.len());
                                    let var_bind = resp.0.variable_bindings.first().unwrap();
                                    assert_eq!(oid, var_bind.name);
                                    let expected_value = ObjectSyntax::ApplicationWide(
                                        ApplicationSyntax::Gauge(Gauge(7)));
                                    assert_eq!(expected_value, var_bind.value);
                                }
                                else {
                                    println!("{:?}", response);
                                    debug_assert!(false, "Not a valid response");
                                }
                            }
                            else {
                                println!("{:?}", response);
                                debug_assert!(false, "Not a valid response");
                            }
                        }
                    }

                    describe "with_oid_returning_address" {

                        before {
                            let remote_addr = format!("{host_ipaddr}:{device_port}");
                            let oid = string_to_oid(".1.3.6.1.2.1.4.21.1.1.169.254.0.0");
                            let response = Client::new(remote_addr.parse().unwrap()).unwrap()
                                .send_request(get_request_v1(1, "public", vec![oid.clone()]))
                                .await;
                        }

                        async it "returns_valid_address" {
                            if let Ok(GenericSnmpMessage::V1Message(msg)) = &response {
                                if let Pdus::GetResponse(resp) = &msg.data {
                                    assert_eq!(0, resp.0.error_index.to_u32().unwrap());
                                    assert_eq!(0, resp.0.error_status.to_u32().unwrap());
                                    assert_eq!(1, resp.0.variable_bindings.len());
                                    let var_bind = resp.0.variable_bindings.first().unwrap();
                                    assert_eq!(oid, var_bind.name);
                                    let value: std::net::Ipv4Addr = "169.254.0.0".parse().unwrap();
                                    let expected_value = ObjectSyntax::ApplicationWide(
                                        ApplicationSyntax::Address(
                                            NetworkAddress::Internet(
                                                IpAddress(bytes::Bytes::from(value.octets().to_vec())))));
                                    assert_eq!(expected_value, var_bind.value);
                                }
                                else {
                                    println!("{:?}", response);
                                    debug_assert!(false, "Not a valid response");
                                }
                            }
                            else {
                                println!("{:?}", response);
                                debug_assert!(false, "Not a valid response");
                            }
                        }
                    }
                }

                describe "get_next_request" {
                    describe "with_known_oid" {

                        before {
                            let remote_addr = format!("{host_ipaddr}:{device_port}");
                            let oid = string_to_oid(".1.3.6.1.2.1.1.1.0");
                            let response = Client::new(remote_addr.parse().unwrap()).unwrap()
                                .send_request(get_next_request_v1(1, "public", vec![oid]))
                                .await;
                        }

                        async it "returns_valid_string" {
                            if let Ok(GenericSnmpMessage::V1Message(msg)) = &response {
                                if let Pdus::GetResponse(resp) = &msg.data {
                                    assert_eq!(0, resp.0.error_index.to_u32().unwrap());
                                    assert_eq!(0, resp.0.error_status.to_u32().unwrap());
                                    assert_eq!(1, resp.0.variable_bindings.len());
                                    let var_bind = resp.0.variable_bindings.first().unwrap();
                                    let expected_oid = string_to_oid(".1.3.6.1.2.1.1.2.0");
                                    assert_eq!(expected_oid, var_bind.name);
                                    let expected_value = ObjectSyntax::Simple(
                                                SimpleSyntax::Object(
                                                    string_to_oid(".1.3.6.1.4.1.8072.3.2.10")));
                                    assert_eq!(expected_value, var_bind.value);
                                }
                                else {
                                    println!("{:?}", response);
                                    debug_assert!(false, "Not a valid response");
                                }
                            }
                            else {
                                println!("{:?}", response);
                                debug_assert!(false, "Not a valid response");
                            }
                        }
                    }
                }
            }
        }
    }
}
