#ifndef __PARSER_TOKENS_H__
#define __PARSER_TOKENS_H__

#define IS_CH_EMPTY(__CH__) (__CH__ == ' ' || __CH__ == '\t')

#define IS_SEPARATOR(__CH__) (IS_CH_EMPTY(__CH__) || IS_COMMENT(__CH__) || __CH__ == ':')

typedef enum Symbol_E
{
    SYM_SEMICOLON   = ';',
    SYM_COLO        = ':'
} Symbol;

#define IS_COMMENT(__CH__)  (__CH__ == SYM_SEMICOLON)

typedef enum Keywords_E
{
    // VAR SIZES
    KW_BYTE         = 0x0U,
    KW_WORD         = 0x1U,
    KW_DWORD        = 0x2U,
    KW_QWORD        = 0x3U,


    KW_DECLARE
} Keyword;

#define IS_KEYWORD(__STR__, __LEN__, __KW__) (strncmp(__STR__, k_keywords[__KW__], __LEN__) == 0)

typedef union Token_U
{
    Keyword kw;
    Symbol sym;
} Token;

typedef enum
{
    TOK_SYMBOL,
    TOK_KEYWORD
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

#define __TITLE_TR_FMT printf("\tLINE\tTOKEN\tTYPE\tERROR\n")
#define TR_FMT "%u\t%u\t%u"
#define TR_FMT_TR(__TR__) (__TR__.type == TOK_SYMBOL ? __TR__.token.sym : __TR__.token.kw), __TR__.type, __TR__.error

#define TOKEN_OK_KW(__KW__)     TokenResult __tr = { .token = { .kw = __KW__ }, .error = TOK_OK, .type = TOK_KEYWORD }; \
                                return __tr;

#define TOKEN_OK_SYM(__SYM__)   TokenResult __tr = { .token = { .sym = __SYM__ }, .error = TOK_OK, .type = TOK_SYMBOL }; \
                                return __tr;

#define TOKEN_ERR(__ERR__)      TokenResult __tok_res = { .error = __ERR__ }; \
                                return __tok_res;


#endif /* __PARSER_TOKENS_H__ */