// Generated from definition io.k8s.api.core.v1.ContainerStatus

/// ContainerStatus contains details for the current status of this container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerStatus {
    /// AllocatedResources represents the compute resources allocated for this container by the node. Kubelet sets this value to Container.Resources.Requests upon successful pod admission and after successfully admitting desired pod resize.
    pub allocated_resources: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// AllocatedResourcesStatus represents the status of various resources allocated for this Pod.
    pub allocated_resources_status: Option<std::vec::Vec<crate::api::core::v1::ResourceStatus>>,

    /// ContainerID is the ID of the container in the format '\<type\>://\<container_id\>'. Where type is a container runtime identifier, returned from Version call of CRI API (for example "containerd").
    pub container_id: Option<std::string::String>,

    /// Image is the name of container image that the container is running. The container image may not match the image used in the PodSpec, as it may have been resolved by the runtime. More info: https://kubernetes.io/docs/concepts/containers/images.
    pub image: std::string::String,

    /// ImageID is the image ID of the container's image. The image ID may not match the image ID of the image used in the PodSpec, as it may have been resolved by the runtime.
    pub image_id: std::string::String,

    /// LastTerminationState holds the last termination state of the container to help debug container crashes and restarts. This field is not populated if the container is still running and RestartCount is 0.
    pub last_state: Option<crate::api::core::v1::ContainerState>,

    /// Name is a DNS_LABEL representing the unique name of the container. Each container in a pod must have a unique name across all container types. Cannot be updated.
    pub name: std::string::String,

    /// Ready specifies whether the container is currently passing its readiness check. The value will change as readiness probes keep executing. If no readiness probes are specified, this field defaults to true once the container is fully started (see Started field).
    ///
    /// The value is typically used to determine whether a container is ready to accept traffic.
    pub ready: bool,

    /// Resources represents the compute resource requests and limits that have been successfully enacted on the running container after it has been started or has been successfully resized.
    pub resources: Option<crate::api::core::v1::ResourceRequirements>,

    /// RestartCount holds the number of times the container has been restarted. Kubelet makes an effort to always increment the value, but there are cases when the state may be lost due to node restarts and then the value may be reset to 0. The value is never negative.
    pub restart_count: i32,

    /// Started indicates whether the container has finished its postStart lifecycle hook and passed its startup probe. Initialized as false, becomes true after startupProbe is considered successful. Resets to false when the container is restarted, or if kubelet loses state temporarily. In both cases, startup probes will run again. Is always true when no startupProbe is defined and container is running and has passed the postStart lifecycle hook. The null value must be treated the same as false.
    pub started: Option<bool>,

    /// State holds details about the container's current condition.
    pub state: Option<crate::api::core::v1::ContainerState>,

    /// StopSignal reports the effective stop signal for this container
    pub stop_signal: Option<std::string::String>,

    /// User represents user identity information initially attached to the first process of the container
    pub user: Option<crate::api::core::v1::ContainerUser>,

    /// Status of volume mounts.
    pub volume_mounts: Option<std::vec::Vec<crate::api::core::v1::VolumeMountStatus>>,
}

impl crate::DeepMerge for ContainerStatus {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::map::granular(&mut self.allocated_resources, other.allocated_resources, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::list::map(
            &mut self.allocated_resources_status,
            other.allocated_resources_status,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.container_id, other.container_id);
        crate::DeepMerge::merge_from(&mut self.image, other.image);
        crate::DeepMerge::merge_from(&mut self.image_id, other.image_id);
        crate::DeepMerge::merge_from(&mut self.last_state, other.last_state);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.ready, other.ready);
        crate::DeepMerge::merge_from(&mut self.resources, other.resources);
        crate::DeepMerge::merge_from(&mut self.restart_count, other.restart_count);
        crate::DeepMerge::merge_from(&mut self.started, other.started);
        crate::DeepMerge::merge_from(&mut self.state, other.state);
        crate::DeepMerge::merge_from(&mut self.stop_signal, other.stop_signal);
        crate::DeepMerge::merge_from(&mut self.user, other.user);
        crate::merge_strategies::list::map(
            &mut self.volume_mounts,
            other.volume_mounts,
            &[|lhs, rhs| lhs.mount_path == rhs.mount_path],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
    }
}

