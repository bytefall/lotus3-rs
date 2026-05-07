use core::cmp::Ordering;

use crate::{
    config::{CFG, Race},
    crc::{smart_crc, sub_ab9a},
    data::*,
    timer::{sub_d6f9, sub_d962},
};

unsafe fn prep_track_hills() {
    if !IS_TR_MOUNTAINS {
        WORD_2448 = 0;
        WORD_244A = 0;
    }

    WORD_245C = (192 * WORD_2448 as u32 / 100) as i16;
    WORD_245E = (160 * WORD_2448 as u32 / 100 + 32) as i16;

    WORD_2458 = (128 * WORD_244A as u32 / 100) as i16;
    WORD_245A = (160 * WORD_244A as u32 / 100 + 32) as i16;

    WORD_636E = WORD_23FE;
    WORD_6366 = 0;
    WORD_6382 = 0;

    let mut si = WORD_23FC;
    sub_ae5f(18, &mut si);

    loop {
        // loc_AD20
        if ((smart_crc() as u8) as i16) < WORD_245C {
            let mut bx = 2;

            // loc_AD2E
            while bx < 12 && ((smart_crc() as u8) as i16) < WORD_245E {
                bx += 2;
            }

            let mut cx = 1;

            // loc_AD44
            while cx < 6 && ((smart_crc() as u8) as i16) < WORD_245A {
                cx += 1;
            }

            let mut dx: i16 = if WORD_6366 < 0 { 1 } else { -1 };

            // loc_AD61
            if ((smart_crc() as u8) as i16) < WORD_2458 {
                dx = -dx;
            }

            // loc_AD6E
            let al = ((dx * cx * bx) as i8) + (WORD_6366 as i8);

            if !(-12..12).contains(&al) {
                continue;
            }

            sub_ae3c(bx as u16, cx as u16, dx, &mut si);
        } else {
            // loc_AD87
            sub_ae5f(2, &mut si);
        }

        // loc_AD8D
        if !IS_CIRCULAR_TRACK {
            let ax = ARR_2400[WORD_6382];

            if (WORD_636E as i16) >= (ax as i16) {
                WORD_6382 += 1;
                WORD_636E = ax + 18;

                si = (WORD_636E as usize) * 16;

                let mut it = TMP_FILE_BUF
                    .chunks_exact_mut(16)
                    .skip(ax as usize)
                    .take(18 + 1)
                    .peekable();

                while let Some(rec) = it.next() {
                    if let Some(next) = it.peek_mut() {
                        next[13] = rec[13];
                    } else {
                        rec[1] = 0;

                        WORD_6366 = i16::from_le_bytes([rec[13], WORD_6366.to_le_bytes()[1]]);
                    }
                }
            }
        }

        // loc_ADD8
        if WORD_636E > WORD_23F2 {
            break;
        }
    }

    // loc_ADE4
    if !IS_CIRCULAR_TRACK {
        return;
    }

    WORD_6366 = 0;
    let mut si = WORD_23FC + (WORD_23F2 as usize) * 16;

    // loc_ADFD
    loop {
        WORD_6366 += 1;
        si -= 16;

        if (TMP_FILE_BUF[si + 13] as i8).abs() <= WORD_6366 as i8 {
            break;
        }
    }

    // loc_AE15
    let mut al = 0.cmp(&(TMP_FILE_BUF[si + 13] as i8)) as i8;

    // loc_AE23:
    let mut it = TMP_FILE_BUF.chunks_exact_mut(16).skip(si).peekable();

    while let Some(rec) = it.next()
        && WORD_6366 >= 0
    {
        rec[1] = al as u8;

        let cl = (rec[13] as i8) + al;

        if cl == 0 {
            al = 0;
        }

        if let Some(n) = it.peek_mut() {
            n[13] = cl as u8;
        }

        WORD_6366 -= 1;
    }
}

///
/// AE3C:
///
/// in:
/// bx -
/// cx -
/// dx -
/// si - TMP_FILE_BUF
unsafe fn sub_ae3c(bx: u16, cx: u16, dx: i16, si: &mut usize) {
    WORD_636E += bx;

    let al = (cx as i16 * dx) as u8;
    let mut ah = WORD_6366 as u8;

    for _ in 0..bx {
        TMP_FILE_BUF[*si + 1] = al;
        TMP_FILE_BUF[*si + 13] = ah;

        ah += al;
        *si += 16;
    }

    WORD_6366 = ah as i16;
}

unsafe fn sub_ae5f(bx: u16, si: &mut usize) {
    if WORD_6366 != 0 && ((smart_crc() as u8) as i16) < 64 {
        // loc_AE85
        let bl = WORD_6366 as i8;

        sub_ae3c(bl.unsigned_abs() as u16, 1, if bl < 0 { 1 } else { -1 }, si);

        return;
    }

    // loc_AE70
    WORD_636E += bx;

    // loc_AE74
    for _ in 0..bx {
        TMP_FILE_BUF[*si + 1] = 0;
        TMP_FILE_BUF[*si + 13] = WORD_6366 as u8;

        *si += 16;
    }
}

unsafe fn prep_track_turns() {
    WORD_2464 = (160 * WORD_2442 as u32 / 100 + 32) as i16;
    WORD_2466 = (160 * WORD_2442 as u32 / 100 + 32) as i16;

    WORD_2460 = (160 * WORD_2444 as u32 / 100 + 64) as i16;
    WORD_2462 = (160 * WORD_2444 as u32 / 100 + 32) as i16;

    let mut si = WORD_23FC;
    WORD_636E = WORD_23FE;
    WORD_636A = 0;
    WORD_6366 = 0;
    let mut dx = 1;

    sub_b005(16, &mut si);

    loop {
        // loc_AEFA
        if ((smart_crc() as u8) as i16) < WORD_2464 {
            let mut bx = 4;

            // loc_AF08
            while bx < 24 && ((smart_crc() as u8) as i16) < WORD_2466 {
                bx += 4;
            }

            let mut cx = 4;

            // loc_AF1E
            while cx < 6 && ((smart_crc() as u8) as i16) < WORD_2462 {
                cx += 1;
            }

            // loc_AF2F
            if ((smart_crc() as u8) as i16) < WORD_2460 {
                dx = -dx;
            }

            // loc_AF3C
            if IS_CIRCULAR_TRACK {
                let ax = (-96 * (WORD_636E + bx) as i32) / WORD_23F2 as i32
                    + ((dx as i32 * cx as i32 * bx as i32) as i16 as i32)
                    - WORD_6366 as i32;

                if !(-32..32).contains(&ax) {
                    continue;
                }
            }

            sub_afe0(bx, cx, dx, &mut si);
        } else {
            let bx = 4;

            // loc_AF72
            if IS_CIRCULAR_TRACK {
                let ax = (((WORD_636E + bx) as i32 * -96) / WORD_23F2 as i32) - WORD_6366 as i32;

                if !(-32..32).contains(&ax) {
                    continue;
                }
            }

            sub_b005(bx, &mut si);
        }

        // loc_AFA3
        if IS_CIRCULAR_TRACK {
            if WORD_636E > WORD_23F2 {
                // loc_AFBB:
                WORD_249E = TMP_FILE_BUF[12 + (WORD_23F2 as usize) * 16].wrapping_neg() as u16;

                break;
            }
        } else {
            // loc_AFCE
            if WORD_636E >= 1024 {
                WORD_249E = 128;

                break;
            }
        }
    }
}

