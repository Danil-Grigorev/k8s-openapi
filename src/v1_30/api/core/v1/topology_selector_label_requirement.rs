// Generated from definition io.k8s.api.core.v1.TopologySelectorLabelRequirement

/// A topology selector requirement is a selector that matches given label. This is an alpha feature and may change in the future.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TopologySelectorLabelRequirement {
    /// The label key that the selector applies to.
    pub key: std::string::String,

    /// An array of string values. One value must match the label to be selected. Each entry in Values is ORed.
    pub values: std::vec::Vec<std::string::String>,
}

impl crate::DeepMerge for TopologySelectorLabelRequirement {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.key, other.key);
        crate::merge_strategies::list::atomic(&mut self.values, other.values);
    }
}

impl<'de> crate::serde::Deserialize<'de> for TopologySelectorLabelRequirement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_key,
            Key_values,
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
                            "key" => Field::Key_key,
                            "values" => Field::Key_values,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = TopologySelectorLabelRequirement;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("TopologySelectorLabelRequirement")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_key: Option<std::string::String> = None;
                let mut value_values: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_key => value_key = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_values => value_values = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TopologySelectorLabelRequirement {
                    key: value_key.unwrap_or_default(),
                    values: value_values.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "TopologySelectorLabelRequirement",
            &[
                "key",
                "values",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for TopologySelectorLabelRequirement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TopologySelectorLabelRequirement",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "key", &self.key)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "values", &self.values)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for TopologySelectorLabelRequirement {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.TopologySelectorLabelRequirement".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "A topology selector requirement is a selector that matches given label. This is an alpha feature and may change in the future.",
            "type": "object",
            "properties": {
                "key": {
                    "description": "The label key that the selector applies to.",
                    "type": "string",
                },
                "values": {
                    "description": "An array of string values. One value must match the label to be selected. Each entry in Values is ORed.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
            },
            "required": [
                "key",
                "values",
            ],
        })
    }
}
