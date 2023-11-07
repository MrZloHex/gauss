#ifndef __PARSER_TOKENS_H__
#define __PARSER_TOKENS_H__

#define IS_CH_EMPTY(__CH__) (__CH__ == ' ' || __CH__ == '\t')

#define IS_SEPARATOR(__CH__) (IS_CH_EMPTY(__CH__) || IS_COMMENT(__CH__) || __CH__ == ':')

typedef enum Symbol_E
{
	SYM_SEMICOLON   = ';',
	SYM_COLON       = ':',
	SYM_PIPE		= '|',
	SYM_O_SQ_BRACK	= '[',
	SYM_C_SQ_BRACK	= ']',
	SYM_EQUAL		= '=',
	SYM_BACKSLASH	= '\\',
	SYM_UNDESCORE	= '_',
	SYM_PLUS		= '+',
	SYM_MINUS		= '-',
	SYM_GREATER		= '>',
	SYM_LESS		= '<',
	SYM_HASH		= '#'
} Symbol;

#define IS_COMMENT(__CH__)  (__CH__ == SYM_SEMICOLON)

typedef enum Keywords_E
{
	// VAR SIZES
	KW_BYTE         = 0x0U,
	KW_WORD         = 0x1U,
	KW_DWORD        = 0x2U,
	KW_QWORD        = 0x3U,

	KW_SIGNED		= 0x4U,

	KW_DECLARE,

	KW_SYSCALL,
	KW_RET,
	KW_BREAK,
	KW_IF,
	KW_THEN,
	KW_LOOP
} Keyword;

const static char *k_keywords[] =
{
	"BYTE", "WORD", "DOWRD", "QWORD", "SIGNED",
	"DECLARE",
	"SYSCALL", "RET", "BREAK", "IF", "THEN", "LOOP"
};

#define IS_KEYWORD(__STR__, __LEN__, __KW__) (strncmp(__STR__, k_keywords[__KW__], __LEN__) == 0)

typedef struct
{
	char *id;
	size_t id_len;
} Identifier;

typedef union Token_U
{
	Keyword kw;
	Symbol sym;
	Identifier id;
} Token;

typedef enum
{
	TOK_SYMBOL,
	TOK_KEYWORD,
	TOK_IDENTIFIER
} TokenType;

typedef enum
{
	TOK_OK,
	TOK_EOL
} TokenError;

typedef struct
{
	Token token;
	TokenType type;
	TokenError error;
} TokenResult;


#include <stdio.h>

#define __TITLE_TR_FMT printf("`\tLINE\tTYPE\tERROR\tTOKEN\n")

static void
__print_tr(TokenResult tr)
{
	if (tr.type == TOK_SYMBOL)
		printf("SYM\t%u\t%c\n", tr.error, tr.token.sym);
	else if (tr.type == TOK_KEYWORD)
		printf("KEYWRD\t%u\t%s\n", tr.error, k_keywords[tr.token.kw]);
	else if (tr.type == TOK_IDENTIFIER)
		printf("IDENT\t%u\t%.*s\n", tr.error, tr.token.id.id_len, tr.token.id.id);
	
}

#define TOKEN_OK_KW(__KW__)     TokenResult __tr = { .token = { .kw = __KW__ }, .error = TOK_OK, .type = TOK_KEYWORD }; \
								return __tr;

#define TOKEN_OK_SYM(__SYM__)   TokenResult __tr = { .token = { .sym = __SYM__ }, .error = TOK_OK, .type = TOK_SYMBOL }; \
								return __tr;

#define TOKEN_OK_IDENT(__PTR__,__LEN__)     TokenResult __tr = { .token = { .id = { .id = __PTR__, .id_len = __LEN__}}, .error = TOK_OK, .type = TOK_IDENTIFIER }; \
											return __tr;

#define TOKEN_ERR(__ERR__)      TokenResult __tok_res = { .error = __ERR__, .type = TOK_SYMBOL }; \
								return __tok_res;


#endif /* __PARSER_TOKENS_H__ */