unsafe fn sub_afe0(bx: u16, cx: i16, dx: i16, si: &mut usize) {
    WORD_636E += bx;
    WORD_636A = dx * cx;

    let ax = WORD_636A;
    let mut bp = WORD_6366;

    for _ in 0..bx {
        TMP_FILE_BUF[*si] = ax as u8;
        TMP_FILE_BUF[*si + 12] = bp as u8;

        bp += ax;
        *si += 16;
    }

    WORD_6366 = bp;
}

/// when demo:
/// bx = 16, si = 81D6 (&TMP_FILE_BUF[128])
/// bx = 4, si = 82D6 (&arr_8256[128])
/// bx = 4, si = 8316 (+64)
/// bx = 4, si = 8356 (+64)
/// bx = 4, si = 8396 (+64)
/// bx = 4, si = 8416 (+128)
/// bx = 4, si = 8456 (+64)
/// bx = 4, si = 84D6 (+128)
/// bx = 4, si = 8556 (+128)
/// bx = 4, si = 8596
/// bx = 4, si = 85D6
/// bx = 4, si = 8616
/// bx = 4, si = 8656
/// bx = 4, si = 8756
/// bx = 4, si = 8796
/// bx = 4, si = 87D6
/// bx = 4, si = 8816
/// bx = 4, si = 8856
/// bx = 4, si = 8896
/// bx = 4, si = 88D6
/// bx = 4, si = 8916
/// bx = 4, si = 8956
unsafe fn sub_b005(bx: u16, si: &mut usize) {
    WORD_636E += bx;

    let mut ax = WORD_636A;
    let mut bp = WORD_6366;

    for _ in 0..bx {
        if ax != 0 {
            if ax > 0 {
                ax -= 1;
            } else {
                ax += 1;
            }
        }

        TMP_FILE_BUF[*si] = ax as u8;
        TMP_FILE_BUF[*si + 12] = bp as u8;

        bp += ax;
        *si += 16;
    }

    WORD_636A = ax;
    WORD_6366 = bp;
}

unsafe fn prep_track_signs() {
    const ARR_24A0: [(i8, i8); 26] = [
        (-18, -16),
        (18, -128),
        (-18, -12),
        (18, -67),
        (-18, -15),
        (18, -91),
        (-18, -16),
        (18, -128),
        (-18, -16),
        (18, -128),
        (-18, -16),
        (18, -128),
        (-18, -16),
        (18, -128),
        (-18, -16),
        (18, -128),
        (-18, -16),
        (18, -128),
        (-18, -16),
        (18, -128),
        (-18, -16),
        (18, -128),
        (-18, -16),
        (18, -128),
        (-18, -16),
        (18, -128),
    ];

    if matches!(
        TRACK_NUM,
        Scenario::Desert | Scenario::Rally | Scenario::Mountains
    ) {
        return;
    }

    let [al, ah] = (((WORD_2AB6 + 2) / 4) + 2).to_le_bytes();

    let bl = ARR_24A0[TRACK_NUM as usize * 2].1;
    let bh = al.wrapping_neg();

    let cl = ARR_24A0[TRACK_NUM as usize * 2 + 1].1;
    let ch = al;

    let dl = 0;
    let mut _dh = 0;

    let mut si = 0;
    let mut bp = 0;

    while si < TMP_FILE_BUF.len() {
        let al = TMP_FILE_BUF[si + 64];

        match al as i8 {
            6 => {
                // loc_B08C
                if bp < 4 {
                    TMP_FILE_BUF[si + 4] = cl as u8;
                    TMP_FILE_BUF[si + 5] = ch;

                    TMP_FILE_BUF[si + 8] = cl as u8;
                    TMP_FILE_BUF[si + 9] = ch;
                }

                TMP_FILE_BUF[si + 2] = cl as u8;
                TMP_FILE_BUF[si + 3] = ch;

                TMP_FILE_BUF[si + 6] = cl as u8;
                TMP_FILE_BUF[si + 7] = ch;
            }
            5 | 4 => {
                // loc_B09F
                if bp < 4 {
                    TMP_FILE_BUF[si + 6] = cl as u8;
                    TMP_FILE_BUF[si + 7] = ch;
                }

                TMP_FILE_BUF[si + 2] = cl as u8;
                TMP_FILE_BUF[si + 3] = ch;
            }
            -6 => {
                // loc_B0AC
                if bp < 4 {
                    TMP_FILE_BUF[si + 4] = bl as u8;
                    TMP_FILE_BUF[si + 5] = bh;

                    TMP_FILE_BUF[si + 8] = bl as u8;
                    TMP_FILE_BUF[si + 9] = bh;
                }

                TMP_FILE_BUF[si + 2] = bl as u8;
                TMP_FILE_BUF[si + 3] = bh;

                TMP_FILE_BUF[si + 6] = bl as u8;
                TMP_FILE_BUF[si + 7] = bh;
            }
            -5 | -4 => {
                // loc_B0BF
                if bp < 4 {
                    TMP_FILE_BUF[si + 6] = bl as u8;
                    TMP_FILE_BUF[si + 7] = bh;
                }

                TMP_FILE_BUF[si + 2] = bl as u8;
                TMP_FILE_BUF[si + 2] = bh;
            }
            _ => {}
        }

        // loc_B0CA
        if dl == ah {
            bp += 1;
        } else {
            bp = 0;
            _dh = al;
        }

        // loc_B0D6
        if IS_TR_FUTURISTIC && al != 0 {
            if (al as i8) < 0 {
                TMP_FILE_BUF[si + 8] = 0xD5;
                TMP_FILE_BUF[si + 9] = 0x28;
            } else {
                TMP_FILE_BUF[si + 8] = 0xD2;
                TMP_FILE_BUF[si + 9] = 0xD8;
            }
        }

        si += 16;
    }
}

