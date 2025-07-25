// Generated from definition io.k8s.api.core.v1.FCVolumeSource

/// Represents a Fibre Channel volume. Fibre Channel volumes can only be mounted as read/write once. Fibre Channel volumes support ownership management and SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FCVolumeSource {
    /// fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    pub fs_type: Option<std::string::String>,

    /// lun is Optional: FC target lun number
    pub lun: Option<i32>,

    /// readOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// targetWWNs is Optional: FC target worldwide names (WWNs)
    pub target_wwns: Option<std::vec::Vec<std::string::String>>,

    /// wwids Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
    pub wwids: Option<std::vec::Vec<std::string::String>>,
}

impl crate::DeepMerge for FCVolumeSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.fs_type, other.fs_type);
        crate::DeepMerge::merge_from(&mut self.lun, other.lun);
        crate::DeepMerge::merge_from(&mut self.read_only, other.read_only);
        crate::merge_strategies::list::atomic(&mut self.target_wwns, other.target_wwns);
        crate::merge_strategies::list::atomic(&mut self.wwids, other.wwids);
    }
}

impl<'de> crate::serde::Deserialize<'de> for FCVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_fs_type,
            Key_lun,
            Key_read_only,
            Key_target_wwns,
            Key_wwids,
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
                            "lun" => Field::Key_lun,
                            "readOnly" => Field::Key_read_only,
                            "targetWWNs" => Field::Key_target_wwns,
                            "wwids" => Field::Key_wwids,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = FCVolumeSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("FCVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_fs_type: Option<std::string::String> = None;
                let mut value_lun: Option<i32> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_target_wwns: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_wwids: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_lun => value_lun = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_wwns => value_target_wwns = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_wwids => value_wwids = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FCVolumeSource {
                    fs_type: value_fs_type,
                    lun: value_lun,
                    read_only: value_read_only,
                    target_wwns: value_target_wwns,
                    wwids: value_wwids,
                })
            }
        }

        deserializer.deserialize_struct(
            "FCVolumeSource",
            &[
                "fsType",
                "lun",
                "readOnly",
                "targetWWNs",
                "wwids",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for FCVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FCVolumeSource",
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.lun.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.target_wwns.as_ref().map_or(0, |_| 1) +
            self.wwids.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.lun {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lun", value)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.target_wwns {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "targetWWNs", value)?;
        }
        if let Some(value) = &self.wwids {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "wwids", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FCVolumeSource {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.FCVolumeSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Represents a Fibre Channel volume. Fibre Channel volumes can only be mounted as read/write once. Fibre Channel volumes support ownership management and SELinux relabeling.",
            "type": "object",
            "properties": {
                "fsType": {
                    "description": "fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified.",
                    "type": "string",
                },
                "lun": {
                    "description": "lun is Optional: FC target lun number",
                    "type": "integer",
                    "format": "int32",
                },
                "readOnly": {
                    "description": "readOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.",
                    "type": "boolean",
                },
                "targetWWNs": {
                    "description": "targetWWNs is Optional: FC target worldwide names (WWNs)",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "wwids": {
                    "description": "wwids Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
            },
        })
    }
}
