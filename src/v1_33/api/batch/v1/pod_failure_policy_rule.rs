// Generated from definition io.k8s.api.batch.v1.PodFailurePolicyRule

/// PodFailurePolicyRule describes how a pod failure is handled when the requirements are met. One of onExitCodes and onPodConditions, but not both, can be used in each rule.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodFailurePolicyRule {
    /// Specifies the action taken on a pod failure when the requirements are satisfied. Possible values are:
    ///
    /// - FailJob: indicates that the pod's job is marked as Failed and all
    ///   running pods are terminated.
    /// - FailIndex: indicates that the pod's index is marked as Failed and will
    ///   not be restarted.
    /// - Ignore: indicates that the counter towards the .backoffLimit is not
    ///   incremented and a replacement pod is created.
    /// - Count: indicates that the pod is handled in the default way - the
    ///   counter towards the .backoffLimit is incremented.
    /// Additional values are considered to be added in the future. Clients should react to an unknown action by skipping the rule.
    pub action: std::string::String,

    /// Represents the requirement on the container exit codes.
    pub on_exit_codes: Option<crate::api::batch::v1::PodFailurePolicyOnExitCodesRequirement>,

    /// Represents the requirement on the pod conditions. The requirement is represented as a list of pod condition patterns. The requirement is satisfied if at least one pattern matches an actual pod condition. At most 20 elements are allowed.
    pub on_pod_conditions: Option<std::vec::Vec<crate::api::batch::v1::PodFailurePolicyOnPodConditionsPattern>>,
}

impl crate::DeepMerge for PodFailurePolicyRule {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.action, other.action);
        crate::DeepMerge::merge_from(&mut self.on_exit_codes, other.on_exit_codes);
        crate::merge_strategies::list::atomic(&mut self.on_pod_conditions, other.on_pod_conditions);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PodFailurePolicyRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_action,
            Key_on_exit_codes,
            Key_on_pod_conditions,
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
                            "action" => Field::Key_action,
                            "onExitCodes" => Field::Key_on_exit_codes,
                            "onPodConditions" => Field::Key_on_pod_conditions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodFailurePolicyRule;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PodFailurePolicyRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_action: Option<std::string::String> = None;
                let mut value_on_exit_codes: Option<crate::api::batch::v1::PodFailurePolicyOnExitCodesRequirement> = None;
                let mut value_on_pod_conditions: Option<std::vec::Vec<crate::api::batch::v1::PodFailurePolicyOnPodConditionsPattern>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_action => value_action = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_on_exit_codes => value_on_exit_codes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_on_pod_conditions => value_on_pod_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodFailurePolicyRule {
                    action: value_action.unwrap_or_default(),
                    on_exit_codes: value_on_exit_codes,
                    on_pod_conditions: value_on_pod_conditions,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodFailurePolicyRule",
            &[
                "action",
                "onExitCodes",
                "onPodConditions",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodFailurePolicyRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodFailurePolicyRule",
            1 +
            self.on_exit_codes.as_ref().map_or(0, |_| 1) +
            self.on_pod_conditions.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "action", &self.action)?;
        if let Some(value) = &self.on_exit_codes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "onExitCodes", value)?;
        }
        if let Some(value) = &self.on_pod_conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "onPodConditions", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodFailurePolicyRule {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.batch.v1.PodFailurePolicyRule".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "PodFailurePolicyRule describes how a pod failure is handled when the requirements are met. One of onExitCodes and onPodConditions, but not both, can be used in each rule.",
            "type": "object",
            "properties": {
                "action": {
                    "description": "Specifies the action taken on a pod failure when the requirements are satisfied. Possible values are:\n\n- FailJob: indicates that the pod's job is marked as Failed and all\n  running pods are terminated.\n- FailIndex: indicates that the pod's index is marked as Failed and will\n  not be restarted.\n- Ignore: indicates that the counter towards the .backoffLimit is not\n  incremented and a replacement pod is created.\n- Count: indicates that the pod is handled in the default way - the\n  counter towards the .backoffLimit is incremented.\nAdditional values are considered to be added in the future. Clients should react to an unknown action by skipping the rule.",
                    "type": "string",
                },
                "onExitCodes": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::batch::v1::PodFailurePolicyOnExitCodesRequirement>();
                    schema_obj.ensure_object().insert("description".into(), "Represents the requirement on the container exit codes.".into());
                    schema_obj
                }),
                "onPodConditions": {
                    "description": "Represents the requirement on the pod conditions. The requirement is represented as a list of pod condition patterns. The requirement is satisfied if at least one pattern matches an actual pod condition. At most 20 elements are allowed.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::batch::v1::PodFailurePolicyOnPodConditionsPattern>()),
                },
            },
            "required": [
                "action",
            ],
        })
    }
}