unsafe fn sub_b0fc() {
    const ARR_24D4: [u16; 104] = [
        0x94C8, 0, 0x9980, 0x9880, 0, 0x9510, 0x9720, 0, 0xC0F2, 0xF5F1, 0xC1F8, 0xC2F8, 0, 0, 0,
        0, 0, 0xA3F0, 0xA4F8, 0, 0, 0, 0, 0, 0xADF8, 0, 0xAFF0, 0xB0F0, 0xB1F1, 0xF2F2, 0xB3F0, 0,
        0xB5F0, 0xB6F0, 0xB7F0, 0xB8F0, 0xB9F0, 0xBAF1, 0xF3F2, 0, 0x9BF0, 0xA2F0, 0xA2F0, 0xA2F3,
        0x9BF0, 0xA2F0, 0xA2F0, 0xA2F3, 0xC9F8, 0, 0, 0xCCF0, 0, 0, 0, 0, 0xD0F8, 0x97F0, 0xE8F0,
        0xEFF0, 0, 0, 0, 0, 0, 0xD9F0, 0xD7F8, 0, 0, 0, 0, 0, 0xE7F8, 0, 0xE920, 0xEA20, 0xED20, 0,
        0, 0, 0xB2F8, 0xA6F8, 0x86C0, 0xBBF0, 0x81F0, 0, 0, 0, 0xDDF0, 0, 0, 0, 0, 0, 0, 0, 0x86F1,
        0x86F1, 0x86F1, 0x86F1, 0, 0, 0, 0,
    ];

    if IS_TR_MOUNTAINS {
        // add pillars for Scenario::Mountains

        // loc_B10A
        const BL: u8 = 0x96;
        const CL: u8 = 0xE2;

        for rec in TMP_FILE_BUF.chunks_exact_mut(16) {
            let ax = smart_crc();
            let dl = if (ax as i16) < 0 { CL } else { BL };

            rec[2] = dl;
            rec[3] = ((sub_ab9a() as u8) & 3) + 17;
            rec[4] = 0;
            rec[5] = 0;

            let ax = smart_crc();
            let dl = if (ax as i16) < 0 { CL } else { BL };

            rec[6] = dl;
            rec[7] = ((smart_crc() as u8) & 3) + 17;
            rec[8] = 0;
            rec[9] = 0;
        }

        return;
    }

    // add billboards, rocks, trees on the side or a road

    // loc_B14A
    WORD_246E = ((-48 * WORD_2450 as i32) / 100 + 64) as u16;
    WORD_246A = ((-96 * WORD_2450 as i32) / 100 + 224) as i16;
    WORD_2468 = ((128 * WORD_244E as i32) / 100 + 64) as i16;
    WORD_246C = ((64 * WORD_244E as i32) / 100 + 160) as u16;

    let di = (TRACK_NUM as usize) * 8;
    let mut si = 0;

    // loc_B197
    while si < TMP_FILE_BUF.len() {
        let mut _cx = 1;

        if ((smart_crc() as u8) as u16) >= WORD_246E {
            si += 16;
            continue;
        }

        // loc_B1A8
        let mut bp: u16;

        loop {
            let ax = smart_crc() & 0xE;
            _cx = ARR_24D4[di + ax as usize / 2];
            bp = _cx & 0xFFFC;

            if bp != 0 {
                WORD_6354 = ax;

                if ((WORD_6354 as u8) as u16) < bp {
                    break;
                }
            }
        }

        let bh = bp.to_le_bytes()[1];
        WORD_6362.0 = 0;
        WORD_6362.1 = bh;

        let mut dx = sub_b67c(bh as u16);

        smart_crc();
        let dl = if (WORD_6354 as i16) < 0 {
            dx.wrapping_neg()
        } else {
            dx
        } as u8;

        // loc_B1DE:
        WORD_6362.0 = dl;
        WORD_636A = _cx as i16;

        WORD_6362.0 = match _cx & 3 {
            1 if (WORD_6362.0 as i8) < 0 => WORD_6362.0.wrapping_neg(), // loc_B205
            2 if (WORD_6362.0 as i8) >= 0 => WORD_6362.0.wrapping_neg(), // loc_B1FC
            3 => 0,
            _ => WORD_6362.0,
        };

        // loc_B210
        _cx = 3;

        while _cx < 12 && ((smart_crc() as u8) as i16) < WORD_246A {
            _cx += 2;
        }

        // loc_B226
        dx = 4;

        while dx > 1 && ((smart_crc() as u8) as i16) < WORD_2468 {
            dx /= 2;
        }

        // loc_B23B
        WORD_636A &= 8;

        if WORD_636A == 0 || dx == 1 {
            dx *= 2;
        }

        // loc_B249
        for _ in 0..=_cx {
            if sub_b7cd(WORD_6362.1, si) {
                si += 16;
                continue;
            }

            if u16::from_le_bytes([TMP_FILE_BUF[si + 2], TMP_FILE_BUF[si + 3]]) != 0 {
                si += 16;
                continue;
            }

            let bp = u16::from_le_bytes([WORD_6362.0, WORD_6362.1])
                .rotate_right(8)
                .to_le_bytes();

            if ((smart_crc() as u8) as u16) < WORD_246C {
                TMP_FILE_BUF[si + 2] = bp[0];
                TMP_FILE_BUF[si + 3] = bp[1];
            }

            // loc_B26D
            if dx == 4 {
                si += 16;
                continue;
            }

            if ((smart_crc() as u8) as u16) < WORD_246C {
                TMP_FILE_BUF[si + 6] = bp[0];
                TMP_FILE_BUF[si + 7] = bp[1];
            }

            // loc_B280
            if dx == 2 {
                si += 16;
                continue;
            }

            if ((smart_crc() as u8) as u16) < WORD_246C {
                TMP_FILE_BUF[si + 8] = bp[0];
                TMP_FILE_BUF[si + 9] = bp[1];
            }

            // loc_B293
            if ((smart_crc() as u8) as u16) < WORD_246C {
                TMP_FILE_BUF[si + 4] = bp[0];
                TMP_FILE_BUF[si + 5] = bp[1];
            }

            si += 16;
        }
    }
}

