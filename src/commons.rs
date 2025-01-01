// Copyright (C) 2023-2025 RabbitMQ Core Team (teamrabbitmq@gmail.com)
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all(serialize = "lowercase", deserialize = "PascalCase"))]
pub enum SupportedProtocol {
    Clustering,
    /// Represents both AMQP 1.0 and AMQP 0-9-1 because they share a listener
    #[serde(rename = "amqp")]
    AMQP,
    /// Represents both AMQP 1.0 with TLS enabled and AMQP 0-9-1 with TLS enabled
    #[serde(rename = "amqp/ssl")]
    AMQPWithTLS,
    /// Represents the RabbitMQ Stream protocol
    #[serde(rename = "stream")]
    Stream,
    /// Represents the RabbitMQ Stream protocol with TLS enabled
    #[serde(rename = "stream/ssl")]
    StreamWithTLS,
    #[serde(rename = "mqtt")]
    MQTT,
    #[serde(rename = "mqtt/ssl")]
    MQTTWithTLS,
    #[serde(rename = "stomp")]
    STOMP,
    #[serde(rename = "stomp/ssl")]
    STOMPWithTLS,
    #[serde(rename = "http/web-mqtt")]
    MQTTOverWebSockets,
    #[serde(rename = "https/web-mqtt")]
    MQTTOverWebSocketsWithTLS,
    #[serde(rename = "http/web-stomp")]
    STOMPOverWebsockets,
    #[serde(rename = "https/web-stomp")]
    STOMPOverWebsocketsWithTLS,
    #[serde(rename = "http/prometheus")]
    Prometheus,
    #[serde(rename = "https/prometheus")]
    PrometheusWithTLS,
    #[serde(rename = "http")]
    HTTP,
    #[serde(rename = "https")]
    HTTPWithTLS,
    Other(String),
}

const SUPPORTED_PROTOCOL_CLUSTERING: &str = "clustering";

const SUPPORTED_PROTOCOL_AMQP: &str = "amqp";
const SUPPORTED_PROTOCOL_AMQP_WITH_TLS: &str = "amqps";

const SUPPORTED_PROTOCOL_STREAM: &str = "stream";
const SUPPORTED_PROTOCOL_STREAM_WITH_TLS: &str = "stream/ssl";

const SUPPORTED_PROTOCOL_MQTT: &str = "mqtt";
const SUPPORTED_PROTOCOL_MQTT_WITH_TLS: &str = "mqtt/ssl";
const SUPPORTED_PROTOCOL_MQTT_OVER_WEBSOCKETS: &str = "http/web-mqtt";
const SUPPORTED_PROTOCOL_MQTT_OVER_WEBSOCKETS_WITH_TLS: &str = "https/web-mqtt";

const SUPPORTED_PROTOCOL_STOMP: &str = "stomp";
const SUPPORTED_PROTOCOL_STOMP_WITH_TLS: &str = "stomp/ssl";
const SUPPORTED_PROTOCOL_STOMP_OVER_WEBSOCKETS: &str = "http/stomp-mqtt";
const SUPPORTED_PROTOCOL_STOMP_OVER_WEBSOCKETS_WITH_TLS: &str = "https/stomp-mqtt";

const SUPPORTED_PROTOCOL_PROMETHEUS: &str = "http/prometheus";
const SUPPORTED_PROTOCOL_PROMETHEUS_WITH_TLS: &str = "https/prometheus";

const SUPPORTED_PROTOCOL_HTTP: &str = "http";
const SUPPORTED_PROTOCOL_HTTP_WITH_TLS: &str = "https";

