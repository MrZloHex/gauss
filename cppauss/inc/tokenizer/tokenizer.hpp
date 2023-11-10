#pragma once

#include <string>
#include <vector>

static constexpr std::string_view TokenTypes[] = {
	"ID",

    "#", "'", "\"",
    "LIT",

    "+", "-", "*", "/", "=", ":", "@", "!",
    // "(", ")", "{", "}",
    "[", "]", "<", ">",
    ",", "|", "\\", "_",

    "BYTE", "WORD", "DWORD", "QWORD",
    "VOID", "RET", "SYSCALL",
    "LOOP", "IF", "ELIF", "ELSE",
    "BREAK", "CONTINUE"

};

enum TokenType
{
	TokenType_IDENTIFIER,

	//Literals
    TokenType_HASH,
    TokenType_QUOTE,
    TokenType_DQOUTE,
	TokenType_INTLIT,

	//Operators
	TokenType_PLUS,
	TokenType_MINUS,
	TokenType_TIMES,
	TokenType_SLASH,
	TokenType_EQL,
	TokenType_COLON,
    TokenType_AT,
    TokenType_EXCLM,

	//Seperators
	// TokenType_LPAREN,
	// TokenType_RPAREN,
	// TokenType_LBRACE,
	// TokenType_RBRACE,
	TokenType_LBRACK,
	TokenType_RBRACK,
    TokenType_LCHEV,
    TokenType_RCHEV,
	TokenType_COMMA,
    TokenType_PIPE,
	TokenType_BSLASH,
    TokenType_UNDSCR,

	//Keywords
    TokenType_BYTE,
    TokenType_WORD,
    TokenType_DWORD,
    TokenType_QWORD,
    TokenType_VOID,
    TokenType_RET,
    TokenType_SYSCALL,
    TokenType_LOOP,
	TokenType_IF,
    TokenType_ELIF,
    TokenType_ELSE,
    TokenType_BREAK,
    TokenType_CONTINUE,


	//Unique
	TokenType_UNKNOWN,
	TokenType_EOF
};

struct Position {
    std::size_t row = 1, col = 1;
};

struct Token {
    TokenType type;
    std::string value;
    Position pos;

    friend std::ostream &operator<<(std::ostream &os, const Token &t);

    bool is_var_size() {
        return (type == TokenType_BYTE) || (type == TokenType_WORD) || (type == TokenType_DWORD) || (type == TokenType_QWORD);
    }
};

class Tokenizer {
    private:
        std::vector<Token> tokens;
        
        std::string raw_input;
        std::size_t raw_p = 0;

        Position pos;

        bool next_token(Token *token);

        bool pass_comment_space();

        bool get_literal(Token *literal);
        bool get_word(Token *word);
        bool get_spec_char(Token *spec_char);

        inline bool is_number();
        inline bool is_valid_word();
        inline bool is_white_space();
        inline bool is_end_line();
        
        inline char get_curr_char();

        void debug_print();

    public:
        explicit Tokenizer(std::string raw_input) : raw_input(raw_input) {};

        void tokenize();
        std::vector<Token> get_tokens() { return this->tokens; }
};