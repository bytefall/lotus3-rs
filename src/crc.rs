use crate::{data::*, video::sub_d097};

static mut RND_248E: [u32; 4] = [0; 4]; // 248E
pub static mut RND_3EEE: [u32; 4] = [0; 4]; // 3EEE
pub static mut RND_3EFE: [u32; 4] = [0; 4]; // 3EFE

pub unsafe fn sub_a9d3() -> Option<()> {
    const ARR_23E2: [Scenario; 13] = [
        Scenario::Rally,
        Scenario::Futuristic,
        Scenario::Night,
        Scenario::Marsh,
        Scenario::Mountains,
        Scenario::Snow,
        Scenario::Roadworks,
        Scenario::Storm,
        Scenario::Desert,
        Scenario::Fog,
        Scenario::MotorWay,
        Scenario::Wind,
        Scenario::Forest,
    ];

    let mut it = ARR_1F3E.into_iter();
    let mut ax = it.next()?;

    if ax >= 13 {
        ax -= 1;
    }

    IS_CIRCULAR_TRACK = ax & 1 == 1;
    TRACK_NUM = ARR_23E2[(ax / 2) as usize];

    WORD_2442 = sub_d097(it.next()?);
    WORD_2444 = sub_d097(it.next()?);
    WORD_2446 = sub_d097(it.next()?);
    WORD_2448 = sub_d097(it.next()?);
    WORD_244A = sub_d097(it.next()?);
    WORD_244E = sub_d097(it.next()?);
    WORD_2450 = sub_d097(it.next()?);
    WORD_244C = it.next()?;

    let mut eax: u32 = 0;
    let mut di = 7;

    for i in ARR_1F3E.into_iter().take(8) {
        for cx in (0..i + di).rev() {
            eax = (eax + cx as u32).rotate_left(3);
        }

        di += 11;
    }

    WORD_2454 = eax as u16;
    WORD_2456 = (eax >> 16) as u16;
    RND_248E.fill(eax);

    if IS_CIRCULAR_TRACK {
        WORD_23FC = 0;
        WORD_23FE = 0;
    } else {
        WORD_23FC = 128;
        WORD_23FE = 8;
    }

    WORD_2446 = (WORD_2456 & 0x1FF) + (WORD_2446 << 8);

    let (ax, bx) = if IS_CIRCULAR_TRACK {
        (2048, 512)
    } else {
        (992, 256)
    };

    WORD_635E = (ax - bx) * WORD_2446 / 26_111 + bx;
    WORD_635A = 3.max(WORD_635E / 240 + 1);

    NUM_OF_TRACKS = (((15.min(WORD_635E as u32 / 128) - WORD_635A as u32 + 1) * WORD_2456 as u32)
        >> 16) as u16
        + WORD_635A;

    WORD_23F2 = WORD_635E / NUM_OF_TRACKS;

    if IS_CIRCULAR_TRACK {
        return Some(());
    }

    let mut cx = 8;
    ARR_2400[0] = cx;

    for i in ARR_2400.iter_mut().skip(1).take(NUM_OF_TRACKS as usize) {
        smart_crc();

        cx += WORD_23F2;
        *i = cx;
    }

    WORD_23F2 = WORD_635E;

    ARR_2400[NUM_OF_TRACKS as usize - 1] = WORD_635E + 8;
    ARR_2400[NUM_OF_TRACKS as usize] = 0xFFFF;

    Some(())
}

// TODO: make it returning (u32, bool) = (val, cf)
pub unsafe fn smart_crc() -> u16 {
    let val = RND_248E[0];

    let (val, cf) = val.overflowing_add(0x736F_A41F);
    RND_248E[0] = val;

    let (val, cf) = val.overflowing_add(RND_248E[1] + cf as u32);
    let (val, cf) = val.overflowing_add(0x1324_3959 + cf as u32);
    RND_248E[1] = val;

    let (val, cf) = val.overflowing_add(RND_248E[2] + cf as u32);
    let (val, cf) = val.overflowing_add(0x8686_1142 + cf as u32);
    RND_248E[2] = val;

    let (val, cf) = val.overflowing_add(RND_248E[3] + cf as u32);
    let (val, _cf) = val.overflowing_add(0x44F0_2127 + cf as u32);
    RND_248E[3] = val;

    WORD_6354 = (val >> 16) as u16; // get the higher word

    val as u16 // return lower word
}

pub unsafe fn sub_ab9a() -> u16 {
    crc_common(&mut RND_3EEE)
}

pub unsafe fn sub_abf0() -> u16 {
    crc_common(&mut RND_3EFE)
}

unsafe fn crc_common(rnd: &mut [u32; 4]) -> u16 {
    let val = rnd[0];

    let (val, cf) = val.overflowing_add(0x7362_541F);
    rnd[0] = val;

    let (val, cf) = val.overflowing_add(rnd[1] + cf as u32);
    let (val, cf) = val.overflowing_add(0x7FBE_6475 + cf as u32);
    rnd[1] = val;

    let (val, cf) = val.overflowing_add(rnd[2] + cf as u32);
    let (val, cf) = val.overflowing_add(0x8326_5984 + cf as u32);
    rnd[2] = val;

    let (val, cf) = val.overflowing_add(rnd[3] + cf as u32);
    let (val, _cf) = val.overflowing_add(0xF029_3747 + cf as u32);
    rnd[3] = val;

    WORD_6354 = (val >> 16) as u16; // get the higher word

    val as u16 // return lower word
}
