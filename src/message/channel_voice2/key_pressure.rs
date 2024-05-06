use crate::{
    message::{channel_voice2::UMP_MESSAGE_TYPE, common_properties},
    numeric_types::{u4, u7},
    util::schema,
};

pub(crate) const STATUS: u8 = 0b1010;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(2))]
struct KeyPressure {
    #[property(crate::message::utility::JitterReductionProperty)]
    jitter_reduction: Option<crate::message::utility::JitterReduction>,
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
    status: (),
    #[property(common_properties::UmpSchemaProperty<u4, schema::Ump<0x000F_0000, 0x0, 0x0, 0x0>>)]
    channel: u4,
    #[property(common_properties::GroupProperty)]
    group: u4,
    #[property(common_properties::UmpSchemaProperty<u7, schema::Ump<0x0000_7F00, 0x0, 0x0, 0x0>>)]
    note: u7,
    #[property(common_properties::UmpSchemaProperty<u32, schema::Ump<0x0000_0000, 0xFFFF_FFFF, 0x0, 0x0>>)]
    key_pressure_data: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        use crate::traits::{Channeled, Grouped};

        let mut message = KeyPressure::new_arr();
        message.set_group(u4::new(0xB));
        message.set_channel(u4::new(0xC));
        message.set_note(u7::new(0x59));
        message.set_key_pressure_data(0xC0B83064);
        assert_eq!(
            message,
            KeyPressure([0x0, 0x4BAC_5900, 0xC0B83064, 0x0, 0x0]),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            KeyPressure::try_from(&[0x4BAC_5900, 0xC0B83064][..])
                .unwrap()
                .note(),
            u7::new(0x59),
        );
    }

    #[test]
    fn key_pressure_data() {
        assert_eq!(
            KeyPressure::try_from(&[0x4BAC_5900, 0xC0B83064][..])
                .unwrap()
                .key_pressure_data(),
            0xC0B83064,
        );
    }
}