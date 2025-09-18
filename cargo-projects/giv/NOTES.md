- Whitespace
- Interpreted and compiled languages 
    - Interpreted - execute line by line by the interpreter immediately without translating to machine code.
    - Compiled - translate programming code into machine code to be executed by the computer. Compilation (lexical analysis, syntax analysis, semantic analysis, code generation and optimisation)
- define "eat", "eating" in lexing
- All of the different lexemes can be recognised using regular expressions
- Lex and Flex tools
- Should be able to handle arbitrary identifiers
- No whitespace tokens (for our use case)
- Why regular expression vs scanner
- regular language - 
- LL and LR parsing techniques - 
    - LL (left-to-right, leftmost derivation) parses input from [L]eft to right performing [L]eftmost derivation of the sentence
    - LR (left-to-right, rightmost derivation in reverse) parses input from [L]eft to right performing [R]ightmost derivation of the sentence
- Symbols - 

## Scanning / Lexing

- 

## Parsing

- [LL and LR Parsing Demystified](https://blog.reverberate.org/2013/07/ll-and-lr-parsing-demystified.html)

## Rust-specific

- Should we try to use &str or String for lexemes?

## Resources

- [Lexical Analysis and Regular Expressions](https://www.cs.cornell.edu/courses/cs4120/2022sp/notes.html?id=lexing)
