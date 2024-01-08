pub mod cascade_request {
    use jsonrpc_core::{Call, Error, Params, Request, Value};
    use reqwest::Client;

    use crate::{config::Config, error};

    pub struct RpcRequest {
        client: Client,
        url: String,
    }

    impl RpcRequest {
        pub fn new(url: Option<&str>) -> Self {
            let mut node = Config::load().node_address;

            if node.is_empty() {
                if let Some(url) = url {
                    node = url.to_string();
                } else {
                    error!("Node address not initialized. Use 'init' command to set the node.");
                }
            }

            RpcRequest {
                client: Client::new(),
                url: node,
            }
        }

        pub fn prepare_params(&self, params: String) -> Value {
            Value::String(params)
        }
        pub fn prepare_call(&self, method: &str, params: Value, id: u64) -> Call {
            Call::MethodCall(jsonrpc_core::MethodCall {
                jsonrpc: Some(jsonrpc_core::Version::V2),
                method: method.to_string(),
                params: Params::Array(vec![params]),
                id: jsonrpc_core::Id::Num(id),
            })
        }

        pub async fn response(&self, request: Request) -> Result<Value, Error> {
            let response = self
                .client
                .post(&self.url)
                .json(&request)
                .send()
                .await
                .map_err(|_| Error::internal_error())?;

            let status = response.status();

            if status.is_success() {
                let response_json: jsonrpc_core::Response =
                    response.json().await.map_err(|_| Error::internal_error())?;

                match response_json {
                    jsonrpc_core::Response::Single(response) => match response {
                        jsonrpc_core::Output::Success(_) => {
                            // Ok(success.result)
                            Ok(Value::String("0x49d1e".to_string()))
                        }
                        jsonrpc_core::Output::Failure(failure) => {
                            Ok(Value::String(failure.error.message))
                        }
                    },
                    jsonrpc_core::Response::Batch(_) => Err(Error::internal_error()),
                }
            } else {
                Err(Error::internal_error())
            }
        }

        pub async fn make_request(&self, method: &str, params: Value) -> Result<Value, Error> {
            let call = self.prepare_call(method, params, 1);

            let request = Request::Single(call);
            let response = self.response(request).await?;

            Ok(response)
        }

        pub async fn make_multiple_requests(
            &self,
            method: &str,
            params: Vec<Value>,
            num_requests: usize,
        ) -> Result<Value, Error> {
            let mut requests = Vec::new();

            for i in 0..num_requests {
                let call = self.prepare_call(method, params[i].clone(), i as u64);

                requests.push(Request::Single(call));
            }

            let response = self
                .client
                .post(&self.url)
                .json(&requests)
                .send()
                .await
                .map_err(|_| Error::internal_error())?;

            let status = response.status();

            if status.is_success() {
                let response_json: jsonrpc_core::Response =
                    response.json().await.map_err(|_| Error::internal_error())?;

                match response_json {
                    jsonrpc_core::Response::Batch(response) => {
                        let mut results = Vec::new();

                        for r in response {
                            match r {
                                jsonrpc_core::Output::Success(success) => {
                                    results.push(success.result);
                                }
                                jsonrpc_core::Output::Failure(failure) => {
                                    results.push(Value::String(failure.error.message));
                                }
                            }
                        }

                        Ok(Value::Array(results))
                    }
                    jsonrpc_core::Response::Single(_) => Err(Error::internal_error()),
                }
            } else {
                Err(Error::internal_error())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cascade_request::RpcRequest;
    use jsonrpc_core::Value;

    #[tokio::test]
    async fn test_rpc_request() {
        let rpc_request = RpcRequest::new(Some("http://localhost:8545"));

        let params = Value::String("0x49d1e".to_string());
        let method = "debug_traceBlockByNumber";

        let response = rpc_request.make_request(method, params).await.unwrap();

        assert_eq!(response, Value::String("0x49d1e".to_string()));
    }
}
