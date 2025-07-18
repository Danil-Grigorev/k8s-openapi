// Generated from definition io.k8s.api.core.v1.ScopeSelector

/// A scope selector represents the AND of the selectors represented by the scoped-resource selector requirements.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScopeSelector {
    /// A list of scope selector requirements by scope of the resources.
    pub match_expressions: Option<std::vec::Vec<crate::api::core::v1::ScopedResourceSelectorRequirement>>,
}

impl crate::DeepMerge for ScopeSelector {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.match_expressions, other.match_expressions);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ScopeSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_match_expressions,
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
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ScopeSelector;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ScopeSelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_match_expressions: Option<std::vec::Vec<crate::api::core::v1::ScopedResourceSelectorRequirement>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_match_expressions => value_match_expressions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ScopeSelector {
                    match_expressions: value_match_expressions,
                })
            }
        }

        deserializer.deserialize_struct(
            "ScopeSelector",
            &[
                "matchExpressions",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ScopeSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ScopeSelector",
            self.match_expressions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.match_expressions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchExpressions", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ScopeSelector {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.ScopeSelector".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "A scope selector represents the AND of the selectors represented by the scoped-resource selector requirements.",
            "type": "object",
            "properties": {
                "matchExpressions": {
                    "description": "A list of scope selector requirements by scope of the resources.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::ScopedResourceSelectorRequirement>()),
                },
            },
        })
    }
}
