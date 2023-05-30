use std::str::FromStr;
use crate::drv::core::sysfs::get_kernel_driver_version;
use crate::datatypes::Version;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct SdbpkCheck {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[allow(unused)]
impl SdbpkCheck {

    pub fn to_version(&self) -> Version {
        return Version::new(self.major as u16,self.minor as u16,self.patch as u16);
    }

    pub fn check_version(&self) -> Result<SdbpkCheck, String> {
        let version = get_kernel_driver_version();
        let version = match version {
            Ok(version) => {
                let mut error = false;
                let mut version = version;
                let clean_version = version.strip_suffix("\n");
                version = match clean_version {
                    None => {"".to_string()} // Length check will fail
                    Some(version) => {version.to_string()}
                };
                let split: Vec<&str> = version.split(".").collect();
                if split.len() != 3 {
                    error = true;
                }

                let mut version: Vec<u32> = Vec::new();
                for element in split {
                    let number = u32::from_str(element);
                    match number {
                        Ok(val) => { version.push(val) }
                        Err(_) => {
                            error = true;
                        }
                    }
                }

                if error {
                    Err("Could not parse SDBPK driver version")
                } else {
                    Ok(SdbpkCheck {
                        major: version[0],
                        minor: version[1],
                        patch: version[2]
                    })
                }
            }
            Err(_) => {
                Err("Could not open sdbpk driver file")
            }
        };

        let result = match version {
            Ok(version) => {
                let err_msg: String;
                loop {
                    if version.major > self.major {
                        err_msg = "SDBPK driver: major version is incompatible".to_string();
                        break;
                    }
                    if version.minor < self.minor {
                        err_msg = "SDBPK driver: minor version is incompatible".to_string();
                        break;
                    }
                    err_msg="".to_string();
                    break;
                }
                if !err_msg.is_empty() {
                    Err(err_msg)
                } else {
                    Ok(version)
                }
            }
            Err(err) => { Err(err.to_string()) }
        };

        match result {
            Ok(result) => {
                Ok(result)
            }
            Err(_) => {
                return match result.err() {
                    None => { Err("Unknown error".to_string()) }
                    Some(err) => {
                        Err(err)
                    }
                }
            }
        }
    }
}