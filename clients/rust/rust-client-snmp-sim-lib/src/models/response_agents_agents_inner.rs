/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ResponseAgentsAgentsInner : An agent as a response body.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResponseAgentsAgentsInner {
    /// Agent's optional description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The unique identifier of this agent.
    #[serde(rename = "id")]
    pub id: String,
    /// Agent's name.
    #[serde(rename = "name")]
    pub name: String,
    /// The URL to the SNMP data f.e. \"file://./os/linux.dat\"
    #[serde(rename = "snmp_data_url")]
    pub snmp_data_url: String,
}

impl ResponseAgentsAgentsInner {
    /// An agent as a response body.
    pub fn new(id: String, name: String, snmp_data_url: String) -> ResponseAgentsAgentsInner {
        ResponseAgentsAgentsInner {
            description: None,
            id,
            name,
            snmp_data_url,
        }
    }
}


