// Generated from definition io.k8s.api.admissionregistration.v1.MutatingWebhook

/// MutatingWebhook describes an admission webhook and the resources and operations it applies to.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MutatingWebhook {
    /// AdmissionReviewVersions is an ordered list of preferred `AdmissionReview` versions the Webhook expects. API server will try to use first version in the list which it supports. If none of the versions specified in this list supported by API server, validation will fail for this object. If a persisted webhook configuration specifies allowed versions and does not include any versions known to the API Server, calls to the webhook will fail and be subject to the failure policy.
    pub admission_review_versions: std::vec::Vec<std::string::String>,

    /// ClientConfig defines how to communicate with the hook. Required
    pub client_config: crate::api::admissionregistration::v1::WebhookClientConfig,

    /// FailurePolicy defines how unrecognized errors from the admission endpoint are handled - allowed values are Ignore or Fail. Defaults to Fail.
    pub failure_policy: Option<std::string::String>,

    /// MatchConditions is a list of conditions that must be met for a request to be sent to this webhook. Match conditions filter requests that have already been matched by the rules, namespaceSelector, and objectSelector. An empty list of matchConditions matches all requests. There are a maximum of 64 match conditions allowed.
    ///
    /// The exact matching logic is (in order):
    ///   1. If ANY matchCondition evaluates to FALSE, the webhook is skipped.
    ///   2. If ALL matchConditions evaluate to TRUE, the webhook is called.
    ///   3. If any matchCondition evaluates to an error (but none are FALSE):
    ///      - If failurePolicy=Fail, reject the request
    ///      - If failurePolicy=Ignore, the error is ignored and the webhook is skipped
    pub match_conditions: Option<std::vec::Vec<crate::api::admissionregistration::v1::MatchCondition>>,

    /// matchPolicy defines how the "rules" list is used to match incoming requests. Allowed values are "Exact" or "Equivalent".
    ///
    /// - Exact: match a request only if it exactly matches a specified rule. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, but "rules" only included `apiGroups:\["apps"\], apiVersions:\["v1"\], resources: \["deployments"\]`, a request to apps/v1beta1 or extensions/v1beta1 would not be sent to the webhook.
    ///
    /// - Equivalent: match a request if modifies a resource listed in rules, even via another API group or version. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, and "rules" only included `apiGroups:\["apps"\], apiVersions:\["v1"\], resources: \["deployments"\]`, a request to apps/v1beta1 or extensions/v1beta1 would be converted to apps/v1 and sent to the webhook.
    ///
    /// Defaults to "Equivalent"
    pub match_policy: Option<std::string::String>,

    /// The name of the admission webhook. Name should be fully qualified, e.g., imagepolicy.kubernetes.io, where "imagepolicy" is the name of the webhook, and kubernetes.io is the name of the organization. Required.
    pub name: std::string::String,

    /// NamespaceSelector decides whether to run the webhook on an object based on whether the namespace for that object matches the selector. If the object itself is a namespace, the matching is performed on object.metadata.labels. If the object is another cluster scoped resource, it never skips the webhook.
    ///
    /// For example, to run the webhook on any objects whose namespace is not associated with "runlevel" of "0" or "1";  you will set the selector as follows: "namespaceSelector": {
    ///   "matchExpressions": \[
    ///     {
    ///       "key": "runlevel",
    ///       "operator": "NotIn",
    ///       "values": \[
    ///         "0",
    ///         "1"
    ///       \]
    ///     }
    ///   \]
    /// }
    ///
    /// If instead you want to only run the webhook on any objects whose namespace is associated with the "environment" of "prod" or "staging"; you will set the selector as follows: "namespaceSelector": {
    ///   "matchExpressions": \[
    ///     {
    ///       "key": "environment",
    ///       "operator": "In",
    ///       "values": \[
    ///         "prod",
    ///         "staging"
    ///       \]
    ///     }
    ///   \]
    /// }
    ///
    /// See https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/ for more examples of label selectors.
    ///
    /// Default to the empty LabelSelector, which matches everything.
    pub namespace_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// ObjectSelector decides whether to run the webhook based on if the object has matching labels. objectSelector is evaluated against both the oldObject and newObject that would be sent to the webhook, and is considered to match if either object matches the selector. A null object (oldObject in the case of create, or newObject in the case of delete) or an object that cannot have labels (like a DeploymentRollback or a PodProxyOptions object) is not considered to match. Use the object selector only if the webhook is opt-in, because end users may skip the admission webhook by setting the labels. Default to the empty LabelSelector, which matches everything.
    pub object_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// reinvocationPolicy indicates whether this webhook should be called multiple times as part of a single admission evaluation. Allowed values are "Never" and "IfNeeded".
    ///
    /// Never: the webhook will not be called more than once in a single admission evaluation.
    ///
    /// IfNeeded: the webhook will be called at least one additional time as part of the admission evaluation if the object being admitted is modified by other admission plugins after the initial webhook call. Webhooks that specify this option *must* be idempotent, able to process objects they previously admitted. Note: * the number of additional invocations is not guaranteed to be exactly one. * if additional invocations result in further modifications to the object, webhooks are not guaranteed to be invoked again. * webhooks that use this option may be reordered to minimize the number of additional invocations. * to validate an object after all mutations are guaranteed complete, use a validating admission webhook instead.
    ///
    /// Defaults to "Never".
    pub reinvocation_policy: Option<std::string::String>,

    /// Rules describes what operations on what resources/subresources the webhook cares about. The webhook cares about an operation if it matches _any_ Rule. However, in order to prevent ValidatingAdmissionWebhooks and MutatingAdmissionWebhooks from putting the cluster in a state which cannot be recovered from without completely disabling the plugin, ValidatingAdmissionWebhooks and MutatingAdmissionWebhooks are never called on admission requests for ValidatingWebhookConfiguration and MutatingWebhookConfiguration objects.
    pub rules: Option<std::vec::Vec<crate::api::admissionregistration::v1::RuleWithOperations>>,

    /// SideEffects states whether this webhook has side effects. Acceptable values are: None, NoneOnDryRun (webhooks created via v1beta1 may also specify Some or Unknown). Webhooks with side effects MUST implement a reconciliation system, since a request may be rejected by a future step in the admission chain and the side effects therefore need to be undone. Requests with the dryRun attribute will be auto-rejected if they match a webhook with sideEffects == Unknown or Some.
    pub side_effects: std::string::String,

    /// TimeoutSeconds specifies the timeout for this webhook. After the timeout passes, the webhook call will be ignored or the API call will fail based on the failure policy. The timeout value must be between 1 and 30 seconds. Default to 10 seconds.
    pub timeout_seconds: Option<i32>,
}

