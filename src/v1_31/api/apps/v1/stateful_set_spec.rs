// Generated from definition io.k8s.api.apps.v1.StatefulSetSpec

/// A StatefulSetSpec is the specification of a StatefulSet.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StatefulSetSpec {
    /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
    pub min_ready_seconds: Option<i32>,

    /// ordinals controls the numbering of replica indices in a StatefulSet. The default ordinals behavior assigns a "0" index to the first replica and increments the index by one for each additional replica requested.
    pub ordinals: Option<crate::api::apps::v1::StatefulSetOrdinals>,

    /// persistentVolumeClaimRetentionPolicy describes the lifecycle of persistent volume claims created from volumeClaimTemplates. By default, all persistent volume claims are created as needed and retained until manually deleted. This policy allows the lifecycle to be altered, for example by deleting persistent volume claims when their stateful set is deleted, or when their pod is scaled down. This requires the StatefulSetAutoDeletePVC feature gate to be enabled, which is beta.
    pub persistent_volume_claim_retention_policy: Option<crate::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy>,

    /// podManagementPolicy controls how pods are created during initial scale up, when replacing pods on nodes, or when scaling down. The default policy is `OrderedReady`, where pods are created in increasing order (pod-0, then pod-1, etc) and the controller will wait until each pod is ready before continuing. When scaling down, the pods are removed in the opposite order. The alternative policy is `Parallel` which will create pods in parallel to match the desired scale without waiting, and on scale down will delete all pods at once.
    pub pod_management_policy: Option<std::string::String>,

    /// replicas is the desired number of replicas of the given Template. These are replicas in the sense that they are instantiations of the same Template, but individual replicas also have a consistent identity. If unspecified, defaults to 1.
    pub replicas: Option<i32>,

    /// revisionHistoryLimit is the maximum number of revisions that will be maintained in the StatefulSet's revision history. The revision history consists of all revisions not represented by a currently applied StatefulSetSpec version. The default value is 10.
    pub revision_history_limit: Option<i32>,

    /// selector is a label query over pods that should match the replica count. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    pub selector: crate::apimachinery::pkg::apis::meta::v1::LabelSelector,

    /// serviceName is the name of the service that governs this StatefulSet. This service must exist before the StatefulSet, and is responsible for the network identity of the set. Pods get DNS/hostnames that follow the pattern: pod-specific-string.serviceName.default.svc.cluster.local where "pod-specific-string" is managed by the StatefulSet controller.
    pub service_name: Option<std::string::String>,

    /// template is the object that describes the pod that will be created if insufficient replicas are detected. Each pod stamped out by the StatefulSet will fulfill this Template, but have a unique identity from the rest of the StatefulSet. Each pod will be named with the format \<statefulsetname\>-\<podindex\>. For example, a pod in a StatefulSet named "web" with index number "3" would be named "web-3". The only allowed template.spec.restartPolicy value is "Always".
    pub template: crate::api::core::v1::PodTemplateSpec,

    /// updateStrategy indicates the StatefulSetUpdateStrategy that will be employed to update Pods in the StatefulSet when a revision is made to Template.
    pub update_strategy: Option<crate::api::apps::v1::StatefulSetUpdateStrategy>,

    /// volumeClaimTemplates is a list of claims that pods are allowed to reference. The StatefulSet controller is responsible for mapping network identities to claims in a way that maintains the identity of a pod. Every claim in this list must have at least one matching (by name) volumeMount in one container in the template. A claim in this list takes precedence over any volumes in the template, with the same name.
    pub volume_claim_templates: Option<std::vec::Vec<crate::api::core::v1::PersistentVolumeClaim>>,
}

impl crate::DeepMerge for StatefulSetSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.min_ready_seconds, other.min_ready_seconds);
        crate::DeepMerge::merge_from(&mut self.ordinals, other.ordinals);
        crate::DeepMerge::merge_from(&mut self.persistent_volume_claim_retention_policy, other.persistent_volume_claim_retention_policy);
        crate::DeepMerge::merge_from(&mut self.pod_management_policy, other.pod_management_policy);
        crate::DeepMerge::merge_from(&mut self.replicas, other.replicas);
        crate::DeepMerge::merge_from(&mut self.revision_history_limit, other.revision_history_limit);
        crate::DeepMerge::merge_from(&mut self.selector, other.selector);
        crate::DeepMerge::merge_from(&mut self.service_name, other.service_name);
        crate::DeepMerge::merge_from(&mut self.template, other.template);
        crate::DeepMerge::merge_from(&mut self.update_strategy, other.update_strategy);
        crate::merge_strategies::list::atomic(&mut self.volume_claim_templates, other.volume_claim_templates);
    }
}