impl From<&str> for SupportedProtocol {
    fn from(value: &str) -> Self {
        match value {
            SUPPORTED_PROTOCOL_CLUSTERING => SupportedProtocol::Clustering,
            SUPPORTED_PROTOCOL_AMQP => SupportedProtocol::AMQP,
            SUPPORTED_PROTOCOL_AMQP_WITH_TLS => SupportedProtocol::AMQPWithTLS,
            SUPPORTED_PROTOCOL_STREAM => SupportedProtocol::Stream,
            SUPPORTED_PROTOCOL_STREAM_WITH_TLS => SupportedProtocol::StreamWithTLS,
            SUPPORTED_PROTOCOL_MQTT => SupportedProtocol::MQTT,
            SUPPORTED_PROTOCOL_MQTT_WITH_TLS => SupportedProtocol::MQTTWithTLS,
            SUPPORTED_PROTOCOL_STOMP => SupportedProtocol::STOMP,
            SUPPORTED_PROTOCOL_STOMP_WITH_TLS => SupportedProtocol::STOMPWithTLS,
            SUPPORTED_PROTOCOL_MQTT_OVER_WEBSOCKETS => SupportedProtocol::MQTTOverWebSockets,
            SUPPORTED_PROTOCOL_MQTT_OVER_WEBSOCKETS_WITH_TLS => {
                SupportedProtocol::MQTTOverWebSocketsWithTLS
            }
            SUPPORTED_PROTOCOL_STOMP_OVER_WEBSOCKETS => SupportedProtocol::STOMPOverWebsockets,
            SUPPORTED_PROTOCOL_STOMP_OVER_WEBSOCKETS_WITH_TLS => {
                SupportedProtocol::STOMPOverWebsocketsWithTLS
            }
            SUPPORTED_PROTOCOL_PROMETHEUS => SupportedProtocol::Prometheus,
            SUPPORTED_PROTOCOL_PROMETHEUS_WITH_TLS => SupportedProtocol::PrometheusWithTLS,
            SUPPORTED_PROTOCOL_HTTP => SupportedProtocol::HTTP,
            SUPPORTED_PROTOCOL_HTTP_WITH_TLS => SupportedProtocol::HTTPWithTLS,
            other => SupportedProtocol::Other(other.to_owned()),
        }
    }
}

impl From<String> for SupportedProtocol {
    fn from(value: String) -> Self {
        SupportedProtocol::from(value.as_str())
    }
}

impl From<SupportedProtocol> for String {
    fn from(value: SupportedProtocol) -> String {
        match value {
            SupportedProtocol::Clustering => SUPPORTED_PROTOCOL_CLUSTERING.to_owned(),
            SupportedProtocol::AMQP => SUPPORTED_PROTOCOL_AMQP.to_owned(),
            SupportedProtocol::AMQPWithTLS => SUPPORTED_PROTOCOL_AMQP_WITH_TLS.to_owned(),
            SupportedProtocol::Stream => SUPPORTED_PROTOCOL_STREAM.to_owned(),
            SupportedProtocol::StreamWithTLS => SUPPORTED_PROTOCOL_STREAM_WITH_TLS.to_owned(),
            SupportedProtocol::MQTT => SUPPORTED_PROTOCOL_MQTT.to_owned(),
            SupportedProtocol::MQTTWithTLS => SUPPORTED_PROTOCOL_MQTT_WITH_TLS.to_owned(),
            SupportedProtocol::STOMP => SUPPORTED_PROTOCOL_STOMP.to_owned(),
            SupportedProtocol::STOMPWithTLS => SUPPORTED_PROTOCOL_STOMP_WITH_TLS.to_owned(),
            SupportedProtocol::MQTTOverWebSockets => {
                SUPPORTED_PROTOCOL_MQTT_OVER_WEBSOCKETS.to_owned()
            }
            SupportedProtocol::MQTTOverWebSocketsWithTLS => {
                SUPPORTED_PROTOCOL_MQTT_OVER_WEBSOCKETS_WITH_TLS.to_owned()
            }
            SupportedProtocol::STOMPOverWebsockets => {
                SUPPORTED_PROTOCOL_STOMP_OVER_WEBSOCKETS.to_owned()
            }
            SupportedProtocol::STOMPOverWebsocketsWithTLS => {
                SUPPORTED_PROTOCOL_STOMP_OVER_WEBSOCKETS_WITH_TLS.to_owned()
            }
            SupportedProtocol::Prometheus => SUPPORTED_PROTOCOL_PROMETHEUS.to_owned(),
            SupportedProtocol::PrometheusWithTLS => {
                SUPPORTED_PROTOCOL_PROMETHEUS_WITH_TLS.to_owned()
            }
            SupportedProtocol::HTTP => SUPPORTED_PROTOCOL_HTTP.to_owned(),
            SupportedProtocol::HTTPWithTLS => SUPPORTED_PROTOCOL_HTTP_WITH_TLS.to_owned(),
            SupportedProtocol::Other(s) => s,
        }
    }
}

