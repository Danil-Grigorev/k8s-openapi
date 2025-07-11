// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector

/// A label selector is a label query over a set of resources. The result of matchLabels and matchExpressions are ANDed. An empty label selector matches all objects. A null label selector matches no objects.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    pub match_expressions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement>>,

    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    pub match_labels: Option<std::collections::BTreeMap<std::string::String, std::string::String>>,
}

impl crate::DeepMerge for LabelSelector {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.match_expressions, other.match_expressions);
        crate::merge_strategies::map::granular(&mut self.match_labels, other.match_labels, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
    }
}

impl<'de> crate::serde::Deserialize<'de> for LabelSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_match_expressions,
            Key_match_labels,
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
                            "matchExpressions" => Field::Key_match_expressions,
                            "matchLabels" => Field::Key_match_labels,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = LabelSelector;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("LabelSelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_match_expressions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement>> = None;
                let mut value_match_labels: Option<std::collections::BTreeMap<std::string::String, std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_match_expressions => value_match_expressions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_match_labels => value_match_labels = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LabelSelector {
                    match_expressions: value_match_expressions,
                    match_labels: value_match_labels,
                })
            }
        }

        deserializer.deserialize_struct(
            "LabelSelector",
            &[
                "matchExpressions",
                "matchLabels",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for LabelSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LabelSelector",
            self.match_expressions.as_ref().map_or(0, |_| 1) +
            self.match_labels.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.match_expressions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchExpressions", value)?;
        }
        if let Some(value) = &self.match_labels {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchLabels", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for LabelSelector {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "A label selector is a label query over a set of resources. The result of matchLabels and matchExpressions are ANDed. An empty label selector matches all objects. A null label selector matches no objects.",
            "type": "object",
            "properties": {
                "matchExpressions": {
                    "description": "matchExpressions is a list of label selector requirements. The requirements are ANDed.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement>()),
                },
                "matchLabels": {
                    "description": "matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is \"key\", the operator is \"In\", and the values array contains only \"value\". The requirements are ANDed.",
                    "type": "object",
                    "additionalProperties": {
                        "type": "string",
                    },
                },
            },
        })
    }
}
