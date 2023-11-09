#include "tokenizer/tokenizer.hpp"

#include <iostream>


void Tokenizer::tokenize() {
    Token token;
    while (this->next_token(&token)) {
        this->tokens.push_back(token);
    }

    this->debug_print();
}

bool Tokenizer::next_token(Token *token) {
    token->value = "";

    while (this->pass_comment_space()) {}

    if (this->get_spec_char(&(token->type))) {
        return true;
    }

    if (this->get_word(token)) {
        return true;
    }

    if (this->get_literal(token)) {
        return true;
    }
    
    return false;
}

bool Tokenizer::get_literal(Token *literal) {
    std::string lit;
    if (this->tokens.back().type == TokenType_HASH) {
        while (this->is_number()) {
            lit.push_back(this->get_curr_char());
            ++this->raw_p;
        }

        literal->type = TokenType_INTLIT;
        literal->value = lit;

        return true;
    }

    return false;
}

bool Tokenizer::get_word(Token *word) {
    
    std::string str;
    while (this->is_valid_word()) {
        str.push_back(this->get_curr_char());
        ++this->raw_p;
    }

    if (str.empty()) { return false; }
    
    if      (str == "BYTE")    { word->type = TokenType_BYTE;    }
    else if (str == "WORD")    { word->type = TokenType_WORD;    }
    else if (str == "DWORD")   { word->type = TokenType_DWORD;   }
    else if (str == "QWORD")   { word->type = TokenType_QWORD;   }
    else if (str == "NULL")    { word->type = TokenType_NULL;    }
    else if (str == "UNRET")   { word->type = TokenType_UNRET;   }
    else if (str == "SYSCALL") { word->type = TokenType_SYSCALL; }
    else if (str == "RET")     { word->type = TokenType_RET;     }
    else if (str == "LOOP")    { word->type = TokenType_LOOP;    }
    else if (str == "THEN")    { word->type = TokenType_THEN;    }
    else if (str == "IF")      { word->type = TokenType_IF;      }
    else if (str == "BREAK")   { word->type = TokenType_BREAK;   }
    else {
        word->type  = TokenType_IDENTIFIER;
        word->value = str;
    }

    return true;
}

bool Tokenizer::get_spec_char(TokenType *spec_char) {
    switch (this->get_curr_char()) {
        case '+': *spec_char = TokenType_PLUS;  break;
        case '-': *spec_char = TokenType_MINUS; break;
        case '*': *spec_char = TokenType_TIMES; break;
        case '/': *spec_char = TokenType_SLASH; break;
        case '=': *spec_char = TokenType_EQL;   break;

        case '{': *spec_char = TokenType_LPAREN; break;
        case '}': *spec_char = TokenType_RPAREN; break;
        case '[': *spec_char = TokenType_LBRACK; break;
        case ']': *spec_char = TokenType_RBRACK; break;
        case '(': *spec_char = TokenType_LBRACE; break;
        case ')': *spec_char = TokenType_RBRACE; break;
        case '<': *spec_char = TokenType_LCHEV;  break;
        case '>': *spec_char = TokenType_RCHEV;  break;
        case ',': *spec_char = TokenType_COMMA;  break;
        case ':': *spec_char = TokenType_COLON;  break;
        case '|': *spec_char = TokenType_PIPE;   break;
        case '\\':*spec_char = TokenType_BSLASH; break;
        case '_': *spec_char = TokenType_UNDSCR; break;

        case '#': *spec_char = TokenType_HASH;   break;

        default: return false;
    }

    ++this->raw_p;
    return true;
}

bool Tokenizer::pass_comment_space() {
    bool res = false;

    // check for white spaces
    while (this->is_white_space()) {
        ++this->raw_p;
        ++this->col;
        res = true;
    }

    if (this->get_curr_char() == ';') {
        while (!this->is_end_line()) { ++this->raw_p; }
        this->col = 0;
        ++this->row;
        res = true;
    }
    
    return res;
}


inline bool Tokenizer::is_number() {
    char c = this->get_curr_char();
    return static_cast<bool>(std::isdigit(static_cast<unsigned char>(c)));
}

inline bool Tokenizer::is_valid_word() {
    char c = this->get_curr_char();
    return static_cast<bool>(std::isalpha(static_cast<unsigned char>(c)) || (c == '_'));
}

inline bool Tokenizer::is_white_space() {
    char c = this->get_curr_char();
    // return (c == ' ') || (c == '\t') || (c == '\f') || (c == '\v');
    return static_cast<bool>(std::isspace(static_cast<unsigned char>(c)));
}

inline bool Tokenizer::is_end_line() {
    char c = this->get_curr_char();
    return (c == '\n') || (c == '\r');
}

inline char Tokenizer::get_curr_char() {
    return this->raw_input[this->raw_p];
}

void Tokenizer::debug_print() {
    int n = 0;
    for (Token i: this->tokens)
    {
        std::cout << i << ' ';
        if (n == 20) {
            std::cout << '\n';
            n = 0;
        }
        ++n;
    }
    std::cout << '\n';
}


std::ostream &operator<<(std::ostream &os, const Token &t) { 
    os << TokenTypes[t.type];
    if (t.type == TokenType_IDENTIFIER || t.type == TokenType_INTLIT) {
        os << '(' << t.value << ')';
    }

    return os;
}