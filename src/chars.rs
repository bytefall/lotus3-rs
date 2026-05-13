use alloc::vec::Vec;
use core::{
    iter::{Copied, Peekable},
    slice::Iter,
};
use heapless::Vec as StackVec;

use crate::{archive::res_load, data::WORD_2E78};

// This struct takes 64 bytes
#[derive(Debug)]
pub struct BmpItem {
    pub repeat: u16,
    pub height: u16,
    pub data: Vec<u8>,
}

pub type BmpVec = StackVec<BmpItem, 40>;

/// C505:
unsafe fn bmp_load_and_prepare(chr: u8, ix: u8) -> BmpVec {
    let data = res_load(chr, ix);
    let mut items = BmpVec::new();

    for c in data.chunks_exact(8) {
        let pos = u16::from_le_bytes([c[0], c[1]]);
        let repeat = u16::from_le_bytes([c[2], c[3]]);
        let height = u16::from_le_bytes([c[4], c[5]]);
        let skip = c[6];

        items
            .push(sub_c557(&data[(pos << 4) as usize..], repeat, height, skip))
            .unwrap();

        if c.last() == Some(&0xFF) {
            break;
        }
    }

    items
}

/// C557:
unsafe fn sub_c557(data: &[u8], repeat: u16, height: u16, skip: u8) -> BmpItem {
    let byte_c5e2 = if WORD_2E78.to_le_bytes()[1] == 0 {
        WORD_2E78 as u8
    } else {
        unimplemented!() // WORD_2E7A[chunk_ix] // VAR_30A7
    };

    let mut buf = Vec::with_capacity(112 * 4);
    let mut odd = true;
    let mut it = data.iter().copied().peekable();

    for _ in 0..height {
        let mut dx = repeat;

        loop {
            let cx = dx.min(8);
            dx -= cx;

            sub_c5b0(&mut it, &mut buf, skip, cx, &mut odd, byte_c5e2);

            if dx == 0 {
                break;
            }
        }
    }

    // align buffer to 16-bytes boundary
    if buf.len() % 16 > 0 {
        buf.resize(buf.len() + 16 - buf.len() % 16, 0);
    }

    buf.shrink_to_fit();

    BmpItem {
        repeat,
        height,
        data: buf,
    }
}

/// C5B0:
fn sub_c5b0(
    src: &mut Peekable<Copied<Iter<'_, u8>>>,
    dst: &mut Vec<u8>,
    skip: u8,
    cx: u16,
    odd: &mut bool,
    byte_c5e2: u8,
) {
    let start = dst.len();
    dst.push(0);

    let mut ah = 0x80;
    let mut dh = 0;

    for _ in 0..cx {
        let al = if *odd {
            *src.peek().unwrap() >> 4
        } else {
            src.next().unwrap()
        } & 0xF;

        *odd = !*odd;

        if al != skip {
            dst.push(al + byte_c5e2);
            dh |= ah;
        }

        ah >>= 1;
    }

    dst[start] = dh;
}

/// C942: Load series of "CXX" resources.
pub unsafe fn chr_load_and_prepare_few<const N: usize>(ixs: &[u8; N]) -> [BmpVec; N] {
    let mut res: [BmpVec; N] = [const { BmpVec::new() }; N];
    let mut i = 0;

    while i < N {
        res[i] = bmp_load_and_prepare(b'C', ixs[i]);
        i += 1;
    }

    res
}
