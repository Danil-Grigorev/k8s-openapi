// Generated from definition io.k8s.api.core.v1.EphemeralContainer

/// An EphemeralContainer is a temporary container that you may add to an existing Pod for user-initiated activities such as debugging. Ephemeral containers have no resource or scheduling guarantees, and they will not be restarted when they exit or when a Pod is removed or restarted. The kubelet may evict a Pod if an ephemeral container causes the Pod to exceed its resource allocation.
///
/// To add an ephemeral container, use the ephemeralcontainers subresource of an existing Pod. Ephemeral containers may not be removed or restarted.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EphemeralContainer {
    /// Arguments to the entrypoint. The image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    pub args: Option<std::vec::Vec<std::string::String>>,

    /// Entrypoint array. Not executed within a shell. The image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    pub command: Option<std::vec::Vec<std::string::String>>,

    /// List of environment variables to set in the container. Cannot be updated.
    pub env: Option<std::vec::Vec<crate::api::core::v1::EnvVar>>,

    /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
    pub env_from: Option<std::vec::Vec<crate::api::core::v1::EnvFromSource>>,

    /// Container image name. More info: https://kubernetes.io/docs/concepts/containers/images
    pub image: Option<std::string::String>,

    /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
    pub image_pull_policy: Option<std::string::String>,

    /// Lifecycle is not allowed for ephemeral containers.
    pub lifecycle: Option<crate::api::core::v1::Lifecycle>,

    /// Probes are not allowed for ephemeral containers.
    pub liveness_probe: Option<crate::api::core::v1::Probe>,

    /// Name of the ephemeral container specified as a DNS_LABEL. This name must be unique among all containers, init containers and ephemeral containers.
    pub name: std::string::String,

    /// Ports are not allowed for ephemeral containers.
    pub ports: Option<std::vec::Vec<crate::api::core::v1::ContainerPort>>,

    /// Probes are not allowed for ephemeral containers.
    pub readiness_probe: Option<crate::api::core::v1::Probe>,

    /// Resources resize policy for the container.
    pub resize_policy: Option<std::vec::Vec<crate::api::core::v1::ContainerResizePolicy>>,

    /// Resources are not allowed for ephemeral containers. Ephemeral containers use spare resources already allocated to the pod.
    pub resources: Option<crate::api::core::v1::ResourceRequirements>,

    /// Restart policy for the container to manage the restart behavior of each container within a pod. This may only be set for init containers. You cannot set this field on ephemeral containers.
    pub restart_policy: Option<std::string::String>,

    /// Optional: SecurityContext defines the security options the ephemeral container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext.
    pub security_context: Option<crate::api::core::v1::SecurityContext>,

    /// Probes are not allowed for ephemeral containers.
    pub startup_probe: Option<crate::api::core::v1::Probe>,

    /// Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
    pub stdin: Option<bool>,

    /// Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
    pub stdin_once: Option<bool>,

    /// If set, the name of the container from PodSpec that this ephemeral container targets. The ephemeral container will be run in the namespaces (IPC, PID, etc) of this container. If not set then the ephemeral container uses the namespaces configured in the Pod spec.
    ///
    /// The container runtime must implement support for this feature. If the runtime does not support namespace targeting then the result of setting this field is undefined.
    pub target_container_name: Option<std::string::String>,

    /// Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
    pub termination_message_path: Option<std::string::String>,

    /// Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
    pub termination_message_policy: Option<std::string::String>,

    /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
    pub tty: Option<bool>,

    /// volumeDevices is the list of block devices to be used by the container.
    pub volume_devices: Option<std::vec::Vec<crate::api::core::v1::VolumeDevice>>,

    /// Pod volumes to mount into the container's filesystem. Subpath mounts are not allowed for ephemeral containers. Cannot be updated.
    pub volume_mounts: Option<std::vec::Vec<crate::api::core::v1::VolumeMount>>,

    /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
    pub working_dir: Option<std::string::String>,
}

