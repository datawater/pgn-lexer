# PGN Tokenizer
## Goals
- Use zero-copy byte slices all the way through.
- No heap allocations

## API
- A token based iterator that takes a byte slices and returns tokens that reference sub-slices
