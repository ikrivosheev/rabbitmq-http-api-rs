use rabbitmq_http_client::{blocking::Client, commons::QueueType, requests::QueueParams};
use serde_json::{json, Map, Value};

mod common;
use crate::common::{endpoint, PASSWORD, USERNAME};

#[test]
fn test_declare_and_redeclare_a_classic_queue() {
    let endpoint = endpoint();
    let rc = Client::new(&endpoint, USERNAME, PASSWORD);
    let vhost = "/";
    let name = "rust.tests.cq.69373293479827";

    let _ = rc.delete_queue(vhost, name);

    let result1 = rc.get_queue_info(vhost, name);
    assert!(result1.is_err());

    let mut map = Map::<String, Value>::new();
    map.insert("x-max-length".to_owned(), json!(10_000));
    // note: x-queue-type will be injected by QueueParams::new_durable_classic_queue
    let optional_args = Some(map);
    let params = QueueParams::new_durable_classic_queue(name, optional_args.clone());
    let result2 = rc.declare_queue(vhost, &params);
    assert!(result2.is_ok(), "declare_queue returned {:?}", result2);

    let params2 = QueueParams::new(name, QueueType::Classic, true, false, optional_args.clone());
    let result3 = rc.declare_queue(vhost, &params2);
    assert!(result3.is_ok(), "declare_queue returned {:?}", result3);

    let _ = rc.delete_queue(vhost, name);
}

#[test]
fn test_declare_a_quorum_queue() {
    let endpoint = endpoint();
    let rc = Client::new(&endpoint, USERNAME, PASSWORD);
    let vhost = "/";
    let name = "rust.tests.qq.182374982374";

    let _ = rc.delete_queue(vhost, name);

    let result1 = rc.get_queue_info(vhost, name);
    assert!(result1.is_err());

    let mut map = Map::<String, Value>::new();
    map.insert("x-max-length".to_owned(), json!(10_000));
    let optional_args = Some(map);
    let params = QueueParams::new_quorum_queue(name, optional_args);
    let result2 = rc.declare_queue(vhost, &params);
    assert!(result2.is_ok(), "declare_queue returned {:?}", result2);

    let _ = rc.delete_queue(vhost, name);
}

#[test]
fn test_declare_a_stream() {
    let endpoint = endpoint();
    let rc = Client::new(&endpoint, USERNAME, PASSWORD);
    let vhost = "/";
    let name = "rust.tests.qq.927348926347988623";

    let _ = rc.delete_queue(vhost, name);

    let result1 = rc.get_queue_info(vhost, name);
    assert!(result1.is_err());

    let mut map = Map::<String, Value>::new();
    map.insert("x-max-length-bytes".to_owned(), json!(10_000_000));
    let optional_args = Some(map);
    let params = QueueParams::new_stream(name, optional_args);
    let result2 = rc.declare_queue(vhost, &params);
    assert!(result2.is_ok(), "declare_queue returned {:?}", result2);

    let _ = rc.delete_queue(vhost, name);
}

#[test]
fn test_delete_queue() {
    let endpoint = endpoint();
    let rc = Client::new(&endpoint, USERNAME, PASSWORD);
    let vhost = "/";
    let name = "rust.tests.cq.982734982364982364896";

    let _ = rc.delete_queue(vhost, name);

    let result1 = rc.get_queue_info(vhost, name);
    assert!(result1.is_err());

    let params = QueueParams::new_durable_classic_queue(name, None);
    let result2 = rc.declare_queue(vhost, &params);
    assert!(result2.is_ok(), "declare_queue returned {:?}", result2);

    rc.delete_queue(vhost, name).unwrap();
    let result3 = rc.get_queue_info(vhost, name);
    assert!(result3.is_err());
}

#[test]
fn test_list_all_queues() {
    let endpoint = endpoint();
    let rc = Client::new(&endpoint, USERNAME, PASSWORD);

    let vh_name = "/";

    let params = QueueParams::new_durable_classic_queue("rust.tests.cq.23487866", None);
    let result1 = rc.declare_queue(vh_name, &params);
    assert!(result1.is_ok(), "declare_queue returned {:?}", result1);

    common::await_queue_metric_emission();

    let result2 = rc.list_queues();
    assert!(result2.is_ok(), "list_queues returned {:?}", result2);

    rc.delete_queue(vh_name, params.name).unwrap();
}

#[test]
fn test_list_queues_in_a_virtual_host() {
    let endpoint = endpoint();
    let rc = Client::new(&endpoint, USERNAME, PASSWORD);

    let vh_name = "/";

    let params = QueueParams::new_durable_classic_queue("rust.tests.cq.64692734867", None);
    let result1 = rc.declare_queue(vh_name, &params);
    assert!(result1.is_ok(), "declare_queue returned {:?}", result1);

    common::await_queue_metric_emission();

    let result2 = rc.list_queues_in(vh_name);
    assert!(result2.is_ok(), "list_queues_in returned {:?}", result2);

    rc.delete_queue(vh_name, params.name).unwrap();
}