impl<'de> crate::serde::Deserialize<'de> for ContainerStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allocated_resources,
            Key_allocated_resources_status,
            Key_container_id,
            Key_image,
            Key_image_id,
            Key_last_state,
            Key_name,
            Key_ready,
            Key_resources,
            Key_restart_count,
            Key_started,
            Key_state,
            Key_stop_signal,
            Key_user,
            Key_volume_mounts,
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
                            "allocatedResources" => Field::Key_allocated_resources,
                            "allocatedResourcesStatus" => Field::Key_allocated_resources_status,
                            "containerID" => Field::Key_container_id,
                            "image" => Field::Key_image,
                            "imageID" => Field::Key_image_id,
                            "lastState" => Field::Key_last_state,
                            "name" => Field::Key_name,
                            "ready" => Field::Key_ready,
                            "resources" => Field::Key_resources,
                            "restartCount" => Field::Key_restart_count,
                            "started" => Field::Key_started,
                            "state" => Field::Key_state,
                            "stopSignal" => Field::Key_stop_signal,
                            "user" => Field::Key_user,
                            "volumeMounts" => Field::Key_volume_mounts,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ContainerStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allocated_resources: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_allocated_resources_status: Option<std::vec::Vec<crate::api::core::v1::ResourceStatus>> = None;
                let mut value_container_id: Option<std::string::String> = None;
                let mut value_image: Option<std::string::String> = None;
                let mut value_image_id: Option<std::string::String> = None;
                let mut value_last_state: Option<crate::api::core::v1::ContainerState> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_ready: Option<bool> = None;
                let mut value_resources: Option<crate::api::core::v1::ResourceRequirements> = None;
                let mut value_restart_count: Option<i32> = None;
                let mut value_started: Option<bool> = None;
                let mut value_state: Option<crate::api::core::v1::ContainerState> = None;
                let mut value_stop_signal: Option<std::string::String> = None;
                let mut value_user: Option<crate::api::core::v1::ContainerUser> = None;
                let mut value_volume_mounts: Option<std::vec::Vec<crate::api::core::v1::VolumeMountStatus>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allocated_resources => value_allocated_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allocated_resources_status => value_allocated_resources_status = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_container_id => value_container_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image => value_image = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image_id => value_image_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_last_state => value_last_state = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ready => value_ready = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_restart_count => value_restart_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_started => value_started = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_state => value_state = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_stop_signal => value_stop_signal = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_user => value_user = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_mounts => value_volume_mounts = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerStatus {
                    allocated_resources: value_allocated_resources,
                    allocated_resources_status: value_allocated_resources_status,
                    container_id: value_container_id,
                    image: value_image.unwrap_or_default(),
                    image_id: value_image_id.unwrap_or_default(),
                    last_state: value_last_state,
                    name: value_name.unwrap_or_default(),
                    ready: value_ready.unwrap_or_default(),
                    resources: value_resources,
                    restart_count: value_restart_count.unwrap_or_default(),
                    started: value_started,
                    state: value_state,
                    stop_signal: value_stop_signal,
                    user: value_user,
                    volume_mounts: value_volume_mounts,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerStatus",
            &[
                "allocatedResources",
                "allocatedResourcesStatus",
                "containerID",
                "image",
                "imageID",
                "lastState",
                "name",
                "ready",
                "resources",
                "restartCount",
                "started",
                "state",
                "stopSignal",
                "user",
                "volumeMounts",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ContainerStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerStatus",
            5 +
            self.allocated_resources.as_ref().map_or(0, |_| 1) +
            self.allocated_resources_status.as_ref().map_or(0, |_| 1) +
            self.container_id.as_ref().map_or(0, |_| 1) +
            self.last_state.as_ref().map_or(0, |_| 1) +
            self.resources.as_ref().map_or(0, |_| 1) +
            self.started.as_ref().map_or(0, |_| 1) +
            self.state.as_ref().map_or(0, |_| 1) +
            self.stop_signal.as_ref().map_or(0, |_| 1) +
            self.user.as_ref().map_or(0, |_| 1) +
            self.volume_mounts.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allocated_resources {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allocatedResources", value)?;
        }
        if let Some(value) = &self.allocated_resources_status {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allocatedResourcesStatus", value)?;
        }
        if let Some(value) = &self.container_id {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "containerID", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "image", &self.image)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "imageID", &self.image_id)?;
        if let Some(value) = &self.last_state {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lastState", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ready", &self.ready)?;
        if let Some(value) = &self.resources {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resources", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "restartCount", &self.restart_count)?;
        if let Some(value) = &self.started {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "started", value)?;
        }
        if let Some(value) = &self.state {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "state", value)?;
        }
        if let Some(value) = &self.stop_signal {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "stopSignal", value)?;
        }
        if let Some(value) = &self.user {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "user", value)?;
        }
        if let Some(value) = &self.volume_mounts {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeMounts", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ContainerStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.ContainerStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ContainerStatus contains details for the current status of this container.",
            "type": "object",
            "properties": {
                "allocatedResources": {
                    "description": "AllocatedResources represents the compute resources allocated for this container by the node. Kubelet sets this value to Container.Resources.Requests upon successful pod admission and after successfully admitting desired pod resize.",
                    "type": "object",
                    "additionalProperties": (__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>()),
                },
                "allocatedResourcesStatus": {
                    "description": "AllocatedResourcesStatus represents the status of various resources allocated for this Pod.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::ResourceStatus>()),
                },
                "containerID": {
                    "description": "ContainerID is the ID of the container in the format '<type>://<container_id>'. Where type is a container runtime identifier, returned from Version call of CRI API (for example \"containerd\").",
                    "type": "string",
                },
                "image": {
                    "description": "Image is the name of container image that the container is running. The container image may not match the image used in the PodSpec, as it may have been resolved by the runtime. More info: https://kubernetes.io/docs/concepts/containers/images.",
                    "type": "string",
                },
                "imageID": {
                    "description": "ImageID is the image ID of the container's image. The image ID may not match the image ID of the image used in the PodSpec, as it may have been resolved by the runtime.",
                    "type": "string",
                },
                "lastState": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ContainerState>();
                    schema_obj.ensure_object().insert("description".into(), "LastTerminationState holds the last termination state of the container to help debug container crashes and restarts. This field is not populated if the container is still running and RestartCount is 0.".into());
                    schema_obj
                }),
                "name": {
                    "description": "Name is a DNS_LABEL representing the unique name of the container. Each container in a pod must have a unique name across all container types. Cannot be updated.",
                    "type": "string",
                },
                "ready": {
                    "description": "Ready specifies whether the container is currently passing its readiness check. The value will change as readiness probes keep executing. If no readiness probes are specified, this field defaults to true once the container is fully started (see Started field).\n\nThe value is typically used to determine whether a container is ready to accept traffic.",
                    "type": "boolean",
                },
                "resources": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ResourceRequirements>();
                    schema_obj.ensure_object().insert("description".into(), "Resources represents the compute resource requests and limits that have been successfully enacted on the running container after it has been started or has been successfully resized.".into());
                    schema_obj
                }),
                "restartCount": {
                    "description": "RestartCount holds the number of times the container has been restarted. Kubelet makes an effort to always increment the value, but there are cases when the state may be lost due to node restarts and then the value may be reset to 0. The value is never negative.",
                    "type": "integer",
                    "format": "int32",
                },
                "started": {
                    "description": "Started indicates whether the container has finished its postStart lifecycle hook and passed its startup probe. Initialized as false, becomes true after startupProbe is considered successful. Resets to false when the container is restarted, or if kubelet loses state temporarily. In both cases, startup probes will run again. Is always true when no startupProbe is defined and container is running and has passed the postStart lifecycle hook. The null value must be treated the same as false.",
                    "type": "boolean",
                },
                "state": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ContainerState>();
                    schema_obj.ensure_object().insert("description".into(), "State holds details about the container's current condition.".into());
                    schema_obj
                }),
                "stopSignal": {
                    "description": "StopSignal reports the effective stop signal for this container",
                    "type": "string",
                },
                "user": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ContainerUser>();
                    schema_obj.ensure_object().insert("description".into(), "User represents user identity information initially attached to the first process of the container".into());
                    schema_obj
                }),
                "volumeMounts": {
                    "description": "Status of volume mounts.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::VolumeMountStatus>()),
                },
            },
            "required": [
                "image",
                "imageID",
                "name",
                "ready",
                "restartCount",
            ],
        })
    }
}
