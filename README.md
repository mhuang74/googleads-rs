# googleads-rs

![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/mhuang74/googleads-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/mhuang74/googleads-rs/actions)
[![codecov](https://codecov.io/github/mhuang74/googleads-rs/graph/badge.svg?token=0HUG6VCKU2)](https://codecov.io/github/mhuang74/googleads-rs)
[![crates-io](https://img.shields.io/crates/v/googleads-rs.svg)](https://crates.io/crates/googleads-rs)
[![api-docs](https://docs.rs/googleads-rs/badge.svg)](https://docs.rs/googleads-rs)


Current Version 23.2.1 uses [Google Ads API v23.2](https://developers.google.com/google-ads/api/docs/release-notes)

---

> **Versioning Convention**
>
> The crate version directly corresponds to the Google Ads API version it supports.
> - **major.minor** = Google Ads API version (e.g., `23.2.1` → API v23.2)
> - **patch** = bug fixes and library updates
>
> This eliminates the mental friction of mapping between the `googleads-rs` version and the Google Ads API version.

---

A gRPC client library for Google Ads API, generated automatically from the API definition files.

I use it for my [mcc-gaql](https://github.com/mhuang74/mcc-gaql-rs) command line tool to run Google Ads Query Language queries across large number of MCC-linked accounts.

The `GoogleAdsRow.get(path: &str)` accessor method uses `prost-reflect` for dynamic field access, allowing you to retrieve any field from the protobuf message by its GAQL path without hardcoding match arms for each field. 

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
* Download latest proto files for new Google Ads API version
* Replace references to old api version in build.rs, lib.rs, and README.md

```
./utils/update.sh v17
```


## Build process

* build.rs dynamically scans for available proto files, filters them, and feeds them to tonic to generate `protos.rs` (following strategy by [aquarhead](https://blog.aqd.is/2021/07/rust-protobuf))
* build.rs also generates a file descriptor set using `prost-reflect`'s `compile_fds()` for runtime reflection
* lib.rs includes `protos.rs`
* lib.rs implements `get()` and `get_many()` using `prost-reflect` for dynamic field access:

```rust
pub fn get(&self, field_name: &str) -> String {
    // Encode the GoogleAdsRow to bytes, then decode as DynamicMessage
    let encoded = self.encode_to_vec();

    let descriptor = DESCRIPTOR_POOL
        .get_message_by_name(GOOGLE_ADS_ROW_FQN)
        .expect("GoogleAdsRow descriptor not found");

    let dynamic_msg = DynamicMessage::decode(descriptor, Cursor::new(&encoded))
        .expect("Failed to decode GoogleAdsRow as DynamicMessage");

    self.get_field_from_dynamic(&dynamic_msg, field_name)
}
```

The `get()` method uses `prost-reflect` to:
1. Encode the `GoogleAdsRow` to protobuf bytes
2. Decode it as a `DynamicMessage` using the descriptor pool loaded at startup
3. Walk the dotted field path (e.g., `"campaign.id"`) recursively through the message structure
4. Format the value appropriately (scalars, enums, nested messages, repeated fields)

This approach supports **any field** in the Google Ads API without manual match arms, including:
- Scalar fields (strings, numbers, booleans)
- Enum fields (resolved to their name)
- Nested messages (automatically traversed)
- Repeated fields (formatted as comma-separated lists)
- Special cases like `campaign.asset_automation_settings` and `responsive_search_ad` headlines/descriptions

* cargo test may fail due to compilation errors with example code in comment blocks of proto files. enclose them in ignore blocks.

```
// A message representing the message types used by a long-running operation.
// ```ignore
// Example:
//
//     rpc Export(ExportRequest) returns (google.longrunning.Operation) {
//       option (google.longrunning.operation_info) = {
//         response_type: "ExportResponse"
//         metadata_type: "ExportMetadata"
//       };
//     }
// ```
```

## Documentation

* to generate docs quicker, exclude dependent crates

```
$$ cargo doc --no-deps --open
```

## Credits
* Originally forked from [gkkachi's gapi-grpc-rs](https://github.com/gkkachi/gapi-grpc-rs), which used Python to generate `protos.rs`
* Dropped Python and migrated to custom build.rs per [aquarhead](https://blog.aqd.is/2021/07/rust-protobuf)
