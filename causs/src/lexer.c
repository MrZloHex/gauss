#include "lexer.h"

lexer_T *
lexer_init(char *code)
{
	lexer_T *lexer = calloc(1, sizeof(struct LEXER_STCT));
	lexer->code = code;
	lexer->i = 0;
	lexer->ch = *code;

	return 0;
}

token_T *
lexer_next_token(lexer_T *lexer)
{
	while (lexer->ch != 0)
	{
		if (isalpha(lexer->ch))
		{
			
		}
	}
}

