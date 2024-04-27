use crate::{
    buffer::{
        Buffer, BufferMut, SpecialiseU32, SpecialiseU8, UnitPrivate, UNIT_ID_U32, UNIT_ID_U8,
    },
    numeric_types::*,
    util::{property::Property, schema, BitOps},
};

pub struct UmpMessageTypeProperty<const TYPE: u8>;

impl<const TYPE: u8, B: Buffer> Property<B> for UmpMessageTypeProperty<TYPE> {
    type Type = ();
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        if <B::Unit as UnitPrivate>::UNIT_ID == UNIT_ID_U32 {
            let b = buffer.buffer().specialise_u32()[0];
            if b.nibble(0) != crate::u4::new(TYPE) {
                return Err(crate::Error::InvalidData("Incorrect ump message type"));
            }
        }
        Ok(())
    }
    fn write(buffer: &mut B, _v: Self::Type) -> crate::result::Result<()>
    where
        B: BufferMut,
    {
        if <B::Unit as UnitPrivate>::UNIT_ID == UNIT_ID_U32 {
            buffer.buffer_mut().specialise_u32_mut()[0].set_nibble(0, crate::u4::new(TYPE));
        }
        Ok(())
    }
    fn default() -> Self::Type {
        ()
    }
}

pub struct ChannelVoiceStatusProperty<const STATUS: u8>;

impl<const STATUS: u8, B: Buffer> Property<B> for ChannelVoiceStatusProperty<STATUS> {
    type Type = ();
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        let status = match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => {
                let b = buffer.buffer().specialise_u32()[0];
                b.nibble(2)
            }
            UNIT_ID_U8 => {
                let b = buffer.buffer().specialise_u8()[0];
                b.nibble(0)
            }
            _ => unreachable!(),
        };
        if status == u4::new(STATUS) {
            Ok(())
        } else {
            Err(crate::Error::InvalidData("Incorrect message status"))
        }
    }
    fn write(buffer: &mut B, _v: Self::Type) -> crate::result::Result<()>
    where
        B: BufferMut,
    {
        match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => {
                buffer.buffer_mut().specialise_u32_mut()[0].set_nibble(2, crate::u4::new(STATUS));
            }
            UNIT_ID_U8 => {
                buffer.buffer_mut().specialise_u8_mut()[0].set_nibble(0, crate::u4::new(STATUS));
            }
            _ => unreachable!(),
        }
        Ok(())
    }
    fn default() -> Self::Type {
        ()
    }
}

pub type ChannelProperty = HybridSchemaProperty<
    u4,
    schema::Bytes<0x0F, 0x0, 0x0>,
    schema::Ump<0x000F_0000, 0x0, 0x0, 0x0>,
>;
pub type GroupProperty = UmpSchemaProperty<u4, schema::Ump<0x0F00_0000, 0x0, 0x0, 0x0>>;

pub struct HybridSchemaProperty<T, B: schema::BytesSchema, U: schema::UmpSchema>(
    core::marker::PhantomData<(T, B, U)>,
);

impl<
        B: Buffer,
        BytesSchema: schema::BytesSchema,
        UmpSchema: schema::UmpSchema,
        T: Default + schema::UmpSchemaRepr<UmpSchema> + schema::BytesSchemaRepr<BytesSchema>,
    > Property<B> for HybridSchemaProperty<T, BytesSchema, UmpSchema>
{
    type Type = T;
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => Ok(<T as schema::UmpSchemaRepr<UmpSchema>>::read(
                buffer.buffer().specialise_u32(),
            )),
            UNIT_ID_U8 => Ok(<T as schema::BytesSchemaRepr<BytesSchema>>::read(
                buffer.buffer().specialise_u8(),
            )),
            _ => unreachable!(),
        }
    }
    fn write(buffer: &mut B, v: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => {
                <T as schema::UmpSchemaRepr<UmpSchema>>::write(
                    buffer.buffer_mut().specialise_u32_mut(),
                    v,
                );
                Ok(())
            }
            UNIT_ID_U8 => {
                <T as schema::BytesSchemaRepr<BytesSchema>>::write(
                    buffer.buffer_mut().specialise_u8_mut(),
                    v,
                );
                Ok(())
            }
            _ => unreachable!(),
        }
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

pub struct BytesSchemaProperty<T, B: schema::BytesSchema>(core::marker::PhantomData<(T, B)>);

impl<
        B: Buffer,
        BytesSchema: schema::BytesSchema,
        T: Default + schema::BytesSchemaRepr<BytesSchema>,
    > Property<B> for BytesSchemaProperty<T, BytesSchema>
{
    type Type = T;
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => Ok(Default::default()),
            UNIT_ID_U8 => Ok(<T as schema::BytesSchemaRepr<BytesSchema>>::read(
                buffer.buffer().specialise_u8(),
            )),
            _ => unreachable!(),
        }
    }
    fn write(buffer: &mut B, v: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        if <B::Unit as UnitPrivate>::UNIT_ID == UNIT_ID_U8 {
            <T as schema::BytesSchemaRepr<BytesSchema>>::write(
                buffer.buffer_mut().specialise_u8_mut(),
                v,
            );
        }
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

pub struct UmpSchemaProperty<T, B: schema::UmpSchema>(core::marker::PhantomData<(T, B)>);

impl<B: Buffer, UmpSchema: schema::UmpSchema, T: Default + schema::UmpSchemaRepr<UmpSchema>>
    Property<B> for UmpSchemaProperty<T, UmpSchema>
{
    type Type = T;
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => Ok(<T as schema::UmpSchemaRepr<UmpSchema>>::read(
                buffer.buffer().specialise_u32(),
            )),
            UNIT_ID_U8 => Ok(Default::default()),
            _ => unreachable!(),
        }
    }
    fn write(buffer: &mut B, v: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        if <B::Unit as UnitPrivate>::UNIT_ID == UNIT_ID_U32 {
            <T as schema::UmpSchemaRepr<UmpSchema>>::write(
                buffer.buffer_mut().specialise_u32_mut(),
                v,
            );
        }
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
}