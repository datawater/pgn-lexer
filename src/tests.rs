#[cfg(test)]
mod tests {
    use crate::parser::*;

    #[test]
    fn test_san_move_pawn_single_square() {
        assert_eq!(
            Ok((&b""[..], Token::Move("a1".into()))),
            san_move_token(b"a1")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("a8".into()))),
            san_move_token(b"a8")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("h1".into()))),
            san_move_token(b"h1")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("h8".into()))),
            san_move_token(b"h8")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("e4".into()))),
            san_move_token(b"e4")
        );
        assert_eq!(
            Ok((&b" e5"[..], Token::Move("e4".into()))),
            san_move_token(b"e4 e5")
        );
        assert_eq!(
            Ok((&b"!! e5"[..], Token::Move("e4".into()))),
            san_move_token(b"e4!! e5")
        );
        assert_eq!(
            Ok((&b"!? e5"[..], Token::Move("e4".into()))),
            san_move_token(b"e4!? e5")
        );
        assert_eq!(
            Ok((&b"!? e5"[..], Token::Move("e4".into()))),
            san_move_token(b"e4!? e5")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("e4=N".into()))),
            san_move_token(b"e4=N")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("e4=N#".into()))),
            san_move_token(b"e4=N#")
        );
    }

    #[test]
    fn test_san_pawn_capture() {
        assert_eq!(
            Ok((&b""[..], Token::Move("bxc1".into()))),
            san_move_token(b"bxc1")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("axe4".into()))),
            san_move_token(b"axe4")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("bxc1+".into()))),
            san_move_token(b"bxc1+")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("bxc1=R+".into()))),
            san_move_token(b"bxc1=R+")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("bxc1=R#".into()))),
            san_move_token(b"bxc1=R#")
        );
    }

    #[test]
    fn test_san_move_piece_move() {
        assert_eq!(
            Ok((&b""[..], Token::Move("Nf3".into()))),
            san_move_token(b"Nf3")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("Nf6".into()))),
            san_move_token(b"Nf6")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("Nf6+".into()))),
            san_move_token(b"Nf6+")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("N2f6".into()))),
            san_move_token(b"N2f6")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("N2f6+".into()))),
            san_move_token(b"N2f6+")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("Ng1f3".into()))),
            san_move_token(b"Ng1f3")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("Ng1f3+".into()))),
            san_move_token(b"Ng1f3+")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("Rd3d1".into()))),
            san_move_token(b"Rd3d1")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("Rd3d1+".into()))),
            san_move_token(b"Rd3d1+")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("Rd3d1#".into()))),
            san_move_token(b"Rd3d1#")
        );
    }

    #[test]
    fn test_uci_piece_move() {
        assert_eq!(
            Ok((&b""[..], Token::Move("g1f3".into()))),
            uci_piece_move(b"g1f3")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("d3d1".into()))),
            uci_piece_move(b"d3d1")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("d7d8q".into()))),
            uci_piece_move(b"d7d8q")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("e7d8q".into()))),
            uci_piece_move(b"e7d8q")
        );
    }

    #[test]
    fn test_san_move_piece_capture() {
        assert_eq!(
            Ok((&b""[..], Token::Move("Nxf3".into()))),
            san_move_token(b"Nxf3")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("Nxf6".into()))),
            san_move_token(b"Nxf6")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("Nxf6+".into()))),
            san_move_token(b"Nxf6+")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("N2xf6".into()))),
            san_move_token(b"N2xf6")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("N2xf6+".into()))),
            san_move_token(b"N2xf6+")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("Ng1xf3".into()))),
            san_move_token(b"Ng1xf3")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("Ng1xf3+".into()))),
            san_move_token(b"Ng1xf3+")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("Rd3xd1".into()))),
            san_move_token(b"Rd3xd1")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("Rd3xd1+".into()))),
            san_move_token(b"Rd3xd1+")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("Rd3xd1#".into()))),
            san_move_token(b"Rd3xd1#")
        );
    }

    #[test]
    fn test_castles() {
        assert_eq!(
            Ok((&b""[..], Token::Move("O-O".into()))),
            san_move_token(b"O-O")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("O-O-O".into()))),
            san_move_token(b"O-O-O")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("O-O+".into()))),
            san_move_token(b"O-O+")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("O-O#".into()))),
            san_move_token(b"O-O#")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("O-O-O+".into()))),
            san_move_token(b"O-O-O+")
        );
        assert_eq!(
            Ok((&b""[..], Token::Move("O-O-O#".into()))),
            san_move_token(b"O-O-O#")
        );
    }
    #[test]
    fn test_null_move() {
        assert_eq!(
            Ok((&b""[..], Token::NullMove("--".into()))),
            san_move_token(b"--")
        );
        assert_eq!(
            Ok((&b""[..], Token::NullMove("--+".into()))),
            san_move_token(b"--+")
        );
        assert_eq!(
            Ok((&b""[..], Token::NullMove("--#".into()))),
            san_move_token(b"--#")
        );
        assert_eq!(
            Ok((&b""[..], Token::NullMove("Z0".into()))),
            san_move_token(b"Z0")
        );
        assert_eq!(
            Ok((&b""[..], Token::NullMove("Z0+".into()))),
            san_move_token(b"Z0+")
        );
        assert_eq!(
            Ok((&b""[..], Token::NullMove("Z0#".into()))),
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
            Ok((&b"\n"[..], Token::EscapeComment("1234".into()))),
            pgn_escape_comment_token(b"%1234\n")
        );
        assert_eq!(
            Ok((&b"\n"[..], Token::EscapeComment("%234".into()))),
            pgn_escape_comment_token(b"%%234\n")
        );
        assert_eq!(
            Ok((&b"\r"[..], Token::EscapeComment("% 234".into()))),
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
            Ok((&b""[..], Token::MoveAnnotation("!".into()))),
            pgn_move_annotation_token(b"!")
        );
        assert_eq!(
            Ok((&b""[..], Token::MoveAnnotation("?".into()))),
            pgn_move_annotation_token(b"?")
        );
        assert_eq!(
            Ok((&b""[..], Token::MoveAnnotation("!!".into()))),
            pgn_move_annotation_token(b"!!")
        );
        assert_eq!(
            Ok((&b""[..], Token::MoveAnnotation("??".into()))),
            pgn_move_annotation_token(b"??")
        );
        assert_eq!(
            Ok((&b""[..], Token::MoveAnnotation("?!".into()))),
            pgn_move_annotation_token(b"?!")
        );
        assert_eq!(
            Ok((&b""[..], Token::MoveAnnotation("!?".into()))),
            pgn_move_annotation_token(b"!?")
        );
        assert_eq!(
            Ok((&b" e4"[..], Token::MoveAnnotation("!?".into()))),
            pgn_move_annotation_token(b"!? e4")
        );
        assert_eq!(
            Ok((&b" e4"[..], Token::MoveAnnotation("!??".into()))),
            pgn_move_annotation_token(b"!?? e4")
        );
    }

    #[test]
    fn test_pgn_nag_token() {
        assert_eq!(Err(Error(PgnError::PgnNagEmpty)), pgn_nag_token(b""));
        assert_eq!(
            Ok((&b""[..], Token::NAG("1234".into()))),
            pgn_nag_token(b"$1234")
        );
        assert_eq!(
            Ok((&b""[..], Token::NAG("234".into()))),
            pgn_nag_token(b"$234")
        );
        assert_eq!(
            Ok((&b" e4"[..], Token::NAG("234".into()))),
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
            Ok((&b""[..], Token::Result("1-0".into()))),
            pgn_game_result_token(b"1-0")
        );
        assert_eq!(
            Ok((&b""[..], Token::Result("0-1".into()))),
            pgn_game_result_token(b"0-1")
        );
        assert_eq!(
            Ok((&b""[..], Token::Result("1/2-1/2".into()))),
            pgn_game_result_token(b"1/2-1/2")
        );
        assert_eq!(
            Ok((&b""[..], Token::Result("*".into()))),
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
            Ok((&b""[..], Token::Commentary("this is a comment".into()))),
            pgn_commentary_token(b"{this is a comment}")
        );
        assert_eq!(
            Ok((&b""[..], Token::Commentary("this is a\n comment".into()))),
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
            Ok((&b" \"?\"]"[..], Token::TagSymbol("Event".into()))),
            pgn_tag_symbol_token(b"[Event \"?\"]")
        );
        assert_eq!(
            Ok((&b" \"Tony Rotella\"]"[..], Token::TagSymbol("Event".into()))),
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
            Ok((&b""[..], Token::TagString("?".into()))),
            pgn_tag_string_token(b"\"?\"]")
        );
        assert_eq!(
            Ok((&b""[..], Token::TagString("Tony Rotella".into()))),
            pgn_tag_string_token(b"\"Tony Rotella\"]")
        );
        assert_eq!(
            Ok((&b""[..], Token::TagString("2016.05.22".into()))),
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
            Ok((&b""[..], Token::StartVariation("(".into()))),
            pgn_start_variation_token(b"(")
        );
        assert_eq!(
            Ok((&b" 1."[..], Token::StartVariation("(".into()))),
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
            Ok((&b""[..], Token::EndVariation(")".into()))),
            pgn_end_variation_token(b")")
        );
        assert_eq!(
            Ok((&b" 1."[..], Token::EndVariation(")".into()))),
            pgn_end_variation_token(b") 1.")
        );
    }
    // #[test]
    fn test_pgn_game_parser_1() {
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
        let results: Vec<Token> = results.collect();
        // 24 tag tokens
        // 42 full moves (84 tokens)
        // 1 result
        assert_eq!(results.len(), 24 + 84 + 1);
        assert_eq!(results[0], Token::TagSymbol("Event".into()));
        assert_eq!(
            results[1],
            Token::TagString("World Senior Teams +50".into())
        );
        assert_eq!(results[2], Token::TagSymbol("Site".into()));
        assert_eq!(results[3], Token::TagString("Radebeul GER".into()));
        assert_eq!(results[4], Token::TagSymbol("Date".into()));
        assert_eq!(results[5], Token::TagString("2016.07.03".into()));
        assert_eq!(results[6], Token::TagSymbol("Round".into()));
        assert_eq!(results[7], Token::TagString("8.2".into()));

        let last = results.len() - 1;
        assert_eq!(results[last], Token::Result("1-0".into()));
        assert_eq!(results[last - 1], Token::Move("Qh1+".into()));
        assert_eq!(results[last - 2], Token::Move("Ke1".into()));
        assert_eq!(results[last - 3], Token::NullMove("Z0".into()));
    }

    // #[test]
    fn test_pgn_game_parser_2() {
        let results = PGNTokenIterator::new(&b"[Event \"Rated Blitz game\"]
[Site \"https://lichess.org/oUDzbB2j\"]
[White \"exhilarate\"]
[Black \"Svetlana-55\"]
[Result \"1-0\"]
[UTCDate \"2016.12.31\"]
[UTCTime \"23:00:22\"]
[WhiteElo \"1570\"]
[BlackElo \"1630\"]
[WhiteRatingDiff \"+12\"]
[BlackRatingDiff \"-12\"]
[ECO \"B00\"]
[Opening \"Owen Defense\"]
[TimeControl \"180+0\"]
[Termination \"Normal\"]

1. e4 { [%eval 0.26] } 1... b6 { [%eval 0.51] } 2. Nc3 { [%eval 0.51] } 2... Bb7 { [%eval 0.52] } 3. Nf3 { [%eval 0.24] } 3... e6 { [%eval 0.22] } 4. d4 { [%eval 0.29] } 4... d5 { [%eval 0.7] } 5. e5?! { [%eval 0.19] } 5... Ne7 { [%eval 0.29] } 6. Bb5+ { [%eval 0.36] } 6... c6 { [%eval 0.37] } 7. Bd3 { [%eval -0.01] } 7... Nd7?! { [%eval 0.64] } 8. O-O { [%eval 0.58] } 8... g6 { [%eval 0.61] } 9. Bg5 { [%eval 0.47] } 9... h6 { [%eval 0.55] } 10. Be3 { [%eval 0.52] } 10... Qc7 { [%eval 0.81] } 11. Re1 { [%eval 0.83] } 11... O-O-O { [%eval 1.14] } 12. a4 { [%eval 1.0] } 12... g5?! { [%eval 1.51] } 13. a5 { [%eval 1.54] } 13... b5 { [%eval 1.91] } 14. a6 { [%eval 1.89] } 14... Ba8 { [%eval 2.22] } 15. b3?! { [%eval 1.58] } 15... Nb6 { [%eval 1.86] } 16. Ne2 { [%eval 1.77] } 16... Nf5 { [%eval 1.67] } 17. Bd2? { [%eval 0.11] } 17... g4 { [%eval 0.17] } 18. Bxf5 { [%eval 0.24] } 18... gxf3 { [%eval 0.43] } 19. Nf4 { [%eval 0.0] } 19... exf5 { [%eval 0.0] } 20. Qxf3 { [%eval 0.0] } 20... Kb8 { [%eval 0.33] } 21. Ba5 { [%eval -0.11] } 21... Rg8 { [%eval 0.25] } 22. e6? { [%eval -1.27] } 22... fxe6?? { [%eval 2.58] } 23. Nxe6 { [%eval 2.48] } 23... Qd6 { [%eval 2.5] } 24. Nxd8 { [%eval 2.47] } 24... Qxd8 { [%eval 2.18] } 25. Bxb6 { [%eval 2.05] } 25... Qxb6?? { [%eval #13] } 26. Re8+ { [%eval #13] } 26... Kc7 { [%eval #13] } 27. Qf4+?! { [%eval 10.37] } 27... Kd7 { [%eval 28.14] } 28. Rxa8 { [%eval 23.16] } 28... Bd6?! { [%eval #5] } 29. Qxh6?? { [%eval 0.0] } 29... Rxa8 { [%eval 0.0] } 30. Qg7+?! { [%eval -0.77] } 30... Kc8?? { [%eval 8.15] } 31. Re1 { [%eval 7.62] } 31... Qd8? { [%eval #1] } 32. Qb7# 1-0"[..]);
        let results: Vec<Token> = results.collect();
        assert_eq!(results[0], Token::TagSymbol("Event".into()));
        assert_eq!(results[1], Token::TagString("Rated Blitz game".into()));
        assert_eq!(results[2], Token::TagSymbol("Site".into()));
        assert_eq!(
            results[3],
            Token::TagString("https://lichess.org/oUDzbB2j".into())
        );

        let last = results.len() - 1;
        assert_eq!(results[last], Token::Result("1-0".into()));
        assert_eq!(results[last - 1], Token::Move("Qb7#".into()));
        assert_eq!(results[last - 2], Token::Commentary(" [%eval #1] ".into()));
        assert_eq!(results[last - 3], Token::MoveAnnotation("?".into()));
        assert_eq!(results[last - 4], Token::Move("Qd8".into()));
    }
}
