// Generated from definition io.k8s.api.networking.v1.IngressTLS

/// IngressTLS describes the transport layer security associated with an ingress.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IngressTLS {
    /// hosts is a list of hosts included in the TLS certificate. The values in this list must match the name/s used in the tlsSecret. Defaults to the wildcard host setting for the loadbalancer controller fulfilling this Ingress, if left unspecified.
    pub hosts: Option<std::vec::Vec<std::string::String>>,

    /// secretName is the name of the secret used to terminate TLS traffic on port 443. Field is left optional to allow TLS routing based on SNI hostname alone. If the SNI host in a listener conflicts with the "Host" header field used by an IngressRule, the SNI host is used for termination and value of the "Host" header is used for routing.
    pub secret_name: Option<std::string::String>,
}

impl crate::DeepMerge for IngressTLS {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.hosts, other.hosts);
        crate::DeepMerge::merge_from(&mut self.secret_name, other.secret_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for IngressTLS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_hosts,
            Key_secret_name,
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
                            "hosts" => Field::Key_hosts,
                            "secretName" => Field::Key_secret_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = IngressTLS;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("IngressTLS")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_hosts: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_secret_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_hosts => value_hosts = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_name => value_secret_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IngressTLS {
                    hosts: value_hosts,
                    secret_name: value_secret_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "IngressTLS",
            &[
                "hosts",
                "secretName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for IngressTLS {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IngressTLS",
            self.hosts.as_ref().map_or(0, |_| 1) +
            self.secret_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.hosts {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hosts", value)?;
        }
        if let Some(value) = &self.secret_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretName", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for IngressTLS {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.networking.v1.IngressTLS".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "IngressTLS describes the transport layer security associated with an ingress.",
            "type": "object",
            "properties": {
                "hosts": {
                    "description": "hosts is a list of hosts included in the TLS certificate. The values in this list must match the name/s used in the tlsSecret. Defaults to the wildcard host setting for the loadbalancer controller fulfilling this Ingress, if left unspecified.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "secretName": {
                    "description": "secretName is the name of the secret used to terminate TLS traffic on port 443. Field is left optional to allow TLS routing based on SNI hostname alone. If the SNI host in a listener conflicts with the \"Host\" header field used by an IngressRule, the SNI host is used for termination and value of the \"Host\" header is used for routing.",
                    "type": "string",
                },
            },
        })
    }
}
