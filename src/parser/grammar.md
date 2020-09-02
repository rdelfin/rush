# Grammar

This document defines the grammar used by the lexer and parser in BNF.

First off, the lexical definitions:
```ebnf
argument    = { letter | digit | "-" | "_" | "." | "/"}
whitespaces = { whitespace };

// Base definitons
letter      = "A" | "B" | "C" | "D" | "E" | "F" | "G"
            | "H" | "I" | "J" | "K" | "L" | "M" | "N"
            | "O" | "P" | "Q" | "R" | "S" | "T" | "U"
            | "V" | "W" | "X" | "Y" | "Z" | "a" | "b"
            | "c" | "d" | "e" | "f" | "g" | "h" | "i"
            | "j" | "k" | "l" | "m" | "n" | "o" | "p"
            | "q" | "r" | "s" | "t" | "u" | "v" | "w"
            | "x" | "y" | "z" ;
digit       = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
whitespace  = ? white_space characters ? ;
```

Second off, the "grammar"
```enf
command     = { argument };
```
