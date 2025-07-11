// Generated from definition io.k8s.api.networking.v1.HTTPIngressRuleValue

/// HTTPIngressRuleValue is a list of http selectors pointing to backends. In the example: http://\<host\>/\<path\>?\<searchpart\> -\> backend where where parts of the url correspond to RFC 3986, this resource will be used to match against everything after the last '/' and before the first '?' or '#'.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HTTPIngressRuleValue {
    /// paths is a collection of paths that map requests to backends.
    pub paths: std::vec::Vec<crate::api::networking::v1::HTTPIngressPath>,
}

impl crate::DeepMerge for HTTPIngressRuleValue {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.paths, other.paths);
    }
}

impl<'de> crate::serde::Deserialize<'de> for HTTPIngressRuleValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_paths,
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
                            "paths" => Field::Key_paths,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = HTTPIngressRuleValue;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("HTTPIngressRuleValue")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_paths: Option<std::vec::Vec<crate::api::networking::v1::HTTPIngressPath>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_paths => value_paths = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HTTPIngressRuleValue {
                    paths: value_paths.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "HTTPIngressRuleValue",
            &[
                "paths",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for HTTPIngressRuleValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HTTPIngressRuleValue",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "paths", &self.paths)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for HTTPIngressRuleValue {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.networking.v1.HTTPIngressRuleValue".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "HTTPIngressRuleValue is a list of http selectors pointing to backends. In the example: http://<host>/<path>?<searchpart> -> backend where where parts of the url correspond to RFC 3986, this resource will be used to match against everything after the last '/' and before the first '?' or '#'.",
            "type": "object",
            "properties": {
                "paths": {
                    "description": "paths is a collection of paths that map requests to backends.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::networking::v1::HTTPIngressPath>()),
                },
            },
            "required": [
                "paths",
            ],
        })
    }
}
