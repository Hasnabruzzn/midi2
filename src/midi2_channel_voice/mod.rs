use crate::{
    error::Error,
    packet::Packet,
};

pub mod key_pressure;

mod attribute;
mod note;

pub use attribute::Attribute;
pub use note::note_on;
pub use note::note_off;

const TYPE_CODE: ux::u4 = ux::u4::new(0x4);

fn validate_packet(p: &Packet) -> Result<(), Error> {
    if p.nibble(0) != TYPE_CODE {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}