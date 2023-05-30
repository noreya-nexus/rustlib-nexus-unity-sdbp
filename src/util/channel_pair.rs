use crossbeam_channel::{Sender, Receiver, unbounded, bounded};

/// Creates a connected Crossbeam channel pair
pub struct ChannelPair<T> {
    tx : Sender<T>,
    rx : Receiver<T>,
}

impl <T>ChannelPair<T> {

    /// Creates a new Channelpair tuple. They are linked together.
    ///
    /// # Example
    /// ```
    ///
    /// use crate::noreya_sdbp::util::ChannelPair;
    ///
    /// let (mut server, mut client) = ChannelPair::new();
    /// server.tx().send("abc");
    /// let result = client.rx().recv().unwrap();
    ///
    /// assert_eq!(result,"abc");
    /// ```
    pub fn new() -> (ChannelPair<T>,ChannelPair<T>) {

        let (tx0,rx0) = unbounded();
        let (tx1,rx1) = unbounded();

        (ChannelPair {tx: tx0, rx: rx1},ChannelPair{tx: tx1,rx: rx0})
    }

    pub fn new_bound() -> (ChannelPair<T>,ChannelPair<T>) {

        let (tx0,rx0) = bounded(1);
        let (tx1,rx1) = bounded(1);

        (ChannelPair {tx: tx0, rx: rx1},ChannelPair{tx: tx1,rx: rx0})
    }

    /// Packs existing channels into a channel pair object
    /// # Arguments
    /// * sender  - Sender Channel
    /// * receiver - Receiver Channel
    ///
    /// # Result
    /// ChannelPair Object
    ///
    pub fn from_channels_to_single_pair(tx: Sender<T>,rx: Receiver<T>) -> ChannelPair<T> {
        ChannelPair{tx,rx}
    }

    /// Packs existing channels into a channel pair tuple and connects them.
    ///
    /// # Arguments
    /// * tx  - Sender Channel (Server)
    /// * tx1 - Sender Channel (Client)
    /// * rx - Receiver Channel (Server)
    /// * rx1 - Receiver Channel (Client)
    ///
    /// # Result
    /// ChannelPair Tuple which are connected together
    ///
    pub fn from_channels(tx : Sender<T>, tx1 : Sender<T>,rx: Receiver<T>,rx1: Receiver<T>) -> (ChannelPair<T>,ChannelPair<T>){
        (ChannelPair::from_channels_to_single_pair(tx,rx1),ChannelPair::from_channels_to_single_pair(tx1,rx))
    }

    /// Returns the Sender part of the channel
    pub fn tx(&self) -> &Sender<T>{
        &self.tx
    }

    /// Returns the receiver part of the channel
    pub fn rx(&self) -> &Receiver<T>{
        &self.rx
    }
}