#[allow(unused_imports)]
use nom::Err::Error;

#[cfg(test)]
use crate::parser::{Token, *};
//use test::Bencher;

#[test]
fn test_san_move_pawn_single_square() {
    assert_eq!(Ok((&b""[..], Token::Move(b"a1"))), san_move_token(b"a1"));
    assert_eq!(Ok((&b""[..], Token::Move(b"a8"))), san_move_token(b"a8"));
    assert_eq!(Ok((&b""[..], Token::Move(b"h1"))), san_move_token(b"h1"));
    assert_eq!(Ok((&b""[..], Token::Move(b"h8"))), san_move_token(b"h8"));
    assert_eq!(Ok((&b""[..], Token::Move(b"e4"))), san_move_token(b"e4"));
    assert_eq!(
        Ok((&b" e5"[..], Token::Move(b"e4"))),
        san_move_token(b"e4 e5")
    );
    assert_eq!(
        Ok((&b"!! e5"[..], Token::Move(b"e4"))),
        san_move_token(b"e4!! e5")
    );
    assert_eq!(
        Ok((&b"!? e5"[..], Token::Move(b"e4"))),
        san_move_token(b"e4!? e5")
    );
    assert_eq!(
        Ok((&b"!? e5"[..], Token::Move(b"e4"))),
        san_move_token(b"e4!? e5")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"e4=N"))),
        san_move_token(b"e4=N")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"e4=N#"))),
        san_move_token(b"e4=N#")
    );
}

#[test]
fn test_san_pawn_capture() {
    assert_eq!(
        Ok((&b""[..], Token::Move(b"bxc1"))),
        san_move_token(b"bxc1")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"axe4"))),
        san_move_token(b"axe4")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"bxc1+"))),
        san_move_token(b"bxc1+")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"bxc1=R+"))),
        san_move_token(b"bxc1=R+")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"bxc1=R#"))),
        san_move_token(b"bxc1=R#")
    );
}

#[test]
fn test_san_move_piece_move() {
    assert_eq!(Ok((&b""[..], Token::Move(b"Nf3"))), san_move_token(b"Nf3"));
    assert_eq!(Ok((&b""[..], Token::Move(b"Nf6"))), san_move_token(b"Nf6"));
    assert_eq!(
        Ok((&b""[..], Token::Move(b"Nf6+"))),
        san_move_token(b"Nf6+")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"N2f6"))),
        san_move_token(b"N2f6")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"N2f6+"))),
        san_move_token(b"N2f6+")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"Ng1f3"))),
        san_move_token(b"Ng1f3")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"Ng1f3+"))),
        san_move_token(b"Ng1f3+")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"Rd3d1"))),
        san_move_token(b"Rd3d1")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"Rd3d1+"))),
        san_move_token(b"Rd3d1+")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"Rd3d1#"))),
        san_move_token(b"Rd3d1#")
    );
}

#[test]
fn test_uci_piece_move() {
    assert_eq!(
        Ok((&b""[..], Token::Move(b"g1f3"))),
        uci_piece_move(b"g1f3")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"d3d1"))),
        uci_piece_move(b"d3d1")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"d7d8q"))),
        uci_piece_move(b"d7d8q")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"e7d8q"))),
        uci_piece_move(b"e7d8q")
    );
}

#[test]
fn test_san_move_piece_capture() {
    assert_eq!(
        Ok((&b""[..], Token::Move(b"Nxf3"))),
        san_move_token(b"Nxf3")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"Nxf6"))),
        san_move_token(b"Nxf6")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"Nxf6+"))),
        san_move_token(b"Nxf6+")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"N2xf6"))),
        san_move_token(b"N2xf6")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"N2xf6+"))),
        san_move_token(b"N2xf6+")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"Ng1xf3"))),
        san_move_token(b"Ng1xf3")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"Ng1xf3+"))),
        san_move_token(b"Ng1xf3+")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"Rd3xd1"))),
        san_move_token(b"Rd3xd1")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"Rd3xd1+"))),
        san_move_token(b"Rd3xd1+")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"Rd3xd1#"))),
        san_move_token(b"Rd3xd1#")
    );
}

#[test]
fn test_castles() {
    assert_eq!(Ok((&b""[..], Token::Move(b"O-O"))), san_move_token(b"O-O"));
    assert_eq!(
        Ok((&b""[..], Token::Move(b"O-O-O"))),
        san_move_token(b"O-O-O")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"O-O+"))),
        san_move_token(b"O-O+")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"O-O#"))),
        san_move_token(b"O-O#")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"O-O-O+"))),
        san_move_token(b"O-O-O+")
    );
    assert_eq!(
        Ok((&b""[..], Token::Move(b"O-O-O#"))),
        san_move_token(b"O-O-O#")
    );
}
#[test]
fn test_null_move() {
    assert_eq!(
        Ok((&b""[..], Token::NullMove(b"--"))),
        san_move_token(b"--")
    );
    assert_eq!(
        Ok((&b""[..], Token::NullMove(b"--+"))),
        san_move_token(b"--+")
    );
    assert_eq!(
        Ok((&b""[..], Token::NullMove(b"--#"))),
        san_move_token(b"--#")
    );
    assert_eq!(
        Ok((&b""[..], Token::NullMove(b"Z0"))),
        san_move_token(b"Z0")
    );
    assert_eq!(
        Ok((&b""[..], Token::NullMove(b"Z0+"))),
        san_move_token(b"Z0+")
    );
    assert_eq!(
        Ok((&b""[..], Token::NullMove(b"Z0#"))),
        san_move_token(b"Z0#")
    );
}

