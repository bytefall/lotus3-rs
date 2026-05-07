use alloc::{vec, vec::Vec};
use core::{arch::asm, mem::transmute};
use heapless::Vec as StackVec;

use crate::{
    data::*,
    dos::{file_open, file_read, loc_dc60},
    video::get_flags,
};

/// C95F
pub unsafe fn sub_c95f() {
    const BYTES_NUM: usize = 32 * 3;

    PALETTE[PALETTE.len() - ARR48_3126.len()..].copy_from_slice(&ARR48_3126);
}

/// C96D
pub unsafe fn sub_c96d(chr: u8, ix: u8) {
    ARR4_3156 = [chr, sub_c97e(ix >> 4), sub_c97e(ix), 0];
}

/// C97E
pub unsafe fn sub_c97e(num: u8) -> u8 {
    let n = (num & 0xF) + b'0';

    if n > b'9' { n + 7 } else { n }
}

/// C98C: Opens DAT file and reads 2816 bytes from the header.
pub unsafe fn open_and_read_data_file() {
    ARC_FILE_HANDLE = file_open(ARC_FILE_NAME);
    file_read(ARC_FILE_HANDLE, &mut ARC_HEADER);
}

const RES_PAIR_SIZE: usize = 10; // each resource pair (key and offset) is 10 bytes long
const RES_BUF_SIZE: usize = 17408; // maximal size of packed resource

// pub static mut FILE_BUF: [u8; RES_BUF_SIZE] = [0; RES_BUF_SIZE];
pub type Resource = StackVec<u8, RES_BUF_SIZE>;

/// C9A2: Set archive file handler to the resource's position.
pub unsafe fn arc_seek_res() -> u16 {
    ERR_STR_PTR = transmute(ARR4_3156.as_ptr());
    RES_KEY = [b' '; 8];

    for (dst, src) in
        RES_KEY.iter_mut().zip(
            ARR4_3156
                .iter()
                .copied()
                .take_while(|c| *c != 0)
                .map(|c: u8| if c.is_ascii_lowercase() { c - b' ' } else { c }),
        )
    {
        *dst = src;
    }

    let Some((curr, next)) = ARC_HEADER[2..]
        .windows(RES_PAIR_SIZE * 2)
        .step_by(RES_PAIR_SIZE)
        .find_map(|x| {
            if x[..8] != RES_KEY {
                return None;
            }

            let (l, r) = x.split_at(RES_PAIR_SIZE);
            Some((
                u16::from_le_bytes([l[8], l[9]]),
                u16::from_le_bytes([r[8], r[9]]),
            ))
        })
    else {
        loc_dc60(OPEN_ERROR);
    };

    let [lo, hi] = curr.to_le_bytes();
    let offset = (((hi as u32) << 8) | (lo as u32)) << 9;
    let cx = (offset >> 16) as u16;
    let dx = offset as u16;
    // let (dx, cf) = (lo as u16).overflowing_shl(9);
    // let cx = ((hi as u16) << 1) + if cf { 1 } else { 0 };

    // INT 21h / AH = 42h - SEEK - set current file position.
    asm!(
        "push ds",
        "push es",
        "int 0x21",
        "pop es",
        "pop ds",
        in("ax") 0x4200_u16,
        in("bx") ARC_FILE_HANDLE,
        in("cx") cx,
        in("dx") dx,
    );

    if get_flags() & 1 != 0 {
        loc_dc60(OPEN_ERROR);
    }

    (next - curr) << 9
}

/// CA15: Load resource from data file and unpack it.
pub unsafe fn res_load(chr: u8, ix: u8) -> Vec<u8> {
    // res_load -> arc_get_res
    sub_c96d(chr, ix);
    let len = arc_seek_res();
    DAT_FILE_HANDLE = ARC_FILE_HANDLE;

    let mut buf = vec![0; len.into()];
    let len = file_read(ARC_FILE_HANDLE, &mut buf);
    buf.resize(len.into(), 0);

    arc_unpack_res(&buf).unwrap()
}

const TABLE: [(u16, u16); 256] = create_table::<256>();

const fn create_table<const N: usize>() -> [(u16, u16); N] {
    let mut t = [(0, 0); N];
    let mut i = 0;

    while i < N {
        t[i].0 = i as u16;
        i += 1;
    }

    t
}