unsafe fn sub_b2b0() {
    let mut si = 256; // 256 = 0x8256 - 0x8156

    // add "road works" signs and obstacles
    if IS_TR_ROADWORKS {
        // loc_B470
        WORD_2478 = ((-48 * WORD_2452 as i32) / 100 + 64) as i16;
        WORD_2476 = ((-96 * WORD_2450 as i32) / 100 + 224) as i16;

        while si < TMP_FILE_BUF.len() {
            for _ in 0..=1 {
                if ((smart_crc() as u8) as i16) < WORD_2478 {
                    break;
                }

                si += 16;
            }

            smart_crc();
            WORD_6362 = (if (WORD_6354 as i16) < 0 { 16i16 } else { -16 })
                .to_le_bytes()
                .into();
            WORD_6366 = ((smart_crc() & 0xF) + 0xF8) as i16;

            let mut cx = 4;

            while cx < 12 && ((smart_crc() as u8) as i16) < WORD_2476 {
                cx += 2;
            }

            for _ in 0..=cx {
                TMP_FILE_BUF[si + 2] = 0xD1;
                TMP_FILE_BUF[si + 3] = WORD_6362.0;

                sub_b521();

                let ax = smart_crc();
                let bl = if ax < 0x4000 { 0xD3 } else { 0xD4 };

                TMP_FILE_BUF[si + 4] = bl;
                TMP_FILE_BUF[si + 5] =
                    ((WORD_6362.0 as i8) + if (WORD_6354 as i16) < 0 { -8 } else { 8 }) as u8;

                sub_b521();

                TMP_FILE_BUF[si + 6] = 0xD1;
                TMP_FILE_BUF[si + 7] = WORD_6362.0;

                sub_b521();

                TMP_FILE_BUF[si + 8] = if (WORD_6354 as i16) < 0 { 0xF6 } else { 0xD8 };
                TMP_FILE_BUF[si + 9] = WORD_6362.0;

                si += 16;
            }
        }

        return;
    }

    // add flood lines and obstacles (logs)
    if IS_TR_FOREST {
        // loc_B2ED
        WORD_2472 = ((16 * WORD_2452 as i32) / 100) as i16;

        while si < TMP_FILE_BUF.len() {
            if ((smart_crc() as u8) as i16) < WORD_2472 {
                let al = ((smart_crc() & 0x18) - 12) as u8;

                if TMP_FILE_BUF[si] == 0
                    && TMP_FILE_BUF[si + 4] == 0
                    && TMP_FILE_BUF[si + 42] == 0
                    && TMP_FILE_BUF[si + 43] == 0
                    && TMP_FILE_BUF[si + 45] == 0
                {
                    // add log
                    TMP_FILE_BUF[si + 4] = 0x95; // S95 - wood log
                    TMP_FILE_BUF[si + 5] = al; // position: 0 = center, < 0 = on the left, > 0 = on the right
                    // add flood line
                    TMP_FILE_BUF[si + 42] = 0;
                    TMP_FILE_BUF[si + 43] = 1;
                }
            }

            si += 16;
        }

        return;
    }

    if IS_TR_RALLY {
        // loc_B33D
        WORD_2474 = ((24 * WORD_2452 as i32) / 100 + 8) as i16;

        while si < TMP_FILE_BUF.len() {
            if ((smart_crc() as u8) as i16) < WORD_2474 {
                smart_crc();

                if TMP_FILE_BUF[si + 13] == 0 {
                    TMP_FILE_BUF[si + 10] = 0;
                    TMP_FILE_BUF[si + 11] = 0x10;
                }
            }

            si += 16;
        }

        return;
    }

    if IS_TR_MOTORWAY {
        // loc_B36F
        WORD_2470 = ((16 * WORD_2452 as i32) / 100) as i16;

        while si < TMP_FILE_BUF.len() {
            if ((smart_crc() as u8) as i16) < WORD_2470 {
                let al = smart_crc() as u8;

                if TMP_FILE_BUF[si + 4] == 0
                    && TMP_FILE_BUF[si + 10] == 0
                    && TMP_FILE_BUF[si + 11] == 0
                {
                    TMP_FILE_BUF[si + 4] = 0xA0; // SA0 - truck parts
                    TMP_FILE_BUF[si + 5] = al; // position: 0 = center, < 0 = on the left, > 0 = on the right
                    TMP_FILE_BUF[si + 10] = 0x22;
                    TMP_FILE_BUF[si + 11] = 0x20;
                }
            }

            si += 16;
        }

        return;
    }

    if IS_TR_MARSH {
        // loc_B3AE
        WORD_247E = 8;
        WORD_2480 = ((-128 * WORD_2450 as i32) / 100 + 224) as i16;

        'main: while si < TMP_FILE_BUF.len() {
            if ((smart_crc() as u8) as i16) < WORD_247E {
                let mut cx = 16;

                while cx < 64 && ((smart_crc() as u8) as i16) < WORD_2480 {
                    cx += 8;
                }

                for _ in 0..=cx {
                    if TMP_FILE_BUF[si + 13] != 0 {
                        break;
                    }

                    TMP_FILE_BUF[si + 2] = 0;
                    TMP_FILE_BUF[si + 3] = 0;
                    TMP_FILE_BUF[si + 4] = 0;
                    TMP_FILE_BUF[si + 5] = 0;
                    TMP_FILE_BUF[si + 6] = 0;
                    TMP_FILE_BUF[si + 7] = 0;
                    TMP_FILE_BUF[si + 8] = 0;
                    TMP_FILE_BUF[si + 9] = 0;
                    TMP_FILE_BUF[si + 10] = 0x22;
                    TMP_FILE_BUF[si + 11] = 0x22;

                    si += 16;

                    if si == TMP_FILE_BUF.len() {
                        break 'main;
                    }
                }

                si += 256;
            }

            si += 16;
        }

        // loc_B41C
        WORD_2472 = ((16 * WORD_2452 as i32) / 100) as i16;
        si = 256; // 256 = 0x8256 - 0x8156

        while si < TMP_FILE_BUF.len() {
            if ((smart_crc() as u8) as i16) < WORD_2472 {
                let al = ((smart_crc() & 0x18) - 12) as u8;

                if TMP_FILE_BUF[si] == 0
                    && TMP_FILE_BUF[si + 4] == 0
                    && TMP_FILE_BUF[si + 42] == 0
                    && TMP_FILE_BUF[si + 43] == 0
                    && TMP_FILE_BUF[si + 45] == 0
                {
                    TMP_FILE_BUF[si + 4] = 0xCE; // SCE - grass
                    TMP_FILE_BUF[si + 5] = al;
                    TMP_FILE_BUF[si + 42] = 0;
                    TMP_FILE_BUF[si + 43] = 1;
                }
            }

            si += 16;
        }

        return;
    }

    if IS_TR_FUTURISTIC {
        // loc_B538
        WORD_247A = 4;
        WORD_247C = ((-128 * WORD_2450 as i32) / 100 + 224) as i16;

        while si < TMP_FILE_BUF.len() {
            if ((smart_crc() as u8) as i16) < WORD_247A {
                let mut cx = 16;

                while cx < 64 && ((smart_crc() as u8) as i16) < WORD_247C {
                    cx += 8;
                }

                // turbo zone obstacles
                // SDE - observatory
                for _ in 0..=cx {
                    TMP_FILE_BUF[si + 2] = 0xDE; // 1-st SDE
                    TMP_FILE_BUF[si + 3] = 0xE0; // -32 = on the left
                    TMP_FILE_BUF[si + 4] = 0xDE; // 2-nd SDE
                    TMP_FILE_BUF[si + 5] = 0x20; // 32 = on the right
                    TMP_FILE_BUF[si + 6] = 0xDE; // 3-rd SDE
                    TMP_FILE_BUF[si + 7] = 0xE0; // -32 = on the left
                    TMP_FILE_BUF[si + 8] = 0xDE; // 4-th SDE
                    TMP_FILE_BUF[si + 9] = 0x20; // 32 = on the right
                    TMP_FILE_BUF[si + 10] = 0x22;
                    TMP_FILE_BUF[si + 11] = 0x22;

                    si += 16;

                    if si == TMP_FILE_BUF.len() {
                        return;
                    }
                }

                si += 256;
            }

            si += 16;
        }
    }
}