#[test]
fn test_pgn_integer() {
    assert_eq!(Ok((&b""[..], &b"99"[..])), pgn_integer(b"99"));
    assert_eq!(Ok((&b" e4"[..], &b"99"[..])), pgn_integer(b"99 e4"));
    assert_eq!(Ok((&b"..."[..], &b"99"[..])), pgn_integer(b"99..."));
    assert_eq!(Err(Error(PgnError::PgnIntegerEmpty)), pgn_integer(b""));
}

#[test]
fn test_pgn_string_token() {
    assert_eq!(Err(Error(PgnError::PgnStringEmpty)), pgn_string(b""));
    assert_eq!(Ok((&b""[..], &b"aaaaaaa"[..])), pgn_string(b"\"aaaaaaa\""));
    assert_eq!(
        Ok((&b""[..], &b"aaaaaaa \\\" aaaaaaa"[..])),
        pgn_string(b"\"aaaaaaa \\\" aaaaaaa\"")
    );
    assert_eq!(
        Ok((&b""[..], &b"GER/CCM-E/01-C (GER)"[..])),
        pgn_string(b"\"GER/CCM-E/01-C (GER)\"")
    );
    assert_eq!(
        Ok((&b" aaaaaaaa"[..], &b"aaaaaaa \\\\"[..])),
        pgn_string(b"\"aaaaaaa \\\\\" aaaaaaaa")
    );
}

#[test]
fn test_pgn_escape_comment_token() {
    assert_eq!(
        Err(Error(PgnError::PgnEscapeCommentEmpty)),
        pgn_escape_comment_token(b"")
    );
    assert_eq!(
        Ok((&b"\n"[..], Token::EscapeComment(b"1234"))),
        pgn_escape_comment_token(b"%1234\n")
    );
    assert_eq!(
        Ok((&b"\n"[..], Token::EscapeComment(b"%234"))),
        pgn_escape_comment_token(b"%%234\n")
    );
    assert_eq!(
        Ok((&b"\r"[..], Token::EscapeComment(b"% 234"))),
        pgn_escape_comment_token(b"%% 234\r")
    );
}

#[test]
fn test_pgn_move_annotation_token() {
    assert_eq!(
        Err(Error(PgnError::PgnMoveAnnotationEmpty)),
        pgn_move_annotation_token(b"")
    );
    assert_eq!(
        Ok((&b""[..], Token::MoveAnnotation(b"!"))),
        pgn_move_annotation_token(b"!")
    );
    assert_eq!(
        Ok((&b""[..], Token::MoveAnnotation(b"?"))),
        pgn_move_annotation_token(b"?")
    );
    assert_eq!(
        Ok((&b""[..], Token::MoveAnnotation(b"!!"))),
        pgn_move_annotation_token(b"!!")
    );
    assert_eq!(
        Ok((&b""[..], Token::MoveAnnotation(b"??"))),
        pgn_move_annotation_token(b"??")
    );
    assert_eq!(
        Ok((&b""[..], Token::MoveAnnotation(b"?!"))),
        pgn_move_annotation_token(b"?!")
    );
    assert_eq!(
        Ok((&b""[..], Token::MoveAnnotation(b"!?"))),
        pgn_move_annotation_token(b"!?")
    );
    assert_eq!(
        Ok((&b" e4"[..], Token::MoveAnnotation(b"!?"))),
        pgn_move_annotation_token(b"!? e4")
    );
    assert_eq!(
        Ok((&b" e4"[..], Token::MoveAnnotation(b"!??"))),
        pgn_move_annotation_token(b"!?? e4")
    );
}

#[test]
fn test_pgn_nag_token() {
    assert_eq!(Err(Error(PgnError::PgnNagEmpty)), pgn_nag_token(b""));
    assert_eq!(Ok((&b""[..], Token::NAG(b"1234"))), pgn_nag_token(b"$1234"));
    assert_eq!(Ok((&b""[..], Token::NAG(b"234"))), pgn_nag_token(b"$234"));
    assert_eq!(
        Ok((&b" e4"[..], Token::NAG(b"234"))),
        pgn_nag_token(b"$234 e4")
    );
}