/// CA67: Unpack the resource.
pub fn arc_unpack_res(data: impl AsRef<[u8]>) -> Option<Vec<u8>> {
    let mut data = data.as_ref().iter().cloned();

    let len = data.next().filter(|x| x != &0)?;
    let end = data.next()?;

    let mut table = TABLE;

    // loc_CABF
    for _ in 0..len {
        let (ix, lo, hi) = (data.next()? as usize, data.next()?, data.next()?);

        table[ix].0 = u16::from_le_bytes([lo, if table[lo as usize].1 != 0 { 2 } else { 1 }]);
        table[ix].1 = u16::from_le_bytes([hi, if table[hi as usize].1 != 0 { 2 } else { 1 }]);
    }

    let mut state = Action::Start;
    let mut unpacked = Vec::with_capacity(64768);

    // loc_CB14
    'main: while let Some(mut al) = data.next() {
        if al == end {
            // loc_CB38
            if state.step(&mut unpacked, data.next()?) {
                break 'main;
            }

            continue 'main;
        }

        let mut stack = StackVec::<u16, 16>::new();

        // loc_CB1B
        'inner: loop {
            let (mut ax, bx) = table[al as usize];

            if ax.to_le_bytes()[1] == 0 {
                if state.step(&mut unpacked, ax as u8) {
                    break 'main;
                }

                break 'inner;
            }

            // loc_CB2A
            stack.push(bx).unwrap();

            // loc_CB31
            while ax.to_le_bytes()[1] == 1 {
                if state.step(&mut unpacked, ax as u8) {
                    break 'main;
                }

                if let Some(a) = stack.pop() {
                    ax = a;
                } else {
                    break 'inner; // goto loc_CB14
                };
            }

            al = ax as u8;

            // goto loc_CB1B
        }
    }

    unpacked.shrink_to_fit();

    Some(unpacked)
}

enum Action {
    Start,
    Unpack(u16),
    KeepHighByte(u8),
    Extend(u16),
    KeepLowByte(u8),
}

impl Action {
    fn step(&mut self, unpacked: &mut Vec<u8>, al: u8) -> bool {
        *self = match *self {
            // loc_CB43
            Action::Start => {
                if al == 0 {
                    return true;
                } else if al < 0x40 {
                    Action::Unpack((al & 0x3F).into())
                } else if al < 0x80 {
                    Action::KeepHighByte(al & 0x3F)
                } else if al < 0xC0 {
                    Action::Extend((al & 0x3F).into())
                } else {
                    Action::KeepLowByte(al & 0x3F)
                }
            }
            // loc_CB8D
            Action::Unpack(mut len) => {
                unpacked.push(al);
                len -= 1;

                if len == 0 {
                    Action::Start
                } else {
                    Action::Unpack(len)
                }
            }
            // loc_CB85
            Action::KeepHighByte(hi) => Action::Unpack(u16::from_le_bytes([al, hi])),
            // loc_CBA6
            Action::Extend(len) => {
                unpacked.resize(unpacked.len() + len as usize, al);

                Action::Start
            }
            // loc_CB9E
            Action::KeepLowByte(lo) => Action::Extend(u16::from_le_bytes([al, lo])),
        };

        false
    }
}

/// CBC3:
pub unsafe fn sub_cbc3(chr: u8, ix: u8) -> Vec<u8> {
    sub_c96d(chr, ix);
    let len = arc_seek_res();

    let mut buf = vec![0; len.into()];

    let len = file_read(ARC_FILE_HANDLE, &mut buf);
    buf.resize(len.into(), 0);
    buf
}

/// CBDD: Get resources.
pub unsafe fn load_resources<const N: usize>(chr: u8, ixs: &[u8; N]) -> [Vec<u8>; N] {
    let mut res: [Vec<u8>; N] = [const { Vec::new() }; N];
    let mut i = 0;

    while i < N {
        res[i] = res_load(chr, ixs[i]);
        i += 1;
    }

    res
}

/// CBFA: Get resources without unpacking them.
pub unsafe fn load_resource_series<const N: usize>(chr: u8, ixs: &[u8; N]) -> [Vec<u8>; N] {
    let mut res: [Vec<u8>; N] = [const { Vec::new() }; N];
    let mut i = 0;

    while i < N {
        res[i] = sub_cbc3(chr, ixs[i]);
        i += 1;
    }

    res
}
