use std::thread::JoinHandle;
use std::time::Duration;
use std::io::Error;

use super::*;

#[derive(Debug,PartialEq)]

pub enum ManagedThreadState {
    UNDEFINED = -1,
    OK = 0x00,
    STOPPED = 0xFF,
}

/// Managed Thread, which provides standard thread control features like  stop
pub struct ManagedThreadHandle<T> {
    chn : ChannelPair<ManagedThreadState>,
    handle : JoinHandle<T>,
}

impl <T>ManagedThreadHandle<T> {
    pub fn stop(&self, timeout: Duration) -> Result<(), std::io::Error> {
        let result = self.chn.tx().send(ManagedThreadState::STOPPED);

        match result {
            Err(_err) => {
                trace!("{:?}", _err);
                trace!("Thread already stopped or channel is inactive");
                return Ok(());
            }
            _ => (),
        }

        let input = self.chn.rx().recv_timeout(timeout).unwrap_or(ManagedThreadState::UNDEFINED);

        match input {
            ManagedThreadState::OK => {
                trace!("Successfully stopped thread: {:?}", self.handle.thread().name());
                return Ok(());
            },
            _ => (),
        };
        return Err(Error::new(std::io::ErrorKind::TimedOut, format!("Cannot stopp thread {:?}", self.handle.thread().name())));
    }
}

pub struct ManagedThreadUtil{}
impl ManagedThreadUtil {

    pub fn is_stopped(status: &mut bool, ctl_pair: &ChannelPair<ManagedThreadState>) {
        if !ctl_pair.rx().is_empty() && *status != true {
            let cmd = ctl_pair.rx().recv_timeout(Duration::from_millis(100)).unwrap_or(ManagedThreadState::UNDEFINED);
            match cmd {
                ManagedThreadState::STOPPED => {
                    let _ = ctl_pair.tx().send(ManagedThreadState::OK);
                    *status = true;
                },
                _ => trace!("Received something {:?}", cmd),
            };
        }
    }
}

pub fn spawn<F, T>(name: String,f: F) -> ManagedThreadHandle<T> where
    F: FnOnce(ChannelPair<ManagedThreadState>) -> T,
    F: Send + 'static, T: Send + 'static {


    let (host_pair, task_pair) = ChannelPair::<ManagedThreadState>::new();
    let builder = std::thread::Builder::new().name(name);

    let handle = builder.spawn(move || f(task_pair)).unwrap();

    ManagedThreadHandle{chn: host_pair, handle}
}

#[allow(unused_imports)]
mod tests {

    use super::*;
    use std::thread::sleep;
    use std::io::ErrorKind;

    #[test]
    fn managed_thread_spawn_ok() {
        let t = spawn("test-thread".to_string(), move |ctl_pair| {
            let mut is_stopped = false;

            while !is_stopped {
                ManagedThreadUtil::is_stopped(&mut is_stopped,&ctl_pair);
            }
        });

        sleep(Duration::from_secs(5));

        let  result = t.stop(Duration::from_millis(100));
        assert_eq!(Some(()),result.ok());
    }

    #[test]
    fn managed_thread_spawn_err() {
        let t = spawn("test-thread".to_string(), move |_ctl_pair| {
            loop {}
        });

        sleep(Duration::from_secs(5));

        let  result = t.stop(Duration::from_millis(100));
        assert_eq!(ErrorKind::TimedOut,result.err().unwrap().kind());
    }
}


