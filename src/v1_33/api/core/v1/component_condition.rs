// Generated from definition io.k8s.api.core.v1.ComponentCondition

/// Information about the condition of a component.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ComponentCondition {
    /// Condition error code for a component. For example, a health check error code.
    pub error: Option<std::string::String>,

    /// Message about the condition for a component. For example, information about a health check.
    pub message: Option<std::string::String>,

    /// Status of the condition for a component. Valid values for "Healthy": "True", "False", or "Unknown".
    pub status: std::string::String,

    /// Type of condition for a component. Valid value: "Healthy"
    pub type_: std::string::String,
}

impl crate::DeepMerge for ComponentCondition {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.error, other.error);
        crate::DeepMerge::merge_from(&mut self.message, other.message);
        crate::DeepMerge::merge_from(&mut self.status, other.status);
        crate::DeepMerge::merge_from(&mut self.type_, other.type_);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ComponentCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_error,
            Key_message,
            Key_status,
            Key_type_,
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
                            "error" => Field::Key_error,
                            "message" => Field::Key_message,
                            "status" => Field::Key_status,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ComponentCondition;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ComponentCondition")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_error: Option<std::string::String> = None;
                let mut value_message: Option<std::string::String> = None;
                let mut value_status: Option<std::string::String> = None;
                let mut value_type_: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_error => value_error = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ComponentCondition {
                    error: value_error,
                    message: value_message,
                    status: value_status.unwrap_or_default(),
                    type_: value_type_.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ComponentCondition",
            &[
                "error",
                "message",
                "status",
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ComponentCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ComponentCondition",
            2 +
            self.error.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.error {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "error", value)?;
        }
        if let Some(value) = &self.message {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "status", &self.status)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ComponentCondition {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.ComponentCondition".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Information about the condition of a component.",
            "type": "object",
            "properties": {
                "error": {
                    "description": "Condition error code for a component. For example, a health check error code.",
                    "type": "string",
                },
                "message": {
                    "description": "Message about the condition for a component. For example, information about a health check.",
                    "type": "string",
                },
                "status": {
                    "description": "Status of the condition for a component. Valid values for \"Healthy\": \"True\", \"False\", or \"Unknown\".",
                    "type": "string",
                },
                "type": {
                    "description": "Type of condition for a component. Valid value: \"Healthy\"",
                    "type": "string",
                },
            },
            "required": [
                "status",
                "type",
            ],
        })
    }
}