unsafe fn sub_b521() {
    match (WORD_6362.0 as i8).cmp(&(WORD_6366 as i8)) {
        Ordering::Less => WORD_6362.0 += 1,
        Ordering::Greater => WORD_6362.0 -= 1,
        Ordering::Equal => (),
    }
}

unsafe fn add_laser_beams() {
    const ARR_25A4: [u16; 104] = [
        0, 0, 0, 0x9840, 0, 0x9520, 0x97F0, 0, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0xF0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0xB0F0, 0xB181, 0xF282, 0, 0, 0, 0, 0, 0xB8F0, 0xB940, 0, 0, 0, 0xF0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0xCDF0, 0, 0, 0x97C0, 0, 0x97F0, 0xEF80, 0, 0, 0, 0, 0, 0, 0xD3F0,
        0, 0, 0, 0, 0, 0, 0xEBF0, 0, 0xECF0, 0, 0, 0, 0, 0, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0xDBF0, 0,
        0, 0, 0, 0, 0, 0, 0xE3F2, 0xE4C2, 0xE582, 0, 0, 0, 0, 0,
    ];

    WORD_248C = (16 * WORD_2452 as u32 / 100) as u16;

    let di = (TRACK_NUM as usize) * 16;
    let mut si = 256; // 256 = 0x8256 - 0x8156

    while si < TMP_FILE_BUF.len() {
        WORD_6376 = si + 2;
        WORD_636A = 4;

        while WORD_636A > 0 {
            if ((smart_crc() as u8) as u16) < WORD_248C {
                let mut bx;
                let mut cx;

                loop {
                    bx = smart_crc() & 0xE;
                    cx = ARR_25A4[(di + bx as usize) / 2].to_le_bytes();

                    WORD_6362.0 = cx[0] & 0xFC;
                    WORD_6362.1 = cx[1];

                    if WORD_6362 != (0, 0) && WORD_6362.0 > (WORD_6354 as u8) {
                        break;
                    }
                }

                WORD_6362.0 = 0;

                sub_b69d(WORD_6362.1 as u16);

                let ax = (0..)
                    .map(|_| smart_crc() & 3)
                    .find(|x| *x != 0)
                    .unwrap_or_default();

                let dx = bx - ax - 1;

                smart_crc();

                WORD_6362.0 = if (WORD_6354 as i16) < 0 {
                    dx.wrapping_neg()
                } else {
                    dx
                } as u8;
                WORD_6362.0 = match cx[0] & 3 {
                    1 if (WORD_6362.0 as i8) >= 0 => WORD_6362.0.wrapping_neg(),
                    2 if (WORD_6362.0 as i8) < 0 => WORD_6362.0.wrapping_neg(),
                    3 => 0,
                    _ => WORD_6362.0,
                };

                // loc_B649
                if !sub_b7cd(WORD_6362.1, si)
                    && TMP_FILE_BUF[WORD_6376] == 0
                    && TMP_FILE_BUF[WORD_6376 + 1] == 0
                {
                    TMP_FILE_BUF[WORD_6376] = WORD_6362.1;
                    TMP_FILE_BUF[WORD_6376 + 1] = WORD_6362.0;
                }
            }

            // loc_B661
            WORD_6376 += 2;
            WORD_636A -= 1;
        }

        si += 16;
    }
}

unsafe fn sub_b67c(ax: u16) -> u16 {
    ((WORD_2AB6 + 2) >> 2) + (((ARR_2674[((ax - 128) << 2) as usize] as u16) + 0xF) >> 4)
}

unsafe fn sub_b69d(ax: u16) -> u16 {
    ((WORD_2AB6 + 2) >> 2) - (((ARR_2674[((ax - 128) << 2) as usize] as u16) + 0xF) >> 4)
}

unsafe fn add_trees() {
    const ARR_2874: [u16; 104] = [
        0x94F0, 0, 0x9920, 0x9820, 0, 9510, 9720, 0, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0xA9F0, 0xA3F0,
        0xA4F0, 0, 0, 0, 0, 0, 0xADF0, 0, 0xAFF0, 0xB0F0, 0xB1F1, 0xF2F2, 0, 0, 0xB5F0, 0xB6F0,
        0xB7F0, 0xB8F0, 0xB9F0, 0xBAF1, 0xF3F2, 0, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0xC9F0, 0xCAF0,
        0xCBF0, 0xCCF0, 0xCDF0, 0xCEF0, 0, 0x97F0, 0xD0F0, 0x97F0, 0xE8F0, 0xEFF0, 0, 0, 0, 0,
        0xF0, 0, 0, 0, 0, 0, 0, 0, 0xE7F0, 0, 0xE920, 0xEA20, 0, 0xEB20, 0xEC20, 0, 0x8140, 0x8640,
        0, 0, 0xBB40, 0xDAF0, 0xDAF0, 0xDAF0, 0, 0, 0xDDF0, 0, 0, 0, 0, 0, 0xF0, 0, 0, 0, 0, 0, 0,
        0,
    ];

    WORD_2488 = (176 * WORD_244E as u32 / 100 + 16) as u16;
    WORD_248A = (56 * WORD_2450 as u32 / 100 + 64) as u16;

    let di = (TRACK_NUM as usize) * 16;
    let mut si = 0;

    while si < TMP_FILE_BUF.len() {
        WORD_6376 = si + 2;
        WORD_636A = 4;

        while WORD_636A > 0 {
            if ((smart_crc() as u8) as u16) < WORD_2488 {
                let mut bx;
                let mut cx;

                loop {
                    bx = smart_crc() & 0xE;
                    cx = ARR_2874[(di + bx as usize) / 2].to_le_bytes();

                    WORD_6362.0 = cx[0] & 0xFC;
                    WORD_6362.1 = cx[1];

                    if WORD_6362 != (0, 0) && WORD_6362.0 > (WORD_6354 as u8) {
                        break;
                    }
                }

                WORD_6362.0 = 0;

                sub_b67c(WORD_6362.1 as u16);

                let [_, _, dl, dh] =
                    (((smart_crc() as u8) as u32).pow(2) * (WORD_248A - bx) as u32).to_le_bytes();
                let dx = u16::from_le_bytes([dl, dh]);

                smart_crc();

                WORD_6362.0 = if (WORD_6354 as i16) < 0 {
                    dx.wrapping_neg()
                } else {
                    dx
                } as u8;
                WORD_6362.0 = match cx[0] & 3 {
                    1 if (WORD_6362.0 as i8) >= 0 => WORD_6362.0.wrapping_neg(),
                    2 if (WORD_6362.0 as i8) < 0 => WORD_6362.0.wrapping_neg(),
                    3 => 0,
                    _ => WORD_6362.0,
                };

                // loc_B77B
                if !sub_b7cd(WORD_6362.1, si) {
                    let a = matches!(WORD_6362.1, 0xCA | 0xCB);
                    let v = u16::from_le_bytes([
                        TMP_FILE_BUF[WORD_6376 + 10],
                        TMP_FILE_BUF[WORD_6376 + 11],
                    ]);

                    if !IS_TR_MARSH || (a && v == 0x2222 || !a && v != 0x2222) {
                        // loc_B7A2
                        if TMP_FILE_BUF[WORD_6376] == 0 && TMP_FILE_BUF[WORD_6376 + 1] == 0 {
                            TMP_FILE_BUF[WORD_6376] = WORD_6362.1;
                            TMP_FILE_BUF[WORD_6376 + 1] = WORD_6362.0;
                        }
                    }
                }
            }

            WORD_6376 += 2;
            WORD_636A -= 1;
        }

        si += 16;
    }
}

