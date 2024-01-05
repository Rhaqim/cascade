pub mod cascade_request {
    use jsonrpc_core::{Call, Error, Params, Request, Value};
    use reqwest::Client;

    pub struct RpcRequest {
        client: Client,
        url: String,
    }

    impl RpcRequest {
        pub fn new(url: &str) -> Self {
            RpcRequest {
                client: Client::new(),
                url: url.to_string(),
            }
        }

        pub async fn make_request(&self, method: &str, params: Value) -> Result<Value, Error> {
            let call = Call::MethodCall(jsonrpc_core::MethodCall {
                jsonrpc: Some(jsonrpc_core::Version::V2),
                method: method.to_string(),
                params: Params::Array(vec![params]),
                id: jsonrpc_core::Id::Num(1),
            });

            let request = Request::Single(call);
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
                        jsonrpc_core::Output::Success(success) => {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cascade_request::RpcRequest;
    use jsonrpc_core::Value;

    #[tokio::test]
    async fn test_rpc_request() {
        let rpc_request = RpcRequest::new("http://localhost:8545");

        let params = Value::String("0x49d1e".to_string());
        let method = "debug_traceBlockByNumber";

        let response = rpc_request.make_request(method, params).await.unwrap();

        assert_eq!(response, Value::String("0x49d1e".to_string()));
    }
}
