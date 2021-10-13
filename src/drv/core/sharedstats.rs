use std::fmt;
use std::time::{SystemTime};
use crate::datatypes::*;
use crate::util::SharedObject;


#[derive(Debug,Clone)]
pub struct Stats{
    name : String,
    version : Version,
    sdbpk_version : Version,
    devices : Vec<Descriptor>,
    timestamp : u128,

}

impl Stats {
    pub fn new(name : String,version : Version, sdbpk_version : Version) -> Stats {
        Stats{name, devices : Vec::new(), timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() ,version, sdbpk_version}
    }

    pub fn get_devices(&mut self) -> &mut Vec<Descriptor> {
        &mut self.devices
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_version(&self) -> &Version {
        &self.version
    }

    pub fn get_sdbpk_version(&self) -> &Version {
        &self.sdbpk_version
    }

    pub fn get_timestamp(&self) -> u128 {
        self.timestamp
    }

    pub fn update(&mut self) {
        self.timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    }


}

impl fmt::Display for Stats {

    fn fmt(&self, fmt : &mut fmt::Formatter) -> fmt::Result {

        fmt.write_str(fmt::format(format_args!("Name: {}\n",self.name)).as_str()).unwrap();
        fmt.write_str(fmt::format(format_args!("timestamp: {}\n",self.timestamp)).as_str()).unwrap();

        for device in &self.devices {
            fmt.write_str(fmt::format(format_args!("{}\n",device)).as_str()).unwrap();
            fmt.write_str(fmt::format(format_args!("\n")).as_str()).unwrap();
        }
        Ok(())
    }
}

pub type SharedStats = SharedObject<Stats>;

/*

#[derive(Debug,Clone)]
pub struct SharedStats {
    stats : Arc<RwLock<Stats>>,
    ucnt: u32,
}

impl SharedStats {

    pub fn new(stats : Stats) -> SharedStats{
        SharedStats {stats : Arc::new(RwLock::new(stats)),ucnt : 0}
    }

    pub fn is_changed(&self) -> bool{

        if self.ucnt != self.stats.read().unwrap().ucnt {
            return true;
        }
        return false;
    }

    pub fn read_stats(&mut self) -> Stats {
        let guard = self.stats.read().unwrap();
        let result = guard.clone();
        self.ucnt = result.ucnt;
        result
    }

    pub fn write_stats(&self, stats: Stats) {
        let mut _stats = stats;
        _stats.ucnt += 1;
        let mut stats_guard = self.stats.write().unwrap();
        *stats_guard = _stats;

    }
}
*/
