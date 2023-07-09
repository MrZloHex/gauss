#include "parser/parser.h"
#include <string.h>

const static char *k_keywords[] =
{
    "BYTE", "WORD", "DOWRD", "QWORD",
    "DECLARE"
};

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
    if (ps->next_tok >= ps->line_len)
    { TOKEN_ERR(TOK_EOL); }


    TokenResult tok_res;
    char fst_tok_ch;
redo:
    fst_tok_ch = ps->line[ps->next_tok];

    if (IS_CH_EMPTY(fst_tok_ch))
    {
        ps->next_tok++;
        goto redo;
    }

    // ONE CHAR LEN TOKENS
    if (IS_COMMENT(fst_tok_ch))
    {
        return;
    }

    size_t wrd_len = parser_get_word(ps);
    const char *wrd = ps->line[ps->next_tok];
    if (IS_KEYWORD(wrd, wrd_len, KW_DECLARE))
    {
        TOKEN_OK_KW(KW_DECLARE);
    }
    else if (IS_KEYWORD(wrd, wrd_len, KW_BYTE))
    {   TOKEN_OK_KW(KW_BYTE);   }
}