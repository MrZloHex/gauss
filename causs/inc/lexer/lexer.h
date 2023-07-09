#ifndef __LEXER_LEXER_H__
#define __LEXER_LEXER_H__

#include "lexer/lexemes.h"
#include <stdlib.h>
#include "translation_unit.h"

typedef struct Lexer_S
{
    TransUnit_Type tut;
    size_t q_in_lines;
    char **in_lines;
} Lexer;


Lexeme *
lex(Lexer *lx); // Parse file to lexemes

#endif /* __LEXER_LEXER_H__ */