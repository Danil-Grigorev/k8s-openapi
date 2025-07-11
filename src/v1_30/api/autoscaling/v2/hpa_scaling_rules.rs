// Generated from definition io.k8s.api.autoscaling.v2.HPAScalingRules

/// HPAScalingRules configures the scaling behavior for one direction. These Rules are applied after calculating DesiredReplicas from metrics for the HPA. They can limit the scaling velocity by specifying scaling policies. They can prevent flapping by specifying the stabilization window, so that the number of replicas is not set instantly, instead, the safest value from the stabilization window is chosen.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HPAScalingRules {
    /// policies is a list of potential scaling polices which can be used during scaling. At least one policy must be specified, otherwise the HPAScalingRules will be discarded as invalid
    pub policies: Option<std::vec::Vec<crate::api::autoscaling::v2::HPAScalingPolicy>>,

    /// selectPolicy is used to specify which policy should be used. If not set, the default value Max is used.
    pub select_policy: Option<std::string::String>,

    /// stabilizationWindowSeconds is the number of seconds for which past recommendations should be considered while scaling up or scaling down. StabilizationWindowSeconds must be greater than or equal to zero and less than or equal to 3600 (one hour). If not set, use the default values: - For scale up: 0 (i.e. no stabilization is done). - For scale down: 300 (i.e. the stabilization window is 300 seconds long).
    pub stabilization_window_seconds: Option<i32>,
}

impl crate::DeepMerge for HPAScalingRules {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.policies, other.policies);
        crate::DeepMerge::merge_from(&mut self.select_policy, other.select_policy);
        crate::DeepMerge::merge_from(&mut self.stabilization_window_seconds, other.stabilization_window_seconds);
    }
}

impl<'de> crate::serde::Deserialize<'de> for HPAScalingRules {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_policies,
            Key_select_policy,
            Key_stabilization_window_seconds,
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
                            "policies" => Field::Key_policies,
                            "selectPolicy" => Field::Key_select_policy,
                            "stabilizationWindowSeconds" => Field::Key_stabilization_window_seconds,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = HPAScalingRules;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("HPAScalingRules")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_policies: Option<std::vec::Vec<crate::api::autoscaling::v2::HPAScalingPolicy>> = None;
                let mut value_select_policy: Option<std::string::String> = None;
                let mut value_stabilization_window_seconds: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_policies => value_policies = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_select_policy => value_select_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_stabilization_window_seconds => value_stabilization_window_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(HPAScalingRules {
                    policies: value_policies,
                    select_policy: value_select_policy,
                    stabilization_window_seconds: value_stabilization_window_seconds,
                })
            }
        }

        deserializer.deserialize_struct(
            "HPAScalingRules",
            &[
                "policies",
                "selectPolicy",
                "stabilizationWindowSeconds",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for HPAScalingRules {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "HPAScalingRules",
            self.policies.as_ref().map_or(0, |_| 1) +
            self.select_policy.as_ref().map_or(0, |_| 1) +
            self.stabilization_window_seconds.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.policies {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "policies", value)?;
        }
        if let Some(value) = &self.select_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selectPolicy", value)?;
        }
        if let Some(value) = &self.stabilization_window_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "stabilizationWindowSeconds", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for HPAScalingRules {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.autoscaling.v2.HPAScalingRules".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "HPAScalingRules configures the scaling behavior for one direction. These Rules are applied after calculating DesiredReplicas from metrics for the HPA. They can limit the scaling velocity by specifying scaling policies. They can prevent flapping by specifying the stabilization window, so that the number of replicas is not set instantly, instead, the safest value from the stabilization window is chosen.",
            "type": "object",
            "properties": {
                "policies": {
                    "description": "policies is a list of potential scaling polices which can be used during scaling. At least one policy must be specified, otherwise the HPAScalingRules will be discarded as invalid",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::autoscaling::v2::HPAScalingPolicy>()),
                },
                "selectPolicy": {
                    "description": "selectPolicy is used to specify which policy should be used. If not set, the default value Max is used.",
                    "type": "string",
                },
                "stabilizationWindowSeconds": {
                    "description": "stabilizationWindowSeconds is the number of seconds for which past recommendations should be considered while scaling up or scaling down. StabilizationWindowSeconds must be greater than or equal to zero and less than or equal to 3600 (one hour). If not set, use the default values: - For scale up: 0 (i.e. no stabilization is done). - For scale down: 300 (i.e. the stabilization window is 300 seconds long).",
                    "type": "integer",
                    "format": "int32",
                },
            },
        })
    }
}
