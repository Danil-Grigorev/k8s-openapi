// Generated from definition io.k8s.api.core.v1.LocalVolumeSource

/// Local represents directly-attached storage with node affinity (Beta feature)
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LocalVolumeSource {
    /// fsType is the filesystem type to mount. It applies only when the Path is a block device. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default value is to auto-select a filesystem if unspecified.
    pub fs_type: Option<std::string::String>,

    /// path of the full path to the volume on the node. It can be either a directory or block device (disk, partition, ...).
    pub path: std::string::String,
}

impl crate::DeepMerge for LocalVolumeSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.fs_type, other.fs_type);
        crate::DeepMerge::merge_from(&mut self.path, other.path);
    }
}

impl<'de> crate::serde::Deserialize<'de> for LocalVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fs_type,
            Key_path,
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
                            "fsType" => Field::Key_fs_type,
                            "path" => Field::Key_path,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = LocalVolumeSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("LocalVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<std::string::String> = None;
                let mut value_path: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_path => value_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LocalVolumeSource {
                    fs_type: value_fs_type,
                    path: value_path.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "LocalVolumeSource",
            &[
                "fsType",
                "path",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for LocalVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LocalVolumeSource",
            1 +
            self.fs_type.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "path", &self.path)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for LocalVolumeSource {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.LocalVolumeSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Local represents directly-attached storage with node affinity (Beta feature)",
            "type": "object",
            "properties": {
                "fsType": {
                    "description": "fsType is the filesystem type to mount. It applies only when the Path is a block device. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". The default value is to auto-select a filesystem if unspecified.",
                    "type": "string",
                },
                "path": {
                    "description": "path of the full path to the volume on the node. It can be either a directory or block device (disk, partition, ...).",
                    "type": "string",
                },
            },
            "required": [
                "path",
            ],
        })
    }
}