impl From<&SupportedProtocol> for String {
    fn from(value: &SupportedProtocol) -> Self {
        match value {
            SupportedProtocol::Clustering => SUPPORTED_PROTOCOL_CLUSTERING.to_owned(),
            SupportedProtocol::AMQP => SUPPORTED_PROTOCOL_AMQP.to_owned(),
            SupportedProtocol::AMQPWithTLS => SUPPORTED_PROTOCOL_AMQP_WITH_TLS.to_owned(),
            SupportedProtocol::Stream => SUPPORTED_PROTOCOL_STREAM.to_owned(),
            SupportedProtocol::StreamWithTLS => SUPPORTED_PROTOCOL_STREAM_WITH_TLS.to_owned(),
            SupportedProtocol::MQTT => SUPPORTED_PROTOCOL_MQTT.to_owned(),
            SupportedProtocol::MQTTWithTLS => SUPPORTED_PROTOCOL_MQTT_WITH_TLS.to_owned(),
            SupportedProtocol::STOMP => SUPPORTED_PROTOCOL_STOMP.to_owned(),
            SupportedProtocol::STOMPWithTLS => SUPPORTED_PROTOCOL_STOMP_WITH_TLS.to_owned(),
            SupportedProtocol::MQTTOverWebSockets => {
                SUPPORTED_PROTOCOL_MQTT_OVER_WEBSOCKETS.to_owned()
            }
            SupportedProtocol::MQTTOverWebSocketsWithTLS => {
                SUPPORTED_PROTOCOL_MQTT_OVER_WEBSOCKETS_WITH_TLS.to_owned()
            }
            SupportedProtocol::STOMPOverWebsockets => {
                SUPPORTED_PROTOCOL_STOMP_OVER_WEBSOCKETS.to_owned()
            }
            SupportedProtocol::STOMPOverWebsocketsWithTLS => {
                SUPPORTED_PROTOCOL_STOMP_OVER_WEBSOCKETS_WITH_TLS.to_owned()
            }
            SupportedProtocol::Prometheus => SUPPORTED_PROTOCOL_PROMETHEUS.to_owned(),
            SupportedProtocol::PrometheusWithTLS => {
                SUPPORTED_PROTOCOL_PROMETHEUS_WITH_TLS.to_owned()
            }
            SupportedProtocol::HTTP => SUPPORTED_PROTOCOL_HTTP.to_owned(),
            SupportedProtocol::HTTPWithTLS => SUPPORTED_PROTOCOL_HTTP_WITH_TLS.to_owned(),
            SupportedProtocol::Other(s) => (*s).clone(),
        }
    }
}

impl fmt::Display for SupportedProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let proto: String = self.into();
        write!(f, "{}", proto)
    }
}

/// Exchange types. Most variants are for exchange types included with modern RabbitMQ distributions.
/// For custom types provided by 3rd party plugins, use the `Plugin(String)` variant.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all(serialize = "lowercase", deserialize = "PascalCase"))]
pub enum ExchangeType {
    /// Fanout exchange
    Fanout,
    /// Topic exchange
    Topic,
    /// Direct exchange
    Direct,
    /// Headers exchange
    Headers,
    /// Consistent hashing (consistent hash) exchange
    #[serde(rename = "x-consistent-hash")]
    ConsistentHashing,
    /// Modulus hash, ships with the 'rabbitmq-sharding' plugin
    #[serde(rename = "x-modulus-hash")]
    ModulusHash,
    /// Random exchange
    #[serde(rename = "x-random")]
    Random,
    /// Local random exchange
    #[serde(rename = "x-local-random")]
    LocalRandom,
    /// JMS topic exchange
    #[serde(rename = "x-jms-topic")]
    JmsTopic,
    /// Recent history exchange
    #[serde(rename = "x-recent-history")]
    RecentHistory,
    /// x-delayed-message exchange
    #[serde(rename = "x-delayed-message")]
    DelayedMessage,
    /// x-message-deduplication
    #[serde(rename = "x-message-deduplication")]
    MessageDeduplication,
    /// Other types
    #[serde(untagged)]
    Plugin(String),
}

const EXCHANGE_TYPE_FANOUT: &str = "fanout";
const EXCHANGE_TYPE_TOPIC: &str = "topic";
const EXCHANGE_TYPE_DIRECT: &str = "direct";
const EXCHANGE_TYPE_HEADERS: &str = "headers";
const EXCHANGE_TYPE_CONSISTENT_HASHING: &str = "x-consistent-hash";
const EXCHANGE_TYPE_MODULUS_HASH: &str = "x-modulus-hash";
const EXCHANGE_TYPE_RANDOM: &str = "x-random";
const EXCHANGE_TYPE_JMS_TOPIC: &str = "x-jms-topic";
const EXCHANGE_TYPE_LOCAL_RANDOM: &str = "x-local-random";
const EXCHANGE_TYPE_RECENT_HISTORY: &str = "x-recent-history";
const EXCHANGE_TYPE_DELAYED_MESSAGE: &str = "x-delayed-message";
const EXCHANGE_TYPE_MESSAGE_DEDUPLICATION: &str = "x-message-deduplication";

