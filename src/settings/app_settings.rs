use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize, Clone)]
/// Structure representing the application settings
pub struct ApplicationSettings {
    /// The IP address or the TCP/IP hostname of the SNMP Simulator HTTP server
    /// serving the management REST API.
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    /// Specifies the port for which the service is configured to accept client
    /// requests. The port value is used in conjunction with the host name.
    pub port: u16,
    #[serde(default)]
    pub uri_prefix: String,
    /// Verbosity level of the logger. Following values are supported error,
    /// warn, info, debug and trace.
    pub level: String,
}
