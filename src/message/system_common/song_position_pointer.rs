use crate::{
    message::{
        helpers as message_helpers,
        system_common::{self, TYPE_CODE as SYSTEM_COMMON_TYPE_CODE},
    },
    result::Result,
    util::{debug, BitOps, Encode7Bit},
    *,
};

const OP_CODE: u8 = 0xF2;

#[derive(Clone, PartialEq, Eq)]
pub struct SongPositionPointerMessage<'a>(&'a [u32]);

debug::message_debug_impl!(SongPositionPointerMessage);

impl<'a> SongPositionPointerMessage<'a> {
    pub fn builder(buffer: &mut [u32]) -> SongPositionPointerBuilder {
        SongPositionPointerBuilder::new(buffer)
    }
    pub fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn position(&self) -> u14 {
        u14::from_u7s(&[self.0[0].octet(2), self.0[0].octet(3)])
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        system_common::validate_packet(data, OP_CODE)?;
        Ok(Self(data))
    }
}

#[derive(PartialEq, Eq)]
pub struct SongPositionPointerBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> SongPositionPointerBuilder<'a> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        match system_common::validate_buffer_size(buffer) {
            Ok(()) => {
                message_helpers::clear_buffer(buffer);
                system_common::write_op_code_to_packet(buffer, OP_CODE);
                message_helpers::write_type_to_packet(SYSTEM_COMMON_TYPE_CODE, buffer);
                Self(Ok(buffer))
            }
            Err(e) => Self(Err(e)),
        }
    }
    pub fn group(mut self, v: u4) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_group_to_packet(v, buffer);
        }
        self
    }
    pub fn position(mut self, v: u14) -> Self {
        if let Ok(buffer) = &mut self.0 {
            let u7s = v.to_u7s();
            buffer[0].set_octet(2, u7s[0].into());
            buffer[0].set_octet(3, u7s[1].into());
        }
        self
    }
    pub fn build(self) -> Result<SongPositionPointerMessage<'a>> {
        match self.0 {
            Ok(buffer) => Ok(SongPositionPointerMessage(buffer)),
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
            SongPositionPointerMessage::builder(&mut random_buffer::<1>())
                .group(u4::new(0xA))
                .position(u14::new(0x367D))
                .build(),
            Ok(SongPositionPointerMessage(&[0x1AF2_7D6C])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            SongPositionPointerMessage::from_data(&[0x1AF2_7D6C])
                .unwrap()
                .group(),
            u4::new(0xA),
        );
    }

    #[test]
    fn position() {
        assert_eq!(
            SongPositionPointerMessage::from_data(&[0x1AF2_7D6C])
                .unwrap()
                .position(),
            u14::new(0x367D),
        );
    }
}