use crate::util::BitOps;

pub struct UmpMessageTypeProperty<const TYPE: u8>;

impl<const TYPE: u8, B: crate::buffer::Ump> crate::util::property::Property<B>
    for UmpMessageTypeProperty<TYPE>
{
    type Type = u8;
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        if buffer.buffer()[0].nibble(0) == crate::u4::new(TYPE) {
            Ok(<Self as crate::util::property::Property<B>>::default())
        } else {
            Err(crate::Error::InvalidData)
        }
    }
    fn write(buffer: &mut B, _v: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        buffer.buffer_mut()[0].set_nibble(2, crate::u4::new(TYPE));
        Ok(())
    }
    fn default() -> Self::Type {
        0x0
    }
}

pub struct UtilityStatusProperty<const STATUS: u8>;

impl<const STATUS: u8, B: crate::buffer::Ump> crate::util::property::Property<B>
    for UtilityStatusProperty<STATUS>
{
    type Type = u8;
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        if buffer.buffer()[0].nibble(2) == crate::u4::new(STATUS) {
            Ok(<Self as crate::util::property::Property<B>>::default())
        } else {
            Err(crate::Error::InvalidData)
        }
    }
    fn write(buffer: &mut B, _v: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        buffer.buffer_mut()[0].set_nibble(2, crate::u4::new(STATUS));
        Ok(())
    }
    fn default() -> Self::Type {
        0x0
    }
}
