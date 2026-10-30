use ts_kernel::StableId;

use crate::candidates::{CandidateKind, CandidateSpot};
use crate::public::{PublicMap, PublicPlace};
use crate::server::{ServerPart, ServerPlacement};
use crate::*;

fn cm(v: i32) -> Cm {
    Cm::new(v).unwrap()
}

fn pt(x: i32, y: i32) -> Point2 {
    Point2::new(cm(x), cm(y))
}

fn square() -> Polygon2 {
    Polygon2::new(vec![pt(0, 0), pt(100, 0), pt(100, 100), pt(0, 100)]).unwrap()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// --- Cm and quantize ---------------------------------------------------------------------

#[test]
fn cm_range_is_enforced_at_both_ends() {
    assert_eq!(Cm::new(MAX_ABS_CM), Ok(Cm::MAX));
    assert_eq!(Cm::new(-MAX_ABS_CM), Ok(Cm::MIN));
    assert_eq!(Cm::new(MAX_ABS_CM + 1), Err(GeometryError::OutOfRange));
    assert_eq!(Cm::new(-MAX_ABS_CM - 1), Err(GeometryError::OutOfRange));
    assert_eq!(Cm::new(i32::MAX), Err(GeometryError::OutOfRange));
    assert_eq!(Cm::new(i32::MIN), Err(GeometryError::OutOfRange));
}

#[test]
fn cm_arithmetic_is_checked() {
    assert_eq!(Cm::MAX.checked_add(Cm::ZERO), Ok(Cm::MAX));
    assert_eq!(Cm::MAX.checked_add(cm(1)), Err(GeometryError::OutOfRange));
    assert_eq!(Cm::MIN.checked_sub(cm(1)), Err(GeometryError::OutOfRange));
    assert_eq!(Cm::MAX.checked_sub(Cm::MIN), Err(GeometryError::OutOfRange));
    assert_eq!(Cm::MIN.negated(), Cm::MAX);
    assert_eq!(cm(5).wide(), 5_i64);
}

#[test]
fn quantize_rounds_halves_to_even() {
    // Every input below is exactly representable, so the product is an exact half.
    for (metres, expected) in [
        (0.125, 12),
        (0.375, 38),
        (0.625, 62),
        (0.875, 88),
        (-0.125, -12),
        (-0.375, -38),
        (-0.625, -62),
        (-0.875, -88),
        (0.0, 0),
        (-0.0, 0),
        (0.01, 1),
        (1.0, 100),
        (-2.5, -250),
    ] {
        assert_eq!(quantize(metres).unwrap().get(), expected, "{metres} m");
    }
}

#[test]
fn quantize_rounds_off_the_half_to_nearest() {
    assert_eq!(quantize(0.1249).unwrap().get(), 12);
    assert_eq!(quantize(0.1251).unwrap().get(), 13);
    assert_eq!(quantize(-0.1251).unwrap().get(), -13);
}

#[test]
fn quantize_pins_halves_that_are_not_representable() {
    // 1.005 is stored as 1.00499999999999989…, and 1.005 × 100.0 evaluates to
    // 100.49999999999999 in IEEE 754: it is not a tie, it rounds down. People expect 101.
    assert_eq!(1.005_f64 * 100.0, 100.49999999999999);
    assert_eq!(quantize(1.005).unwrap().get(), 100);
    // 0.005 is stored slightly above 0.005, but the product rounds to exactly 0.5, which is a
    // tie and goes to the even neighbour, 0 (not 1).
    assert_eq!(0.005_f64 * 100.0, 0.5);
    assert_eq!(quantize(0.005).unwrap().get(), 0);
    assert_eq!(quantize(-0.005).unwrap().get(), 0);
}

#[test]
fn quantize_accepts_the_bounds_and_refuses_beyond() {
    assert_eq!(quantize(100_000.0), Ok(Cm::MAX));
    assert_eq!(quantize(-100_000.0), Ok(Cm::MIN));
    assert_eq!(quantize(100_000.01), Err(GeometryError::OutOfRange));
    assert_eq!(quantize(-100_000.01), Err(GeometryError::OutOfRange));
    assert_eq!(quantize(1e300), Err(GeometryError::OutOfRange));
    assert_eq!(quantize(f64::MAX), Err(GeometryError::OutOfRange));
}

#[test]
fn quantize_refuses_non_finite_values() {
    assert_eq!(quantize(f64::NAN), Err(GeometryError::NotFinite));
    assert_eq!(quantize(f64::INFINITY), Err(GeometryError::NotFinite));
    assert_eq!(quantize(f64::NEG_INFINITY), Err(GeometryError::NotFinite));
}

// --- N1: widths ----------------------------------------------------------------------------

#[test]
fn products_at_the_bounds_fit_in_i64() {
    let max = i64::from(MAX_ABS_CM);
    let diag = Vec2::new(Cm::MAX, Cm::MAX);
    assert_eq!(diag.dot(diag), 2 * max * max);
    assert_eq!(diag.length_squared(), 2 * 100_000_000_000_000);
    assert_eq!(
        Vec2::new(Cm::MAX, Cm::ZERO).cross(Vec2::new(Cm::ZERO, Cm::MAX)),
        max * max
    );

    let wide = Segment2::new(pt(-MAX_ABS_CM, -MAX_ABS_CM), pt(MAX_ABS_CM, MAX_ABS_CM)).unwrap();
    assert_eq!(wide.length_squared(), 8 * 100_000_000_000_000);

    let all = Aabb2::new(Point2::new(Cm::MIN, Cm::MIN), Point2::new(Cm::MAX, Cm::MAX)).unwrap();
    assert_eq!(all.width(), 2 * max);
    assert_eq!(all.area(), 4 * max * max);

    let h = HeightRange::new(Cm::MIN, Cm::MAX).unwrap();
    assert_eq!(h.extent(), 2 * max);
}

#[test]
fn vectors_between_far_points_are_refused() {
    let a = Point2::new(Cm::MIN, Cm::ZERO);
    let b = Point2::new(Cm::MAX, Cm::ZERO);
    assert_eq!(a.to(b), Err(GeometryError::OutOfRange));
    assert_eq!(a.to(a), Ok(Vec2::default()));
    assert_eq!(
        b.translated(Vec2::new(cm(1), Cm::ZERO)),
        Err(GeometryError::OutOfRange)
    );
    assert_eq!(pt(1, 2).translated(Vec2::new(cm(3), cm(4))), Ok(pt(4, 6)));
}

#[test]
fn polygon_area_sum_exceeds_i64_and_stays_exact() {
    // A ring wound ~15,000 times around the full square: every shoelace term is below 2×10^14,
    // but their sum is above `i64::MAX`.
    let m = MAX_ABS_CM;
    let mut ring = Vec::new();
    for j in 0..60_000_i32 {
        let k = j / 4;
        ring.push(match j % 4 {
            0 => pt(m - k, -m),
            1 => pt(m, m - k),
            2 => pt(-m + k, m),
            _ => pt(-m, -m + k),
        });
    }
    let poly = Polygon2::new(ring).unwrap();
    assert!(poly.twice_area() > i128::from(i64::MAX));
}

// --- Shapes --------------------------------------------------------------------------------

#[test]
fn aabb_rejects_inverted_bounds_and_tests_contact() {
    assert_eq!(
        Aabb2::new(pt(1, 0), pt(0, 0)),
        Err(GeometryError::InvalidBounds)
    );
    assert_eq!(
        Aabb2::new(pt(0, 1), pt(0, 0)),
        Err(GeometryError::InvalidBounds)
    );
    let a = Aabb2::new(pt(0, 0), pt(10, 10)).unwrap();
    let b = Aabb2::new(pt(10, 10), pt(20, 20)).unwrap();
    let c = Aabb2::new(pt(11, 0), pt(20, 5)).unwrap();
    assert!(a.contains(pt(10, 10)) && !a.contains(pt(11, 10)));
    assert!(a.intersects(&b) && !a.intersects(&c));
    assert_eq!(Aabb2::new(pt(3, 3), pt(3, 3)).unwrap().area(), 0);
}

#[test]
fn segment_rejects_equal_endpoints() {
    assert_eq!(
        Segment2::new(pt(1, 1), pt(1, 1)),
        Err(GeometryError::DegenerateSegment)
    );
    let s = Segment2::new(pt(3, 4), pt(0, 0)).unwrap();
    assert_eq!(s.length_squared(), 25);
    assert_eq!(s.bounds(), Aabb2::new(pt(0, 0), pt(3, 4)).unwrap());
}

#[test]
fn polygon_needs_three_distinct_vertices_with_area() {
    assert_eq!(Polygon2::new(vec![]), Err(GeometryError::TooFewVertices));
    assert_eq!(
        Polygon2::new(vec![pt(0, 0), pt(1, 0)]),
        Err(GeometryError::TooFewVertices)
    );
    assert_eq!(
        Polygon2::new(vec![pt(0, 0), pt(10, 0), pt(10, 10), pt(0, 0)]),
        Err(GeometryError::RepeatedVertex)
    );
    // Repeated, but not adjacent.
    assert_eq!(
        Polygon2::new(vec![pt(0, 0), pt(10, 0), pt(10, 10), pt(0, 10), pt(10, 0)]),
        Err(GeometryError::RepeatedVertex)
    );
    assert_eq!(
        Polygon2::new(vec![pt(0, 0), pt(5, 5), pt(10, 10)]),
        Err(GeometryError::CollinearVertices)
    );
    // A collinear vertex in the middle of a side is refused, not removed.
    assert_eq!(
        Polygon2::new(vec![
            pt(0, 0),
            pt(50, 0),
            pt(100, 0),
            pt(100, 100),
            pt(0, 100)
        ]),
        Err(GeometryError::CollinearVertices)
    );
    // A spike that doubles back is collinear too.
    assert_eq!(
        Polygon2::new(vec![
            pt(0, 0),
            pt(100, 0),
            pt(50, 0),
            pt(100, 100),
            pt(0, 100)
        ]),
        Err(GeometryError::CollinearVertices)
    );
    // Bow-tie: no collinear triple, but the two lobes cancel (documented on `ZeroArea`).
    assert_eq!(
        Polygon2::new(vec![pt(0, 0), pt(2, 2), pt(2, 0), pt(0, 2)]),
        Err(GeometryError::ZeroArea)
    );
}

#[test]
fn polygon_has_one_canonical_form() {
    let expected = canonical_bytes(&square());
    let inputs = [
        vec![pt(0, 0), pt(100, 0), pt(100, 100), pt(0, 100)],
        vec![pt(100, 0), pt(100, 100), pt(0, 100), pt(0, 0)],
        vec![pt(100, 100), pt(0, 100), pt(0, 0), pt(100, 0)],
        vec![pt(0, 100), pt(0, 0), pt(100, 0), pt(100, 100)],
        // Clockwise, from every start vertex.
        vec![pt(0, 0), pt(0, 100), pt(100, 100), pt(100, 0)],
        vec![pt(0, 100), pt(100, 100), pt(100, 0), pt(0, 0)],
        vec![pt(100, 100), pt(100, 0), pt(0, 0), pt(0, 100)],
        vec![pt(100, 0), pt(0, 0), pt(0, 100), pt(100, 100)],
    ];
    for input in inputs {
        let poly = Polygon2::new(input.clone()).unwrap();
        assert_eq!(poly, square(), "{input:?}");
        assert_eq!(canonical_bytes(&poly), expected, "{input:?}");
    }
    // The smallest vertex is the one with the smallest x, then the smallest y.
    let tri = Polygon2::new(vec![pt(5, 9), pt(5, 1), pt(7, 4)]).unwrap();
    assert_eq!(tri.vertices()[0], pt(5, 1));
}

#[test]
fn polygon_vertex_count_is_capped() {
    let many = |n: i32| {
        (0..n)
            .map(|i| pt(i, (i % 1000) * (i % 7)))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        Polygon2::new(many(Polygon2::MAX_VERTICES as i32 + 1)),
        Err(GeometryError::TooManyVertices)
    );
    // At the cap the vertex count is accepted (this ring may fail later checks, but not this one).
    assert_ne!(
        Polygon2::new(many(Polygon2::MAX_VERTICES as i32)),
        Err(GeometryError::TooManyVertices)
    );
}

#[test]
fn polygon_is_normalized_counter_clockwise() {
    let cw = Polygon2::new(vec![pt(0, 0), pt(0, 100), pt(100, 100), pt(100, 0)]).unwrap();
    assert_eq!(cw, square());
    assert_eq!(cw.vertices()[0], pt(0, 0));
    assert_eq!(cw.twice_area(), 20_000);
    assert_eq!(
        square().bounds(),
        Aabb2::new(pt(0, 0), pt(100, 100)).unwrap()
    );
}

#[test]
fn levels_and_height_ranges_validate() {
    assert_eq!(Level::floor(0), Err(GeometryError::InvalidLevel));
    assert_eq!(
        Level::floor(Level::MAX_FLOORS + 1),
        Err(GeometryError::InvalidLevel)
    );
    assert_eq!(Level::cellar(0), Err(GeometryError::InvalidLevel));
    assert_eq!(
        Level::cellar(Level::MAX_CELLARS + 1),
        Err(GeometryError::InvalidLevel)
    );
    assert_eq!(Level::floor(Level::MAX_FLOORS).unwrap().index(), 100);
    assert_eq!(Level::cellar(1).unwrap().kind(), LevelKind::Cellar);
    assert_eq!(Level::ground().index() + Level::roof().index(), 0);

    assert_eq!(
        HeightRange::new(cm(5), cm(5)),
        Err(GeometryError::InvalidBounds)
    );
    assert_eq!(
        HeightRange::new(cm(6), cm(5)),
        Err(GeometryError::InvalidBounds)
    );
    let low = HeightRange::new(cm(0), cm(300)).unwrap();
    let high = HeightRange::new(cm(300), cm(600)).unwrap();
    let mid = HeightRange::new(cm(299), cm(301)).unwrap();
    assert!(!low.overlaps(&high) && low.overlaps(&mid) && high.overlaps(&mid));
}

// --- Parts and identities (N3, N4) ------------------------------------------------------------

fn place(index: u64) -> PublicPlace {
    PublicPlace::new(
        index,
        Level::ground(),
        square(),
        HeightRange::new(cm(0), cm(300)).unwrap(),
    )
    .unwrap()
}

#[test]
fn map_entities_take_map_identifiers() {
    assert!(map_id(0).unwrap().is_map());
    assert_eq!(
        map_id(StableId::MAX_INDEX).unwrap().index(),
        StableId::MAX_INDEX
    );
    assert_eq!(
        map_id(StableId::MAX_INDEX + 1),
        Err(GeometryError::MapIndexOutOfRange)
    );
    assert_eq!(map_id(u64::MAX), Err(GeometryError::MapIndexOutOfRange));

    assert!(place(7).id().is_map());
    let spot = CandidateSpot::new(9, CandidateKind::Camera, Level::ground(), pt(1, 1)).unwrap();
    assert!(spot.id().is_map());
    assert!(ServerPlacement::new(9).unwrap().candidate().is_map());
    assert_eq!(
        ServerPlacement::new(u64::MAX),
        Err(GeometryError::MapIndexOutOfRange)
    );
    assert_eq!(
        CandidateSpot::new(1 << 63, CandidateKind::Vault, Level::ground(), pt(0, 0)),
        Err(GeometryError::MapIndexOutOfRange)
    );
}

#[test]
fn public_map_sorts_places_and_refuses_duplicates() {
    let map = PublicMap::new(vec![place(5), place(2), place(9)]).unwrap();
    let indices: Vec<u64> = map.places().iter().map(|p| p.id().index()).collect();
    assert_eq!(indices, [2, 5, 9]);
    assert_eq!(
        PublicMap::new(vec![place(5), place(5)]),
        Err(GeometryError::DuplicateId)
    );
    assert!(PublicMap::new(vec![]).unwrap().places().is_empty());
}

#[test]
fn server_part_holds_placements() {
    let part = ServerPart::new(vec![ServerPlacement::new(3).unwrap()]);
    assert_eq!(part.placements().len(), 1);
    assert!(ServerPart::default().placements().is_empty());
}

#[test]
fn public_types_are_client_shippable() {
    fn ship<T: ClientShippable>() {}
    ship::<PublicPlace>();
    ship::<PublicMap>();
    ship::<CandidateSpot>();
}

// --- Canonical encoding: reference vectors ------------------------------------------------------
//
// These strings freeze the encoding. A change here changes every map fingerprint: update them
// only on purpose, in a PR that says so.

#[test]
fn canonical_vectors_for_scalars_and_shapes() {
    assert_eq!(hex(&canonical_bytes(&cm(100))), "0164000000");
    assert_eq!(hex(&canonical_bytes(&cm(-1))), "01ffffffff");
    assert_eq!(hex(&canonical_bytes(&pt(1, -1))), "0201000000ffffffff");
    assert_eq!(
        hex(&canonical_bytes(&Vec2::new(cm(2), cm(3)))),
        "030200000003000000"
    );
    assert_eq!(
        hex(&canonical_bytes(&Aabb2::new(pt(0, 0), pt(10, 20)).unwrap())),
        concat!("04", "0000000000000000", "0a00000014000000")
    );
    assert_eq!(
        hex(&canonical_bytes(
            &Segment2::new(pt(0, 0), pt(0, 5)).unwrap()
        )),
        concat!("05", "0000000000000000", "0000000005000000")
    );
    assert_eq!(hex(&canonical_bytes(&Level::floor(2).unwrap())), "070102");
    assert_eq!(hex(&canonical_bytes(&Level::roof())), "070300");
    assert_eq!(
        hex(&canonical_bytes(&HeightRange::new(cm(0), cm(300)).unwrap())),
        "08000000002c010000"
    );
}

#[test]
fn canonical_vector_for_a_polygon() {
    assert_eq!(
        hex(&canonical_bytes(&square())),
        concat!(
            "06",
            "04000000",
            "0000000000000000",
            "6400000000000000",
            "6400000064000000",
            "0000000064000000",
        )
    );
}

#[test]
fn canonical_vectors_for_parts() {
    let expected_place = concat!(
        "10",
        "0500000000000080", // id: bit 63 + index 5
        "070000",           // ground
        "06",
        "04000000",
        "0000000000000000",
        "6400000000000000",
        "6400000064000000",
        "0000000064000000",
        "08",
        "00000000",
        "2c010000",
    );
    assert_eq!(hex(&canonical_bytes(&place(5))), expected_place);

    let map = PublicMap::new(vec![place(5)]).unwrap();
    assert_eq!(
        hex(&canonical_bytes(&map)),
        format!("1101000000{expected_place}")
    );

    let spot =
        CandidateSpot::new(1, CandidateKind::Cache, Level::cellar(1).unwrap(), pt(2, 3)).unwrap();
    assert_eq!(
        hex(&canonical_bytes(&spot)),
        concat!("20", "0100000000000080", "02", "070201", "0200000003000000")
    );
}

#[test]
fn canonical_encoding_separates_types_and_orders() {
    assert_ne!(
        canonical_bytes(&Vec2::new(cm(1), cm(2))),
        canonical_bytes(&pt(1, 2))
    );
    assert_ne!(canonical_bytes(&pt(1, 2)), canonical_bytes(&pt(2, 1)));
    let a = PublicMap::new(vec![place(1), place(2)]).unwrap();
    let b = PublicMap::new(vec![place(2), place(1)]).unwrap();
    assert_eq!(canonical_bytes(&a), canonical_bytes(&b));
}
