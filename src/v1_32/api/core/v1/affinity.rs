// Generated from definition io.k8s.api.core.v1.Affinity

/// Affinity is a group of affinity scheduling rules.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Affinity {
    /// Describes node affinity scheduling rules for the pod.
    pub node_affinity: Option<crate::api::core::v1::NodeAffinity>,

    /// Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
    pub pod_affinity: Option<crate::api::core::v1::PodAffinity>,

    /// Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
    pub pod_anti_affinity: Option<crate::api::core::v1::PodAntiAffinity>,
}

impl crate::DeepMerge for Affinity {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.node_affinity, other.node_affinity);
        crate::DeepMerge::merge_from(&mut self.pod_affinity, other.pod_affinity);
        crate::DeepMerge::merge_from(&mut self.pod_anti_affinity, other.pod_anti_affinity);
    }
}

impl<'de> crate::serde::Deserialize<'de> for Affinity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_node_affinity,
            Key_pod_affinity,
            Key_pod_anti_affinity,
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
                            "nodeAffinity" => Field::Key_node_affinity,
                            "podAffinity" => Field::Key_pod_affinity,
                            "podAntiAffinity" => Field::Key_pod_anti_affinity,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Affinity;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("Affinity")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_node_affinity: Option<crate::api::core::v1::NodeAffinity> = None;
                let mut value_pod_affinity: Option<crate::api::core::v1::PodAffinity> = None;
                let mut value_pod_anti_affinity: Option<crate::api::core::v1::PodAntiAffinity> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_node_affinity => value_node_affinity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_affinity => value_pod_affinity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_anti_affinity => value_pod_anti_affinity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Affinity {
                    node_affinity: value_node_affinity,
                    pod_affinity: value_pod_affinity,
                    pod_anti_affinity: value_pod_anti_affinity,
                })
            }
        }

        deserializer.deserialize_struct(
            "Affinity",
            &[
                "nodeAffinity",
                "podAffinity",
                "podAntiAffinity",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Affinity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Affinity",
            self.node_affinity.as_ref().map_or(0, |_| 1) +
            self.pod_affinity.as_ref().map_or(0, |_| 1) +
            self.pod_anti_affinity.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.node_affinity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeAffinity", value)?;
        }
        if let Some(value) = &self.pod_affinity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podAffinity", value)?;
        }
        if let Some(value) = &self.pod_anti_affinity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podAntiAffinity", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Affinity {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.Affinity".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Affinity is a group of affinity scheduling rules.",
            "type": "object",
            "properties": {
                "nodeAffinity": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NodeAffinity>();
                    schema_obj.ensure_object().insert("description".into(), "Describes node affinity scheduling rules for the pod.".into());
                    schema_obj
                }),
                "podAffinity": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PodAffinity>();
                    schema_obj.ensure_object().insert("description".into(), "Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).".into());
                    schema_obj
                }),
                "podAntiAffinity": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PodAntiAffinity>();
                    schema_obj.ensure_object().insert("description".into(), "Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).".into());
                    schema_obj
                }),
            },
        })
    }
}
