use std::result::Result;
use std::fmt;
use std::convert::TryFrom;
use super::VersionError;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {

    major: u16,
    minor: u16,
    patch: u16,
}

impl Version {

    pub fn new (major : u16, minor : u16, patch : u16) -> Version {
        Version {major,minor, patch}
    }

    pub fn from_string(input : &String) -> Result<Version,VersionError> {
        Version::from_str(input.as_str())
    }

    pub fn from_str(input : &str) -> Result<Version,VersionError> {

        let mut buf : [u16;3] = [0;3];
        let mut idx:  usize = 0;

        for token in input.split(".") {

            if idx == 3 || token.len() < 1 || token.len() > 5 {
                return Err(VersionError);
            }

            let value = match token.parse::<u16>() {
                Ok(value) => value,
                Err(_err) => return Err(VersionError),
            };

            buf[idx] = value;
            idx += 1;
        }

        if idx != 3 {
            return Err(VersionError)
        }
        Ok(Version{major: buf[0],minor : buf[1], patch: buf[2]})
    }

    pub fn major(&self) -> u16 {
        self.major
    }

    pub fn minor(&self) -> u16 {
        self.minor
    }

    pub fn patch(&self) -> u16 {
        self.patch
    }

    pub fn to_bytes(&self) -> Vec<u8> {

        let mut result = Vec::<u8>::new();

        result.extend_from_slice( &self.major.to_ne_bytes());
        result.extend_from_slice( &self.minor.to_ne_bytes());
        result.extend_from_slice( &self.patch.to_ne_bytes());
        result
    }

    pub fn to_string(&self) -> String {

        format!("{:05}.{:05}.{:05}",self.major,self.minor,self.patch)
    }
}

impl TryFrom<&[u8]> for Version {
    type Error = VersionError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {

        if value.len() != 6 {
           return Err(VersionError);
        }

        let major = u16::from_ne_bytes([value[0],value[1]]);
        let minor = u16::from_ne_bytes([value[2],value[3]]);
        let patch = u16::from_ne_bytes([value[4],value[5]]);
        Ok(Version {major,minor,patch})

    }
}

impl fmt::Display for Version {

    fn fmt(&self, fmt : &mut fmt::Formatter) -> fmt::Result {

        let result = fmt.write_str(fmt::format(format_args!("{:05}.{:05}.{:05}",self.major,self.minor,self.patch)).as_str());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    #[test]
    fn test_from_str() {

        /*
         * Positive Test
         */
        assert_eq!(Version { major: 1, minor: 0, patch: 0 }, Version::from_str("001.000.000").unwrap());
        assert_eq!(Version { major: 1, minor: 0, patch: 0 }, Version::from_str("0001.0000.0000").unwrap());
        assert_eq!(Version { major: 1, minor: 0, patch: 0 }, Version::from_str("00001.00000.00000").unwrap());

        /*
         * Error Test
         */
        //Too Short
        assert_eq!(Version::from_str("").err(), Some(VersionError));
        assert_eq!(Version::from_str("1").err(), Some(VersionError));
        assert_eq!(Version::from_str("12").err(), Some(VersionError));
        assert_eq!(Version::from_str("123").err(), Some(VersionError));
        assert_eq!(Version::from_str("1.123").err(), Some(VersionError));
        assert_eq!(Version::from_str("12.123").err(), Some(VersionError));
        assert_eq!(Version::from_str("123.123").err(), Some(VersionError));


        //Wrong Value
        assert_eq!(Version::from_str("A00.000.000").err(), Some(VersionError));
        assert_eq!(Version::from_str("0A0.000.000").err(), Some(VersionError));
        assert_eq!(Version::from_str("00A.000.000").err(), Some(VersionError));
        assert_eq!(Version::from_str("000.A00.000").err(), Some(VersionError));
        assert_eq!(Version::from_str("000.0A0.000").err(), Some(VersionError));
        assert_eq!(Version::from_str("000.00A.000").err(), Some(VersionError));
        assert_eq!(Version::from_str("000.000.A00").err(), Some(VersionError));
        assert_eq!(Version::from_str("000.000.0A0").err(), Some(VersionError));
        assert_eq!(Version::from_str("000.000.00A").err(), Some(VersionError));

        //Length too long
        assert_eq!(Version::from_str("00001.00001.00001.0000A").err(), Some(VersionError));
        assert_eq!(Version::from_str("000001.000001.000001").err(), Some(VersionError));
    }

    #[test]
    fn test_try_from_binary() {

        /*
         * Positive Test
         */
        assert_eq!(Ok(Version { major: 1, minor: 0, patch: 0 }), Version::try_from([1 as u8,0,0,0,0,0].as_ref()));

        /*
         * Error Test
         */
        assert_eq!(Err(VersionError),Version::try_from([1 as u8,0,0,0,0,0,1].as_ref()));
        assert_eq!(Err(VersionError),Version::try_from([1 as u8,0,0,0,0].as_ref()));
    }

    #[test]
    fn test_to_bytes() {

        /*
         * Positive Test
         */
        let version = Version::new(1,2,3);
        assert_eq!([1,0,2,0,3,0],version.to_bytes().as_slice());
    }
}