impl From<&str> for ExchangeType {
    fn from(value: &str) -> Self {
        match value {
            EXCHANGE_TYPE_FANOUT => ExchangeType::Fanout,
            EXCHANGE_TYPE_TOPIC => ExchangeType::Topic,
            EXCHANGE_TYPE_DIRECT => ExchangeType::Direct,
            EXCHANGE_TYPE_HEADERS => ExchangeType::Headers,
            EXCHANGE_TYPE_CONSISTENT_HASHING => ExchangeType::ConsistentHashing,
            EXCHANGE_TYPE_MODULUS_HASH => ExchangeType::ModulusHash,
            EXCHANGE_TYPE_RANDOM => ExchangeType::Random,
            EXCHANGE_TYPE_LOCAL_RANDOM => ExchangeType::LocalRandom,
            EXCHANGE_TYPE_JMS_TOPIC => ExchangeType::JmsTopic,
            EXCHANGE_TYPE_RECENT_HISTORY => ExchangeType::RecentHistory,
            EXCHANGE_TYPE_DELAYED_MESSAGE => ExchangeType::DelayedMessage,
            EXCHANGE_TYPE_MESSAGE_DEDUPLICATION => ExchangeType::MessageDeduplication,
            other => ExchangeType::Plugin(other.to_owned()),
        }
    }
}

impl From<String> for ExchangeType {
    fn from(value: String) -> Self {
        ExchangeType::from(value.as_str())
    }
}

impl From<ExchangeType> for String {
    fn from(value: ExchangeType) -> String {
        match value {
            ExchangeType::Fanout => EXCHANGE_TYPE_FANOUT.to_owned(),
            ExchangeType::Topic => EXCHANGE_TYPE_TOPIC.to_owned(),
            ExchangeType::Direct => EXCHANGE_TYPE_DIRECT.to_owned(),
            ExchangeType::Headers => EXCHANGE_TYPE_HEADERS.to_owned(),
            ExchangeType::ConsistentHashing => EXCHANGE_TYPE_CONSISTENT_HASHING.to_owned(),
            ExchangeType::ModulusHash => EXCHANGE_TYPE_MODULUS_HASH.to_owned(),
            ExchangeType::Random => EXCHANGE_TYPE_RANDOM.to_owned(),
            ExchangeType::LocalRandom => EXCHANGE_TYPE_LOCAL_RANDOM.to_owned(),
            ExchangeType::JmsTopic => EXCHANGE_TYPE_JMS_TOPIC.to_owned(),
            ExchangeType::RecentHistory => EXCHANGE_TYPE_RECENT_HISTORY.to_owned(),
            ExchangeType::DelayedMessage => EXCHANGE_TYPE_DELAYED_MESSAGE.to_owned(),
            ExchangeType::MessageDeduplication => EXCHANGE_TYPE_MESSAGE_DEDUPLICATION.to_owned(),
            ExchangeType::Plugin(exchange_type) => exchange_type,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all(serialize = "lowercase", deserialize = "PascalCase"))]
pub enum QueueType {
    Classic,
    Quorum,
    Stream,
}

impl From<&str> for QueueType {
    fn from(value: &str) -> Self {
        match value {
            "classic" => QueueType::Classic,
            "quorum" => QueueType::Quorum,
            "stream" => QueueType::Stream,
            _ => QueueType::Classic,
        }
    }
}

impl From<String> for QueueType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "classic" => QueueType::Classic,
            "quorum" => QueueType::Quorum,
            "stream" => QueueType::Stream,
            _ => QueueType::Classic,
        }
    }
}

impl From<QueueType> for String {
    fn from(value: QueueType) -> Self {
        match value {
            QueueType::Classic => "classic".to_owned(),
            QueueType::Quorum => "quorum".to_owned(),
            QueueType::Stream => "stream".to_owned(),
        }
    }
}

/// Binding destination can be either a queue or another exchange
/// (in the case of [exchange-to-exchange bindings](https://rabbitmq.com/docs/e2e/)).
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BindingDestinationType {
    Queue,
    Exchange,
}

impl fmt::Display for BindingDestinationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BindingDestinationType::Queue => write!(f, "queue")?,
            BindingDestinationType::Exchange => write!(f, "exchange")?,
        };

        Ok(())
    }
}

impl BindingDestinationType {
    pub fn path_appreviation(&self) -> String {
        match *self {
            BindingDestinationType::Queue => "q".to_owned(),
            BindingDestinationType::Exchange => "e".to_owned(),
        }
    }
}