/// Returns `true` if the position is valid for placing an object, `false` otherwise.
unsafe fn sub_b7cd(al: u8, si: usize) -> bool {
    matches!(al, 0xB0 | 0xEC | 0xEB | 0xA9 | 0xCD | 0x97)
        && (TMP_FILE_BUF[si + 1] != 0 || TMP_FILE_BUF[si - 15] != 0 || TMP_FILE_BUF[si + 17] != 0)
}

unsafe fn sub_b7fd() {
    sub_b803();

    // loc_B97F
    if !IS_TR_FUTURISTIC {
        return;
    }

    // paint the road in black/white for each checkpoint (START)
    if IS_CIRCULAR_TRACK {
        // loc_B9B1
        for rec in TMP_FILE_BUF.chunks_exact_mut(16).take(8) {
            rec[10] = 0x33;
            rec[11] = 0x33;
        }

        for rec in TMP_FILE_BUF
            .chunks_exact_mut(16)
            .skip((WORD_23F2 - 8) as usize)
            .take(8)
        {
            rec[10] = 0x33;
            rec[11] = 0x33;
        }
    } else {
        // loc_B994
        for di in ARR_2400.iter().take(NUM_OF_TRACKS as usize + 1) {
            for rec in TMP_FILE_BUF
                .chunks_exact_mut(16)
                .skip(*di as usize - 8)
                .take(16)
            {
                rec[10] = 0x33; // 3rd stripe
                rec[11] = 0x33; // 1st stripe
            }
        }
    }
}

unsafe fn sub_b803() {
    if WORD_3EE4 != Race::TimeLimit {
        // loc_B913
        if IS_CIRCULAR_TRACK {
            // loc_B95C
            sub_b9f8(0, &mut TMP_FILE_BUF);

            TMP_FILE_BUF[2] = 0x8A;
            TMP_FILE_BUF[3] = 0;
            TMP_FILE_BUF[10] = 0x30;
            TMP_FILE_BUF[11] = 0;

            let di = WORD_23F2 as usize * 16;
            TMP_FILE_BUF[di + 2] = 0x8A;
            TMP_FILE_BUF[di + 3] = 0;

            sub_b9dd();
        } else {
            let mut si = 0; // arr_2400
            let mut di = (ARR_2400[si] * 16) as usize;

            TMP_FILE_BUF[di + 2] = 0x88;
            TMP_FILE_BUF[di + 3] = 0;
            TMP_FILE_BUF[di + 10] = 0x30;
            TMP_FILE_BUF[di + 11] = 0;

            sub_b9f8(di, &mut TMP_FILE_BUF);

            for _ in 0..NUM_OF_TRACKS {
                si += 1;
                di = (ARR_2400[si] * 16) as usize;

                TMP_FILE_BUF[di + 2] = 0x8A;
                TMP_FILE_BUF[di + 3] = 0;
                TMP_FILE_BUF[di + 10] = 0x30;
                TMP_FILE_BUF[di + 11] = 0;

                sub_b9f8(di, &mut TMP_FILE_BUF);
            }

            TMP_FILE_BUF[di + 2] = 0x89;
            TMP_FILE_BUF[di + 3] = 0;
            TMP_FILE_BUF[di + 10] = 0x30;
            TMP_FILE_BUF[di + 11] = 0;
        }

        sub_ba67();
    } else if IS_CIRCULAR_TRACK {
        // loc_B8A3
        TMP_FILE_BUF[2] = 0x8A;
        TMP_FILE_BUF[10] = 0x30;

        let bx = (WORD_23F2 as usize) * 16;
        TMP_FILE_BUF[bx + 2] = 0x8A;
        TMP_FILE_BUF[bx + 10] = 0x30;

        let ax = WORD_23F2 * 0xC13 / 0xE1 / sub_ba67();
        // let mut di = 0; // word_2422

        for (di, bp) in (0..NUM_OF_TRACKS).enumerate() {
            let cx = (NUM_OF_TRACKS - bp * 2) * 2 + ax + bp / 4 + if bp == 0 { 7 } else { 0 };

            WORD_2422[di] = cx.clamp(5, 99);
        }

        sub_b9dd();
    } else {
        // loc_B817
        let bx = sub_ba67();

        let mut si = 0; // arr_2400
        let mut di = 0; // word_2422

        for bp in 0..NUM_OF_TRACKS {
            let ax = (((ARR_2400[si + 1] - ARR_2400[si]) as u32 * 0xC13 / 0xE1) / bx as u32) as u16
                + NUM_OF_TRACKS * 2
                - bp * 4
                + if si == 0 { 7 } else { 0 }
                + 1;

            WORD_2422[di] = ax.clamp(5, 99); // timeout (seconds) for a track; this number is enlarged later by the time left from the previous track
            si += 1;
            di += 1;
        }

        si = 0;
        di = (ARR_2400[si] as usize) * 16;

        TMP_FILE_BUF[di + 2] = 0x88;
        TMP_FILE_BUF[di + 10] = 0x30;

        for _ in 0..NUM_OF_TRACKS {
            si += 1;
            di = (ARR_2400[si] as usize) * 16;

            TMP_FILE_BUF[di + 2] = 0x8A;
            TMP_FILE_BUF[di + 10] = 0x30;
        }

        TMP_FILE_BUF[di + 2] = 0x89;
    }
}

