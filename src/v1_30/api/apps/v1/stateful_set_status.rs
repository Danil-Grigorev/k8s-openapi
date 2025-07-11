// Generated from definition io.k8s.api.apps.v1.StatefulSetStatus

/// StatefulSetStatus represents the current state of a StatefulSet.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StatefulSetStatus {
    /// Total number of available pods (ready for at least minReadySeconds) targeted by this statefulset.
    pub available_replicas: Option<i32>,

    /// collisionCount is the count of hash collisions for the StatefulSet. The StatefulSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision.
    pub collision_count: Option<i32>,

    /// Represents the latest available observations of a statefulset's current state.
    pub conditions: Option<std::vec::Vec<crate::api::apps::v1::StatefulSetCondition>>,

    /// currentReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by currentRevision.
    pub current_replicas: Option<i32>,

    /// currentRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence \[0,currentReplicas).
    pub current_revision: Option<std::string::String>,

    /// observedGeneration is the most recent generation observed for this StatefulSet. It corresponds to the StatefulSet's generation, which is updated on mutation by the API Server.
    pub observed_generation: Option<i64>,

    /// readyReplicas is the number of pods created for this StatefulSet with a Ready Condition.
    pub ready_replicas: Option<i32>,

    /// replicas is the number of Pods created by the StatefulSet controller.
    pub replicas: i32,

    /// updateRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence \[replicas-updatedReplicas,replicas)
    pub update_revision: Option<std::string::String>,

    /// updatedReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by updateRevision.
    pub updated_replicas: Option<i32>,
}

impl crate::DeepMerge for StatefulSetStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.available_replicas, other.available_replicas);
        crate::DeepMerge::merge_from(&mut self.collision_count, other.collision_count);
        crate::merge_strategies::list::map(
            &mut self.conditions,
            other.conditions,
            &[|lhs, rhs| lhs.type_ == rhs.type_],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.current_replicas, other.current_replicas);
        crate::DeepMerge::merge_from(&mut self.current_revision, other.current_revision);
        crate::DeepMerge::merge_from(&mut self.observed_generation, other.observed_generation);
        crate::DeepMerge::merge_from(&mut self.ready_replicas, other.ready_replicas);
        crate::DeepMerge::merge_from(&mut self.replicas, other.replicas);
        crate::DeepMerge::merge_from(&mut self.update_revision, other.update_revision);
        crate::DeepMerge::merge_from(&mut self.updated_replicas, other.updated_replicas);
    }
}

