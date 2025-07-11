// Generated from definition io.k8s.api.storage.v1.CSINodeDriver

/// CSINodeDriver holds information about the specification of one CSI driver installed on a node
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CSINodeDriver {
    /// allocatable represents the volume resources of a node that are available for scheduling. This field is beta.
    pub allocatable: Option<crate::api::storage::v1::VolumeNodeResources>,

    /// name represents the name of the CSI driver that this object refers to. This MUST be the same name returned by the CSI GetPluginName() call for that driver.
    pub name: std::string::String,

    /// nodeID of the node from the driver point of view. This field enables Kubernetes to communicate with storage systems that do not share the same nomenclature for nodes. For example, Kubernetes may refer to a given node as "node1", but the storage system may refer to the same node as "nodeA". When Kubernetes issues a command to the storage system to attach a volume to a specific node, it can use this field to refer to the node name using the ID that the storage system will understand, e.g. "nodeA" instead of "node1". This field is required.
    pub node_id: std::string::String,

    /// topologyKeys is the list of keys supported by the driver. When a driver is initialized on a cluster, it provides a set of topology keys that it understands (e.g. "company.com/zone", "company.com/region"). When a driver is initialized on a node, it provides the same topology keys along with values. Kubelet will expose these topology keys as labels on its own node object. When Kubernetes does topology aware provisioning, it can use this list to determine which labels it should retrieve from the node object and pass back to the driver. It is possible for different nodes to use different topology keys. This can be empty if driver does not support topology.
    pub topology_keys: Option<std::vec::Vec<std::string::String>>,
}

impl crate::DeepMerge for CSINodeDriver {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.allocatable, other.allocatable);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.node_id, other.node_id);
        crate::merge_strategies::list::atomic(&mut self.topology_keys, other.topology_keys);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CSINodeDriver {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allocatable,
            Key_name,
            Key_node_id,
            Key_topology_keys,
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
                            "allocatable" => Field::Key_allocatable,
                            "name" => Field::Key_name,
                            "nodeID" => Field::Key_node_id,
                            "topologyKeys" => Field::Key_topology_keys,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CSINodeDriver;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CSINodeDriver")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allocatable: Option<crate::api::storage::v1::VolumeNodeResources> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_node_id: Option<std::string::String> = None;
                let mut value_topology_keys: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allocatable => value_allocatable = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_id => value_node_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_topology_keys => value_topology_keys = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CSINodeDriver {
                    allocatable: value_allocatable,
                    name: value_name.unwrap_or_default(),
                    node_id: value_node_id.unwrap_or_default(),
                    topology_keys: value_topology_keys,
                })
            }
        }

        deserializer.deserialize_struct(
            "CSINodeDriver",
            &[
                "allocatable",
                "name",
                "nodeID",
                "topologyKeys",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CSINodeDriver {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CSINodeDriver",
            2 +
            self.allocatable.as_ref().map_or(0, |_| 1) +
            self.topology_keys.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allocatable {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allocatable", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeID", &self.node_id)?;
        if let Some(value) = &self.topology_keys {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "topologyKeys", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CSINodeDriver {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.storage.v1.CSINodeDriver".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "CSINodeDriver holds information about the specification of one CSI driver installed on a node",
            "type": "object",
            "properties": {
                "allocatable": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::storage::v1::VolumeNodeResources>();
                    schema_obj.ensure_object().insert("description".into(), "allocatable represents the volume resources of a node that are available for scheduling. This field is beta.".into());
                    schema_obj
                }),
                "name": {
                    "description": "name represents the name of the CSI driver that this object refers to. This MUST be the same name returned by the CSI GetPluginName() call for that driver.",
                    "type": "string",
                },
                "nodeID": {
                    "description": "nodeID of the node from the driver point of view. This field enables Kubernetes to communicate with storage systems that do not share the same nomenclature for nodes. For example, Kubernetes may refer to a given node as \"node1\", but the storage system may refer to the same node as \"nodeA\". When Kubernetes issues a command to the storage system to attach a volume to a specific node, it can use this field to refer to the node name using the ID that the storage system will understand, e.g. \"nodeA\" instead of \"node1\". This field is required.",
                    "type": "string",
                },
                "topologyKeys": {
                    "description": "topologyKeys is the list of keys supported by the driver. When a driver is initialized on a cluster, it provides a set of topology keys that it understands (e.g. \"company.com/zone\", \"company.com/region\"). When a driver is initialized on a node, it provides the same topology keys along with values. Kubelet will expose these topology keys as labels on its own node object. When Kubernetes does topology aware provisioning, it can use this list to determine which labels it should retrieve from the node object and pass back to the driver. It is possible for different nodes to use different topology keys. This can be empty if driver does not support topology.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
            },
            "required": [
                "name",
                "nodeID",
            ],
        })
    }
}
