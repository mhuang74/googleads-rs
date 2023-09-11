# googleads-rs

Current Version 0.4.1

A gRPC client library for Google Ads API, generated automatically from the API definition files.

Provides `GoogleAdsRow.get(path: &str)` accessor method to easily retrieve fields selected in GAQL

## Example

```
    let client: GoogleAdsServiceClient<InterceptedService<Channel, GoogleAdsAPIAccess>> =
        GoogleAdsServiceClient::with_interceptor(api_context.channel.clone(), api_context);

    let result: Result<Response<Streaming<SearchGoogleAdsStreamResponse>>, Status> = client
        .search_stream(SearchGoogleAdsStreamRequest {
            customer_id: customer_id.clone(),
            query,
            summary_row_setting: 0,
        })
        .await;

    match result {
        Ok(response) => {
            let mut stream = response.into_inner();

            let mut columns: Vec<Vec<String>> = Vec::new();
            let mut headers: Option<Vec<String>> = None;

            while let Some(item) = stream.next().await {
                match item {
                    Ok(stream_response) => {
                        let field_mask = stream_response.field_mask.unwrap();
                        if headers.is_none() {
                            headers = Some(field_mask.paths.clone());
                        }
                        for r in stream_response.results {
                            let row: GoogleAdsRow = r;

                            // go through all columns specified in query, pull out string value, and insert into columns
                            for i in 0..headers.as_ref().unwrap().len() {
                                let path = &headers.as_ref().unwrap()[i];
                                let string_val: String = row.get(path).trim_matches('"').to_string();
                                match columns.get_mut(i) {
                                    Some(v) => {
                                        v.push(string_val);
                                    }
                                    None => {
                                        let v: Vec<String> = vec![string_val];
                                        columns.insert(i, v);
                                    }
                                }
                            }
                        }
                    }
                    Err(status) => {
                        bail!(
                            "GoogleAdsClient streaming error. Account: {customer_id}, Message: {}, Details: {}",
                            status.message(),
                            String::from_utf8_lossy(status.details()).into_owned()
                        );
                    }
                }
            }
        }
    }
```

## Download latest proto files

```
./utils/update.sh
```

## Build process

* build.rs dynamically scans for available proto files, filters them, and feeds them to tonic to generate `protos.rs` (following strategy by [aquarhead](https://blog.aqd.is/2021/07/rust-protobuf))
* lib.rs includes `protos.rs`
* lib.rs also includes hand-crafted `get()` function

```
    pub fn get(&self, field_name: &str) -> String {
        match field_name {
            "ad_group_criterion.criterion_id" => format!("{}", self.ad_group_criterion.as_ref().unwrap().criterion_id),
            "ad_group_criterion.status" => format!("{}", self.ad_group_criterion.as_ref().unwrap().status()),
            <snip>
        }
    }
```