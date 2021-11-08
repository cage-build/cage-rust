use std::io;

/// Simple hash to sha256.
pub fn sha3_256<R: io::Read>(mut reader: R) -> io::Result<[u8; 32]> {
    let mut state = [0u64; 25];
    const BUF_LEN: usize = 136;
    let mut buf = [0u8; BUF_LEN];

    loop {
        match reader.read(&mut buf)? {
            BUF_LEN => absorbing(&mut state, &mut buf),
            n => {
                buf[n..].fill(0);
                buf[n] = 0x06;
                buf[BUF_LEN - 1] ^= 0x80;
                absorbing(&mut state, &mut buf);
                break;
            }
        };
    }

    let mut hash = [0u8; 32];
    hash[0..8].copy_from_slice(&state[0].to_le_bytes());
    hash[8..16].copy_from_slice(&state[1].to_le_bytes());
    hash[16..24].copy_from_slice(&state[2].to_le_bytes());
    hash[24..32].copy_from_slice(&state[3].to_le_bytes());
    Ok(hash)
}
/// Compute state ^= buf and run [`keccak`] on the state.
fn absorbing(state: &mut [u64; 25], buf: &mut [u8; 136]) {
    for i in 0..buf.len() / 8 {
        let mut u64_buf = [0u8; 8];
        let buf_index = i * 8;
        u64_buf.copy_from_slice(&buf[buf_index..buf_index + 8]);
        state[i] ^= u64::from_le_bytes(u64_buf);
    }
    keccak(state);
}
#[test]
fn test_sha3_256() {
    assert_eq!(
        [
            0x64, 0x4b, 0xcc, 0x7e, 0x56, 0x43, 0x73, 0x04, 0x09, 0x99, 0xaa, 0xc8, 0x9e, 0x76,
            0x22, 0xf3, 0xca, 0x71, 0xfb, 0xa1, 0xd9, 0x72, 0xfd, 0x94, 0xa3, 0x1c, 0x3b, 0xfb,
            0xf2, 0x4e, 0x39, 0x38,
        ],
        sha3_256(std::io::BufReader::new(&b"hello world"[..])).unwrap()
    );
    assert_eq!(
		[
		0x42, 0x26, 0x18, 0x64, 0x42, 0x46, 0xc2, 0x1a, 0x63, 0x2a, 0x6e, 0x3d, 0xb0, 0x97,
		0x53, 0x18, 0x98, 0x17, 0x77, 0x82, 0xd7, 0xc8, 0x66, 0xec, 0x0b, 0xf2, 0xdf, 0x9d,
		0x7b, 0x41, 0x9f, 0x30,
		],
		sha3_256(std::io::BufReader::new(&b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed non risus. Suspendisse lectus tortor, dignissim sit amet, adipiscing nec, ultricies sed, dolor. Cras elementum ultrices diam. Maecenas ligula massa, varius a, semper congue, euismod non, mi. Proin porttitor, orci nec nonummy molestie, enim est eleifend mi, non fermentum diam nisl sit amet erat. Duis semper. Duis arcu massa, scelerisque vitae, consequat in, pretium a, enim. Pellentesque congue. Ut in risus volutpat libero pharetra tempor. Cras vestibulum bibendum augue. Praesent egestas leo in pede. Praesent blandit odio eu enim. Pellentesque sed dui ut augue blandit sodales. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Aliquam nibh. Mauris ac mauris sed pede pellentesque fermentum. Maecenas adipiscing ante non diam sodales hendrerit."[..])).unwrap()
	);
}

fn keccak(state: &mut [u64; 25]) {
    [
        0x0000000000000001,
        0x0000000000008082,
        0x800000000000808a,
        0x8000000080008000,
        0x000000000000808b,
        0x0000000080000001,
        0x8000000080008081,
        0x8000000000008009,
        0x000000000000008a,
        0x0000000000000088,
        0x0000000080008009,
        0x000000008000000a,
        0x000000008000808b,
        0x800000000000008b,
        0x8000000000008089,
        0x8000000000008003,
        0x8000000000008002,
        0x8000000000000080,
        0x000000000000800a,
        0x800000008000000a,
        0x8000000080008081,
        0x8000000000008080,
        0x0000000080000001,
        0x8000000080008008,
    ]
    .iter()
    .for_each(|round_constant| keccak_one(state, *round_constant))
}
fn keccak_one(state: &mut [u64; 25], round_constant: u64) {
    // Procedure THETA
    let mut c = [0u64; 5];
    for x in 0..5 {
        for y in 0..5 {
            c[x] ^= state[x + y * 5];
        }
    }
    for x in 0..5 {
        let d = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
        for y in 0..5 {
            state[x + y * 5] ^= d;
        }
    }

    // Procedure RHO
    const RHO_OFFSETS: [u32; 25] = [
        0, 1, 62, 28, 27, 36, 44, 6, 55, 20, 3, 10, 43, 25, 39, 41, 45, 15, 21, 8, 18, 2, 61, 56,
        14,
    ];
    for x in 0..5 {
        for y in 0..5 {
            let index = x + y * 5;
            state[index] = state[index].rotate_left(RHO_OFFSETS[index]);
        }
    }

    // Procedure PI
    let temp_a = state.clone();
    for x in 0..5 {
        for y in 0..5 {
            let index = y + ((2 * x + 3 * y) % 5) * 5;
            state[index] = temp_a[x + y * 5];
        }
    }

    // Procedure CHI
    let mut c = [0u64; 25];
    for y in 0..5 {
        for x in 0..5 {
            c[x] = state[x + 5 * y] ^ ((!state[(x + 1) % 5 + y * 5]) & state[(x + 2) % 5 + 5 * y]);
        }
        for x in 0..5 {
            state[x + 5 * y] = c[x];
        }
    }

    // Procedure IOTA
    state[0] ^= round_constant;
}
#[test]
fn test_keccak() {
    let mut buf = [0u64; 25];
    keccak(&mut buf);
    assert_eq!(
        [
            0xf1258f7940e1dde7,
            0x84d5ccf933c0478a,
            0xd598261ea65aa9ee,
            0xbd1547306f80494d,
            0x8b284e056253d057,
            0xff97a42d7f8e6fd4,
            0x90fee5a0a44647c4,
            0x8c5bda0cd6192e76,
            0xad30a6f71b19059c,
            0x30935ab7d08ffc64,
            0xeb5aa93f2317d635,
            0xa9a6e6260d712103,
            0x81a57c16dbcf555f,
            0x43b831cd0347c826,
            0x01f22f1a11a5569f,
            0x05e5635a21d9ae61,
            0x64befef28cc970f2,
            0x613670957bc46611,
            0xb87c5a554fd00ecb,
            0x8c3ee88a1ccf32c8,
            0x940c7922ae3a2614,
            0x1841f924a2c509e4,
            0x16f53526e70465c2,
            0x75f644e97f30a13b,
            0xeaf1ff7b5ceca249
        ],
        buf
    );
}
