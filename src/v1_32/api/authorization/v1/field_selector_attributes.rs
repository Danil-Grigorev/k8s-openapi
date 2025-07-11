// Generated from definition io.k8s.api.authorization.v1.FieldSelectorAttributes

/// FieldSelectorAttributes indicates a field limited access. Webhook authors are encouraged to * ensure rawSelector and requirements are not both set * consider the requirements field if set * not try to parse or consider the rawSelector field if set. This is to avoid another CVE-2022-2880 (i.e. getting different systems to agree on how exactly to parse a query is not something we want), see https://www.oxeye.io/resources/golang-parameter-smuggling-attack for more details. For the *SubjectAccessReview endpoints of the kube-apiserver: * If rawSelector is empty and requirements are empty, the request is not limited. * If rawSelector is present and requirements are empty, the rawSelector will be parsed and limited if the parsing succeeds. * If rawSelector is empty and requirements are present, the requirements should be honored * If rawSelector is present and requirements are present, the request is invalid.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FieldSelectorAttributes {
    /// rawSelector is the serialization of a field selector that would be included in a query parameter. Webhook implementations are encouraged to ignore rawSelector. The kube-apiserver's *SubjectAccessReview will parse the rawSelector as long as the requirements are not present.
    pub raw_selector: Option<std::string::String>,

    /// requirements is the parsed interpretation of a field selector. All requirements must be met for a resource instance to match the selector. Webhook implementations should handle requirements, but how to handle them is up to the webhook. Since requirements can only limit the request, it is safe to authorize as unlimited request if the requirements are not understood.
    pub requirements: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement>>,
}

impl crate::DeepMerge for FieldSelectorAttributes {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.raw_selector, other.raw_selector);
        crate::merge_strategies::list::atomic(&mut self.requirements, other.requirements);
    }
}

impl<'de> crate::serde::Deserialize<'de> for FieldSelectorAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_raw_selector,
            Key_requirements,
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
                            "rawSelector" => Field::Key_raw_selector,
                            "requirements" => Field::Key_requirements,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = FieldSelectorAttributes;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("FieldSelectorAttributes")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_raw_selector: Option<std::string::String> = None;
                let mut value_requirements: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_raw_selector => value_raw_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_requirements => value_requirements = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FieldSelectorAttributes {
                    raw_selector: value_raw_selector,
                    requirements: value_requirements,
                })
            }
        }

        deserializer.deserialize_struct(
            "FieldSelectorAttributes",
            &[
                "rawSelector",
                "requirements",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for FieldSelectorAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FieldSelectorAttributes",
            self.raw_selector.as_ref().map_or(0, |_| 1) +
            self.requirements.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.raw_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rawSelector", value)?;
        }
        if let Some(value) = &self.requirements {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "requirements", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FieldSelectorAttributes {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.authorization.v1.FieldSelectorAttributes".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "FieldSelectorAttributes indicates a field limited access. Webhook authors are encouraged to * ensure rawSelector and requirements are not both set * consider the requirements field if set * not try to parse or consider the rawSelector field if set. This is to avoid another CVE-2022-2880 (i.e. getting different systems to agree on how exactly to parse a query is not something we want), see https://www.oxeye.io/resources/golang-parameter-smuggling-attack for more details. For the *SubjectAccessReview endpoints of the kube-apiserver: * If rawSelector is empty and requirements are empty, the request is not limited. * If rawSelector is present and requirements are empty, the rawSelector will be parsed and limited if the parsing succeeds. * If rawSelector is empty and requirements are present, the requirements should be honored * If rawSelector is present and requirements are present, the request is invalid.",
            "type": "object",
            "properties": {
                "rawSelector": {
                    "description": "rawSelector is the serialization of a field selector that would be included in a query parameter. Webhook implementations are encouraged to ignore rawSelector. The kube-apiserver's *SubjectAccessReview will parse the rawSelector as long as the requirements are not present.",
                    "type": "string",
                },
                "requirements": {
                    "description": "requirements is the parsed interpretation of a field selector. All requirements must be met for a resource instance to match the selector. Webhook implementations should handle requirements, but how to handle them is up to the webhook. Since requirements can only limit the request, it is safe to authorize as unlimited request if the requirements are not understood.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement>()),
                },
            },
        })
    }
}
