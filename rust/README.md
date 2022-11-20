# 4DK - Rust Implementation

## Requirements
rustc 1.57.0 <br/>
Cargo <br/>
Docker (for sample) </br>

No additional library are used in this project. <br/>

## 4dk Crates
Clone the project. <br/><br />
Enter to `/rust/project` to access to the differents crates. 

### Crate Descriptions
- dddk_core: (path: `/rust/project/core-rust`) > it contains the main logic)
- dddk_macro: (path: `/rust/project/macro-fourdk`) > it contains all `macro derive` default impls of `trait` defined in `dddk_core`
- ddd_security: (path: `/rust/project/security-rust`) > it completes `dddk_core` logic with security features

### Build crates
execute following command `make release`.<br />

### Launch crate tests
execute following command `make test`.<br />

### Samples
Check `/rust/samples` folder. <br/>
`basic-4dk-sample` is a standalone sample. (there is a fictive events storming to understand the crate integration<br />
`basic-4dk-sample-integrated-with-rocket-and-diesel` and `securized-4dk-sample-integrated-with-rocket-diesel` are samples which are integrated in a web context. (with `rocket` and `diesel`)<br />
`basic-4dk-sample-integrated-with-actix-and-diesel` and `basic-4dk-sample-integrated-with-actix-and-kafka-and-diesel` are samples which are integrated in a web context. (with `actix` and `diesel`)<br />
`complex-4dk-sample-integrated-with-rocket-and-sqlx` is a sample which are integrated in a web context. (with `rocket` and `sqlx`)<br />
