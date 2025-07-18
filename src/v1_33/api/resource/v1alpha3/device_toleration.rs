// Generated from definition io.k8s.api.resource.v1alpha3.DeviceToleration

/// The ResourceClaim this DeviceToleration is attached to tolerates any taint that matches the triple \<key,value,effect\> using the matching operator \<operator\>.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceToleration {
    /// Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule and NoExecute.
    pub effect: Option<std::string::String>,

    /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys. Must be a label name.
    pub key: Option<std::string::String>,

    /// Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a ResourceClaim can tolerate all taints of a particular category.
    pub operator: Option<std::string::String>,

    /// TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system. If larger than zero, the time when the pod needs to be evicted is calculated as \<time when taint was adedd\> + \<toleration seconds\>.
    pub toleration_seconds: Option<i64>,

    /// Value is the taint value the toleration matches to. If the operator is Exists, the value must be empty, otherwise just a regular string. Must be a label value.
    pub value: Option<std::string::String>,
}

impl crate::DeepMerge for DeviceToleration {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.effect, other.effect);
        crate::DeepMerge::merge_from(&mut self.key, other.key);
        crate::DeepMerge::merge_from(&mut self.operator, other.operator);
        crate::DeepMerge::merge_from(&mut self.toleration_seconds, other.toleration_seconds);
        crate::DeepMerge::merge_from(&mut self.value, other.value);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceToleration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_effect,
            Key_key,
            Key_operator,
            Key_toleration_seconds,
            Key_value,
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
                            "effect" => Field::Key_effect,
                            "key" => Field::Key_key,
                            "operator" => Field::Key_operator,
                            "tolerationSeconds" => Field::Key_toleration_seconds,
                            "value" => Field::Key_value,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceToleration;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceToleration")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_effect: Option<std::string::String> = None;
                let mut value_key: Option<std::string::String> = None;
                let mut value_operator: Option<std::string::String> = None;
                let mut value_toleration_seconds: Option<i64> = None;
                let mut value_value: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_effect => value_effect = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_key => value_key = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_operator => value_operator = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_toleration_seconds => value_toleration_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_value => value_value = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceToleration {
                    effect: value_effect,
                    key: value_key,
                    operator: value_operator,
                    toleration_seconds: value_toleration_seconds,
                    value: value_value,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceToleration",
            &[
                "effect",
                "key",
                "operator",
                "tolerationSeconds",
                "value",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceToleration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceToleration",
            self.effect.as_ref().map_or(0, |_| 1) +
            self.key.as_ref().map_or(0, |_| 1) +
            self.operator.as_ref().map_or(0, |_| 1) +
            self.toleration_seconds.as_ref().map_or(0, |_| 1) +
            self.value.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.effect {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "effect", value)?;
        }
        if let Some(value) = &self.key {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "key", value)?;
        }
        if let Some(value) = &self.operator {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "operator", value)?;
        }
        if let Some(value) = &self.toleration_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tolerationSeconds", value)?;
        }
        if let Some(value) = &self.value {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "value", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceToleration {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1alpha3.DeviceToleration".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "The ResourceClaim this DeviceToleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.",
            "type": "object",
            "properties": {
                "effect": {
                    "description": "Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule and NoExecute.",
                    "type": "string",
                },
                "key": {
                    "description": "Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys. Must be a label name.",
                    "type": "string",
                },
                "operator": {
                    "description": "Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a ResourceClaim can tolerate all taints of a particular category.",
                    "type": "string",
                },
                "tolerationSeconds": {
                    "description": "TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system. If larger than zero, the time when the pod needs to be evicted is calculated as <time when taint was adedd> + <toleration seconds>.",
                    "type": "integer",
                    "format": "int64",
                },
                "value": {
                    "description": "Value is the taint value the toleration matches to. If the operator is Exists, the value must be empty, otherwise just a regular string. Must be a label value.",
                    "type": "string",
                },
            },
        })
    }
}
