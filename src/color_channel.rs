#[derive(Clone, Copy, Debug)]
pub struct Channel {
    pub red: bool,
    pub green: bool,
    pub blue: bool,
    pub alpha: bool,
}

pub struct ChannelIter {
    index: usize,
    channels: Vec<Channel>,
}

impl ChannelIter {
    pub fn new(channels: Vec<Channel>) -> Self {
        Self { index: 0, channels }
    }
}

impl Iterator for ChannelIter {
    type Item = Channel;

    fn next(&mut self) -> Option<Self::Item> {
        let channel = self.channels[self.index];
        self.index = (self.index + 1) % self.channels.len();
        Some(channel)
    }
}
