// Generated from definition io.k8s.api.core.v1.HTTPGetAction

/// HTTPGetAction describes an action based on HTTP Get requests.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HTTPGetAction {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    pub host: Option<std::string::String>,

    /// Custom headers to set in the request. HTTP allows repeated headers.
    pub http_headers: Option<std::vec::Vec<crate::api::core::v1::HTTPHeader>>,

    /// Path to access on the HTTP server.
    pub path: Option<std::string::String>,

    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: crate::apimachinery::pkg::util::intstr::IntOrString,

    /// Scheme to use for connecting to the host. Defaults to HTTP.
    pub scheme: Option<std::string::String>,
}

impl crate::DeepMerge for HTTPGetAction {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.host, other.host);
        crate::merge_strategies::list::atomic(&mut self.http_headers, other.http_headers);
        crate::DeepMerge::merge_from(&mut self.path, other.path);
        crate::DeepMerge::merge_from(&mut self.port, other.port);
        crate::DeepMerge::merge_from(&mut self.scheme, other.scheme);
    }
}

impl<'de> crate::serde::Deserialize<'de> for HTTPGetAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_host,
            Key_http_headers,
            Key_path,
            Key_port,
            Key_scheme,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "host" => Field::Key_host,
                            "httpHeaders" => Field::Key_http_headers,
                            "path" => Field::Key_path,
                            "port" => Field::Key_port,
                            "scheme" => Field::Key_scheme,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = HTTPGetAction;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("HTTPGetAction")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_host: Option<std::string::String> = None;
                let mut value_http_headers: Option<std::vec::Vec<crate::api::core::v1::HTTPHeader>> = None;
                let mut value_path: Option<std::string::String> = None;
                let mut value_port: Option<crate::apimachinery::pkg::util::intstr::IntOrString> = None;
                let mut value_scheme: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_host => value_host = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_http_headers => value_http_headers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_path => value_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port => value_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scheme => value_scheme = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HTTPGetAction {
                    host: value_host,
                    http_headers: value_http_headers,
                    path: value_path,
                    port: value_port.unwrap_or_default(),
                    scheme: value_scheme,
                })
            }
        }

        deserializer.deserialize_struct(
            "HTTPGetAction",
            &[
                "host",
                "httpHeaders",
                "path",
                "port",
                "scheme",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for HTTPGetAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HTTPGetAction",
            1 +
            self.host.as_ref().map_or(0, |_| 1) +
            self.http_headers.as_ref().map_or(0, |_| 1) +
            self.path.as_ref().map_or(0, |_| 1) +
            self.scheme.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.host {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "host", value)?;
        }
        if let Some(value) = &self.http_headers {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "httpHeaders", value)?;
        }
        if let Some(value) = &self.path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "path", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "port", &self.port)?;
        if let Some(value) = &self.scheme {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scheme", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for HTTPGetAction {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.HTTPGetAction".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "HTTPGetAction describes an action based on HTTP Get requests.",
            "type": "object",
            "properties": {
                "host": {
                    "description": "Host name to connect to, defaults to the pod IP. You probably want to set \"Host\" in httpHeaders instead.",
                    "type": "string",
                },
                "httpHeaders": {
                    "description": "Custom headers to set in the request. HTTP allows repeated headers.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::HTTPHeader>()),
                },
                "path": {
                    "description": "Path to access on the HTTP server.",
                    "type": "string",
                },
                "port": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::util::intstr::IntOrString>();
                    schema_obj.ensure_object().insert("description".into(), "Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.".into());
                    schema_obj
                }),
                "scheme": {
                    "description": "Scheme to use for connecting to the host. Defaults to HTTP.",
                    "type": "string",
                },
            },
            "required": [
                "port",
            ],
        })
    }
}
