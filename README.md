# Driver library for SDBP based modules
[![Rust](https://github.com/nexus-unity/rustlib-nexus-unity-sdbp/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/nexus-unity/rustlib-nexus-unity-sdbp/actions/workflows/rust.yml)

This library contains all the functions to build drivers and applications
for Serial Device Bus Protocol (SDBP) based devices.  
For details see [nexus-unity.com](https://nexus-unity.com/).

The crate provides support for modules/devices by features.
e.g: 
```
nexus_unity_sdbp = { package = "nexus_unity_sdbp", git = "ssh://github.com/nexus-unity/rustlib-nexus-unity-sdbp.git", version = "0.9.*", features = ["io","power-mgmt"] }
nexus_unity_sdbp = { package = "nexus_unity_sdbp", git = "ssh://github.com/nexus-unity/rustlib-nexus-unity-sdbp.git", version = "0.9.*", features = ["power"] }
nexus_unity_sdbp = { package = "nexus_unity_sdbp", git = "ssh://github.com/nexus-unity/rustlib-nexus-unity-sdbp.git", version = "0.9.*", features = ["bmc"] }
```

## License
This library is licensed under the ["GNU LESSER GENERAL PUBLIC LICENSE Version 3"](LICENSE).
