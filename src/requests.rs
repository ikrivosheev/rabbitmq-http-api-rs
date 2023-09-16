use crate::commons::{ExchangeType, PolicyTarget, QueueType};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

/// Properties of a [virtual host](https://rabbitmq.com/vhosts.html) to be created or updated.
#[derive(Serialize)]
pub struct VirtualHostParams<'a> {
    /// Virtual host name
    pub name: &'a str,
    /// Optional description, e.g. what purpose does this virtual host serve?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    // A list of virtual host tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<&'a str>>,
    //
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_queue_type: Option<QueueType>,
    pub tracing: bool,
}

impl<'a> VirtualHostParams<'a> {
    pub fn named(name: &'a str) -> Self {
        VirtualHostParams {
            name,
            description: None,
            tags: None,
            default_queue_type: None,
            tracing: false,
        }
    }
}

/// Represents resource usage a limit to be enforced
/// on a [virtual host](https://rabbitmq.com/vhosts.html) or a user.
#[derive(Serialize)]
pub struct EnforcedLimitParams<T> {
    pub kind: T,
    pub value: i64,
}

impl<T> EnforcedLimitParams<T> {
    pub fn new(kind: T, value: i64) -> Self {
        EnforcedLimitParams { kind, value }
    }
}

/// Properties of a [user](https://rabbitmq.com/access-control.html#user-management) to be created or updated.
#[derive(Serialize)]
pub struct UserParams<'a> {
    pub name: &'a str,
    pub password_hash: &'a str,
    pub tags: &'a str,
}

pub type XArguments = Option<Map<String, Value>>;

/// [Queue](https://rabbitmq.com/queues.html) properties used at queue declaration time
#[derive(Serialize)]
pub struct QueueParams<'a> {
    /// The name of the queue to declare.
    /// Must be no longer than 255 bytes in length.
    pub name: &'a str,
    /// The type of the queue to declare, such as
    /// [quorum](https://rabbitmq.com/quorum-queues.html), classic, or [stream](https://rabbitmq.com/streams.html)
    #[serde(skip_serializing)]
    pub queue_type: QueueType,
    /// [Queue durability](https://rabbitmq.com/queues.html#durability)
    pub durable: bool,
    /// Should this queue be an [auto-delete](https://rabbitmq.com/queues.html#temporary-queues) one?
    pub auto_delete: bool,
    /// Should this queue be an [exclusive](https://rabbitmq.com/queues.html#temporary-queues) one?
    pub exclusive: bool,
    /// [Optional queue arguments](https://rabbitmq.com/queues.html#optional-arguments)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: XArguments,
}

impl<'a> QueueParams<'a> {
    /// Instantiates a [`QueueParams`] of a [quorum queue](https://rabbitmq.com/quorum-queues.html).
    pub fn new_quorum_queue(name: &'a str, optional_args: XArguments) -> Self {
        let typ = QueueType::Quorum;
        let args = Self::combined_args(optional_args, &typ);
        Self {
            name,
            queue_type: QueueType::Quorum,
            durable: true,
            auto_delete: false,
            exclusive: false,
            arguments: args,
        }
    }

    /// Instantiates a [`QueueParams`] of a [stream](https://rabbitmq.com/streams.html).
    pub fn new_stream(name: &'a str, optional_args: XArguments) -> Self {
        let typ = QueueType::Stream;
        let args = Self::combined_args(optional_args, &typ);
        Self {
            name,
            queue_type: QueueType::Stream,
            durable: true,
            auto_delete: false,
            exclusive: false,
            arguments: args,
        }
    }

    /// Instantiates a [`QueueParams`] of a classic [durable queue](https://rabbitmq.com/queues.html).
    pub fn new_durable_classic_queue(name: &'a str, optional_args: XArguments) -> Self {
        let typ = QueueType::Classic;
        let args = Self::combined_args(optional_args, &typ);
        Self {
            name,
            queue_type: QueueType::Classic,
            durable: true,
            auto_delete: false,
            exclusive: false,
            arguments: args,
        }
    }

    pub fn new(
        name: &'a str,
        queue_type: QueueType,
        durable: bool,
        auto_delete: bool,
        optional_args: XArguments,
    ) -> Self {
        let args = Self::combined_args(optional_args, &queue_type);
        Self {
            name,
            queue_type,
            durable,
            auto_delete,
            exclusive: false,
            arguments: args,
        }
    }

    pub fn combined_args(optional_args: XArguments, queue_type: &QueueType) -> XArguments {
        let mut result = Map::<String, Value>::new();
        result.insert("x-queue-type".to_owned(), json!(queue_type));

        if let Some(mut val) = optional_args {
            result.append(&mut val)
        }

        Some(result)
    }
}

