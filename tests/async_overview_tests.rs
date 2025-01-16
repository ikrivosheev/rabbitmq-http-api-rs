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
use rabbitmq_http_client::api::Client;

mod test_helpers;
use crate::test_helpers::{endpoint, PASSWORD, USERNAME};

#[tokio::test]
async fn test_async_overview() {
    let endpoint = endpoint();
    let rc = Client::new(&endpoint, USERNAME, PASSWORD);

    let result1 = rc.overview().await;
    assert!(result1.is_ok(), "overview returned {:?}", result1);

    let ov = result1.unwrap();
    assert!(ov.object_totals.exchanges > 0);
}
