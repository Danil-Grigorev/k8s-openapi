// Generated from definition io.k8s.api.core.v1.NodeSpec

/// NodeSpec describes the attributes that a node is created with.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeSpec {
    /// Deprecated: Previously used to specify the source of the node's configuration for the DynamicKubeletConfig feature. This feature is removed.
    pub config_source: Option<crate::api::core::v1::NodeConfigSource>,

    /// Deprecated. Not all kubelets will set this field. Remove field after 1.13. see: https://issues.k8s.io/61966
    pub external_id: Option<std::string::String>,

    /// PodCIDR represents the pod IP range assigned to the node.
    pub pod_cidr: Option<std::string::String>,

    /// podCIDRs represents the IP ranges assigned to the node for usage by Pods on that node. If this field is specified, the 0th entry must match the podCIDR field. It may contain at most 1 value for each of IPv4 and IPv6.
    pub pod_cidrs: Option<std::vec::Vec<std::string::String>>,

    /// ID of the node assigned by the cloud provider in the format: \<ProviderName\>://\<ProviderSpecificNodeID\>
    pub provider_id: Option<std::string::String>,

    /// If specified, the node's taints.
    pub taints: Option<std::vec::Vec<crate::api::core::v1::Taint>>,

    /// Unschedulable controls node schedulability of new pods. By default, node is schedulable. More info: https://kubernetes.io/docs/concepts/nodes/node/#manual-node-administration
    pub unschedulable: Option<bool>,
}

impl crate::DeepMerge for NodeSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.config_source, other.config_source);
        crate::DeepMerge::merge_from(&mut self.external_id, other.external_id);
        crate::DeepMerge::merge_from(&mut self.pod_cidr, other.pod_cidr);
        crate::merge_strategies::list::set(&mut self.pod_cidrs, other.pod_cidrs);
        crate::DeepMerge::merge_from(&mut self.provider_id, other.provider_id);
        crate::merge_strategies::list::atomic(&mut self.taints, other.taints);
        crate::DeepMerge::merge_from(&mut self.unschedulable, other.unschedulable);
    }
}

impl<'de> crate::serde::Deserialize<'de> for NodeSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_config_source,
            Key_external_id,
            Key_pod_cidr,
            Key_pod_cidrs,
            Key_provider_id,
            Key_taints,
            Key_unschedulable,
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
                            "configSource" => Field::Key_config_source,
                            "externalID" => Field::Key_external_id,
                            "podCIDR" => Field::Key_pod_cidr,
                            "podCIDRs" => Field::Key_pod_cidrs,
                            "providerID" => Field::Key_provider_id,
                            "taints" => Field::Key_taints,
                            "unschedulable" => Field::Key_unschedulable,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodeSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("NodeSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_config_source: Option<crate::api::core::v1::NodeConfigSource> = None;
                let mut value_external_id: Option<std::string::String> = None;
                let mut value_pod_cidr: Option<std::string::String> = None;
                let mut value_pod_cidrs: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_provider_id: Option<std::string::String> = None;
                let mut value_taints: Option<std::vec::Vec<crate::api::core::v1::Taint>> = None;
                let mut value_unschedulable: Option<bool> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_config_source => value_config_source = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_id => value_external_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_cidr => value_pod_cidr = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_cidrs => value_pod_cidrs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_provider_id => value_provider_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_taints => value_taints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_unschedulable => value_unschedulable = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeSpec {
                    config_source: value_config_source,
                    external_id: value_external_id,
                    pod_cidr: value_pod_cidr,
                    pod_cidrs: value_pod_cidrs,
                    provider_id: value_provider_id,
                    taints: value_taints,
                    unschedulable: value_unschedulable,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeSpec",
            &[
                "configSource",
                "externalID",
                "podCIDR",
                "podCIDRs",
                "providerID",
                "taints",
                "unschedulable",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NodeSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeSpec",
            self.config_source.as_ref().map_or(0, |_| 1) +
            self.external_id.as_ref().map_or(0, |_| 1) +
            self.pod_cidr.as_ref().map_or(0, |_| 1) +
            self.pod_cidrs.as_ref().map_or(0, |_| 1) +
            self.provider_id.as_ref().map_or(0, |_| 1) +
            self.taints.as_ref().map_or(0, |_| 1) +
            self.unschedulable.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.config_source {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "configSource", value)?;
        }
        if let Some(value) = &self.external_id {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "externalID", value)?;
        }
        if let Some(value) = &self.pod_cidr {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podCIDR", value)?;
        }
        if let Some(value) = &self.pod_cidrs {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podCIDRs", value)?;
        }
        if let Some(value) = &self.provider_id {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "providerID", value)?;
        }
        if let Some(value) = &self.taints {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "taints", value)?;
        }
        if let Some(value) = &self.unschedulable {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "unschedulable", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NodeSpec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.NodeSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "NodeSpec describes the attributes that a node is created with.",
            "type": "object",
            "properties": {
                "configSource": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NodeConfigSource>();
                    schema_obj.ensure_object().insert("description".into(), "Deprecated: Previously used to specify the source of the node's configuration for the DynamicKubeletConfig feature. This feature is removed.".into());
                    schema_obj
                }),
                "externalID": {
                    "description": "Deprecated. Not all kubelets will set this field. Remove field after 1.13. see: https://issues.k8s.io/61966",
                    "type": "string",
                },
                "podCIDR": {
                    "description": "PodCIDR represents the pod IP range assigned to the node.",
                    "type": "string",
                },
                "podCIDRs": {
                    "description": "podCIDRs represents the IP ranges assigned to the node for usage by Pods on that node. If this field is specified, the 0th entry must match the podCIDR field. It may contain at most 1 value for each of IPv4 and IPv6.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "providerID": {
                    "description": "ID of the node assigned by the cloud provider in the format: <ProviderName>://<ProviderSpecificNodeID>",
                    "type": "string",
                },
                "taints": {
                    "description": "If specified, the node's taints.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::Taint>()),
                },
                "unschedulable": {
                    "description": "Unschedulable controls node schedulability of new pods. By default, node is schedulable. More info: https://kubernetes.io/docs/concepts/nodes/node/#manual-node-administration",
                    "type": "boolean",
                },
            },
        })
    }
}