/// Exchange properties used at queue declaration time
#[derive(Debug, Serialize)]
pub struct ExchangeParams<'a> {
    pub name: &'a str,
    #[serde(rename(serialize = "type"))]
    pub exchange_type: ExchangeType,
    pub durable: bool,
    pub auto_delete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: XArguments,
}

impl<'a> ExchangeParams<'a> {
    pub fn durable(name: &'a str, exchange_type: ExchangeType, optional_args: XArguments) -> Self {
        Self::new(name, exchange_type, true, false, optional_args)
    }

    /// Instantiates a [`ExchangeParams`] of a [fanout exchange]](https://rabbitmq.com/tutorials/tutorial-three-python.html).
    pub fn fanout(
        name: &'a str,
        durable: bool,
        auto_delete: bool,
        optional_args: XArguments,
    ) -> Self {
        Self::new(
            name,
            ExchangeType::Fanout,
            durable,
            auto_delete,
            optional_args,
        )
    }

    /// Instantiates a [`ExchangeParams`] of a durable [fanout exchange]](https://rabbitmq.com/tutorials/tutorial-three-python.html).
    pub fn durable_fanout(name: &'a str, optional_args: XArguments) -> Self {
        Self::new(name, ExchangeType::Fanout, true, false, optional_args)
    }

    pub fn topic(
        name: &'a str,
        durable: bool,
        auto_delete: bool,
        optional_args: XArguments,
    ) -> Self {
        Self::new(
            name,
            ExchangeType::Topic,
            durable,
            auto_delete,
            optional_args,
        )
    }

    /// Instantiates a [`ExchangeParams`] of a durable [fanout exchange]](https://rabbitmq.com/tutorials/tutorial-five-python.html).
    pub fn durable_topic(name: &'a str, optional_args: XArguments) -> Self {
        Self::new(name, ExchangeType::Topic, true, false, optional_args)
    }

    /// Instantiates a [`ExchangeParams`] of a [direct exchange]](https://rabbitmq.com/tutorials/tutorial-four-python.html).
    pub fn direct(
        name: &'a str,
        durable: bool,
        auto_delete: bool,
        optional_args: XArguments,
    ) -> Self {
        Self::new(
            name,
            ExchangeType::Direct,
            durable,
            auto_delete,
            optional_args,
        )
    }

    /// Instantiates a [`ExchangeParams`] of a durable [direct exchange]](https://rabbitmq.com/tutorials/tutorial-four-python.html).
    pub fn durable_direct(name: &'a str, optional_args: XArguments) -> Self {
        Self::new(name, ExchangeType::Direct, true, false, optional_args)
    }

    /// Instantiates a [`ExchangeParams`] of a headers exchange
    pub fn headers(
        name: &'a str,
        durable: bool,
        auto_delete: bool,
        optional_args: XArguments,
    ) -> Self {
        Self::new(
            name,
            ExchangeType::Headers,
            durable,
            auto_delete,
            optional_args,
        )
    }

    /// Instantiates a [`ExchangeParams`] of a durable headers exchange
    pub fn durable_headers(name: &'a str, optional_args: XArguments) -> Self {
        Self::new(name, ExchangeType::Headers, true, false, optional_args)
    }

    pub fn new(
        name: &'a str,
        exchange_type: ExchangeType,
        durable: bool,
        auto_delete: bool,
        optional_args: XArguments,
    ) -> Self {
        Self {
            name,
            exchange_type,
            durable,
            auto_delete,
            arguments: optional_args,
        }
    }
}

pub type RuntimeParameterValue = Map<String, Value>;

/// Represents a [runtime parameter](https://rabbitmq.com/parameters.html).
#[derive(Serialize, Deserialize)]
pub struct RuntimeParameterDefinition {
    pub name: String,
    pub vhost: String,
    pub component: String,
    pub value: RuntimeParameterValue,
}

pub type PolicyDefinition = Option<Map<String, Value>>;

/// Represents a [policy](https://rabbitmq.com/parameters.html#policies).
#[derive(Serialize)]
pub struct PolicyParams<'a> {
    pub vhost: &'a str,
    pub name: &'a str,
    pub pattern: &'a str,
    #[serde(rename(serialize = "apply-to"))]
    pub apply_to: PolicyTarget,
    pub priority: i32,
    pub definition: PolicyDefinition,
}

/// Represents a user's [permission in a particular virtual host](https://rabbitmq.com/access-control.html).
#[derive(Serialize)]
pub struct Permissions<'a> {
    pub user: &'a str,
    pub vhost: &'a str,
    pub configure: &'a str,
    pub read: &'a str,
    pub write: &'a str,
}

pub type MessageProperties = Map<String, Value>;
