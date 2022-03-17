# googleads-rs

A gRPC client library for Google Ads API, generated automatically from the API definition files.

* Currently only supports Google Ads API v10
* Provides `GoogleAdsRow.get(path: &str)` accessor method to more easily retrieve fields selected in GAQL

## Install dependencies

```
pip3 install jinja2
```

## Download latest proto files

```
./utils/update.sh
```

## Build

```
./prebuild.py
cargo build
```

## Document

```
cargo doc --open
```
