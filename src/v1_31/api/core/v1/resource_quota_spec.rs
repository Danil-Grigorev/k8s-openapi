// Generated from definition io.k8s.api.core.v1.ResourceQuotaSpec

/// ResourceQuotaSpec defines the desired hard limits to enforce for Quota.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceQuotaSpec {
    /// hard is the set of desired hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/
    pub hard: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// scopeSelector is also a collection of filters like scopes that must match each object tracked by a quota but expressed using ScopeSelectorOperator in combination with possible values. For a resource to match, both scopes AND scopeSelector (if specified in spec), must be matched.
    pub scope_selector: Option<crate::api::core::v1::ScopeSelector>,

    /// A collection of filters that must match each object tracked by a quota. If not specified, the quota matches all objects.
    pub scopes: Option<std::vec::Vec<std::string::String>>,
}

impl crate::DeepMerge for ResourceQuotaSpec {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::map::granular(&mut self.hard, other.hard, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::DeepMerge::merge_from(&mut self.scope_selector, other.scope_selector);
        crate::merge_strategies::list::atomic(&mut self.scopes, other.scopes);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ResourceQuotaSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_hard,
            Key_scope_selector,
            Key_scopes,
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
                            "hard" => Field::Key_hard,
                            "scopeSelector" => Field::Key_scope_selector,
                            "scopes" => Field::Key_scopes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceQuotaSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ResourceQuotaSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_hard: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_scope_selector: Option<crate::api::core::v1::ScopeSelector> = None;
                let mut value_scopes: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_hard => value_hard = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scope_selector => value_scope_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scopes => value_scopes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceQuotaSpec {
                    hard: value_hard,
                    scope_selector: value_scope_selector,
                    scopes: value_scopes,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceQuotaSpec",
            &[
                "hard",
                "scopeSelector",
                "scopes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ResourceQuotaSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceQuotaSpec",
            self.hard.as_ref().map_or(0, |_| 1) +
            self.scope_selector.as_ref().map_or(0, |_| 1) +
            self.scopes.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.hard {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hard", value)?;
        }
        if let Some(value) = &self.scope_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scopeSelector", value)?;
        }
        if let Some(value) = &self.scopes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scopes", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ResourceQuotaSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.ResourceQuotaSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ResourceQuotaSpec defines the desired hard limits to enforce for Quota.",
            "type": "object",
            "properties": {
                "hard": {
                    "description": "hard is the set of desired hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/",
                    "type": "object",
                    "additionalProperties": (__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>()),
                },
                "scopeSelector": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ScopeSelector>();
                    schema_obj.ensure_object().insert("description".into(), "scopeSelector is also a collection of filters like scopes that must match each object tracked by a quota but expressed using ScopeSelectorOperator in combination with possible values. For a resource to match, both scopes AND scopeSelector (if specified in spec), must be matched.".into());
                    schema_obj
                }),
                "scopes": {
                    "description": "A collection of filters that must match each object tracked by a quota. If not specified, the quota matches all objects.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
            },
        })
    }
}
