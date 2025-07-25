// Generated from definition io.k8s.api.core.v1.CSIVolumeSource

/// Represents a source location of a volume to mount, managed by an external CSI driver
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CSIVolumeSource {
    /// driver is the name of the CSI driver that handles this volume. Consult with your admin for the correct name as registered in the cluster.
    pub driver: std::string::String,

    /// fsType to mount. Ex. "ext4", "xfs", "ntfs". If not provided, the empty value is passed to the associated CSI driver which will determine the default filesystem to apply.
    pub fs_type: Option<std::string::String>,

    /// nodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secret references are passed.
    pub node_publish_secret_ref: Option<crate::api::core::v1::LocalObjectReference>,

    /// readOnly specifies a read-only configuration for the volume. Defaults to false (read/write).
    pub read_only: Option<bool>,

    /// volumeAttributes stores driver-specific properties that are passed to the CSI driver. Consult your driver's documentation for supported values.
    pub volume_attributes: Option<std::collections::BTreeMap<std::string::String, std::string::String>>,
}

impl crate::DeepMerge for CSIVolumeSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.driver, other.driver);
        crate::DeepMerge::merge_from(&mut self.fs_type, other.fs_type);
        crate::DeepMerge::merge_from(&mut self.node_publish_secret_ref, other.node_publish_secret_ref);
        crate::DeepMerge::merge_from(&mut self.read_only, other.read_only);
        crate::merge_strategies::map::granular(&mut self.volume_attributes, other.volume_attributes, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
    }
}

impl<'de> crate::serde::Deserialize<'de> for CSIVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_driver,
            Key_fs_type,
            Key_node_publish_secret_ref,
            Key_read_only,
            Key_volume_attributes,
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
                            "driver" => Field::Key_driver,
                            "fsType" => Field::Key_fs_type,
                            "nodePublishSecretRef" => Field::Key_node_publish_secret_ref,
                            "readOnly" => Field::Key_read_only,
                            "volumeAttributes" => Field::Key_volume_attributes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CSIVolumeSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CSIVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_driver: Option<std::string::String> = None;
                let mut value_fs_type: Option<std::string::String> = None;
                let mut value_node_publish_secret_ref: Option<crate::api::core::v1::LocalObjectReference> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_volume_attributes: Option<std::collections::BTreeMap<std::string::String, std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_driver => value_driver = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_publish_secret_ref => value_node_publish_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_attributes => value_volume_attributes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CSIVolumeSource {
                    driver: value_driver.unwrap_or_default(),
                    fs_type: value_fs_type,
                    node_publish_secret_ref: value_node_publish_secret_ref,
                    read_only: value_read_only,
                    volume_attributes: value_volume_attributes,
                })
            }
        }

        deserializer.deserialize_struct(
            "CSIVolumeSource",
            &[
                "driver",
                "fsType",
                "nodePublishSecretRef",
                "readOnly",
                "volumeAttributes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CSIVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CSIVolumeSource",
            1 +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.node_publish_secret_ref.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.volume_attributes.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driver", &self.driver)?;
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.node_publish_secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodePublishSecretRef", value)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.volume_attributes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeAttributes", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CSIVolumeSource {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.CSIVolumeSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Represents a source location of a volume to mount, managed by an external CSI driver",
            "type": "object",
            "properties": {
                "driver": {
                    "description": "driver is the name of the CSI driver that handles this volume. Consult with your admin for the correct name as registered in the cluster.",
                    "type": "string",
                },
                "fsType": {
                    "description": "fsType to mount. Ex. \"ext4\", \"xfs\", \"ntfs\". If not provided, the empty value is passed to the associated CSI driver which will determine the default filesystem to apply.",
                    "type": "string",
                },
                "nodePublishSecretRef": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::LocalObjectReference>();
                    schema_obj.ensure_object().insert("description".into(), "nodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secret references are passed.".into());
                    schema_obj
                }),
                "readOnly": {
                    "description": "readOnly specifies a read-only configuration for the volume. Defaults to false (read/write).",
                    "type": "boolean",
                },
                "volumeAttributes": {
                    "description": "volumeAttributes stores driver-specific properties that are passed to the CSI driver. Consult your driver's documentation for supported values.",
                    "type": "object",
                    "additionalProperties": {
                        "type": "string",
                    },
                },
            },
            "required": [
                "driver",
            ],
        })
    }
}