#[test]
fn test_pgn_symbol_token() {
    assert_eq!(Err(Error(PgnError::PgnSymbolEmpty)), pgn_symbol(b""));
    assert_eq!(
        Ok((&b""[..], &b"sasd#_+#=:-"[..])),
        pgn_symbol(b"sasd#_+#=:-")
    );
    assert_eq!(
        Ok((&b"!()~{}[]"[..], &b"sasd#_+#=:-"[..])),
        pgn_symbol(b"sasd#_+#=:-!()~{}[]")
    );
}
#[test]
fn test_pgn_game_result_token() {
    assert_eq!(
        Err(Error(PgnError::PgnGameResultEmpty)),
        pgn_game_result_token(b"")
    );
    assert_eq!(
        Ok((&b""[..], Token::Result(b"1-0"))),
        pgn_game_result_token(b"1-0")
    );
    assert_eq!(
        Ok((&b""[..], Token::Result(b"0-1"))),
        pgn_game_result_token(b"0-1")
    );
    assert_eq!(
        Ok((&b""[..], Token::Result(b"1/2-1/2"))),
        pgn_game_result_token(b"1/2-1/2")
    );
    assert_eq!(
        Ok((&b""[..], Token::Result(b"*"))),
        pgn_game_result_token(b"*")
    );
}
#[test]
fn test_pgn_commentary_token() {
    assert_eq!(
        Err(Error(PgnError::PgnCommentaryEmpty)),
        pgn_commentary_token(b"")
    );
    assert_eq!(
        Ok((&b""[..], Token::Commentary(b"this is a comment"))),
        pgn_commentary_token(b"{this is a comment}")
    );
    assert_eq!(
        Ok((&b""[..], Token::Commentary(b"this is a\n comment"))),
        pgn_commentary_token(b"{this is a\n comment}")
    );
}
#[test]
fn test_pgn_tag_symbol_token() {
    assert_eq!(
        Err(Error(PgnError::PgnTagPairEmpty)),
        pgn_tag_symbol_token(b"")
    );
    assert_eq!(
        Ok((&b" \"?\"]"[..], Token::TagSymbol(b"Event"))),
        pgn_tag_symbol_token(b"[Event \"?\"]")
    );
    assert_eq!(
        Ok((&b" \"Tony Rotella\"]"[..], Token::TagSymbol(b"Event"))),
        pgn_tag_symbol_token(b"[Event \"Tony Rotella\"]")
    );
}
#[test]
fn test_pgn_tag_string_token() {
    assert_eq!(
        Err(Error(PgnError::PgnTagPairEmpty)),
        pgn_tag_string_token(b"")
    );
    assert_eq!(
        Ok((&b""[..], Token::TagString(b"?"))),
        pgn_tag_string_token(b"\"?\"]")
    );
    assert_eq!(
        Ok((&b""[..], Token::TagString(b"Tony Rotella"))),
        pgn_tag_string_token(b"\"Tony Rotella\"]")
    );
    assert_eq!(
        Ok((&b""[..], Token::TagString(b"2016.05.22"))),
        pgn_tag_string_token(b"\"2016.05.22\"]")
    );
}
#[test]
fn test_pgn_move_number_token() {
    assert_eq!(
        Err(Error(PgnError::PgnMoveNumberEmpty)),
        pgn_move_number(b"")
    );
    assert_eq!(
        Ok((&b""[..], Token::MoveNumber(1, false))),
        pgn_move_number(b"1")
    );
    assert_eq!(
        Ok((&b""[..], Token::MoveNumber(2, false))),
        pgn_move_number(b"2.")
    );
    assert_eq!(
        Ok((&b""[..], Token::MoveNumber(49, true))),
        pgn_move_number(b"49...")
    );
    assert_eq!(
        Ok((&b"."[..], Token::MoveNumber(3, true))),
        pgn_move_number(b"3....")
    );
    assert_eq!(
        Ok((&b"."[..], Token::MoveNumber(3, true))),
        pgn_move_number(b"3 ....")
    );
}
#[test]
fn test_pgn_start_variation_token() {
    assert_eq!(
        Err(Error(PgnError::PgnStartVariationEmpty)),
        pgn_start_variation_token(b"")
    );
    assert_eq!(
        Ok((&b""[..], Token::StartVariation(b"("))),
        pgn_start_variation_token(b"(")
    );
    assert_eq!(
        Ok((&b" 1."[..], Token::StartVariation(b"("))),
        pgn_start_variation_token(b"( 1.")
    );
}
#[test]
fn test_pgn_end_variation() {
    assert_eq!(
        Err(Error(PgnError::PgnEndVariationEmpty)),
        pgn_end_variation_token(b"")
    );
    assert_eq!(
        Ok((&b""[..], Token::EndVariation(b")"))),
        pgn_end_variation_token(b")")
    );
    assert_eq!(
        Ok((&b" 1."[..], Token::EndVariation(b")"))),
        pgn_end_variation_token(b") 1.")
    );
}