// For circular track only. See sub_a9d3.
unsafe fn sub_b9dd() {
    let mut ax = 0;
    let mut si = 0;

    for _ in 0..=NUM_OF_TRACKS {
        ax += WORD_23F2;

        ARR_2400[si] = ax;

        si += 1;
    }

    ARR_2400[si] = 0xFFFF;
}

unsafe fn sub_b9f8(di_start: usize, buf: &mut [u8]) {
    const ARR13_2944: [u16; 13] = [
        0x0090, 0x0092, 0x0091, 0x0090, 0x0090, 0x0090, 0x0090, 0x0090, 0x0090, 0x0090, 0x0090,
        0x0090, 0x0090,
    ];

    let bx = ARR13_2944[TRACK_NUM as usize];
    let ax = ((WORD_2AB6 + 2) >> 2) + 2;

    let hi = ax as u8;
    let lo = (bx >> 8) as u8;

    let mut di = di_start;

    for _ in 0..8 {
        if !IS_TR_MOUNTAINS {
            buf[di + 4] = lo;
            buf[di + 5] = hi;
            buf[di + 8] = lo;
            buf[di + 9] = hi;
        }

        di += 16;
    }

    for _ in 0..8 {
        for _ in 0..4 {
            di += 2;

            sub_ba51(di, buf);
        }

        buf[di + 2] = 0;
        buf[di + 3] = 8;

        di += 8;
    }
}

unsafe fn sub_ba51(di: usize, buf: &mut [u8]) {
    if buf[di + 1] as i8 <= 0 {
        return;
    }

    buf[di + 1] += 6;

    if IS_TR_ROADWORKS {
        buf[di] = 0;
        buf[di + 1] = 0;
    }
}

unsafe fn sub_ba67() -> u16 {
    const ARR_295E_ESPRIT_S4: [u16; 26] = [
        170, 180, 170, 160, 130, 170, 140, 180, 170, 130, 170, 190, 170, 270, 280, 270, 260, 230,
        270, 240, 280, 270, 230, 270, 290, 270,
    ];
    const ARR_295E_ELAN_SE: [u16; 26] = [
        130, 140, 130, 120, 90, 130, 100, 140, 130, 90, 130, 150, 130, 230, 240, 230, 220, 190,
        230, 200, 240, 230, 190, 230, 250, 230,
    ];
    const ARR_295E_M200: [u16; 26] = [
        150, 160, 150, 140, 110, 150, 120, 160, 150, 110, 150, 170, 150, 250, 260, 250, 240, 210,
        250, 220, 260, 250, 210, 250, 270, 250,
    ];

    let data = match CAR_NUM {
        0 => ARR_295E_ESPRIT_S4,
        1 => ARR_295E_ELAN_SE,
        2 => ARR_295E_M200,
        _ => unimplemented!(),
    };

    let start = data[TRACK_NUM as usize];
    let end = data[TRACK_NUM as usize + 13];

    let cx = WORD_23F2 * if IS_CIRCULAR_TRACK { NUM_OF_TRACKS } else { 1 };
    let dx = WORD_23F2 + if IS_CIRCULAR_TRACK { cx } else { 128 };

    let bx = start + ((end - start) * WORD_244C) / 99;
    let result = (((bx * 5091) / 192) * dx) / cx;

    WORD_23F8 = result;
    WORD_23FA = result;

    (bx * 1000) / 3600
}

// buf = 8256 + 8358 + 8A00 = [u8; 16128]
unsafe fn add_tunnels(mut si: usize, buf: &mut [u8]) {
    // Set base parameters depending on track type
    match TRACK_NUM {
        Scenario::Night => {
            WORD_635E = 0;
            WORD_6360 = 0x82EE;
            WORD_6362 = (0x11, 0x91);
            WORD_6364 = 0x8212;
        }
        Scenario::Futuristic => {
            WORD_635E = 0x8200;
            WORD_6360 = 0x82E2;
            WORD_6362 = (0x11, 0x91);
            WORD_6364 = 0x821E;
        }
        Scenario::MotorWay => {
            WORD_635E = 0x8200;
            WORD_6360 = 0x82E8;
            WORD_6362 = (0x11, 0x91);
            WORD_6364 = 0x8218;
        }
        _ => return,
    }

    WORD_2482 = (32 * WORD_244E as i16) / 100;
    WORD_2484 = (96 * WORD_244E as i16) / 100 + 128;
    let mut di;

    'main: while si < buf.len() {
        // loc_BB54
        while (smart_crc() as u8) as i16 >= WORD_2482 {
            si += 16;
        }

        // loc_BB65
        let mut cx = 4;

        while cx < 16 && ((smart_crc() as u8) as i16) < WORD_2484 {
            cx += 2;
        }

        WORD_636A = 4;
        let mut bx = 0;

        while WORD_636A != 0 {
            if buf[si] != 0 {
                if buf[si] > 0 {
                    // or      bl, 2
                } else {
                    // or      bl, 1
                }
            }

            if buf[si + 1] != 0 {
                // or      bl, 4
            }

            if buf[si + 2] == 0x8A || buf[si + 2] == 0x89 {
                break 'main;
            }

            si += 16;
            WORD_636A -= 1;
        }

        si -= 64;

        if (bx as u8) <= 2 || (bx as u8) == 4 {
            buf[si + 2] = 0xFC;
            buf[si + 3] = 0;

            bx = 0;
            di = si + 2;
        } else {
            break 'main;
        }

        for i in 0..cx {
            if i > 0 {
                // loc_BBD1
                buf[si + 2] = 0;
                buf[si + 3] = 0;
            }

            // loc_BBD6
            let [lo, hi] = WORD_6360.to_le_bytes();
            buf[si + 4] = hi;
            buf[si + 5] = lo;

            let [lo, hi] = WORD_635E.to_le_bytes();
            buf[si + 6] = hi;
            buf[si + 7] = lo;

            let [lo, hi] = WORD_6364.to_le_bytes();
            buf[si + 8] = hi;
            buf[si + 9] = lo;

            buf[si + 10] = WORD_6362.1;
            buf[si + 11] = WORD_6362.0;

            if buf[si + 34] == 0x8A || buf[si + 34] == 0x89 {
                // cx = 1;
                si += 16;
                continue 'main;
            }

            if buf[si + 1] != 0 {
                if (bx as u8) == 0 || (bx as u8) == 2 {
                    bx &= 0xFF02;
                } else {
                    // cx = 1;
                    si += 16;
                    continue 'main;
                }
            }

            // loc_BC14
            if (buf[si] as i8) < 0 {
                if (bx as u8) == 0 || (bx as u8) == 0xFF {
                    bx &= 0xFF02;
                } else {
                    // cx = 1;
                    si += 16;
                    continue 'main;
                }
            }

            // loc_BC25
            if buf[si] > 0 {
                if (bx as u8) == 0 || (bx as u8) == 1 {
                    bx &= 0xFF01;
                } else {
                    // cx = 1;
                    si += 16;
                    continue 'main;
                }
            }

            // loc_BC3B
            si += 16;
        }

        buf[si + 2] = 0xFD;
        buf[si + 3] = 0;
        buf[di + 1] = bx as u8;

        si += 16;
        // loc_BC4B
        si += 16;
    }
}

