#include "lexer/lexer.h"
#include "parser/parser.h"
#include <string.h>

#include "stdio.h"

Lexer
lexer_init(char *in)
{
    Lexer lx = {0};
    for (size_t i = 0; ; ++i)
    {
        if (in[i] == '\n')
        {
            lx.q_in_lines++;
        }
        else if (in[i] == 0)
        {
            lx.q_in_lines++;
            break;
        }
    }

    lx.in_lines = (char **)malloc(sizeof(char *)*lx.q_in_lines);
    if (lx.in_lines == NULL)
    { return; }

    size_t offset = 0;
    for (size_t i = 0; i < lx.q_in_lines; ++i)
    {
        size_t len = 0;
        for (; (in[offset+len] != '\n' && in[offset+len] != 0); ++len) {}

        if (len == 0)
        {
            lx.in_lines[i] = NULL;
            offset++;
            continue;
        }

        lx.in_lines[i] = (char *)calloc(len+1, sizeof(char));
        if (lx.in_lines[i] == NULL)
        { return; }

        strncpy(lx.in_lines[i], in+offset, len);

        offset += len+1;
    }

    for (size_t i = 0; i < lx.q_in_lines; ++i)
    {
        if (lx.in_lines[i] == NULL)
        {
            printf("LINE\t%lu:\t0\n", i+1);
        }
        else
        {
            printf("LINE\t%lu:\t%lu,\t`%s`\n", i+1, strlen(lx.in_lines[i]), lx.in_lines[i]);
        }
    }

    return lx;
}

Lexeme *
lexer_lex(Lexer *lx)
{
        __TITLE_TR_FMT;
    for (size_t line = 0; line < lx->q_in_lines; ++line)
    {
        if (lx->in_lines[line] == NULL)
        { continue; }

        Parser parser = parser_init(lx->in_lines[line], lx->tut);

        TokenResult tr = { .error = TOK_OK };
        
        while (tr.error != TOK_EOL)
        {
            tr = parser_next_token(&parser);
            printf("LINE\t%lu:\t", line+1);
            __print_tr(tr);
            if (tr.token.sym == SYM_SEMICOLON)
            {
                break;
            }
        }
        
        // if (tr.error != TOK_OK)
        // {
            // printf("TR ERR: %u\n", tr.error);
            // continue;
        // }

        // printf("TOK: %u\n", tr.token.kw);


    }

}