#ifndef GAUSS_LEXER_H_
#define GAUSS_LEXER_H_

#include <stdlib.h>
#include <ctype.h>

#include "token.h"

typedef struct LEXER_STCT
{
	char *code;
	char ch;
	size_t i;
} lexer_T;

lexer_T *
lexer_init(char *code);

void
lexer_advance(lexer_T* lexer);

token_T *
lexer_next_token(lexer_T *lexer);

#endif /* GAUSS_LEXER_H_ */
