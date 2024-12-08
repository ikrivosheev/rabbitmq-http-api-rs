// Copyright (C) 2023-2024 RabbitMQ Core Team (teamrabbitmq@gmail.com)
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
use rabbitmq_http_client::blocking_api::Client;

mod common;
use crate::common::{endpoint, PASSWORD, USERNAME};

#[test]
fn test_list_channels() {
    let endpoint = endpoint();
    let rc = Client::new(&endpoint, USERNAME, PASSWORD);

    let result1 = rc.list_channels();
    assert!(result1.is_ok(), "list_channels returned {:?}", result1);
}

#[test]
fn test_list_virtual_host_channels() {
    let endpoint = endpoint();
    let rc = Client::new(&endpoint, USERNAME, PASSWORD);

    let vh_name = "/";
    let result1 = rc.list_channels_in(vh_name);
    assert!(result1.is_ok(), "list_channels_in returned {:?}", result1);
}
