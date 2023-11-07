#include "parser/parser.h"
#include <string.h>


Parser
parser_init(char *line, TransUnit_Type tut)
{
	Parser ps =
	{
		.line = line,
		.line_len = strlen(line),
		.next_tok = 0,
		.tut = tut
	};
	return ps;
}

static size_t
parser_get_word(Parser *ps)
{
	size_t word_len = 0;
	char ch = ps->line[ps->next_tok];
	while (!IS_SEPARATOR(ch))
	{ ch = ps->line[ps->next_tok + ++word_len]; }
	return word_len;
}


TokenResult
parser_next_token(Parser *ps)
{
	char fst_tok_ch;
redo:

	if (ps->next_tok >= ps->line_len || ps->line[ps->next_tok] == 0)
	{ TOKEN_ERR(TOK_EOL); }
	fst_tok_ch = ps->line[ps->next_tok];

	if (IS_CH_EMPTY(fst_tok_ch))
	{
		ps->next_tok++;
		goto redo;
	}

	// ONE CHAR LEN TOKENS
	ps->next_tok += 1;
	if (IS_COMMENT(fst_tok_ch))
	{	TOKEN_OK_SYM(SYM_SEMICOLON);	}
	else if (fst_tok_ch == SYM_COLON)
	{	TOKEN_OK_SYM(SYM_COLON);		}
	else if (fst_tok_ch == SYM_PIPE)
	{	TOKEN_OK_SYM(SYM_PIPE);			}
	else if (fst_tok_ch == SYM_O_SQ_BRACK)
	{	TOKEN_OK_SYM(SYM_O_SQ_BRACK);	}
	else if (fst_tok_ch == SYM_C_SQ_BRACK)
	{	TOKEN_OK_SYM(SYM_C_SQ_BRACK);	}
	else if (fst_tok_ch == SYM_EQUAL)
	{	TOKEN_OK_SYM(SYM_EQUAL);		}
	else if (fst_tok_ch == SYM_BACKSLASH)
	{	TOKEN_OK_SYM(SYM_BACKSLASH);	}
	else if (fst_tok_ch == SYM_UNDESCORE)
	{	TOKEN_OK_SYM(SYM_UNDESCORE);	}
	else if (fst_tok_ch == SYM_PLUS)
	{	TOKEN_OK_SYM(SYM_PLUS);			}
	else if (fst_tok_ch == SYM_MINUS)
	{	TOKEN_OK_SYM(SYM_MINUS);		}
	else if (fst_tok_ch == SYM_GREATER)
	{	TOKEN_OK_SYM(SYM_GREATER);		}
	else if (fst_tok_ch == SYM_LESS)
	{	TOKEN_OK_SYM(SYM_LESS);			}
	else if (fst_tok_ch == SYM_HASH)
	{	TOKEN_OK_SYM(SYM_HASH);			}

	ps->next_tok -= 1;

	size_t wrd_len = parser_get_word(ps);
	char *wrd = ps->line + ps->next_tok;
	ps->next_tok += wrd_len;

	if (IS_KEYWORD(wrd, wrd_len, KW_DECLARE))
	{   TOKEN_OK_KW(KW_DECLARE);    }
	else if (IS_KEYWORD(wrd, wrd_len, KW_BYTE))
	{   TOKEN_OK_KW(KW_BYTE);   }
	else if (IS_KEYWORD(wrd, wrd_len, KW_WORD))
	{   TOKEN_OK_KW(KW_WORD);   }
	else if (IS_KEYWORD(wrd, wrd_len, KW_DWORD))
	{   TOKEN_OK_KW(KW_DWORD);   }
	else if (IS_KEYWORD(wrd, wrd_len, KW_QWORD))
	{   TOKEN_OK_KW(KW_QWORD);   }
	else if (IS_KEYWORD(wrd, wrd_len, KW_SIGNED))
	{   TOKEN_OK_KW(KW_SIGNED);   }
	else if (IS_KEYWORD(wrd, wrd_len, KW_SYSCALL))
	{   TOKEN_OK_KW(KW_SYSCALL);   }
	else if (IS_KEYWORD(wrd, wrd_len, KW_RET))
	{   TOKEN_OK_KW(KW_RET);   }
	else if (IS_KEYWORD(wrd, wrd_len, KW_BREAK))
	{   TOKEN_OK_KW(KW_BREAK);   }
	else if (IS_KEYWORD(wrd, wrd_len, KW_IF))
	{   TOKEN_OK_KW(KW_IF);   }
	else if (IS_KEYWORD(wrd, wrd_len, KW_THEN))
	{   TOKEN_OK_KW(KW_THEN);   }
	else if (IS_KEYWORD(wrd, wrd_len, KW_LOOP))
	{   TOKEN_OK_KW(KW_LOOP);   }
	else
	{	TOKEN_OK_IDENT(wrd, wrd_len);	}


	TOKEN_ERR(TOK_EOL);

}