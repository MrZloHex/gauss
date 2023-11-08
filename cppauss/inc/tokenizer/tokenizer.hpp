#pragma once

#include <string>
#include <vector>


enum TokenType
{
	TokenType_IDENTIFIER,

	//Literals
    TokenType_HASH,
	TokenType_INTLIT,

	//Operators
	TokenType_PLUS,
	TokenType_MINUS,
	TokenType_TIMES,
	TokenType_SLASH,
	TokenType_EQL,

	//Seperators
	TokenType_LPAREN,
	TokenType_RPAREN,
	TokenType_LBRACK,
	TokenType_RBRACK,
	TokenType_LBRACE,
	TokenType_RBRACE,
	TokenType_COMMA,
	TokenType_COLON,
    TokenType_PIPE,
	TokenType_BSLASH,
    TokenType_UNDSCR,
    TokenType_LANGLE,
    TokenType_RANGLE,

	//Keywords
    TokenType_BYTE,
    TokenType_WORD,
    TokenType_DWORD,
    TokenType_QWORD,
    TokenType_UNRET,
    TokenType_NULL,
    TokenType_SYSCALL,
    TokenType_RET,
    TokenType_LOOP,
    TokenType_THEN,
	TokenType_IF,
    TokenType_BREAK,


	//Unique
	TokenType_UNKNOWN,
	TokenType_EOF
};

struct Token {
    TokenType type;
    std::string value;
};

class Tokenizer {
    private:
        std::vector<Token> tokens;
        
        std::string raw_input;
        std::size_t raw_p = 0;

        std::size_t row = 1, col = 1;

        bool next_token(Token *token);

        bool pass_comment_space();

        bool get_literal(Token *literal);
        bool get_word(Token *word);
        bool get_spec_char(TokenType *spec_char);

        inline bool is_number();
        inline bool is_valid_word();
        inline bool is_white_space();
        inline bool is_end_line();
        inline char get_curr_char();

    public:
        explicit Tokenizer(std::string raw_input) : raw_input(raw_input) {};

        void tokenize();

};