impl<'de> crate::serde::Deserialize<'de> for StatefulSetSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_min_ready_seconds,
            Key_ordinals,
            Key_persistent_volume_claim_retention_policy,
            Key_pod_management_policy,
            Key_replicas,
            Key_revision_history_limit,
            Key_selector,
            Key_service_name,
            Key_template,
            Key_update_strategy,
            Key_volume_claim_templates,
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
                            "minReadySeconds" => Field::Key_min_ready_seconds,
                            "ordinals" => Field::Key_ordinals,
                            "persistentVolumeClaimRetentionPolicy" => Field::Key_persistent_volume_claim_retention_policy,
                            "podManagementPolicy" => Field::Key_pod_management_policy,
                            "replicas" => Field::Key_replicas,
                            "revisionHistoryLimit" => Field::Key_revision_history_limit,
                            "selector" => Field::Key_selector,
                            "serviceName" => Field::Key_service_name,
                            "template" => Field::Key_template,
                            "updateStrategy" => Field::Key_update_strategy,
                            "volumeClaimTemplates" => Field::Key_volume_claim_templates,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = StatefulSetSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("StatefulSetSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_min_ready_seconds: Option<i32> = None;
                let mut value_ordinals: Option<crate::api::apps::v1::StatefulSetOrdinals> = None;
                let mut value_persistent_volume_claim_retention_policy: Option<crate::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy> = None;
                let mut value_pod_management_policy: Option<std::string::String> = None;
                let mut value_replicas: Option<i32> = None;
                let mut value_revision_history_limit: Option<i32> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_service_name: Option<std::string::String> = None;
                let mut value_template: Option<crate::api::core::v1::PodTemplateSpec> = None;
                let mut value_update_strategy: Option<crate::api::apps::v1::StatefulSetUpdateStrategy> = None;
                let mut value_volume_claim_templates: Option<std::vec::Vec<crate::api::core::v1::PersistentVolumeClaim>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_min_ready_seconds => value_min_ready_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ordinals => value_ordinals = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_persistent_volume_claim_retention_policy => value_persistent_volume_claim_retention_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_management_policy => value_pod_management_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_replicas => value_replicas = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_revision_history_limit => value_revision_history_limit = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_name => value_service_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_template => value_template = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_update_strategy => value_update_strategy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_claim_templates => value_volume_claim_templates = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StatefulSetSpec {
                    min_ready_seconds: value_min_ready_seconds,
                    ordinals: value_ordinals,
                    persistent_volume_claim_retention_policy: value_persistent_volume_claim_retention_policy,
                    pod_management_policy: value_pod_management_policy,
                    replicas: value_replicas,
                    revision_history_limit: value_revision_history_limit,
                    selector: value_selector.unwrap_or_default(),
                    service_name: value_service_name,
                    template: value_template.unwrap_or_default(),
                    update_strategy: value_update_strategy,
                    volume_claim_templates: value_volume_claim_templates,
                })
            }
        }

        deserializer.deserialize_struct(
            "StatefulSetSpec",
            &[
                "minReadySeconds",
                "ordinals",
                "persistentVolumeClaimRetentionPolicy",
                "podManagementPolicy",
                "replicas",
                "revisionHistoryLimit",
                "selector",
                "serviceName",
                "template",
                "updateStrategy",
                "volumeClaimTemplates",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for StatefulSetSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StatefulSetSpec",
            2 +
            self.min_ready_seconds.as_ref().map_or(0, |_| 1) +
            self.ordinals.as_ref().map_or(0, |_| 1) +
            self.persistent_volume_claim_retention_policy.as_ref().map_or(0, |_| 1) +
            self.pod_management_policy.as_ref().map_or(0, |_| 1) +
            self.replicas.as_ref().map_or(0, |_| 1) +
            self.revision_history_limit.as_ref().map_or(0, |_| 1) +
            self.service_name.as_ref().map_or(0, |_| 1) +
            self.update_strategy.as_ref().map_or(0, |_| 1) +
            self.volume_claim_templates.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.min_ready_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minReadySeconds", value)?;
        }
        if let Some(value) = &self.ordinals {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ordinals", value)?;
        }
        if let Some(value) = &self.persistent_volume_claim_retention_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "persistentVolumeClaimRetentionPolicy", value)?;
        }
        if let Some(value) = &self.pod_management_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podManagementPolicy", value)?;
        }
        if let Some(value) = &self.replicas {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "replicas", value)?;
        }
        if let Some(value) = &self.revision_history_limit {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "revisionHistoryLimit", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", &self.selector)?;
        if let Some(value) = &self.service_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "serviceName", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "template", &self.template)?;
        if let Some(value) = &self.update_strategy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "updateStrategy", value)?;
        }
        if let Some(value) = &self.volume_claim_templates {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeClaimTemplates", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StatefulSetSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.apps.v1.StatefulSetSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "A StatefulSetSpec is the specification of a StatefulSet.",
            "type": "object",
            "properties": {
                "minReadySeconds": {
                    "description": "Minimum number of seconds for which a newly created pod should be ready without any of its container crashing for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)",
                    "type": "integer",
                    "format": "int32",
                },
                "ordinals": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::apps::v1::StatefulSetOrdinals>();
                    schema_obj.ensure_object().insert("description".into(), "ordinals controls the numbering of replica indices in a StatefulSet. The default ordinals behavior assigns a \"0\" index to the first replica and increments the index by one for each additional replica requested.".into());
                    schema_obj
                }),
                "persistentVolumeClaimRetentionPolicy": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy>();
                    schema_obj.ensure_object().insert("description".into(), "persistentVolumeClaimRetentionPolicy describes the lifecycle of persistent volume claims created from volumeClaimTemplates. By default, all persistent volume claims are created as needed and retained until manually deleted. This policy allows the lifecycle to be altered, for example by deleting persistent volume claims when their stateful set is deleted, or when their pod is scaled down. This requires the StatefulSetAutoDeletePVC feature gate to be enabled, which is beta.".into());
                    schema_obj
                }),
                "podManagementPolicy": {
                    "description": "podManagementPolicy controls how pods are created during initial scale up, when replacing pods on nodes, or when scaling down. The default policy is `OrderedReady`, where pods are created in increasing order (pod-0, then pod-1, etc) and the controller will wait until each pod is ready before continuing. When scaling down, the pods are removed in the opposite order. The alternative policy is `Parallel` which will create pods in parallel to match the desired scale without waiting, and on scale down will delete all pods at once.",
                    "type": "string",
                },
                "replicas": {
                    "description": "replicas is the desired number of replicas of the given Template. These are replicas in the sense that they are instantiations of the same Template, but individual replicas also have a consistent identity. If unspecified, defaults to 1.",
                    "type": "integer",
                    "format": "int32",
                },
                "revisionHistoryLimit": {
                    "description": "revisionHistoryLimit is the maximum number of revisions that will be maintained in the StatefulSet's revision history. The revision history consists of all revisions not represented by a currently applied StatefulSetSpec version. The default value is 10.",
                    "type": "integer",
                    "format": "int32",
                },
                "selector": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>();
                    schema_obj.ensure_object().insert("description".into(), "selector is a label query over pods that should match the replica count. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors".into());
                    schema_obj
                }),
                "serviceName": {
                    "description": "serviceName is the name of the service that governs this StatefulSet. This service must exist before the StatefulSet, and is responsible for the network identity of the set. Pods get DNS/hostnames that follow the pattern: pod-specific-string.serviceName.default.svc.cluster.local where \"pod-specific-string\" is managed by the StatefulSet controller.",
                    "type": "string",
                },
                "template": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PodTemplateSpec>();
                    schema_obj.ensure_object().insert("description".into(), "template is the object that describes the pod that will be created if insufficient replicas are detected. Each pod stamped out by the StatefulSet will fulfill this Template, but have a unique identity from the rest of the StatefulSet. Each pod will be named with the format <statefulsetname>-<podindex>. For example, a pod in a StatefulSet named \"web\" with index number \"3\" would be named \"web-3\". The only allowed template.spec.restartPolicy value is \"Always\".".into());
                    schema_obj
                }),
                "updateStrategy": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::apps::v1::StatefulSetUpdateStrategy>();
                    schema_obj.ensure_object().insert("description".into(), "updateStrategy indicates the StatefulSetUpdateStrategy that will be employed to update Pods in the StatefulSet when a revision is made to Template.".into());
                    schema_obj
                }),
                "volumeClaimTemplates": {
                    "description": "volumeClaimTemplates is a list of claims that pods are allowed to reference. The StatefulSet controller is responsible for mapping network identities to claims in a way that maintains the identity of a pod. Every claim in this list must have at least one matching (by name) volumeMount in one container in the template. A claim in this list takes precedence over any volumes in the template, with the same name.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::PersistentVolumeClaim>()),
                },
            },
            "required": [
                "selector",
                "template",
            ],
        })
    }
}
