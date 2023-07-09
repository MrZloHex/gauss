#ifndef __PARSER_PARSER_H__
#define __PARSER_PARSER_H__

#include "translation_unit.h"
#include "stddef.h"
#include "parser/tokens.h"

typedef struct Parser_S
{
    size_t line_len;
    char *line;
    size_t next_tok;
    TransUnit_Type tut;
} Parser;

Parser
parser_init(char *line, TransUnit_Type tut);

TokenResult
parser_next_token(Parser *ps);

#endif /* __PARSER_PARSER_H__ */