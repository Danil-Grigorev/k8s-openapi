// Generated from definition io.k8s.api.core.v1.ReplicationControllerStatus

/// ReplicationControllerStatus represents the current status of a replication controller.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReplicationControllerStatus {
    /// The number of available replicas (ready for at least minReadySeconds) for this replication controller.
    pub available_replicas: Option<i32>,

    /// Represents the latest available observations of a replication controller's current state.
    pub conditions: Option<std::vec::Vec<crate::api::core::v1::ReplicationControllerCondition>>,

    /// The number of pods that have labels matching the labels of the pod template of the replication controller.
    pub fully_labeled_replicas: Option<i32>,

    /// ObservedGeneration reflects the generation of the most recently observed replication controller.
    pub observed_generation: Option<i64>,

    /// The number of ready replicas for this replication controller.
    pub ready_replicas: Option<i32>,

    /// Replicas is the most recently observed number of replicas. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#what-is-a-replicationcontroller
    pub replicas: i32,
}

impl crate::DeepMerge for ReplicationControllerStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.available_replicas, other.available_replicas);
        crate::merge_strategies::list::map(
            &mut self.conditions,
            other.conditions,
            &[|lhs, rhs| lhs.type_ == rhs.type_],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.fully_labeled_replicas, other.fully_labeled_replicas);
        crate::DeepMerge::merge_from(&mut self.observed_generation, other.observed_generation);
        crate::DeepMerge::merge_from(&mut self.ready_replicas, other.ready_replicas);
        crate::DeepMerge::merge_from(&mut self.replicas, other.replicas);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ReplicationControllerStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_available_replicas,
            Key_conditions,
            Key_fully_labeled_replicas,
            Key_observed_generation,
            Key_ready_replicas,
            Key_replicas,
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
                            "availableReplicas" => Field::Key_available_replicas,
                            "conditions" => Field::Key_conditions,
                            "fullyLabeledReplicas" => Field::Key_fully_labeled_replicas,
                            "observedGeneration" => Field::Key_observed_generation,
                            "readyReplicas" => Field::Key_ready_replicas,
                            "replicas" => Field::Key_replicas,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ReplicationControllerStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ReplicationControllerStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_available_replicas: Option<i32> = None;
                let mut value_conditions: Option<std::vec::Vec<crate::api::core::v1::ReplicationControllerCondition>> = None;
                let mut value_fully_labeled_replicas: Option<i32> = None;
                let mut value_observed_generation: Option<i64> = None;
                let mut value_ready_replicas: Option<i32> = None;
                let mut value_replicas: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_available_replicas => value_available_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fully_labeled_replicas => value_fully_labeled_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observed_generation => value_observed_generation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ready_replicas => value_ready_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_replicas => value_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ReplicationControllerStatus {
                    available_replicas: value_available_replicas,
                    conditions: value_conditions,
                    fully_labeled_replicas: value_fully_labeled_replicas,
                    observed_generation: value_observed_generation,
                    ready_replicas: value_ready_replicas,
                    replicas: value_replicas.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ReplicationControllerStatus",
            &[
                "availableReplicas",
                "conditions",
                "fullyLabeledReplicas",
                "observedGeneration",
                "readyReplicas",
                "replicas",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ReplicationControllerStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ReplicationControllerStatus",
            1 +
            self.available_replicas.as_ref().map_or(0, |_| 1) +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.fully_labeled_replicas.as_ref().map_or(0, |_| 1) +
            self.observed_generation.as_ref().map_or(0, |_| 1) +
            self.ready_replicas.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.available_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "availableReplicas", value)?;
        }
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.fully_labeled_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fullyLabeledReplicas", value)?;
        }
        if let Some(value) = &self.observed_generation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "observedGeneration", value)?;
        }
        if let Some(value) = &self.ready_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readyReplicas", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", &self.replicas)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ReplicationControllerStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.ReplicationControllerStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ReplicationControllerStatus represents the current status of a replication controller.",
            "type": "object",
            "properties": {
                "availableReplicas": {
                    "description": "The number of available replicas (ready for at least minReadySeconds) for this replication controller.",
                    "type": "integer",
                    "format": "int32",
                },
                "conditions": {
                    "description": "Represents the latest available observations of a replication controller's current state.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::ReplicationControllerCondition>()),
                },
                "fullyLabeledReplicas": {
                    "description": "The number of pods that have labels matching the labels of the pod template of the replication controller.",
                    "type": "integer",
                    "format": "int32",
                },
                "observedGeneration": {
                    "description": "ObservedGeneration reflects the generation of the most recently observed replication controller.",
                    "type": "integer",
                    "format": "int64",
                },
                "readyReplicas": {
                    "description": "The number of ready replicas for this replication controller.",
                    "type": "integer",
                    "format": "int32",
                },
                "replicas": {
                    "description": "Replicas is the most recently observed number of replicas. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#what-is-a-replicationcontroller",
                    "type": "integer",
                    "format": "int32",
                },
            },
            "required": [
                "replicas",
            ],
        })
    }
}
