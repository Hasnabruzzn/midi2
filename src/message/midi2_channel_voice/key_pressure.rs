use crate::{
    error::Error,
    message::helpers as message_helpers,
    message::midi2_channel_voice::{helpers as midi2cv_helpers, TYPE_CODE as MIDI2CV_TYPE_CODE},
    result::Result,
    util::debug,
    *,
};

#[derive(Clone, PartialEq, Eq)]
pub struct KeyPressureMessage<'a>(&'a [u32]);

debug::message_debug_impl!(KeyPressureMessage);

const OP_CODE: u4 = u4::new(0b1010);

impl<'a> KeyPressureMessage<'a> {
    pub fn builder(buffer: &mut [u32]) -> KeyPressureBuilder {
        KeyPressureBuilder::new(buffer)
    }
    pub fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn channel(&self) -> u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn note(&self) -> u7 {
        message_helpers::note_from_packet(self.0)
    }
    pub fn key_pressure_data(&self) -> u32 {
        self.0[1]
    }
    pub fn data(&self) -> &[u32] {
        self.0
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        midi2cv_helpers::validate_packet(data, MIDI2CV_TYPE_CODE, OP_CODE)?;
        message_helpers::validate_buffer_size(data, 2)?;
        Ok(Self(data))
    }
}

pub struct KeyPressureBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> KeyPressureBuilder<'a> {
    fn new(buffer: &'a mut [u32]) -> Self {
        match buffer.len() {
            0 | 1 => Self(Err(Error::BufferOverflow)),
            _ => {
                message_helpers::clear_buffer(buffer);
                message_helpers::write_type_to_packet(MIDI2CV_TYPE_CODE, buffer);
                message_helpers::write_op_code_to_packet(OP_CODE, buffer);
                Self(Ok(buffer))
            }
        }
    }
    pub fn group(mut self, group: u4) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_group_to_packet(group, buffer);
        }
        self
    }
    pub fn channel(mut self, channel: u4) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_channel_to_packet(channel, buffer);
        }
        self
    }
    pub fn note(mut self, v: u7) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_note_to_packet(v, buffer);
        }
        self
    }
    pub fn key_pressure_data(mut self, data: u32) -> Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[1] = data;
        }
        self
    }
    pub fn build(self) -> Result<KeyPressureMessage<'a>> {
        match self.0 {
            Ok(buffer) => Ok(KeyPressureMessage(buffer)),
            Err(e) => Err(e.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::random_buffer;

    #[test]
    fn builder() {
        assert_eq!(
            KeyPressureMessage::builder(&mut random_buffer::<2>())
                .group(u4::new(0xB))
                .channel(u4::new(0xC))
                .note(u7::new(0x59))
                .key_pressure_data(0xC0B83064)
                .build(),
            Ok(KeyPressureMessage(&[0x4BAC_5900, 0xC0B83064])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            KeyPressureMessage::from_data(&[0x4BAC_5900, 0xC0B83064])
                .unwrap()
                .group(),
            u4::new(0xB),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            KeyPressureMessage::from_data(&[0x4BAC_5900, 0xC0B83064])
                .unwrap()
                .channel(),
            u4::new(0xC),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            KeyPressureMessage::from_data(&[0x4BAC_5900, 0xC0B83064])
                .unwrap()
                .note(),
            u7::new(0x59),
        );
    }

    #[test]
    fn key_pressure_data() {
        assert_eq!(
            KeyPressureMessage::from_data(&[0x4BAC_5900, 0xC0B83064])
                .unwrap()
                .key_pressure_data(),
            0xC0B83064,
        );
    }
}