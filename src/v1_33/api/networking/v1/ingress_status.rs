// Generated from definition io.k8s.api.networking.v1.IngressStatus

/// IngressStatus describe the current state of the Ingress.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IngressStatus {
    /// loadBalancer contains the current status of the load-balancer.
    pub load_balancer: Option<crate::api::networking::v1::IngressLoadBalancerStatus>,
}

impl crate::DeepMerge for IngressStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.load_balancer, other.load_balancer);
    }
}

impl<'de> crate::serde::Deserialize<'de> for IngressStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_load_balancer,
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
                            "loadBalancer" => Field::Key_load_balancer,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = IngressStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("IngressStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_load_balancer: Option<crate::api::networking::v1::IngressLoadBalancerStatus> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_load_balancer => value_load_balancer = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IngressStatus {
                    load_balancer: value_load_balancer,
                })
            }
        }

        deserializer.deserialize_struct(
            "IngressStatus",
            &[
                "loadBalancer",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for IngressStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IngressStatus",
            self.load_balancer.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.load_balancer {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "loadBalancer", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for IngressStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.networking.v1.IngressStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "IngressStatus describe the current state of the Ingress.",
            "type": "object",
            "properties": {
                "loadBalancer": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::networking::v1::IngressLoadBalancerStatus>();
                    schema_obj.ensure_object().insert("description".into(), "loadBalancer contains the current status of the load-balancer.".into());
                    schema_obj
                }),
            },
        })
    }
}
