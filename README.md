# googleads-rs

![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/mhuang74/googleads-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/mhuang74/googleads-rs/actions)
[![crates-io](https://img.shields.io/crates/v/googleads-rs.svg)](https://crates.io/crates/googleads-rs)
[![api-docs](https://docs.rs/googleads-rs/badge.svg)](https://docs.rs/googleads-rs)


Current Version 0.8.0 uses [Google Ads API v17](https://developers.google.com/google-ads/api/docs/release-notes)

A gRPC client library for Google Ads API, generated automatically from the API definition files.

I use it for my [mcc-gaql](https://github.com/mhuang74/mcc-gaql-rs) command line tool to run Google Ads Query Language queries across large number of MCC-linked accounts.

There may be more elegant ways to pull query results from GoogleAdsRow in a reflection-like manner. I couldn't figure it out. So I hand-crafted a `GoogleAdsRow.get(path: &str)` accessor method for fields which I need. 

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

## API Upgrade

Run `update.sh` to update the library for a new Google Ads API version:
* Download latest proto files for new Google Ads API v17version
* Replace references to old api version in build.rs, lib.rs, and README.md

```
./utils/update.sh v17
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

## Credits
* Originally forked from [gkkachi's gapi-grpc-rs](https://github.com/gkkachi/gapi-grpc-rs), which used Python to generate `protos.rs`
* Dropped Python and migrated to custom build.rs per [aquarhead](https://blog.aqd.is/2021/07/rust-protobuf)
