// Generated from definition io.k8s.api.core.v1.FlexPersistentVolumeSource

/// FlexPersistentVolumeSource represents a generic persistent volume resource that is provisioned/attached using an exec based plugin.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlexPersistentVolumeSource {
    /// driver is the name of the driver to use for this volume.
    pub driver: std::string::String,

    /// fsType is the Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
    pub fs_type: Option<std::string::String>,

    /// options is Optional: this field holds extra command options if any.
    pub options: Option<std::collections::BTreeMap<std::string::String, std::string::String>>,

    /// readOnly is Optional: defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// secretRef is Optional: SecretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.
    pub secret_ref: Option<crate::api::core::v1::SecretReference>,
}

impl crate::DeepMerge for FlexPersistentVolumeSource {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.driver, other.driver);
        crate::DeepMerge::merge_from(&mut self.fs_type, other.fs_type);
        crate::merge_strategies::map::granular(&mut self.options, other.options, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::DeepMerge::merge_from(&mut self.read_only, other.read_only);
        crate::DeepMerge::merge_from(&mut self.secret_ref, other.secret_ref);
    }
}

impl<'de> crate::serde::Deserialize<'de> for FlexPersistentVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_driver,
            Key_fs_type,
            Key_options,
            Key_read_only,
            Key_secret_ref,
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
                            "options" => Field::Key_options,
                            "readOnly" => Field::Key_read_only,
                            "secretRef" => Field::Key_secret_ref,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = FlexPersistentVolumeSource;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("FlexPersistentVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_driver: Option<std::string::String> = None;
                let mut value_fs_type: Option<std::string::String> = None;
                let mut value_options: Option<std::collections::BTreeMap<std::string::String, std::string::String>> = None;
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_ref: Option<crate::api::core::v1::SecretReference> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_driver => value_driver = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fs_type => value_fs_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_options => value_options = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_ref => value_secret_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FlexPersistentVolumeSource {
                    driver: value_driver.unwrap_or_default(),
                    fs_type: value_fs_type,
                    options: value_options,
                    read_only: value_read_only,
                    secret_ref: value_secret_ref,
                })
            }
        }

        deserializer.deserialize_struct(
            "FlexPersistentVolumeSource",
            &[
                "driver",
                "fsType",
                "options",
                "readOnly",
                "secretRef",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for FlexPersistentVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FlexPersistentVolumeSource",
            1 +
            self.fs_type.as_ref().map_or(0, |_| 1) +
            self.options.as_ref().map_or(0, |_| 1) +
            self.read_only.as_ref().map_or(0, |_| 1) +
            self.secret_ref.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "driver", &self.driver)?;
        if let Some(value) = &self.fs_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsType", value)?;
        }
        if let Some(value) = &self.options {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "options", value)?;
        }
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        if let Some(value) = &self.secret_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretRef", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FlexPersistentVolumeSource {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.FlexPersistentVolumeSource".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "FlexPersistentVolumeSource represents a generic persistent volume resource that is provisioned/attached using an exec based plugin.",
            "type": "object",
            "properties": {
                "driver": {
                    "description": "driver is the name of the driver to use for this volume.",
                    "type": "string",
                },
                "fsType": {
                    "description": "fsType is the Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". The default filesystem depends on FlexVolume script.",
                    "type": "string",
                },
                "options": {
                    "description": "options is Optional: this field holds extra command options if any.",
                    "type": "object",
                    "additionalProperties": {
                        "type": "string",
                    },
                },
                "readOnly": {
                    "description": "readOnly is Optional: defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.",
                    "type": "boolean",
                },
                "secretRef": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SecretReference>();
                    schema_obj.ensure_object().insert("description".into(), "secretRef is Optional: SecretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.".into());
                    schema_obj
                }),
            },
            "required": [
                "driver",
            ],
        })
    }
}
