// This file is part of the rust-pgn-tokenizer library
// MODIFIED BY DATAWATER
//
// Copyright (C) 2017 Lakin Wecker <lakin@wecker.ca>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

//------------------------------------------------------------------------------
// Parsers for the SAN specification
//------------------------------------------------------------------------------

use nom::{
    error::{ErrorKind, ParseError},
    Err::{Error, Failure, Incomplete},
    IResult,
};
use std::fmt;

///-----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Token<'a> {
    Move(&'a [u8]),
    NullMove(&'a [u8]),
    EscapeComment(&'a [u8]),
    NAG(&'a [u8]),
    MoveAnnotation(&'a [u8]),
    Result(&'a [u8]),
    Commentary(&'a [u8]),
    TagSymbol(&'a [u8]),
    TagString(&'a [u8]),
    StartVariation(&'a [u8]),
    EndVariation(&'a [u8]),
    // Number, is black
    MoveNumber(u16, bool),
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        match *self {
            Token::Move(x) => write!(f, "Move({})", String::from_utf8_lossy(x)),
            Token::NullMove(x) => write!(f, "NullMove({})", String::from_utf8_lossy(x)),
            Token::EscapeComment(x) => write!(f, "EscapeComment({})", String::from_utf8_lossy(x)),
            Token::NAG(x) => write!(f, "NAG({})", String::from_utf8_lossy(x)),
            Token::MoveAnnotation(x) => write!(f, "MoveAnnotation({})", String::from_utf8_lossy(x)),
            Token::Result(x) => write!(f, "Result({})", String::from_utf8_lossy(x)),
            Token::Commentary(x) => write!(f, "Commentary({})", String::from_utf8_lossy(x)),
            Token::TagSymbol(x) => write!(f, "TagSymbol({})", String::from_utf8_lossy(x)),
            Token::TagString(x) => write!(f, "TagString({})", String::from_utf8_lossy(x)),
            Token::StartVariation(x) => write!(f, "StartVariation({})", String::from_utf8_lossy(x)),
            Token::EndVariation(x) => write!(f, "EndVariation({})", String::from_utf8_lossy(x)),
            Token::MoveNumber(x, is_black) => write!(f, "MoveNumber({x}, {is_black})"),
        }
    }
}

// TODO: Figure out better error handling
#[derive(Debug, PartialEq)]
pub enum PgnError<I> {
    SanPawnMoveInvalid,
    SanPieceMoveInvalid,
    UciPieceMoveInvalid,
    SanCastlesInvalid,
    SanNullMoveInvalid,
    SanEmptyInput,
    SanInvalidCharacter,
    PgnIntegerEmpty,
    PgnIntegerInvalid,
    PgnStringEmpty,
    PgnStringInvalid,
    PgnStringInvalidEscapeSequence,
    PgnStringTooLarge,
    PgnEscapeCommentEmpty,
    PgnEscapeCommentInvalid,
    PgnNagEmpty,
    PgnNagInvalid,
    PgnSymbolEmpty,
    PgnSymbolInvalid,
    PgnGameResultEmpty,
    PgnGameResultInvalid,
    PgnCommentaryEmpty,
    PgnCommentaryInvalid,
    PgnCommentaryTooLarge,
    PgnTagPairEmpty,
    PgnTagPairInvalid,
    PgnTagPairInvalidSymbol,
    PgnTagPairInvalidString,
    PgnMoveNumberEmpty,
    PgnMoveNumberInvalid,
    PgnStartVariationEmpty,
    PgnStartVariationInvalid,
    PgnEndVariationEmpty,
    PgnEndVariationInvalid,
    PgnMoveAnnotationEmpty,
    PgnMoveAnnotationInvalid,
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for PgnError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        PgnError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

pub fn is_0(i: u8) -> bool {
    i == b'0'
}
pub fn is_1(i: u8) -> bool {
    i == b'1'
}
pub fn is_2(i: u8) -> bool {
    i == b'2'
}
pub fn is_capture(i: u8) -> bool {
    i == b'x'
}
pub fn is_dash(i: u8) -> bool {
    i == b'-'
}
pub fn is_digit(i: u8) -> bool {
    i >= b'0' && i <= b'9'
}
pub fn is_equals(i: u8) -> bool {
    i == b'='
}
pub fn is_file(i: u8) -> bool {
    i >= b'a' && i <= b'h'
}
pub fn is_letter(i: u8) -> bool {
    is_lowercase_letter(i) || is_uppercase_letter(i)
}
pub fn is_lowercase_letter(i: u8) -> bool {
    i >= b'a' && i <= b'z'
}
pub fn is_o(i: u8) -> bool {
    i == b'O'
}
pub fn is_period(i: u8) -> bool {
    i == b'.'
}
pub fn is_piece(i: u8) -> bool {
    i == b'R' || i == b'N' || i == b'B' || i == b'Q' || i == b'K'
}
pub fn is_plus_or_hash(i: u8) -> bool {
    i == b'#' || i == b'+'
}
pub fn is_rank(i: u8) -> bool {
    i >= b'1' && i <= b'8'
}
pub fn is_slash(i: u8) -> bool {
    i == b'/'
}
pub fn is_space(i: u8) -> bool {
    i == b' '
}
pub fn is_star(i: u8) -> bool {
    i == b'*'
}
pub fn is_uppercase_letter(i: u8) -> bool {
    i >= b'A' && i <= b'Z'
}
pub fn is_whitespace(i: u8) -> bool {
    i == b' ' || i == b'\n' || i == b'\r' || i == b'\t'
}
pub fn is_zero(i: u8) -> bool {
    i == b'0'
}

macro_rules! match_character {
    ($name:ident, $($matcher:ident),+) => {
        fn $name (incoming:&[u8]) -> Option<usize> {
            let mut i: usize = 0;
            $(
                {
                    if incoming.len() <= i || !$matcher(incoming[i]) {
                        return None
                    }
                    i += 1
                }
            )*;
            Some(i)
        }
    };
}

match_character![check, is_plus_or_hash];
match_character![pawn_capture, is_capture, is_file, is_rank];
match_character![pawn_move, is_rank];
match_character![promotion, is_equals, is_piece];

// e4 dxe4 e8=Q dxe8=Q
pub fn san_pawn_move(i: &[u8]) -> IResult<&[u8], Token, PgnError<&str>> {
    let rest = &i[1..];
    let result = pawn_capture(rest)
        .or_else(|| pawn_move(rest))
        .and_then(|length| {
            promotion(&rest[length..])
                .or_else(|| Some(0))
                .and_then(|l2| {
                    let length = length + l2;
                    check(&rest[length..])
                        .or_else(|| Some(0))
                        .and_then(|l3| Some(length + l3))
                })
        });
    match result {
        Some(length) => return Ok((&i[length + 1..], Token::Move(&i[0..length + 1]))),
        None => return Err(Error(PgnError::SanPawnMoveInvalid)),
    }
}

// Ng1xf3
match_character![
    piece_capture_with_rank_and_file,
    is_file,
    is_rank,
    is_capture,
    is_file,
    is_rank
];
// N1xf3
match_character![
    piece_capture_with_rank,
    is_rank,
    is_capture,
    is_file,
    is_rank
];
// Ngxf3
match_character![
    piece_capture_with_file,
    is_file,
    is_capture,
    is_file,
    is_rank
];
// Nxf3
match_character![piece_capture, is_capture, is_file, is_rank];
// Ng1f3
match_character![
    piece_move_with_rank_and_file,
    is_file,
    is_rank,
    is_file,
    is_rank
];
// N1f3
match_character![piece_move_with_rank, is_rank, is_file, is_rank];
// Ngf3
match_character![piece_move_with_file, is_file, is_file, is_rank];
// Nf3
match_character![piece_move, is_file, is_rank];

pub fn san_piece_move(i: &[u8]) -> IResult<&[u8], Token, PgnError<&str>> {
    let rest = &i[1..];
    let result = piece_capture_with_rank_and_file(rest)
        .or_else(|| piece_capture_with_rank(rest))
        .or_else(|| piece_capture_with_file(rest))
        .or_else(|| piece_capture(rest))
        .or_else(|| piece_move_with_rank_and_file(rest))
        .or_else(|| piece_move_with_rank(rest))
        .or_else(|| piece_move_with_file(rest))
        .or_else(|| piece_move(rest))
        .and_then(|length| {
            check(&rest[length..])
                .or_else(|| Some(0))
                .and_then(|l2| Some(length + l2))
        });
    match result {
        Some(length) => return Ok((&i[length + 1..], Token::Move(&i[0..length + 1]))),
        None => return Err(Error(PgnError::SanPieceMoveInvalid)),
    }
}

pub fn uci_piece_move(i: &[u8]) -> IResult<&[u8], Token, PgnError<&str>> {
    let result = piece_move_with_rank_and_file(i).and_then(|length| {
        if length < i.len() {
            match i[length] {
                b'p' | b'n' | b'b' | b'r' | b'q' => Some(length + 1),
                _ => Some(length),
            }
        } else {
            Some(length)
        }
    });

    match result {
        Some(length) => return Ok((&i[length..], Token::Move(&i[0..length]))),
        None => return Err(Error(PgnError::UciPieceMoveInvalid)),
    }
}

match_character![king_side_castles, is_dash, is_o];
match_character![queen_side_castles, is_dash, is_o, is_dash, is_o];

pub fn san_castles(i: &[u8]) -> IResult<&[u8], Token, PgnError<&str>> {
    let rest = &i[1..];
    let result = queen_side_castles(rest)
        .or_else(|| king_side_castles(rest))
        .and_then(|length| {
            return check(&rest[length..])
                .or_else(|| Some(0))
                .and_then(|extra_length| Some(length + extra_length));
        });
    match result {
        Some(length) => return Ok((&i[length + 1..], Token::Move(&i[0..length + 1]))),
        None => return Err(Error(PgnError::SanCastlesInvalid)),
    }
}

// Z0
match_character![null_move_z0, is_zero];
// --
match_character![null_move_dash_dash, is_dash];

pub fn san_null_move(i: &[u8]) -> IResult<&[u8], Token, PgnError<&str>> {
    let rest = &i[1..];
    let result = null_move_dash_dash(rest)
        .or_else(|| null_move_z0(rest))
        .and_then(|length| {
            return check(&rest[length..])
                .or_else(|| Some(0))
                .and_then(|extra_length| Some(length + extra_length));
        });
    match result {
        Some(length) => return Ok((&i[length + 1..], Token::NullMove(&i[0..length + 1]))),
        None => return Err(Error(PgnError::SanNullMoveInvalid)),
    }
}

pub fn san_move_token(i: &[u8]) -> IResult<&[u8], Token, PgnError<&str>> {
    if i.len() < 1 {
        return Err(Error(PgnError::SanEmptyInput));
    }
    match i[0] {
        b'R' | b'N' | b'B' | b'Q' | b'K' => san_piece_move(i),
        b'a'..=b'h' => san_pawn_move(i),
        b'O' => san_castles(i),
        b'-' | b'Z' => san_null_move(i),
        _ => Err(Error(PgnError::SanInvalidCharacter)),
    }
}

// Delimited by quote: ASCII 34
// \\ -> \
// \" -> "
// \t and \n not allowed
// max of 255 length
// printable characters, ASCII [32-126]
//
// Results still include \" and \\
//
// TODO: This does _not_ deal with utf-8
const MAX_LENGTH: usize = 255;
pub fn pgn_string(i: &[u8]) -> IResult<&[u8], &[u8], PgnError<&str>> {
    if i.len() < 1 {
        return Err(Error(PgnError::PgnStringEmpty));
    }
    let mut prev = i[0];
    if prev != b'"' {
        return Err(Error(PgnError::PgnStringInvalid));
    }
    let mut length = 1;
    while length < i.len() {
        let mut cur = i[length];
        if cur == b'"' && prev != b'\\' {
            break;
        } else if prev == b'\\' && (cur != b'\\' && cur != b'"') {
            return Err(Error(PgnError::PgnStringInvalidEscapeSequence));
        } else if prev == b'\\' && cur == b'\\' {
            cur = b' '; // fake that this isn't a \, because for escaping purposes it isn'
        }
        prev = cur;
        length += 1;
        if length > MAX_LENGTH {
            return Err(Error(PgnError::PgnStringTooLarge));
        }
    }
    // Ensure we skip over the quotes
    Ok((&i[length + 1..], &i[1..length]))
}

pub fn pgn_integer(i: &[u8]) -> IResult<&[u8], &[u8], PgnError<&str>> {
    if i.len() < 1 {
        return Err(Error(PgnError::PgnIntegerEmpty));
    }
    let mut length = 0;
    while length < i.len() && i[length] >= b'0' && i[length] <= b'9' {
        length += 1
    }
    if length == 0 {
        return Err(Error(PgnError::PgnIntegerInvalid));
    } else {
        Ok((&i[length..], &i[0..length]))
    }
}

pub fn pgn_escape_comment_token(i: &[u8]) -> IResult<&[u8], Token, PgnError<&str>> {
    if i.len() < 1 {
        return Err(Error(PgnError::PgnEscapeCommentEmpty));
    }
    if i[0] != b'%' {
        return Err(Error(PgnError::PgnEscapeCommentInvalid));
    }
    let mut length = 1;
    while length < i.len() && i[length] != b'\r' && i[length] != b'\n' {
        length += 1
    }
    Ok((&i[length..], Token::EscapeComment(&i[1..length])))
}

const MAX_MOVE_ANNOTATION_LENGTH: usize = 3;
pub fn pgn_move_annotation_token(i: &[u8]) -> IResult<&[u8], Token, PgnError<&str>> {
    if i.len() < 1 {
        return Err(Error(PgnError::PgnMoveAnnotationEmpty));
    }
    if i[0] != b'?' && i[0] != b'!' {
        return Err(Error(PgnError::PgnMoveAnnotationInvalid));
    }
    let mut length = 1;
    while length < i.len()
        && length <= MAX_MOVE_ANNOTATION_LENGTH
        && (i[length] == b'?' || i[length] == b'!')
    {
        length += 1;
    }
    Ok((&i[length..], Token::MoveAnnotation(&i[0..length])))
}

pub fn pgn_nag_token(i: &[u8]) -> IResult<&[u8], Token, PgnError<&str>> {
    if i.len() < 1 {
        return Err(Error(PgnError::PgnNagEmpty));
    }
    if i[0] != b'$' {
        return Err(Error(PgnError::PgnNagInvalid));
    }
    match pgn_integer(&i[1..]) {
        Ok((_left, integer)) => Ok((
            &i[integer.len() + 1..],
            Token::NAG(&i[1..integer.len() + 1]),
        )),
        Err(Incomplete(x)) => Err(Incomplete(x)),
        _ => Err(Error(PgnError::PgnNagInvalid)),
    }
}

pub fn pgn_symbol(i: &[u8]) -> IResult<&[u8], &[u8], PgnError<&str>> {
    if i.len() < 1 {
        return Err(Error(PgnError::PgnSymbolEmpty));
    }
    if !(is_digit(i[0]) || is_letter(i[0])) {
        return Err(Error(PgnError::PgnSymbolInvalid));
    }
    let mut length = 1;
    while length < i.len()
        && (is_digit(i[length])
            || is_letter(i[length])
            || is_plus_or_hash(i[length])
            || is_equals(i[length])
            || i[length] == b':'
            || i[length] == b'_'
            || i[length] == b'-')
    {
        length += 1
    }
    Ok((&i[length..], &i[0..length]))
}

match_character![game_ongoing, is_star];
match_character![game_white_win, is_1, is_dash, is_0];
match_character![game_black_win, is_0, is_dash, is_1];
match_character![game_draw, is_1, is_slash, is_2, is_dash, is_1, is_slash, is_2];

pub fn pgn_game_result_token(i: &[u8]) -> IResult<&[u8], Token, PgnError<&str>> {
    if i.len() < 1 {
        return Err(Error(PgnError::PgnGameResultEmpty));
    }

    let result = game_ongoing(i)
        .or_else(|| game_white_win(i))
        .or_else(|| game_black_win(i))
        .or_else(|| game_draw(i));
    match result {
        Some(length) => return Ok((&i[length..], Token::Result(&i[0..length]))),
        None => return Err(Error(PgnError::PgnGameResultInvalid)),
    }
}

// TODO: ; type comments

// This is somewhat arbitrarily picked to be 2MB. A single commentary token
// can't exceed that. We need to set _some_ sort of limit, this seems reasonable
const MAX_COMMENTARY_LENGTH: usize = 2097152;
pub fn pgn_commentary_token(i: &[u8]) -> IResult<&[u8], Token, PgnError<&str>> {
    if i.len() < 1 {
        return Err(Error(PgnError::PgnCommentaryEmpty));
    }
    if i[0] != b'{' {
        return Err(Error(PgnError::PgnCommentaryInvalid));
    }
    let mut length = 1;
    while length < i.len() && i[length] != b'}' {
        length += 1;
        if length > MAX_COMMENTARY_LENGTH {
            return Err(Error(PgnError::PgnCommentaryTooLarge));
        }
    }
    // Ensure we skip over the braces.
    Ok((&i[length + 1..], Token::Commentary(&i[1..length])))
}

pub fn remove_whitespace(i: &[u8]) -> &[u8] {
    let mut length = 0;
    while length < i.len() && is_whitespace(i[length]) {
        length += 1;
    }
    &i[length..]
}

pub fn get_spaces(i: &[u8]) -> &[u8] {
    let mut length = 0;
    while length < i.len() && is_space(i[length]) {
        length += 1;
    }
    &i[0..length]
}

pub fn pgn_tag_symbol_token(i: &[u8]) -> IResult<&[u8], Token, PgnError<&str>> {
    if i.len() < 1 {
        return Err(Error(PgnError::PgnTagPairEmpty));
    }
    if i[0] != b'[' {
        return Err(Error(PgnError::PgnTagPairInvalid));
    }
    let i = remove_whitespace(&i[1..]);
    match pgn_symbol(i) {
        Ok((i, symbol)) => {
            return Ok((&i[0..], Token::TagSymbol(symbol)));
        }
        Err(Incomplete(x)) => Err(Incomplete(x)),
        _ => Err(Error(PgnError::PgnTagPairInvalidSymbol)),
    }
}

pub fn pgn_tag_string_token(i: &[u8]) -> IResult<&[u8], Token, PgnError<&str>> {
    if i.len() < 1 {
        return Err(Error(PgnError::PgnTagPairEmpty));
    }
    let i = remove_whitespace(&i[0..]);
    match pgn_string(i) {
        Ok((i, string)) => {
            let i = remove_whitespace(i);
            let x = 0;
            if x < i.len() && i[x] == b']' {
                return Ok((&i[1..], Token::TagString(string)));
            } else {
                return Err(Error(PgnError::PgnTagPairInvalid));
            }
        }
        Err(Incomplete(x)) => Err(Incomplete(x)),
        _ => Err(Error(PgnError::PgnTagPairInvalidString)),
    }
}

match_character![one_period, is_period];
match_character![two_periods, is_period, is_period];
match_character![three_periods, is_period, is_period, is_period];

pub fn pgn_move_number(i: &[u8]) -> IResult<&[u8], Token, PgnError<&str>> {
    use std::str;

    if i.len() < 1 {
        return Err(Error(PgnError::PgnMoveNumberEmpty));
    }
    let result = pgn_integer(i);
    match result {
        Ok((left, integer)) => {
            let ws = get_spaces(left);
            let left = &left[ws.len()..];
            let result = three_periods(left)
                .or_else(|| two_periods(left))
                .or_else(|| one_period(left))
                .or_else(|| Some(0))
                .and_then(|periods_length| Some(integer.len() + ws.len() + periods_length));

            match result {
                Some(length) => {
                    let n =
                        str::parse::<u16>(str::from_utf8(&i[0..integer.len()]).unwrap()).unwrap();
                    let len = length - integer.len();
                    let x = if len < 2 { false } else { true };

                    return Ok((&i[length..], Token::MoveNumber(n, x)));
                }

                None => {
                    return Ok((
                        &i[integer.len()..],
                        Token::MoveNumber(
                            str::parse::<u16>(str::from_utf8(&i[0..integer.len()]).unwrap())
                                .unwrap(),
                            false,
                        ),
                    ))
                }
            }
        }
        Err(Incomplete(x)) => Err(Incomplete(x)),
        _ => Err(Error(PgnError::PgnMoveNumberInvalid)),
    }
}

pub fn pgn_start_variation_token(i: &[u8]) -> IResult<&[u8], Token, PgnError<&str>> {
    if i.len() < 1 {
        return Err(Error(PgnError::PgnStartVariationEmpty));
    }
    if i[0] == b'(' {
        return Ok((&i[1..], Token::StartVariation(&i[0..1])));
    } else {
        return Err(Error(PgnError::PgnStartVariationInvalid));
    }
}

pub fn pgn_end_variation_token(i: &[u8]) -> IResult<&[u8], Token, PgnError<&str>> {
    if i.len() < 1 {
        return Err(Error(PgnError::PgnEndVariationEmpty));
    }
    if i[0] == b')' {
        return Ok((&i[1..], Token::EndVariation(&i[0..1])));
    } else {
        return Err(Error(PgnError::PgnEndVariationInvalid));
    }
}

pub fn or_else<I, O, E, Op>(res: IResult<I, O, E>, op: Op) -> IResult<I, O, E>
where
    Op: FnOnce() -> IResult<I, O, E>,
{
    match res {
        Ok((i, o)) => Ok((i, o)),
        Err(Incomplete(_)) => op(),
        Err(Error(_)) => op(),
        Err(Failure(_)) => op(),
    }
}

// A simple PGN token stream. Operates on a byte slice, and streams
// byte slices of the form Token::
pub struct PGNTokenIterator<'a> {
    bytes: &'a [u8],
}

impl<'a> PGNTokenIterator<'a> {
    pub fn new(bytes: &'a [u8]) -> PGNTokenIterator<'a> {
        PGNTokenIterator { bytes: bytes }
    }
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl<'a> Iterator for PGNTokenIterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        let i = self.bytes;
        let i = remove_whitespace(i);
        let mut result = pgn_escape_comment_token(i);
        result = or_else(result, || pgn_game_result_token(i));
        // result = or_else(result, || match pgn_move_number(i) {
        //     Ok((left, _)) => {
        //         let left = remove_whitespace(left);
        //         san_move_token(left)
        //     }
        //     Err(Incomplete(x)) => Err(Incomplete(x)),
        //     Err(Error(e)) => Err(Error(e)),
        //     Err(Failure(e)) => Err(Failure(e)),
        // });
        result = or_else(result, || pgn_tag_symbol_token(i));
        result = or_else(result, || pgn_tag_string_token(i));
        result = or_else(result, || pgn_start_variation_token(i));
        result = or_else(result, || pgn_end_variation_token(i));
        result = or_else(result, || pgn_commentary_token(i));
        result = or_else(result, || pgn_nag_token(i));
        result = or_else(result, || pgn_move_annotation_token(i));
        result = or_else(result, || san_move_token(i));
        result = or_else(result, || pgn_move_number(i));
        match result {
            Ok((i, token)) => {
                self.bytes = i;
                Some(token.clone())
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
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
        assert_eq!(Ok((&b""[..], Token::MoveNumber(1, false))), pgn_move_number(b"1"));
        assert_eq!(Ok((&b""[..], Token::MoveNumber(2, false))), pgn_move_number(b"2."));
        assert_eq!(Ok((&b""[..], Token::MoveNumber(49, true))), pgn_move_number(b"49..."));
        assert_eq!(Ok((&b"."[..], Token::MoveNumber(3, true))), pgn_move_number(b"3...."));
        assert_eq!(Ok((&b"."[..], Token::MoveNumber(3, true))), pgn_move_number(b"3 ...."));
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
        assert_eq!(results[0], Token::TagSymbol(b"Event"));
        assert_eq!(results[1], Token::TagString(b"World Senior Teams +50"));
        assert_eq!(results[2], Token::TagSymbol(b"Site"));
        assert_eq!(results[3], Token::TagString(b"Radebeul GER"));
        assert_eq!(results[4], Token::TagSymbol(b"Date"));
        assert_eq!(results[5], Token::TagString(b"2016.07.03"));
        assert_eq!(results[6], Token::TagSymbol(b"Round"));
        assert_eq!(results[7], Token::TagString(b"8.2"));

        let last = results.len() - 1;
        assert_eq!(results[last], Token::Result(b"1-0"));
        assert_eq!(results[last - 1], Token::Move(b"Qh1+"));
        assert_eq!(results[last - 2], Token::Move(b"Ke1"));
        assert_eq!(results[last - 3], Token::NullMove(b"Z0"));
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
        assert_eq!(results[0], Token::TagSymbol(b"Event"));
        assert_eq!(results[1], Token::TagString(b"Rated Blitz game"));
        assert_eq!(results[2], Token::TagSymbol(b"Site"));
        assert_eq!(
            results[3],
            Token::TagString(b"https://lichess.org/oUDzbB2j")
        );

        let last = results.len() - 1;
        assert_eq!(results[last], Token::Result(b"1-0"));
        assert_eq!(results[last - 1], Token::Move(b"Qb7#"));
        assert_eq!(results[last - 2], Token::Commentary(b" [%eval #1] "));
        assert_eq!(results[last - 3], Token::MoveAnnotation(b"?"));
        assert_eq!(results[last - 4], Token::Move(b"Qd8"));
    }
}
