mod header;
mod order_0;
mod order_1;

use std::io::{self, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use self::header::write_header;
use super::{Order, LOWER_BOUND, STATE_COUNT};

pub fn encode(order: Order, src: &[u8]) -> io::Result<Vec<u8>> {
    match order {
        Order::Zero => order_0::encode(src),
        Order::One => order_1::encode(src),
    }
}

fn write_states<W>(writer: &mut W, states: &[u32; STATE_COUNT]) -> io::Result<()>
where
    W: Write,
{
    for state in states {
        writer.write_u32::<LittleEndian>(*state)?;
    }

    Ok(())
}

fn normalize<W>(writer: &mut W, mut x: u32, freq_i: u16) -> io::Result<u32>
where
    W: Write,
{
    while x >= (LOWER_BOUND >> 4) * u32::from(freq_i) {
        let b = (x & 0xff) as u8;
        writer.write_u8(b)?;
        x >>= 8;
    }

    Ok(x)
}

fn update(x: u32, freq_i: u16, cfreq_i: u16) -> u32 {
    let (q, r) = (x / u32::from(freq_i), x % u32::from(freq_i));
    (q << 12) + r + u32::from(cfreq_i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_with_order_0() -> io::Result<()> {
        let data = b"noodles";
        let actual = encode(Order::Zero, data)?;

        let expected = [
            0x00, 0x25, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x64, 0x82, 0x49, 0x65, 0x00,
            0x82, 0x49, 0x6c, 0x82, 0x49, 0x6e, 0x82, 0x49, 0x6f, 0x00, 0x84, 0x92, 0x73, 0x82,
            0x49, 0x00, 0xe2, 0x06, 0x83, 0x18, 0x74, 0x7b, 0x41, 0x0c, 0x2b, 0xa9, 0x41, 0x0c,
            0x25, 0x31, 0x80, 0x03,
        ];

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_encode_with_order_1() -> io::Result<()> {
        let data = b"noodles";
        let actual = encode(Order::One, data)?;

        let expected = [
            0x01, 0x3b, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x64, 0x83, 0xff, 0x6e,
            0x83, 0xff, 0x6f, 0x00, 0x88, 0x01, 0x00, 0x64, 0x6c, 0x8f, 0xff, 0x00, 0x65, 0x00,
            0x73, 0x8f, 0xff, 0x00, 0x6c, 0x65, 0x8f, 0xff, 0x00, 0x6e, 0x6f, 0x8f, 0xff, 0x00,
            0x6f, 0x00, 0x64, 0x87, 0xff, 0x6f, 0x88, 0x00, 0x00, 0x00, 0x07, 0x84, 0x00, 0x02,
            0x00, 0xe8, 0xff, 0x00, 0x00, 0xe8, 0xff, 0x00, 0x10, 0xe0, 0x00, 0x02,
        ];

        assert_eq!(actual, expected);

        Ok(())
    }
}
