// Generated from definition io.k8s.api.node.v1.Overhead

/// Overhead structure represents the resource overhead associated with running a pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Overhead {
    /// podFixed represents the fixed resource overhead associated with running a pod.
    pub pod_fixed: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>>,
}

impl crate::DeepMerge for Overhead {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::map::granular(&mut self.pod_fixed, other.pod_fixed, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
    }
}

impl<'de> crate::serde::Deserialize<'de> for Overhead {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_pod_fixed,
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
                            "podFixed" => Field::Key_pod_fixed,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Overhead;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("Overhead")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_pod_fixed: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_pod_fixed => value_pod_fixed = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Overhead {
                    pod_fixed: value_pod_fixed,
                })
            }
        }

        deserializer.deserialize_struct(
            "Overhead",
            &[
                "podFixed",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Overhead {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Overhead",
            self.pod_fixed.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.pod_fixed {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podFixed", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Overhead {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.node.v1.Overhead".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Overhead structure represents the resource overhead associated with running a pod.",
            "type": "object",
            "properties": {
                "podFixed": {
                    "description": "podFixed represents the fixed resource overhead associated with running a pod.",
                    "type": "object",
                    "additionalProperties": (__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>()),
                },
            },
        })
    }
}