impl crate::DeepMerge for MutatingWebhook {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.admission_review_versions, other.admission_review_versions);
        crate::DeepMerge::merge_from(&mut self.client_config, other.client_config);
        crate::DeepMerge::merge_from(&mut self.failure_policy, other.failure_policy);
        crate::merge_strategies::list::map(
            &mut self.match_conditions,
            other.match_conditions,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.match_policy, other.match_policy);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.namespace_selector, other.namespace_selector);
        crate::DeepMerge::merge_from(&mut self.object_selector, other.object_selector);
        crate::DeepMerge::merge_from(&mut self.reinvocation_policy, other.reinvocation_policy);
        crate::merge_strategies::list::atomic(&mut self.rules, other.rules);
        crate::DeepMerge::merge_from(&mut self.side_effects, other.side_effects);
        crate::DeepMerge::merge_from(&mut self.timeout_seconds, other.timeout_seconds);
    }
}

impl<'de> crate::serde::Deserialize<'de> for MutatingWebhook {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_admission_review_versions,
            Key_client_config,
            Key_failure_policy,
            Key_match_conditions,
            Key_match_policy,
            Key_name,
            Key_namespace_selector,
            Key_object_selector,
            Key_reinvocation_policy,
            Key_rules,
            Key_side_effects,
            Key_timeout_seconds,
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
                            "admissionReviewVersions" => Field::Key_admission_review_versions,
                            "clientConfig" => Field::Key_client_config,
                            "failurePolicy" => Field::Key_failure_policy,
                            "matchConditions" => Field::Key_match_conditions,
                            "matchPolicy" => Field::Key_match_policy,
                            "name" => Field::Key_name,
                            "namespaceSelector" => Field::Key_namespace_selector,
                            "objectSelector" => Field::Key_object_selector,
                            "reinvocationPolicy" => Field::Key_reinvocation_policy,
                            "rules" => Field::Key_rules,
                            "sideEffects" => Field::Key_side_effects,
                            "timeoutSeconds" => Field::Key_timeout_seconds,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = MutatingWebhook;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("MutatingWebhook")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_admission_review_versions: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_client_config: Option<crate::api::admissionregistration::v1::WebhookClientConfig> = None;
                let mut value_failure_policy: Option<std::string::String> = None;
                let mut value_match_conditions: Option<std::vec::Vec<crate::api::admissionregistration::v1::MatchCondition>> = None;
                let mut value_match_policy: Option<std::string::String> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_namespace_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_object_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_reinvocation_policy: Option<std::string::String> = None;
                let mut value_rules: Option<std::vec::Vec<crate::api::admissionregistration::v1::RuleWithOperations>> = None;
                let mut value_side_effects: Option<std::string::String> = None;
                let mut value_timeout_seconds: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_admission_review_versions => value_admission_review_versions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_client_config => value_client_config = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_failure_policy => value_failure_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_match_conditions => value_match_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_match_policy => value_match_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespace_selector => value_namespace_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_object_selector => value_object_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reinvocation_policy => value_reinvocation_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rules => value_rules = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_side_effects => value_side_effects = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_timeout_seconds => value_timeout_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(MutatingWebhook {
                    admission_review_versions: value_admission_review_versions.unwrap_or_default(),
                    client_config: value_client_config.unwrap_or_default(),
                    failure_policy: value_failure_policy,
                    match_conditions: value_match_conditions,
                    match_policy: value_match_policy,
                    name: value_name.unwrap_or_default(),
                    namespace_selector: value_namespace_selector,
                    object_selector: value_object_selector,
                    reinvocation_policy: value_reinvocation_policy,
                    rules: value_rules,
                    side_effects: value_side_effects.unwrap_or_default(),
                    timeout_seconds: value_timeout_seconds,
                })
            }
        }

        deserializer.deserialize_struct(
            "MutatingWebhook",
            &[
                "admissionReviewVersions",
                "clientConfig",
                "failurePolicy",
                "matchConditions",
                "matchPolicy",
                "name",
                "namespaceSelector",
                "objectSelector",
                "reinvocationPolicy",
                "rules",
                "sideEffects",
                "timeoutSeconds",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for MutatingWebhook {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "MutatingWebhook",
            4 +
            self.failure_policy.as_ref().map_or(0, |_| 1) +
            self.match_conditions.as_ref().map_or(0, |_| 1) +
            self.match_policy.as_ref().map_or(0, |_| 1) +
            self.namespace_selector.as_ref().map_or(0, |_| 1) +
            self.object_selector.as_ref().map_or(0, |_| 1) +
            self.reinvocation_policy.as_ref().map_or(0, |_| 1) +
            self.rules.as_ref().map_or(0, |_| 1) +
            self.timeout_seconds.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "admissionReviewVersions", &self.admission_review_versions)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "clientConfig", &self.client_config)?;
        if let Some(value) = &self.failure_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "failurePolicy", value)?;
        }
        if let Some(value) = &self.match_conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchConditions", value)?;
        }
        if let Some(value) = &self.match_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchPolicy", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.namespace_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namespaceSelector", value)?;
        }
        if let Some(value) = &self.object_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "objectSelector", value)?;
        }
        if let Some(value) = &self.reinvocation_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "reinvocationPolicy", value)?;
        }
        if let Some(value) = &self.rules {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rules", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "sideEffects", &self.side_effects)?;
        if let Some(value) = &self.timeout_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "timeoutSeconds", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for MutatingWebhook {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.admissionregistration.v1.MutatingWebhook".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "MutatingWebhook describes an admission webhook and the resources and operations it applies to.",
            "type": "object",
            "properties": {
                "admissionReviewVersions": {
                    "description": "AdmissionReviewVersions is an ordered list of preferred `AdmissionReview` versions the Webhook expects. API server will try to use first version in the list which it supports. If none of the versions specified in this list supported by API server, validation will fail for this object. If a persisted webhook configuration specifies allowed versions and does not include any versions known to the API Server, calls to the webhook will fail and be subject to the failure policy.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "clientConfig": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::admissionregistration::v1::WebhookClientConfig>();
                    schema_obj.ensure_object().insert("description".into(), "ClientConfig defines how to communicate with the hook. Required".into());
                    schema_obj
                }),
                "failurePolicy": {
                    "description": "FailurePolicy defines how unrecognized errors from the admission endpoint are handled - allowed values are Ignore or Fail. Defaults to Fail.",
                    "type": "string",
                },
                "matchConditions": {
                    "description": "MatchConditions is a list of conditions that must be met for a request to be sent to this webhook. Match conditions filter requests that have already been matched by the rules, namespaceSelector, and objectSelector. An empty list of matchConditions matches all requests. There are a maximum of 64 match conditions allowed.\n\nThe exact matching logic is (in order):\n  1. If ANY matchCondition evaluates to FALSE, the webhook is skipped.\n  2. If ALL matchConditions evaluate to TRUE, the webhook is called.\n  3. If any matchCondition evaluates to an error (but none are FALSE):\n     - If failurePolicy=Fail, reject the request\n     - If failurePolicy=Ignore, the error is ignored and the webhook is skipped",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::admissionregistration::v1::MatchCondition>()),
                },
                "matchPolicy": {
                    "description": "matchPolicy defines how the \"rules\" list is used to match incoming requests. Allowed values are \"Exact\" or \"Equivalent\".\n\n- Exact: match a request only if it exactly matches a specified rule. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, but \"rules\" only included `apiGroups:[\"apps\"], apiVersions:[\"v1\"], resources: [\"deployments\"]`, a request to apps/v1beta1 or extensions/v1beta1 would not be sent to the webhook.\n\n- Equivalent: match a request if modifies a resource listed in rules, even via another API group or version. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, and \"rules\" only included `apiGroups:[\"apps\"], apiVersions:[\"v1\"], resources: [\"deployments\"]`, a request to apps/v1beta1 or extensions/v1beta1 would be converted to apps/v1 and sent to the webhook.\n\nDefaults to \"Equivalent\"",
                    "type": "string",
                },
                "name": {
                    "description": "The name of the admission webhook. Name should be fully qualified, e.g., imagepolicy.kubernetes.io, where \"imagepolicy\" is the name of the webhook, and kubernetes.io is the name of the organization. Required.",
                    "type": "string",
                },
                "namespaceSelector": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>();
                    schema_obj.ensure_object().insert("description".into(), "NamespaceSelector decides whether to run the webhook on an object based on whether the namespace for that object matches the selector. If the object itself is a namespace, the matching is performed on object.metadata.labels. If the object is another cluster scoped resource, it never skips the webhook.\n\nFor example, to run the webhook on any objects whose namespace is not associated with \"runlevel\" of \"0\" or \"1\";  you will set the selector as follows: \"namespaceSelector\": {\n  \"matchExpressions\": [\n    {\n      \"key\": \"runlevel\",\n      \"operator\": \"NotIn\",\n      \"values\": [\n        \"0\",\n        \"1\"\n      ]\n    }\n  ]\n}\n\nIf instead you want to only run the webhook on any objects whose namespace is associated with the \"environment\" of \"prod\" or \"staging\"; you will set the selector as follows: \"namespaceSelector\": {\n  \"matchExpressions\": [\n    {\n      \"key\": \"environment\",\n      \"operator\": \"In\",\n      \"values\": [\n        \"prod\",\n        \"staging\"\n      ]\n    }\n  ]\n}\n\nSee https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/ for more examples of label selectors.\n\nDefault to the empty LabelSelector, which matches everything.".into());
                    schema_obj
                }),
                "objectSelector": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>();
                    schema_obj.ensure_object().insert("description".into(), "ObjectSelector decides whether to run the webhook based on if the object has matching labels. objectSelector is evaluated against both the oldObject and newObject that would be sent to the webhook, and is considered to match if either object matches the selector. A null object (oldObject in the case of create, or newObject in the case of delete) or an object that cannot have labels (like a DeploymentRollback or a PodProxyOptions object) is not considered to match. Use the object selector only if the webhook is opt-in, because end users may skip the admission webhook by setting the labels. Default to the empty LabelSelector, which matches everything.".into());
                    schema_obj
                }),
                "reinvocationPolicy": {
                    "description": "reinvocationPolicy indicates whether this webhook should be called multiple times as part of a single admission evaluation. Allowed values are \"Never\" and \"IfNeeded\".\n\nNever: the webhook will not be called more than once in a single admission evaluation.\n\nIfNeeded: the webhook will be called at least one additional time as part of the admission evaluation if the object being admitted is modified by other admission plugins after the initial webhook call. Webhooks that specify this option *must* be idempotent, able to process objects they previously admitted. Note: * the number of additional invocations is not guaranteed to be exactly one. * if additional invocations result in further modifications to the object, webhooks are not guaranteed to be invoked again. * webhooks that use this option may be reordered to minimize the number of additional invocations. * to validate an object after all mutations are guaranteed complete, use a validating admission webhook instead.\n\nDefaults to \"Never\".",
                    "type": "string",
                },
                "rules": {
                    "description": "Rules describes what operations on what resources/subresources the webhook cares about. The webhook cares about an operation if it matches _any_ Rule. However, in order to prevent ValidatingAdmissionWebhooks and MutatingAdmissionWebhooks from putting the cluster in a state which cannot be recovered from without completely disabling the plugin, ValidatingAdmissionWebhooks and MutatingAdmissionWebhooks are never called on admission requests for ValidatingWebhookConfiguration and MutatingWebhookConfiguration objects.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::admissionregistration::v1::RuleWithOperations>()),
                },
                "sideEffects": {
                    "description": "SideEffects states whether this webhook has side effects. Acceptable values are: None, NoneOnDryRun (webhooks created via v1beta1 may also specify Some or Unknown). Webhooks with side effects MUST implement a reconciliation system, since a request may be rejected by a future step in the admission chain and the side effects therefore need to be undone. Requests with the dryRun attribute will be auto-rejected if they match a webhook with sideEffects == Unknown or Some.",
                    "type": "string",
                },
                "timeoutSeconds": {
                    "description": "TimeoutSeconds specifies the timeout for this webhook. After the timeout passes, the webhook call will be ignored or the API call will fail based on the failure policy. The timeout value must be between 1 and 30 seconds. Default to 10 seconds.",
                    "type": "integer",
                    "format": "int32",
                },
            },
            "required": [
                "admissionReviewVersions",
                "clientConfig",
                "name",
                "sideEffects",
            ],
        })
    }
}