// buf = 8256 + 8358 + 8A00 = [u8; 16128]
unsafe fn add_bridges() {
    if !matches!(TRACK_NUM, Scenario::MotorWay | Scenario::Roadworks) {
        return;
    }

    WORD_2486 = (16 * WORD_244E as i16) / 100;

    let mut si = 256; // 256 = 0x8256 - 0x8156

    while si < TMP_FILE_BUF.len() {
        if ((smart_crc() as u8) as i16) < WORD_2486 {
            if !matches!(TMP_FILE_BUF[si + 2], 0x89 | 0x8A)
                && !matches!(TMP_FILE_BUF[si + 18], 0x89 | 0x8A)
            {
                if (TMP_FILE_BUF[si + 10] as i8) > 0
                    && (TMP_FILE_BUF[si + 26] as i8) > 0
                    && (TMP_FILE_BUF[si - 6] as i8) > 0
                {
                    TMP_FILE_BUF[si + 2] = 0xFA;
                    TMP_FILE_BUF[si + 3] = 0;

                    TMP_FILE_BUF[si + 4] = 0xFB;
                    TMP_FILE_BUF[si + 5] = 0;

                    TMP_FILE_BUF[si + 10] = 0x10;
                    TMP_FILE_BUF[si + 11] = 0;

                    si += 16;
                }
            } else {
                si += 16;
            }
        }

        si += 16;
    }
}

unsafe fn sub_bcca() {
    sub_bccf();
    loc_bcfe();
}

unsafe fn sub_bccf() {
    let mut ax: u16 = 0;
    let mut dx: u16 = 0;

    for rec in TMP_FILE_BUF.chunks_exact_mut(16).take(1024) {
        let mut axb = ax.to_le_bytes();

        rec[12] = axb[0];
        rec[13] = axb[1];

        axb[0] += rec[0];
        axb[1] += rec[1];
        ax = u16::from_le_bytes(axb);

        match rec[2] {
            0xFC => dx = 0x8000,
            0xFD => dx = 0,
            _ => (),
        }

        let w = u16::from_le_bytes([rec[10], rec[11]]) | dx;
        rec[10..].copy_from_slice(&w.to_le_bytes());
    }
}

unsafe fn loc_bcfe() {
    const SKIP: [u8; 15] = [
        0, 0x88, 0x89, 0x8A, 0x97, 0xEB, 0x82, 0xB8, 0xDA, 0xFC, 0xFD, 0xA9, 0xB0, 0xA0, 0xCD,
    ];

    for rec in TMP_FILE_BUF.chunks_exact_mut(16).take(1024) {
        let [mut dl, _dh] = WORD_634E.to_le_bytes();

        let [mut cl, _ch] = if IS_TR_MOTORWAY {
            [2, 0]
        } else {
            (-WORD_634E).to_le_bytes()
        };

        for bx in [6, 4, 2, 0] {
            if !SKIP.contains(&rec[bx + 2]) {
                let al = rec[bx + 3];

                if al > 0 {
                    // loc_BD64
                    if al < dl {
                        dl = al;
                    }
                } else {
                    if al < cl {
                        cl = al;
                    }
                }
            }
        }

        rec[14] = cl + 8;
        rec[15] = dl - 8;
    }

    // loc_BD87
    let mut cl: i8 = -10; // 0xF6
    let mut dl: i8 = 10; // 0x0A

    for rec in TMP_FILE_BUF.chunks_exact_mut(16).take(1024).rev() {
        let al = rec[14] as i8;

        if al != cl {
            if al > cl {
                cl = al;
            } else {
                cl -= 1;
            }
        }

        rec[14] = cl as u8;

        let al = rec[15] as i8;

        if al != dl {
            if al > dl {
                dl += 1;
            } else {
                dl = al;
            }
        }

        rec[15] = dl as u8;
    }
}

// tmp_file_buf = 8156 + 8256 + 8358 + 8A00 = [u8; 16384] = [u8; 1024 * 16]
unsafe fn sub_bdbc(tmp_file_buf: &mut [u8]) {
    if !IS_2PL_MODE {
        return;
    }

    let mut cx = WORD_634E;
    let dx = -WORD_634E;
    cx += 1;

    for rec in tmp_file_buf.chunks_exact_mut(16).take(1024) {
        let mut bp: u16 = 0;

        for i in [2, 4, 6, 8] {
            let (al, ah, bb) = sub_be01(rec[i], rec[i + 1], bp, cx, dx);

            bp = bb;
            rec[i] = al;
            rec[i + 1] = ah;
        }
    }
}

fn sub_be01(al: u8, ah: u8, bp: u16, cx: i16, dx: i16) -> (u8, u8, u16) {
    if al == 0 {
        return (al, ah, bp);
    }

    const SPECIALS: [u8; 12] = [
        0x80, 0x70, 0x82, 0xA0, 0xDA, 0xB8, 0xA5, 0x71, 0xBD, 0x74, 0xDB, 0xDC,
    ];

    if !SPECIALS.contains(&al) {
        let bx = i16::from_le_bytes([ah, (ah << 1) - if ah & 0x80 != 0 { 0xFF } else { 0 }]);

        if (cx..dx).contains(&bx) && bp != 0 {
            return (0, 0, bp);
        }
    }

    (al, ah, 1)
}

/// BFF8: Sleep
pub unsafe fn sleep(delay: u16) {
    while WORD_3E18 < delay {
        sub_d6f9();
    }
}

/// C002: Sleep
pub unsafe fn sub_c002(delay: u16) {
    loop {
        sub_d6f9();

        if WORD_3E18 >= delay {
            break;
        }

        if sub_c011() {
            break;
        }
    }
}

/// C011:
pub unsafe fn sub_c011() -> bool {
    if BYTE_3D86 {
        BYTE_3D88[BYTE_3D87 as usize] = 0x80;
        return true;
    }

    let al = CFG.byte_16fe;
    BYTE_3E2D = 0;
    sub_d962();

    let al = ((al ^ CFG.byte_16fe) & CFG.byte_16fe) & 0x30;

    al != 0 || BYTE_3E2D != 0
}
