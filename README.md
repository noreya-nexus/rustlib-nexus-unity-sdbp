# Driver library for SDBP based modules
[![Rust](https://github.com/noreya-nexus/rustlib-noreya-sdbp/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/noreya-nexus/rustlib-noreya-sdbp/actions/workflows/rust.yml)

This library contains all the functions to build drivers and applications
for Serial Device Bus Protocol (SDBP) based devices.  
For details see [noreya-nexus.tech](https://noreya-nexus.tech/).

The crate provides support for modules/devices by features.
e.g: 
```
noreya_sdbp = { package = "noreya_sdbp", git = "ssh://github.com/noreya-nexus/rustlib-noreya-sdbp.git", version = "1.0.*", features = ["io","power-mgmt"] }
noreya_sdbp = { package = "noreya_sdbp", git = "ssh://github.com/noreya-nexus/rustlib-noreya-sdbp.git", version = "1.0.*", features = ["power"] }
noreya_sdbp = { package = "noreya_sdbp", git = "ssh://github.com/noreya-nexus/rustlib-noreya-sdbp.git", version = "1.0.*", features = ["bmc"] }
```

## License
This library is licensed under the ["GNU LESSER GENERAL PUBLIC LICENSE Version 3"](LICENSE).
