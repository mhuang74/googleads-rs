# googleads-rs

A gRPC client library for Google Ads API, generated automatically from the API definition files.

* Currently only supports Google Ads API v11
* Provides `GoogleAdsRow.get(path: &str)` accessor method to more easily retrieve fields selected in GAQL

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