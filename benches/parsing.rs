use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pgn_lexer::parser::*;

pub fn bench_parse_san_move_null(c: &mut Criterion) {
    c.bench_function("san_move_token_null", |b| {
        b.iter(|| {
            assert_eq!(
                Ok((&b""[..], Token::NullMove(b"--#"))),
                san_move_token(black_box(b"--#"))
            )
        })
    });
}

pub fn bench_parse_san_move_castle_queen_side(c: &mut Criterion) {
    c.bench_function("san_move_castle_queen_side", |b| {
        b.iter(|| {
            assert_eq!(
                Ok((&b""[..], Token::Move(b"O-O-O"))),
                san_move_token(b"O-O-O")
            );
        })
    });
}

pub fn bench_parse_san_move_castle_king_side(c: &mut Criterion) {
    c.bench_function("san_move_castle_king_side", |b| {
        b.iter(|| {
            assert_eq!(Ok((&b""[..], Token::Move(b"O-O"))), san_move_token(b"O-O"));
        })
    });
}

pub fn bench_parse_san_move_simple_capture(c: &mut Criterion) {
    c.bench_function("san_move_simple_capture", |b| {
        b.iter(|| {
            assert_eq!(
                Ok((&b""[..], Token::Move(b"bxc2"))),
                san_move_token(b"bxc2")
            );
        })
    });
}

pub fn bench_parse_san_move_simple(c: &mut Criterion) {
    c.bench_function("san_move_simple", |b| {
        b.iter(|| {
            assert_eq!(Ok((&b""[..], Token::Move(b"e4"))), san_move_token(b"e4"));
        })
    });
}

pub fn bench_parse_san_capture_promotion(c: &mut Criterion) {
    c.bench_function("san_capture_promotion", |b| {
        b.iter(|| {
            assert_eq!(
                Ok((&b""[..], Token::Move(b"bxc1=R"))),
                san_move_token(b"bxc1=R")
            );
        })
    });
}

pub fn bench_parse_san_move_complicated(c: &mut Criterion) {
    c.bench_function("san_move_complicated", |b| {
        b.iter(|| {
            assert_eq!(
                Ok((&b""[..], Token::Move(b"bxc1=R+"))),
                san_move_token(b"bxc1=R+")
            );
        })
    });
}

pub fn bench_parse_game(c: &mut Criterion) {
    c.bench_function("parse_game", |b| {
        b.iter(|| {
            let results = PGNTokenIterator::new(
                &b"[Event \"World Senior Teams +50\"]
    [Site \"Radebeul GER\"]
    [Date \"2016.07.03\"]
    [Round \"8.2\"]
    [White \"Anastasian, A.\"]
    [Black \"Lewis, An\"]
    [Result \"1-0\"]
    [ECO \"E90\"]
    [WhiteElo \"2532\"]
    [BlackElo \"2269\"]
    [PlyCount \"84\"]
    [EventDate \"2016.06.26\"]

    1. d4 Nf6 2. c4 g6 3. Nc3 Bg7 4. e4 d6 5. Nf3 O-O 6. h3 e5 7. d5 Na6 8. Be3 Nh5
    9. Nh2 Qe8 10. Be2 Nf4 11. Bf3 f5 12. a3 Nc5 13. Bxc5 dxc5 14. O-O Qe7 15. Re1
    a6 16. Ne2 Qd6 17. Nf1 Bd7 18. Rb1 b6 19. Nd2 Bh6 20. Nxf4 Bxf4 21. b4 Rae8 22.
    Qc2 Rf6 23. Qc3 Qf8 24. Nb3 cxb4 25. axb4 Bg5 26. Rb2 Rf7 27. Nc1 Qh6 28. Nd3
    fxe4 29. Bxe4 Bxh3 30. gxh3 Qxh3 31. Bg2 Qh4 32. Re4 Qh5 33. Rbe2 Ref8 34. c5
    Bf4 35. Nxe5 Qh2+ 36. Kf1 Rf5 37. Nf3 Qh5 38. Re7 Bh6 39. R2e5 bxc5 40. bxc5
    Rxf3 41. Bxf3 Z0 42. Ke1 Qh1+ 1-0"[..],
            );
            // 24 tag tokens
            // 42 full moves (84 tokens)
            // 1 result
            assert_eq!(results.count(), 24 + 84 + 1);
        })
    });
}

static TWIC_1544: &[u8] = include_bytes!("../data/twic1544.pgn");

pub fn bench_iterator_next(c: &mut Criterion) {
    let parsed = PGNTokenIterator::new(TWIC_1544);

    c.bench_function("token_iterator_next", |b| {
        b.iter(|| {
            let mut parsed = parsed.clone();

            while parsed.next().is_some() {}
        })
    });
}

criterion_group!(
    benches,
    bench_parse_san_move_null,
    bench_parse_san_move_castle_queen_side,
    bench_parse_san_move_castle_king_side,
    bench_parse_san_move_simple_capture,
    bench_parse_san_move_simple,
    bench_parse_san_capture_promotion,
    bench_parse_san_move_complicated,
    bench_iterator_next
);
criterion_main!(benches);
