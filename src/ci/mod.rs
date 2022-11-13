use crate::{error::Error, message::system_exclusive_8bit::Message as Sysex8Message};

pub mod discovery;
mod helpers;

const VERSION: u8 = 0x01;

pub trait CiMessage: Sized {
    fn to_sysex8<'a>(
        &self,
        messages: &'a mut [Sysex8Message],
        stream_id: u8,
    ) -> &'a [Sysex8Message];
    fn from_sysex8(messages: &[Sysex8Message]) -> Self;
    fn validate_sysex8(message: &[Sysex8Message]) -> Result<(), Error>;
    fn validate_to_sysex8_buffer(&self, messages: &[Sysex8Message]) -> Result<(), Error>;
    fn try_from_sysex8(messages: &[Sysex8Message]) -> Result<Self, Error> {
        <Self as CiMessage>::validate_sysex8(messages)?;
        Ok(<Self as CiMessage>::from_sysex8(messages))
    }
    fn try_to_sysex8<'a>(
        &self,
        messages: &'a mut [Sysex8Message],
        stream_id: u8,
    ) -> Result<&'a [Sysex8Message], Error> {
        self.validate_to_sysex8_buffer(messages)?;
        Ok(self.to_sysex8(messages, stream_id))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeviceId {
    Channel(ux::u4),
    MidiPort,
}
