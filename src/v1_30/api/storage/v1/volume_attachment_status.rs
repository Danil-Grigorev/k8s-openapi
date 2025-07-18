// Generated from definition io.k8s.api.storage.v1.VolumeAttachmentStatus

/// VolumeAttachmentStatus is the status of a VolumeAttachment request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeAttachmentStatus {
    /// attachError represents the last error encountered during attach operation, if any. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.
    pub attach_error: Option<crate::api::storage::v1::VolumeError>,

    /// attached indicates the volume is successfully attached. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.
    pub attached: bool,

    /// attachmentMetadata is populated with any information returned by the attach operation, upon successful attach, that must be passed into subsequent WaitForAttach or Mount calls. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.
    pub attachment_metadata: Option<std::collections::BTreeMap<std::string::String, std::string::String>>,

    /// detachError represents the last error encountered during detach operation, if any. This field must only be set by the entity completing the detach operation, i.e. the external-attacher.
    pub detach_error: Option<crate::api::storage::v1::VolumeError>,
}

impl crate::DeepMerge for VolumeAttachmentStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.attach_error, other.attach_error);
        crate::DeepMerge::merge_from(&mut self.attached, other.attached);
        crate::merge_strategies::map::granular(&mut self.attachment_metadata, other.attachment_metadata, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::DeepMerge::merge_from(&mut self.detach_error, other.detach_error);
    }
}

impl<'de> crate::serde::Deserialize<'de> for VolumeAttachmentStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_attach_error,
            Key_attached,
            Key_attachment_metadata,
            Key_detach_error,
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
                            "attachError" => Field::Key_attach_error,
                            "attached" => Field::Key_attached,
                            "attachmentMetadata" => Field::Key_attachment_metadata,
                            "detachError" => Field::Key_detach_error,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeAttachmentStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("VolumeAttachmentStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_attach_error: Option<crate::api::storage::v1::VolumeError> = None;
                let mut value_attached: Option<bool> = None;
                let mut value_attachment_metadata: Option<std::collections::BTreeMap<std::string::String, std::string::String>> = None;
                let mut value_detach_error: Option<crate::api::storage::v1::VolumeError> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_attach_error => value_attach_error = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_attached => value_attached = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_attachment_metadata => value_attachment_metadata = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_detach_error => value_detach_error = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeAttachmentStatus {
                    attach_error: value_attach_error,
                    attached: value_attached.unwrap_or_default(),
                    attachment_metadata: value_attachment_metadata,
                    detach_error: value_detach_error,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeAttachmentStatus",
            &[
                "attachError",
                "attached",
                "attachmentMetadata",
                "detachError",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for VolumeAttachmentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeAttachmentStatus",
            1 +
            self.attach_error.as_ref().map_or(0, |_| 1) +
            self.attachment_metadata.as_ref().map_or(0, |_| 1) +
            self.detach_error.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.attach_error {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "attachError", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "attached", &self.attached)?;
        if let Some(value) = &self.attachment_metadata {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "attachmentMetadata", value)?;
        }
        if let Some(value) = &self.detach_error {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "detachError", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VolumeAttachmentStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.storage.v1.VolumeAttachmentStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "VolumeAttachmentStatus is the status of a VolumeAttachment request.",
            "type": "object",
            "properties": {
                "attachError": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::storage::v1::VolumeError>();
                    schema_obj.ensure_object().insert("description".into(), "attachError represents the last error encountered during attach operation, if any. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.".into());
                    schema_obj
                }),
                "attached": {
                    "description": "attached indicates the volume is successfully attached. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.",
                    "type": "boolean",
                },
                "attachmentMetadata": {
                    "description": "attachmentMetadata is populated with any information returned by the attach operation, upon successful attach, that must be passed into subsequent WaitForAttach or Mount calls. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.",
                    "type": "object",
                    "additionalProperties": {
                        "type": "string",
                    },
                },
                "detachError": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::storage::v1::VolumeError>();
                    schema_obj.ensure_object().insert("description".into(), "detachError represents the last error encountered during detach operation, if any. This field must only be set by the entity completing the detach operation, i.e. the external-attacher.".into());
                    schema_obj
                }),
            },
            "required": [
                "attached",
            ],
        })
    }
}
