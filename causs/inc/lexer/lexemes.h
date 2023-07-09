#ifndef __LEXER_LEXEMES_H__
#define __LEXER_LEXEMES_H__

typedef enum LexemeType_E
{
    LEX_Identifier,
    LEX_Keyword,
    LEX_Separator,
    LEX_Operator,
    LEX_Literal
} LexemesType;

typedef struct Lexeme_S
{
    LexemesType type;
    char *lexeme;
} Lexeme;



#endif /* __LEXER_LEXEMES_H__ */