impl From<&str> for BindingDestinationType {
    fn from(value: &str) -> Self {
        match value {
            "queue" => BindingDestinationType::Queue,
            "exchange" => BindingDestinationType::Exchange,
            _ => BindingDestinationType::Queue,
        }
    }
}

impl From<String> for BindingDestinationType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "queue" => BindingDestinationType::Queue,
            "exchange" => BindingDestinationType::Exchange,
            _ => BindingDestinationType::Queue,
        }
    }
}

impl From<BindingDestinationType> for String {
    fn from(value: BindingDestinationType) -> Self {
        match value {
            BindingDestinationType::Queue => "queue".to_owned(),
            BindingDestinationType::Exchange => "exchange".to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyTarget {
    Queues,
    ClassicQueues,
    QuorumQueues,
    Streams,
    Exchanges,
    All,
}

impl fmt::Display for PolicyTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<String>::into(self.clone()))?;

        Ok(())
    }
}

impl From<&str> for PolicyTarget {
    fn from(value: &str) -> Self {
        match value {
            "queues" => PolicyTarget::Queues,
            "classic_queues" => PolicyTarget::ClassicQueues,
            "quorum_queues" => PolicyTarget::QuorumQueues,
            "streams" => PolicyTarget::Streams,
            "exchanges" => PolicyTarget::Exchanges,
            "all" => PolicyTarget::All,
            _ => PolicyTarget::Queues,
        }
    }
}

impl From<String> for PolicyTarget {
    fn from(value: String) -> Self {
        match value.as_str() {
            "queues" => PolicyTarget::Queues,
            "classic_queues" => PolicyTarget::ClassicQueues,
            "quorum_queues" => PolicyTarget::QuorumQueues,
            "streams" => PolicyTarget::Streams,
            "exchanges" => PolicyTarget::Exchanges,
            "all" => PolicyTarget::All,
            _ => PolicyTarget::Queues,
        }
    }
}

impl From<PolicyTarget> for String {
    fn from(value: PolicyTarget) -> Self {
        match value {
            PolicyTarget::Queues => "queues".to_owned(),
            PolicyTarget::ClassicQueues => "classic_queues".to_owned(),
            PolicyTarget::QuorumQueues => "quorum_queues".to_owned(),
            PolicyTarget::Streams => "streams".to_owned(),
            PolicyTarget::Exchanges => "exchanges".to_owned(),
            PolicyTarget::All => "all".to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum VirtualHostLimitTarget {
    MaxConnections,
    MaxQueues,
}

impl AsRef<str> for VirtualHostLimitTarget {
    fn as_ref(&self) -> &str {
        match self {
            VirtualHostLimitTarget::MaxConnections => "max-connections",
            VirtualHostLimitTarget::MaxQueues => "max-queues",
        }
    }
}

impl From<&str> for VirtualHostLimitTarget {
    fn from(value: &str) -> Self {
        match value {
            "max-connections" => VirtualHostLimitTarget::MaxConnections,
            "max-queues" => VirtualHostLimitTarget::MaxQueues,
            _ => VirtualHostLimitTarget::MaxConnections,
        }
    }
}

impl From<String> for VirtualHostLimitTarget {
    fn from(value: String) -> Self {
        match value.as_str() {
            "max-connections" => VirtualHostLimitTarget::MaxConnections,
            "max-queues" => VirtualHostLimitTarget::MaxQueues,
            _ => VirtualHostLimitTarget::MaxConnections,
        }
    }
}

impl From<VirtualHostLimitTarget> for String {
    fn from(value: VirtualHostLimitTarget) -> Self {
        value.as_ref().to_string()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "kebab-case")]
pub enum UserLimitTarget {
    MaxConnections,
    MaxChannels,
}

impl AsRef<str> for UserLimitTarget {
    fn as_ref(&self) -> &str {
        match self {
            UserLimitTarget::MaxConnections => "max-connections",
            UserLimitTarget::MaxChannels => "max-channels",
        }
    }
}

impl From<&str> for UserLimitTarget {
    fn from(value: &str) -> Self {
        match value {
            "max-connections" => UserLimitTarget::MaxConnections,
            "max-channels" => UserLimitTarget::MaxChannels,
            _ => UserLimitTarget::MaxConnections,
        }
    }
}

impl From<String> for UserLimitTarget {
    fn from(value: String) -> Self {
        match value.as_str() {
            "max-connections" => UserLimitTarget::MaxConnections,
            "max-channels" => UserLimitTarget::MaxChannels,
            _ => UserLimitTarget::MaxConnections,
        }
    }
}

impl From<UserLimitTarget> for String {
    fn from(value: UserLimitTarget) -> Self {
        value.as_ref().to_string()
    }
}
