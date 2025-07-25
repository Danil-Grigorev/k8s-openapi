// Generated from definition io.k8s.api.core.v1.NamespaceSpec

/// NamespaceSpec describes the attributes on a Namespace.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NamespaceSpec {
    /// Finalizers is an opaque list of values that must be empty to permanently remove object from storage. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/
    pub finalizers: Option<std::vec::Vec<std::string::String>>,
}

impl crate::DeepMerge for NamespaceSpec {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.finalizers, other.finalizers);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NamespaceSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_finalizers,
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
                            "finalizers" => Field::Key_finalizers,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NamespaceSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NamespaceSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_finalizers: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_finalizers => value_finalizers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NamespaceSpec {
                    finalizers: value_finalizers,
                })
            }
        }

        deserializer.deserialize_struct(
            "NamespaceSpec",
            &[
                "finalizers",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NamespaceSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NamespaceSpec",
            self.finalizers.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.finalizers {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "finalizers", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NamespaceSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.NamespaceSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "NamespaceSpec describes the attributes on a Namespace.",
            "type": "object",
            "properties": {
                "finalizers": {
                    "description": "Finalizers is an opaque list of values that must be empty to permanently remove object from storage. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
            },
        })
    }
}
