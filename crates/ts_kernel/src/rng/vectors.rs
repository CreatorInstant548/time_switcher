// spec: carnet architecture-securite.md §9.9 K3, K4
//! Vecteurs de référence **figés** de l'aléa à clé. Les modifier rompt la reproductibilité de
//! tous les mondes : un échec ici signale un changement de résultat, jamais une valeur à
//! recopier.

use super::*;

/// Clé de monde des vecteurs : octets 0 à 15.
fn world() -> WorldKey {
    WorldKey::from_bytes([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn sid(raw: u64) -> StableId {
    StableId::from_snapshot_raw(raw)
}

const STREAM_KEYS: [(Stream, &str); Stream::COUNT] = [
    (
        Stream::Ai,
        "cb5b525f8a3964513083deaef9b642bb18514f4f06bd00e416836dc4e844c5ca",
    ),
    (
        Stream::Combat,
        "69b7954402c6eadafa108183bc879c065b010592de01531117c70f62db906179",
    ),
    (
        Stream::Social,
        "f122bb394182eac61402ef9f78ec4139001fc0eba6468f2317c4a2238e50a887",
    ),
    (
        Stream::Economy,
        "a50962c4811b05203e348775e5a2ced4bddb7aa0a13cbe4e407623fbe26f6409",
    ),
    (
        Stream::Population,
        "a3d739ce0f8c49bd026fe3bff808c19499eb09f33b246afe6b67d1b06ddb492d",
    ),
    (
        Stream::Weather,
        "67ec1539eddcca975e3e4ab3a03f99d07d394c5aa8a456be6a72a0156ed40341",
    ),
    (
        Stream::CityGen,
        "1e46fcf9a7e423cca064ccaded6f5302cdd15d6b736a7574ff5a31f5c48ccd54",
    ),
    (
        Stream::CityServer,
        "33d4bf1f878c5f8070c64796fcf47421f79cc178f14518d608303033bf7745eb",
    ),
    (
        Stream::PopGen,
        "255000fa2d4a64784c8ccace5a09979e03d42e0200f2e41c9f5f72779782e3c7",
    ),
    (
        Stream::PopHidden,
        "bf05a5e87507d5ba07f00519b223e7e9e366301675a790235d163539e2862f84",
    ),
    (
        Stream::Furnish,
        "ba4aae30e428d50148135a101748c1d6f68bcece417d5704bec8a9b73ff44198",
    ),
    (
        Stream::Identification,
        "d3cfcfeec51ed89c61d7b26fe7c0d6205be07c88c2cd5672e90b26331cf79e3b",
    ),
];

const COSMETIC_ROOT: &str = "c0327f8d1c8de30502cd037d81f31c26028b31c35ac91b55a992c328b3b602c6";

#[test]
fn stream_keys_and_cosmetic_root_are_frozen() {
    let keys = StreamKeys::derive(&world());
    for (stream, expected) in STREAM_KEYS {
        assert_eq!(hex(&keys.get(stream).0), expected, "{stream:?}");
    }
    assert_eq!(hex(&keys.cosmetic.0), COSMETIC_ROOT);
}

#[test]
fn draws_are_frozen() {
    let keys = StreamKeys::derive(&world());
    let map5 = StableId::map(5).unwrap();
    assert_eq!(
        draw(keys.get(Stream::Combat), sid(0), 0, 0),
        0xe050_ee39_310f_ecb0
    );
    assert_eq!(
        draw(keys.get(Stream::Ai), map5, 123_456_789, 7),
        0x88c9_7b68_9cb8_8369
    );
    assert_eq!(
        draw(
            keys.get(Stream::Identification),
            sid(u64::MAX),
            u64::MAX,
            u32::MAX
        ),
        0x920c_fe4e_c439_3e22
    );
}

#[test]
fn blocks_are_frozen() {
    let keys = StreamKeys::derive(&world());
    let key = keys.get(Stream::Economy);
    let expected: [(u16, [u64; 8]); 3] = [
        (
            0,
            [
                0x0090_6898_d2dd_3468,
                0x12c8_a7df_eba1_dc5c,
                0x82f4_27af_7382_de78,
                0x2b33_1669_905f_380e,
                0xa6dc_810a_6219_00ef,
                0xcacc_a99b_ecb8_ce2c,
                0x91aa_b169_2e31_b51a,
                0x4d7f_a291_639f_dfd4,
            ],
        ),
        (
            1,
            [
                0xcff2_a9c9_1bf2_420e,
                0x378c_a7d7_1b6d_45ab,
                0x984c_bb0c_0bf8_6f97,
                0xf2aa_00d0_fcf3_8235,
                0x4323_67c8_c328_f4a2,
                0xf961_56bd_2846_f7f0,
                0xaaad_7127_4c39_bf54,
                0x21c1_98fc_98f6_056d,
            ],
        ),
        (
            u16::MAX,
            [
                0x3d62_a433_eafd_9d6c,
                0x1cc9_c972_bfbc_4744,
                0x5ad2_6093_dc7f_2f40,
                0x3f31_b922_d22a_f817,
                0x50fb_2d3d_0fe9_d496,
                0x277f_c404_4dae_95bd,
                0xc1c1_8952_0f40_f2c2,
                0x69a2_0c19_01e8_846b,
            ],
        ),
    ];
    for (block, values) in expected {
        assert_eq!(
            draw_block_at(key, sid(42), 1, 2, block),
            values,
            "bloc {block}"
        );
    }
    assert_eq!(draw_block(key, sid(42), 1, 2), expected[0].1);
}

#[test]
fn cosmetic_seeds_are_frozen() {
    let keys = StreamKeys::derive(&world());
    let map3 = StableId::map(3).unwrap();
    assert_eq!(
        keys.cosmetic_seed("hair", sid(1), 0).value(),
        0x05e2_bf93_dc88_fcae
    );
    assert_eq!(
        keys.cosmetic_seed("voice", map3, 9).value(),
        0x1a82_a3e5_880e_57ef
    );
}

#[test]
fn insecure_test_key_is_frozen() {
    let keys = StreamKeys::derive(&WorldKey::insecure_from_u64_for_tests(1));
    assert_eq!(
        draw(keys.get(Stream::Weather), sid(0), 0, 0),
        0x7763_3b5d_eef8_0a8d
    );
}
