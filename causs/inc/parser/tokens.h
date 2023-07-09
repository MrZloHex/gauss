#ifndef __PARSER_TOKENS_H__
#define __PARSER_TOKENS_H__

#define IS_CH_EMPTY(__CH__) (__CH__ == ' ' || __CH__ == '\t')
#define IS_COMMENT(__CH__)  (__CH__ == ';')

#define IS_SEPARATOR(__CH__) (IS_CH_EMPTY(__CH__) || IS_COMMENT(__CH__) || __CH__ == ':')

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
} Token;

typedef enum
{
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

#define TOKEN_OK_KW(__KW__)     TokenResult __tr = { .token = { .kw = __KW__ }, .error = TOK_OK, .type = TOK_KEYWORD }; \
                                return __tr;

#define TOKEN_ERR(__ERR__)      TokenResult __tok_res = { .error = __ERR__ }; \
                                return __tok_res;


#endif /* __PARSER_TOKENS_H__ */