use rabbitmq_http_client::{
    blocking::Client,
    commons::UserLimitTarget,
    password_hashing,
    requests::{EnforcedLimitParams, UserParams},
};

mod common;
use crate::common::{endpoint, PASSWORD, USERNAME};

#[test]
fn test_list_all_user_limits() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, PASSWORD);

    let salt = password_hashing::salt();
    let password_hash =
        password_hashing::base64_encoded_salted_password_hash_sha256(&salt, "rust3_t0p_sEkr37");

    let params = UserParams {
        name: "test_list_all_user_limits",
        password_hash: &password_hash,
        tags: "management",
    };
    let result1 = rc.create_user(&params);
    assert!(result1.is_ok());

    let limit = EnforcedLimitParams::new(UserLimitTarget::MaxChannels, 500);
    let result2 = rc.set_user_limit(params.name, limit);
    assert!(result2.is_ok());

    let result3 = rc.list_all_user_limits();
    assert!(result3.is_ok());
    let vec = result3.unwrap();
    assert!(vec.iter().any(|it| it.username == params.name));

    let key1 = UserLimitTarget::MaxConnections.to_string();
    assert!(!vec
        .iter()
        .any(|it| it.username == params.name && it.limits.get(&key1).is_some()));
    let key2 = UserLimitTarget::MaxChannels.to_string();
    assert!(vec
        .iter()
        .any(|it| it.username == params.name && it.limits.get(&key2).is_some()));

    rc.delete_user(params.name).unwrap();
}

#[test]
fn test_list_user_limits() {
    let endpoint = endpoint();
    let rc = Client::new_with_basic_auth_credentials(&endpoint, USERNAME, PASSWORD);

    let salt = password_hashing::salt();
    let password_hash =
        password_hashing::base64_encoded_salted_password_hash_sha256(&salt, "rust3_t0p_sEkr37");

    let params = UserParams {
        name: "test_list_user_limits",
        password_hash: &password_hash,
        tags: "management",
    };
    let result1 = rc.create_user(&params);
    assert!(result1.is_ok());

    let limit = EnforcedLimitParams::new(UserLimitTarget::MaxChannels, 500);
    let result2 = rc.set_user_limit(params.name, limit);
    assert!(result2.is_ok());

    let result3 = rc.list_user_limits(params.name);
    dbg!(&result3);
    assert!(result3.is_ok());
    let vec = result3.unwrap();

    let key1 = UserLimitTarget::MaxChannels.to_string();
    assert!(vec
        .iter()
        .any(|it| it.username == params.name && it.limits.get(&key1).is_some()));
    let key2 = UserLimitTarget::MaxConnections.to_string();
    assert!(!vec
        .iter()
        .any(|it| it.username == params.name && it.limits.get(&key2).is_some()));

    rc.delete_user(params.name).unwrap();
}
