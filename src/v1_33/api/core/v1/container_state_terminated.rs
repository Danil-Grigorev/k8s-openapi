// Generated from definition io.k8s.api.core.v1.ContainerStateTerminated

/// ContainerStateTerminated is a terminated state of a container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerStateTerminated {
    /// Container's ID in the format '\<type\>://\<container_id\>'
    pub container_id: Option<std::string::String>,

    /// Exit status from the last termination of the container
    pub exit_code: i32,

    /// Time at which the container last terminated
    pub finished_at: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

    /// Message regarding the last termination of the container
    pub message: Option<std::string::String>,

    /// (brief) reason from the last termination of the container
    pub reason: Option<std::string::String>,

    /// Signal from the last termination of the container
    pub signal: Option<i32>,

    /// Time at which previous execution of the container started
    pub started_at: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,
}

impl crate::DeepMerge for ContainerStateTerminated {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.container_id, other.container_id);
        crate::DeepMerge::merge_from(&mut self.exit_code, other.exit_code);
        crate::DeepMerge::merge_from(&mut self.finished_at, other.finished_at);
        crate::DeepMerge::merge_from(&mut self.message, other.message);
        crate::DeepMerge::merge_from(&mut self.reason, other.reason);
        crate::DeepMerge::merge_from(&mut self.signal, other.signal);
        crate::DeepMerge::merge_from(&mut self.started_at, other.started_at);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ContainerStateTerminated {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_container_id,
            Key_exit_code,
            Key_finished_at,
            Key_message,
            Key_reason,
            Key_signal,
            Key_started_at,
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
                            "containerID" => Field::Key_container_id,
                            "exitCode" => Field::Key_exit_code,
                            "finishedAt" => Field::Key_finished_at,
                            "message" => Field::Key_message,
                            "reason" => Field::Key_reason,
                            "signal" => Field::Key_signal,
                            "startedAt" => Field::Key_started_at,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerStateTerminated;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ContainerStateTerminated")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_container_id: Option<std::string::String> = None;
                let mut value_exit_code: Option<i32> = None;
                let mut value_finished_at: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;
                let mut value_message: Option<std::string::String> = None;
                let mut value_reason: Option<std::string::String> = None;
                let mut value_signal: Option<i32> = None;
                let mut value_started_at: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_container_id => value_container_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_exit_code => value_exit_code = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_finished_at => value_finished_at = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_signal => value_signal = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_started_at => value_started_at = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerStateTerminated {
                    container_id: value_container_id,
                    exit_code: value_exit_code.unwrap_or_default(),
                    finished_at: value_finished_at,
                    message: value_message,
                    reason: value_reason,
                    signal: value_signal,
                    started_at: value_started_at,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerStateTerminated",
            &[
                "containerID",
                "exitCode",
                "finishedAt",
                "message",
                "reason",
                "signal",
                "startedAt",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ContainerStateTerminated {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerStateTerminated",
            1 +
            self.container_id.as_ref().map_or(0, |_| 1) +
            self.finished_at.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1) +
            self.signal.as_ref().map_or(0, |_| 1) +
            self.started_at.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.container_id {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "containerID", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "exitCode", &self.exit_code)?;
        if let Some(value) = &self.finished_at {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "finishedAt", value)?;
        }
        if let Some(value) = &self.message {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        if let Some(value) = &self.reason {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        if let Some(value) = &self.signal {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "signal", value)?;
        }
        if let Some(value) = &self.started_at {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "startedAt", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ContainerStateTerminated {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.ContainerStateTerminated".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ContainerStateTerminated is a terminated state of a container.",
            "type": "object",
            "properties": {
                "containerID": {
                    "description": "Container's ID in the format '<type>://<container_id>'",
                    "type": "string",
                },
                "exitCode": {
                    "description": "Exit status from the last termination of the container",
                    "type": "integer",
                    "format": "int32",
                },
                "finishedAt": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>();
                    schema_obj.ensure_object().insert("description".into(), "Time at which the container last terminated".into());
                    schema_obj
                }),
                "message": {
                    "description": "Message regarding the last termination of the container",
                    "type": "string",
                },
                "reason": {
                    "description": "(brief) reason from the last termination of the container",
                    "type": "string",
                },
                "signal": {
                    "description": "Signal from the last termination of the container",
                    "type": "integer",
                    "format": "int32",
                },
                "startedAt": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>();
                    schema_obj.ensure_object().insert("description".into(), "Time at which previous execution of the container started".into());
                    schema_obj
                }),
            },
            "required": [
                "exitCode",
            ],
        })
    }
}