impl<'de> crate::serde::Deserialize<'de> for StatefulSetStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_available_replicas,
            Key_collision_count,
            Key_conditions,
            Key_current_replicas,
            Key_current_revision,
            Key_observed_generation,
            Key_ready_replicas,
            Key_replicas,
            Key_update_revision,
            Key_updated_replicas,
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
                            "collisionCount" => Field::Key_collision_count,
                            "conditions" => Field::Key_conditions,
                            "currentReplicas" => Field::Key_current_replicas,
                            "currentRevision" => Field::Key_current_revision,
                            "observedGeneration" => Field::Key_observed_generation,
                            "readyReplicas" => Field::Key_ready_replicas,
                            "replicas" => Field::Key_replicas,
                            "updateRevision" => Field::Key_update_revision,
                            "updatedReplicas" => Field::Key_updated_replicas,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = StatefulSetStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("StatefulSetStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_available_replicas: Option<i32> = None;
                let mut value_collision_count: Option<i32> = None;
                let mut value_conditions: Option<std::vec::Vec<crate::api::apps::v1::StatefulSetCondition>> = None;
                let mut value_current_replicas: Option<i32> = None;
                let mut value_current_revision: Option<std::string::String> = None;
                let mut value_observed_generation: Option<i64> = None;
                let mut value_ready_replicas: Option<i32> = None;
                let mut value_replicas: Option<i32> = None;
                let mut value_update_revision: Option<std::string::String> = None;
                let mut value_updated_replicas: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_available_replicas => value_available_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_collision_count => value_collision_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_current_replicas => value_current_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_current_revision => value_current_revision = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observed_generation => value_observed_generation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ready_replicas => value_ready_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_replicas => value_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_update_revision => value_update_revision = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_updated_replicas => value_updated_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StatefulSetStatus {
                    available_replicas: value_available_replicas,
                    collision_count: value_collision_count,
                    conditions: value_conditions,
                    current_replicas: value_current_replicas,
                    current_revision: value_current_revision,
                    observed_generation: value_observed_generation,
                    ready_replicas: value_ready_replicas,
                    replicas: value_replicas.unwrap_or_default(),
                    update_revision: value_update_revision,
                    updated_replicas: value_updated_replicas,
                })
            }
        }

        deserializer.deserialize_struct(
            "StatefulSetStatus",
            &[
                "availableReplicas",
                "collisionCount",
                "conditions",
                "currentReplicas",
                "currentRevision",
                "observedGeneration",
                "readyReplicas",
                "replicas",
                "updateRevision",
                "updatedReplicas",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for StatefulSetStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StatefulSetStatus",
            1 +
            self.available_replicas.as_ref().map_or(0, |_| 1) +
            self.collision_count.as_ref().map_or(0, |_| 1) +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.current_replicas.as_ref().map_or(0, |_| 1) +
            self.current_revision.as_ref().map_or(0, |_| 1) +
            self.observed_generation.as_ref().map_or(0, |_| 1) +
            self.ready_replicas.as_ref().map_or(0, |_| 1) +
            self.update_revision.as_ref().map_or(0, |_| 1) +
            self.updated_replicas.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.available_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "availableReplicas", value)?;
        }
        if let Some(value) = &self.collision_count {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "collisionCount", value)?;
        }
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.current_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "currentReplicas", value)?;
        }
        if let Some(value) = &self.current_revision {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "currentRevision", value)?;
        }
        if let Some(value) = &self.observed_generation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "observedGeneration", value)?;
        }
        if let Some(value) = &self.ready_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readyReplicas", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", &self.replicas)?;
        if let Some(value) = &self.update_revision {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "updateRevision", value)?;
        }
        if let Some(value) = &self.updated_replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "updatedReplicas", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StatefulSetStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.apps.v1.StatefulSetStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "StatefulSetStatus represents the current state of a StatefulSet.",
            "type": "object",
            "properties": {
                "availableReplicas": {
                    "description": "Total number of available pods (ready for at least minReadySeconds) targeted by this statefulset.",
                    "type": "integer",
                    "format": "int32",
                },
                "collisionCount": {
                    "description": "collisionCount is the count of hash collisions for the StatefulSet. The StatefulSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision.",
                    "type": "integer",
                    "format": "int32",
                },
                "conditions": {
                    "description": "Represents the latest available observations of a statefulset's current state.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::apps::v1::StatefulSetCondition>()),
                },
                "currentReplicas": {
                    "description": "currentReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by currentRevision.",
                    "type": "integer",
                    "format": "int32",
                },
                "currentRevision": {
                    "description": "currentRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence [0,currentReplicas).",
                    "type": "string",
                },
                "observedGeneration": {
                    "description": "observedGeneration is the most recent generation observed for this StatefulSet. It corresponds to the StatefulSet's generation, which is updated on mutation by the API Server.",
                    "type": "integer",
                    "format": "int64",
                },
                "readyReplicas": {
                    "description": "readyReplicas is the number of pods created for this StatefulSet with a Ready Condition.",
                    "type": "integer",
                    "format": "int32",
                },
                "replicas": {
                    "description": "replicas is the number of Pods created by the StatefulSet controller.",
                    "type": "integer",
                    "format": "int32",
                },
                "updateRevision": {
                    "description": "updateRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence [replicas-updatedReplicas,replicas)",
                    "type": "string",
                },
                "updatedReplicas": {
                    "description": "updatedReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by updateRevision.",
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
