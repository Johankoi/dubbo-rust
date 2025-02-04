/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

pub mod client;
pub mod server;
pub mod protocol;


pub mod request;
pub mod response;
pub mod error;
pub mod util;


pub use request::*;
pub use response::*;
pub use error::*;
pub use util::*;



#[cfg(test)]
mod tests {
    use crate::client::client::RpcClient;
    use crate::server::server::RpcServer;
    use std::net::SocketAddr;
    #[test]
    fn it_works() {
        // RpcClient
        let client = RpcClient::new(String::from("http://127.0.0.1:8972"));

        // RpcServer
        let addr = SocketAddr::from(([127, 0, 0, 1], 8972));
        let server = RpcServer::new(addr);
        println!("it_works");
    }
}
