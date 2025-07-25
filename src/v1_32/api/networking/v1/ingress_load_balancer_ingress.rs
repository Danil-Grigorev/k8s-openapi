// Generated from definition io.k8s.api.networking.v1.IngressLoadBalancerIngress

/// IngressLoadBalancerIngress represents the status of a load-balancer ingress point.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IngressLoadBalancerIngress {
    /// hostname is set for load-balancer ingress points that are DNS based.
    pub hostname: Option<std::string::String>,

    /// ip is set for load-balancer ingress points that are IP based.
    pub ip: Option<std::string::String>,

    /// ports provides information about the ports exposed by this LoadBalancer.
    pub ports: Option<std::vec::Vec<crate::api::networking::v1::IngressPortStatus>>,
}

impl crate::DeepMerge for IngressLoadBalancerIngress {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.hostname, other.hostname);
        crate::DeepMerge::merge_from(&mut self.ip, other.ip);
        crate::merge_strategies::list::atomic(&mut self.ports, other.ports);
    }
}

impl<'de> crate::serde::Deserialize<'de> for IngressLoadBalancerIngress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_hostname,
            Key_ip,
            Key_ports,
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
                            "hostname" => Field::Key_hostname,
                            "ip" => Field::Key_ip,
                            "ports" => Field::Key_ports,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = IngressLoadBalancerIngress;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("IngressLoadBalancerIngress")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_hostname: Option<std::string::String> = None;
                let mut value_ip: Option<std::string::String> = None;
                let mut value_ports: Option<std::vec::Vec<crate::api::networking::v1::IngressPortStatus>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_hostname => value_hostname = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip => value_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ports => value_ports = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IngressLoadBalancerIngress {
                    hostname: value_hostname,
                    ip: value_ip,
                    ports: value_ports,
                })
            }
        }

        deserializer.deserialize_struct(
            "IngressLoadBalancerIngress",
            &[
                "hostname",
                "ip",
                "ports",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for IngressLoadBalancerIngress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IngressLoadBalancerIngress",
            self.hostname.as_ref().map_or(0, |_| 1) +
            self.ip.as_ref().map_or(0, |_| 1) +
            self.ports.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.hostname {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostname", value)?;
        }
        if let Some(value) = &self.ip {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ip", value)?;
        }
        if let Some(value) = &self.ports {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ports", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for IngressLoadBalancerIngress {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.networking.v1.IngressLoadBalancerIngress".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "IngressLoadBalancerIngress represents the status of a load-balancer ingress point.",
            "type": "object",
            "properties": {
                "hostname": {
                    "description": "hostname is set for load-balancer ingress points that are DNS based.",
                    "type": "string",
                },
                "ip": {
                    "description": "ip is set for load-balancer ingress points that are IP based.",
                    "type": "string",
                },
                "ports": {
                    "description": "ports provides information about the ports exposed by this LoadBalancer.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::networking::v1::IngressPortStatus>()),
                },
            },
        })
    }
}