impl crate::DeepMerge for EphemeralContainer {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.args, other.args);
        crate::merge_strategies::list::atomic(&mut self.command, other.command);
        crate::merge_strategies::list::map(
            &mut self.env,
            other.env,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::merge_strategies::list::atomic(&mut self.env_from, other.env_from);
        crate::DeepMerge::merge_from(&mut self.image, other.image);
        crate::DeepMerge::merge_from(&mut self.image_pull_policy, other.image_pull_policy);
        crate::DeepMerge::merge_from(&mut self.lifecycle, other.lifecycle);
        crate::DeepMerge::merge_from(&mut self.liveness_probe, other.liveness_probe);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::merge_strategies::list::map(
            &mut self.ports,
            other.ports,
            &[|lhs, rhs| lhs.container_port == rhs.container_port],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.readiness_probe, other.readiness_probe);
        crate::merge_strategies::list::atomic(&mut self.resize_policy, other.resize_policy);
        crate::DeepMerge::merge_from(&mut self.resources, other.resources);
        crate::DeepMerge::merge_from(&mut self.restart_policy, other.restart_policy);
        crate::DeepMerge::merge_from(&mut self.security_context, other.security_context);
        crate::DeepMerge::merge_from(&mut self.startup_probe, other.startup_probe);
        crate::DeepMerge::merge_from(&mut self.stdin, other.stdin);
        crate::DeepMerge::merge_from(&mut self.stdin_once, other.stdin_once);
        crate::DeepMerge::merge_from(&mut self.target_container_name, other.target_container_name);
        crate::DeepMerge::merge_from(&mut self.termination_message_path, other.termination_message_path);
        crate::DeepMerge::merge_from(&mut self.termination_message_policy, other.termination_message_policy);
        crate::DeepMerge::merge_from(&mut self.tty, other.tty);
        crate::merge_strategies::list::map(
            &mut self.volume_devices,
            other.volume_devices,
            &[|lhs, rhs| lhs.device_path == rhs.device_path],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::merge_strategies::list::map(
            &mut self.volume_mounts,
            other.volume_mounts,
            &[|lhs, rhs| lhs.mount_path == rhs.mount_path],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.working_dir, other.working_dir);
    }
}

