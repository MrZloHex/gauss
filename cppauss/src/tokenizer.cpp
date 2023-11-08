#include "tokenizer/tokenizer.hpp"

#include <iostream>


void Tokenizer::tokenize() {
    Token token;
    while (this->next_token(&token)) {
        this->tokens.push_back(token);
    }

    int n = 0;
    for (auto i: this->tokens)
    {
        std::cout << n << ' ' << i.type << '\n';
    }
}

bool Tokenizer::next_token(Token *token) {
    while (this->pass_comment_space()) {}

    if (this->get_spec_char(&(token->type))) {
        std::cout << "SPEC CHAR " << token->type << '\n';

        ++this->col;

        return true;
    }

    // HASH fro INT LITERAL

    if (this->get_word(token)) {
        if (token->type == TokenType_IDENTIFIER) {
            std::cout << "ID " << token->value << '\n';
        } else {
            std::cout << "KEYWORD " << token->type << '\n';
        }

        return true;
    }
    

    return true;
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
        case ',': *spec_char = TokenType_COMMA;  break;
        case ':': *spec_char = TokenType_COLON;  break;
        case '|': *spec_char = TokenType_PIPE;   break;

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
        
        std::cout << "COMMENT " << col << ' ' << row << '\n';

        while (!this->is_end_line()) { ++this->raw_p; }
        this->col = 0;
        ++this->row;
        res = true;
    }
    
    return res;
}


inline bool Tokenizer::is_valid_word() {
    char c = this->get_curr_char();
    return std::isalpha(static_cast<unsigned char>(c)) || (c == '_');
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