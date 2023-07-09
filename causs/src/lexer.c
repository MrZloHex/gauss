#include "lexer/lexer.h"
#include "parser/parser.h"


Lexeme *
lex(Lexer *lx)
{
    for (size_t line = 0; line < lx->q_in_lines; ++line)
    {
        Parser parser = parser_init(lx->in_lines[line], lx->tut);

        parser_next_token(&parser);
    }

}