impl<'de> crate::serde::Deserialize<'de> for EphemeralContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_args,
            Key_command,
            Key_env,
            Key_env_from,
            Key_image,
            Key_image_pull_policy,
            Key_lifecycle,
            Key_liveness_probe,
            Key_name,
            Key_ports,
            Key_readiness_probe,
            Key_resize_policy,
            Key_resources,
            Key_restart_policy,
            Key_security_context,
            Key_startup_probe,
            Key_stdin,
            Key_stdin_once,
            Key_target_container_name,
            Key_termination_message_path,
            Key_termination_message_policy,
            Key_tty,
            Key_volume_devices,
            Key_volume_mounts,
            Key_working_dir,
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
                            "args" => Field::Key_args,
                            "command" => Field::Key_command,
                            "env" => Field::Key_env,
                            "envFrom" => Field::Key_env_from,
                            "image" => Field::Key_image,
                            "imagePullPolicy" => Field::Key_image_pull_policy,
                            "lifecycle" => Field::Key_lifecycle,
                            "livenessProbe" => Field::Key_liveness_probe,
                            "name" => Field::Key_name,
                            "ports" => Field::Key_ports,
                            "readinessProbe" => Field::Key_readiness_probe,
                            "resizePolicy" => Field::Key_resize_policy,
                            "resources" => Field::Key_resources,
                            "restartPolicy" => Field::Key_restart_policy,
                            "securityContext" => Field::Key_security_context,
                            "startupProbe" => Field::Key_startup_probe,
                            "stdin" => Field::Key_stdin,
                            "stdinOnce" => Field::Key_stdin_once,
                            "targetContainerName" => Field::Key_target_container_name,
                            "terminationMessagePath" => Field::Key_termination_message_path,
                            "terminationMessagePolicy" => Field::Key_termination_message_policy,
                            "tty" => Field::Key_tty,
                            "volumeDevices" => Field::Key_volume_devices,
                            "volumeMounts" => Field::Key_volume_mounts,
                            "workingDir" => Field::Key_working_dir,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EphemeralContainer;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("EphemeralContainer")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_args: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_command: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_env: Option<std::vec::Vec<crate::api::core::v1::EnvVar>> = None;
                let mut value_env_from: Option<std::vec::Vec<crate::api::core::v1::EnvFromSource>> = None;
                let mut value_image: Option<std::string::String> = None;
                let mut value_image_pull_policy: Option<std::string::String> = None;
                let mut value_lifecycle: Option<crate::api::core::v1::Lifecycle> = None;
                let mut value_liveness_probe: Option<crate::api::core::v1::Probe> = None;
                let mut value_name: Option<std::string::String> = None;
                let mut value_ports: Option<std::vec::Vec<crate::api::core::v1::ContainerPort>> = None;
                let mut value_readiness_probe: Option<crate::api::core::v1::Probe> = None;
                let mut value_resize_policy: Option<std::vec::Vec<crate::api::core::v1::ContainerResizePolicy>> = None;
                let mut value_resources: Option<crate::api::core::v1::ResourceRequirements> = None;
                let mut value_restart_policy: Option<std::string::String> = None;
                let mut value_security_context: Option<crate::api::core::v1::SecurityContext> = None;
                let mut value_startup_probe: Option<crate::api::core::v1::Probe> = None;
                let mut value_stdin: Option<bool> = None;
                let mut value_stdin_once: Option<bool> = None;
                let mut value_target_container_name: Option<std::string::String> = None;
                let mut value_termination_message_path: Option<std::string::String> = None;
                let mut value_termination_message_policy: Option<std::string::String> = None;
                let mut value_tty: Option<bool> = None;
                let mut value_volume_devices: Option<std::vec::Vec<crate::api::core::v1::VolumeDevice>> = None;
                let mut value_volume_mounts: Option<std::vec::Vec<crate::api::core::v1::VolumeMount>> = None;
                let mut value_working_dir: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_args => value_args = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_command => value_command = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_env => value_env = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_env_from => value_env_from = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image => value_image = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image_pull_policy => value_image_pull_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_lifecycle => value_lifecycle = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_liveness_probe => value_liveness_probe = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ports => value_ports = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_readiness_probe => value_readiness_probe = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resize_policy => value_resize_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_restart_policy => value_restart_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_security_context => value_security_context = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_startup_probe => value_startup_probe = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_stdin => value_stdin = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_stdin_once => value_stdin_once = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_container_name => value_target_container_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_termination_message_path => value_termination_message_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_termination_message_policy => value_termination_message_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tty => value_tty = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_devices => value_volume_devices = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_mounts => value_volume_mounts = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_working_dir => value_working_dir = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EphemeralContainer {
                    args: value_args,
                    command: value_command,
                    env: value_env,
                    env_from: value_env_from,
                    image: value_image,
                    image_pull_policy: value_image_pull_policy,
                    lifecycle: value_lifecycle,
                    liveness_probe: value_liveness_probe,
                    name: value_name.unwrap_or_default(),
                    ports: value_ports,
                    readiness_probe: value_readiness_probe,
                    resize_policy: value_resize_policy,
                    resources: value_resources,
                    restart_policy: value_restart_policy,
                    security_context: value_security_context,
                    startup_probe: value_startup_probe,
                    stdin: value_stdin,
                    stdin_once: value_stdin_once,
                    target_container_name: value_target_container_name,
                    termination_message_path: value_termination_message_path,
                    termination_message_policy: value_termination_message_policy,
                    tty: value_tty,
                    volume_devices: value_volume_devices,
                    volume_mounts: value_volume_mounts,
                    working_dir: value_working_dir,
                })
            }
        }

        deserializer.deserialize_struct(
            "EphemeralContainer",
            &[
                "args",
                "command",
                "env",
                "envFrom",
                "image",
                "imagePullPolicy",
                "lifecycle",
                "livenessProbe",
                "name",
                "ports",
                "readinessProbe",
                "resizePolicy",
                "resources",
                "restartPolicy",
                "securityContext",
                "startupProbe",
                "stdin",
                "stdinOnce",
                "targetContainerName",
                "terminationMessagePath",
                "terminationMessagePolicy",
                "tty",
                "volumeDevices",
                "volumeMounts",
                "workingDir",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EphemeralContainer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EphemeralContainer",
            1 +
            self.args.as_ref().map_or(0, |_| 1) +
            self.command.as_ref().map_or(0, |_| 1) +
            self.env.as_ref().map_or(0, |_| 1) +
            self.env_from.as_ref().map_or(0, |_| 1) +
            self.image.as_ref().map_or(0, |_| 1) +
            self.image_pull_policy.as_ref().map_or(0, |_| 1) +
            self.lifecycle.as_ref().map_or(0, |_| 1) +
            self.liveness_probe.as_ref().map_or(0, |_| 1) +
            self.ports.as_ref().map_or(0, |_| 1) +
            self.readiness_probe.as_ref().map_or(0, |_| 1) +
            self.resize_policy.as_ref().map_or(0, |_| 1) +
            self.resources.as_ref().map_or(0, |_| 1) +
            self.restart_policy.as_ref().map_or(0, |_| 1) +
            self.security_context.as_ref().map_or(0, |_| 1) +
            self.startup_probe.as_ref().map_or(0, |_| 1) +
            self.stdin.as_ref().map_or(0, |_| 1) +
            self.stdin_once.as_ref().map_or(0, |_| 1) +
            self.target_container_name.as_ref().map_or(0, |_| 1) +
            self.termination_message_path.as_ref().map_or(0, |_| 1) +
            self.termination_message_policy.as_ref().map_or(0, |_| 1) +
            self.tty.as_ref().map_or(0, |_| 1) +
            self.volume_devices.as_ref().map_or(0, |_| 1) +
            self.volume_mounts.as_ref().map_or(0, |_| 1) +
            self.working_dir.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.args {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "args", value)?;
        }
        if let Some(value) = &self.command {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "command", value)?;
        }
        if let Some(value) = &self.env {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "env", value)?;
        }
        if let Some(value) = &self.env_from {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "envFrom", value)?;
        }
        if let Some(value) = &self.image {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "image", value)?;
        }
        if let Some(value) = &self.image_pull_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "imagePullPolicy", value)?;
        }
        if let Some(value) = &self.lifecycle {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lifecycle", value)?;
        }
        if let Some(value) = &self.liveness_probe {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "livenessProbe", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.ports {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ports", value)?;
        }
        if let Some(value) = &self.readiness_probe {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readinessProbe", value)?;
        }
        if let Some(value) = &self.resize_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resizePolicy", value)?;
        }
        if let Some(value) = &self.resources {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resources", value)?;
        }
        if let Some(value) = &self.restart_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "restartPolicy", value)?;
        }
        if let Some(value) = &self.security_context {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "securityContext", value)?;
        }
        if let Some(value) = &self.startup_probe {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "startupProbe", value)?;
        }
        if let Some(value) = &self.stdin {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "stdin", value)?;
        }
        if let Some(value) = &self.stdin_once {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "stdinOnce", value)?;
        }
        if let Some(value) = &self.target_container_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "targetContainerName", value)?;
        }
        if let Some(value) = &self.termination_message_path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "terminationMessagePath", value)?;
        }
        if let Some(value) = &self.termination_message_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "terminationMessagePolicy", value)?;
        }
        if let Some(value) = &self.tty {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tty", value)?;
        }
        if let Some(value) = &self.volume_devices {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeDevices", value)?;
        }
        if let Some(value) = &self.volume_mounts {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeMounts", value)?;
        }
        if let Some(value) = &self.working_dir {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "workingDir", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for EphemeralContainer {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.EphemeralContainer".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "An EphemeralContainer is a temporary container that you may add to an existing Pod for user-initiated activities such as debugging. Ephemeral containers have no resource or scheduling guarantees, and they will not be restarted when they exit or when a Pod is removed or restarted. The kubelet may evict a Pod if an ephemeral container causes the Pod to exceed its resource allocation.\n\nTo add an ephemeral container, use the ephemeralcontainers subresource of an existing Pod. Ephemeral containers may not be removed or restarted.",
            "type": "object",
            "properties": {
                "args": {
                    "description": "Arguments to the entrypoint. The image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. \"$$(VAR_NAME)\" will produce the string literal \"$(VAR_NAME)\". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "command": {
                    "description": "Entrypoint array. Not executed within a shell. The image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. \"$$(VAR_NAME)\" will produce the string literal \"$(VAR_NAME)\". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "env": {
                    "description": "List of environment variables to set in the container. Cannot be updated.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::EnvVar>()),
                },
                "envFrom": {
                    "description": "List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::EnvFromSource>()),
                },
                "image": {
                    "description": "Container image name. More info: https://kubernetes.io/docs/concepts/containers/images",
                    "type": "string",
                },
                "imagePullPolicy": {
                    "description": "Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images",
                    "type": "string",
                },
                "lifecycle": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::Lifecycle>();
                    schema_obj.ensure_object().insert("description".into(), "Lifecycle is not allowed for ephemeral containers.".into());
                    schema_obj
                }),
                "livenessProbe": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::Probe>();
                    schema_obj.ensure_object().insert("description".into(), "Probes are not allowed for ephemeral containers.".into());
                    schema_obj
                }),
                "name": {
                    "description": "Name of the ephemeral container specified as a DNS_LABEL. This name must be unique among all containers, init containers and ephemeral containers.",
                    "type": "string",
                },
                "ports": {
                    "description": "Ports are not allowed for ephemeral containers.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::ContainerPort>()),
                },
                "readinessProbe": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::Probe>();
                    schema_obj.ensure_object().insert("description".into(), "Probes are not allowed for ephemeral containers.".into());
                    schema_obj
                }),
                "resizePolicy": {
                    "description": "Resources resize policy for the container.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::ContainerResizePolicy>()),
                },
                "resources": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ResourceRequirements>();
                    schema_obj.ensure_object().insert("description".into(), "Resources are not allowed for ephemeral containers. Ephemeral containers use spare resources already allocated to the pod.".into());
                    schema_obj
                }),
                "restartPolicy": {
                    "description": "Restart policy for the container to manage the restart behavior of each container within a pod. This may only be set for init containers. You cannot set this field on ephemeral containers.",
                    "type": "string",
                },
                "securityContext": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SecurityContext>();
                    schema_obj.ensure_object().insert("description".into(), "Optional: SecurityContext defines the security options the ephemeral container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext.".into());
                    schema_obj
                }),
                "startupProbe": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::Probe>();
                    schema_obj.ensure_object().insert("description".into(), "Probes are not allowed for ephemeral containers.".into());
                    schema_obj
                }),
                "stdin": {
                    "description": "Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.",
                    "type": "boolean",
                },
                "stdinOnce": {
                    "description": "Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false",
                    "type": "boolean",
                },
                "targetContainerName": {
                    "description": "If set, the name of the container from PodSpec that this ephemeral container targets. The ephemeral container will be run in the namespaces (IPC, PID, etc) of this container. If not set then the ephemeral container uses the namespaces configured in the Pod spec.\n\nThe container runtime must implement support for this feature. If the runtime does not support namespace targeting then the result of setting this field is undefined.",
                    "type": "string",
                },
                "terminationMessagePath": {
                    "description": "Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.",
                    "type": "string",
                },
                "terminationMessagePolicy": {
                    "description": "Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.",
                    "type": "string",
                },
                "tty": {
                    "description": "Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.",
                    "type": "boolean",
                },
                "volumeDevices": {
                    "description": "volumeDevices is the list of block devices to be used by the container.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::VolumeDevice>()),
                },
                "volumeMounts": {
                    "description": "Pod volumes to mount into the container's filesystem. Subpath mounts are not allowed for ephemeral containers. Cannot be updated.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::VolumeMount>()),
                },
                "workingDir": {
                    "description": "Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.",
                    "type": "string",
                },
            },
            "required": [
                "name",
            ],
        })
    }
}
