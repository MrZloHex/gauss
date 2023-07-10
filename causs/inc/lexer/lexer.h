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


Lexer
lexer_init(char *in);

Lexeme *
lexer_lex(Lexer *lx); // Parse file to lexemes

#endif /* __LEXER_LEXER_H__ */