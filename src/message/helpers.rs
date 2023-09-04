use crate::{
    error::Error,
    result::Result,
    util::{BitOps, Truncate},
    *,
};

pub fn validate_packet(p: &[u32], type_code: u4, op_code: u4) -> Result<()> {
    if p.is_empty() || p[0].nibble(0) != type_code || p[0].nibble(2) != op_code {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

pub fn validate_buffer_size(p: &[u32], sz: usize) -> Result<()> {
    if p.len() < sz {
        Err(Error::BufferOverflow)
    } else {
        Ok(())
    }
}

pub fn note_from_packet(p: &[u32]) -> u7 {
    p[0].octet(2).truncate()
}

pub fn clear_buffer(p: &mut [u32]) -> &mut [u32] {
    for d in &mut *p {
        *d = 0x0;
    }
    p
}

pub fn write_type_to_packet(t: u4, p: &mut [u32]) {
    p[0].set_nibble(0, t);
}

pub fn write_group_to_packet(g: u4, p: &mut [u32]) {
    p[0].set_nibble(1, g);
}

pub fn write_channel_to_packet(channel: u4, p: &mut [u32]) {
    p[0].set_nibble(3, channel);
}

pub fn write_op_code_to_packet(op_code: u4, p: &mut [u32]) {
    p[0].set_nibble(2, op_code);
}

pub fn write_note_to_packet(note: u7, p: &mut [u32]) -> &mut [u32] {
    p[0].set_octet(2, note.into());
    p
}

pub fn group_from_packet(p: &[u32]) -> u4 {
    p[0].nibble(1)
}

pub fn channel_from_packet(p: &[u32]) -> u4 {
    p[0].nibble(3)
}

pub fn sysex_group_consistent_groups(buffer: &[u32], stride: usize) -> Result<()> {
    use group_from_packet as gfp;
    if buffer
        .chunks_exact(stride)
        .all(|chunk| gfp(chunk) == gfp(buffer))
    {
        Ok(())
    } else {
        Err(Error::InvalidData)
    }
}

// assumes that buffer contains valid messages
pub fn validate_sysex_group_statuses<
    IsComplete: Fn(u4) -> bool,
    IsBegin: Fn(u4) -> bool,
    IsContinue: Fn(u4) -> bool,
    IsEnd: Fn(u4) -> bool,
>(
    buffer: &[u32],
    is_complete: IsComplete,
    is_begin: IsBegin,
    is_continue: IsContinue,
    is_end: IsEnd,
    stride: usize,
) -> Result<()> {
    let mut iter = buffer.chunks(stride).peekable();
    let first_status = iter.next().unwrap()[0].nibble(2);

    if iter.peek().is_none() {
        if is_complete(first_status) {
            return Ok(());
        } else {
            return Err(Error::InvalidData);
        }
    }

    if !is_begin(first_status) {
        return Err(Error::InvalidData);
    }

    while let Some(chunk) = iter.next() {
        let status = chunk[0].nibble(2);
        if (iter.peek().is_some() && !is_continue(status)) && !is_end(status) {
            return Err(Error::InvalidData);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_buffer_size() {
        assert_eq!(validate_buffer_size(&[], 2), Err(Error::BufferOverflow));
        assert_eq!(validate_buffer_size(&[0x0], 2), Err(Error::BufferOverflow));
        assert_eq!(validate_buffer_size(&[0x0, 0x0], 2), Ok(()));
    }

    #[test]
    fn test_note_from_packet() {
        assert_eq!(note_from_packet(&[0x0000_3200]), u7::new(0x32));
    }

    #[test]
    fn test_write_note_to_packet() {
        assert_eq!(
            write_note_to_packet(u7::new(0x73), &mut [0x0]),
            &[0x0000_7300]
        );
    }

    #[test]
    fn test_clear_buffer() {
        assert_eq!(clear_buffer(&mut [0x1234_5678, 0x8765_4321]), &[0x0, 0x0]);
    }
}