use super::*;
use std::fmt;
use std::convert::TryFrom;

/// AdvancedVersion - Extended Version contains Stability, Major, Minor and Patch
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedVersion {
    stability: char,
    version: Version,
}

impl AdvancedVersion {

    /// Creates a new Advanced Version
    ///
    /// stability - [A - Alpha, B - Beta, S - Stable Release]
    /// major - Major Version [0-65535]
    /// minor - Minor Version [0-65535]
    /// patch - Patch Version [0-65535]
    pub fn new(stability: char, major : u16 , minor : u16 , patch : u16) {
        AdvancedVersion{stability,version: Version::new(major,minor,patch)};
    }

    /// Converts a String into a AdvancedVersion Object
    ///
    /// input - Input string [&str]
    pub fn from_str(input: &str) -> Result<AdvancedVersion,VersionError>{

        if input.len() < (1+3+3+3+3) {
            return Err(VersionError)
        }

        let stability = input.chars().nth(0).unwrap();
        match stability {
            'A' |'B' | 'S' => (),
            _ => return Err(VersionError)
        }

        let version = match Version::from_str(&input[2..]){
            Err(_e) => return Err(VersionError),
            Ok(value) => value,
        };

        Ok(AdvancedVersion {version, stability})
    }

    /// Converts a AdvancedVersion object into a byte array
    pub fn to_bytes(&self) -> Vec<u8> {

        let mut result  = Vec::<u8>::new();
        result.push(self.stability as u8);
        result.extend(self.version.to_bytes());
        result
    }
}

impl TryFrom<&[u8]> for AdvancedVersion {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {

        if value.len() != 7 {
            return Err(());
        }

        let stability = value[0] as char;
        match stability  {
            'A' |'B' | 'S' => (),
            _ => return Err(()),
        }

        let version =  match Version::try_from(&value[1..]) {
            Ok(value) => value,
            Err(_err) => return Err(()),
        };
        Ok(AdvancedVersion {stability,version})
    }
}

impl fmt::Display for AdvancedVersion {

    fn fmt(&self, fmt : &mut fmt::Formatter) -> fmt::Result {

        let result = fmt.write_str(fmt::format(format_args!("{}.{}",self.stability  ,self.version)).as_str());
        result
    }
}

#[cfg(test)]
mod advanced_version_tests {

    use super::*;

    #[test]
    fn test_ok_from_str(){

        let expected =  AdvancedVersion {stability: 'A',version: Version::new(1,2,3)};
        let version =  AdvancedVersion::from_str("A.001.002.003").unwrap();

        assert_eq!(version,expected);

        let expected =  AdvancedVersion {stability: 'B',version: Version::new(1,2,3)};
        let version =  AdvancedVersion::from_str("B.001.002.003");
        assert_eq!(version,Ok(expected));

        let expected =  AdvancedVersion {stability: 'S',version: Version::new(1,2,3)};
        let version =  AdvancedVersion::from_str("S.001.002.003");
        assert_eq!(version,Ok(expected));


        let expected =  AdvancedVersion {stability: 'S',version: Version::new(1,2,3)};
        let version =  AdvancedVersion::from_str("S.00001.00002.00003");
        assert_eq!(version,Ok(expected));
    }

    #[test]
    fn test_err_from_str(){

        // Empty Input Test
        let version = AdvancedVersion::from_str("");
        assert_eq!(version, Err(VersionError));

        // Invalid Stability
        let version = AdvancedVersion::from_str("C.000.000.000");
        assert_eq!(version,Err(VersionError));
        let version = AdvancedVersion::from_str("D.000.000.000");
        assert_eq!(version,Err(VersionError));
        let version = AdvancedVersion::from_str("E.000.000.000");
        assert_eq!(version,Err(VersionError));

        // Major too long
        let version = AdvancedVersion::from_str("A.123456.12345.12345");
        assert_eq!(version,Err(VersionError));

        // Minor too long
        let version = AdvancedVersion::from_str("A.12345.123456.12345");
        assert_eq!(version,Err(VersionError));

        // Patch too long
        let version = AdvancedVersion::from_str("A.12345.12345.123456");
        assert_eq!(version,Err(VersionError));

    }
}
