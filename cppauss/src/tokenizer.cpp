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

    if (this->get_spec_char(token)) {
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
    if (this->tokens.empty()) { return false; }

    std::string lit;
    if (this->tokens.back().type == TokenType_HASH) {
        while (this->is_number()) {
            lit.push_back(this->get_curr_char());
            ++this->raw_p;
        }

        literal->type = TokenType_INTLIT;
        literal->value = lit;

        literal->pos.col = this->pos.col;
        literal->pos.row = this->pos.row;
        this->pos.col += lit.length();

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

    word->pos.col = this->pos.col;
    word->pos.row = this->pos.row;

    this->pos.col += str.length();
    
    if      (str == "BYTE")     { word->type = TokenType_BYTE;      }
    else if (str == "WORD")     { word->type = TokenType_WORD;      }
    else if (str == "DWORD")    { word->type = TokenType_DWORD;     }
    else if (str == "QWORD")    { word->type = TokenType_QWORD;     }
    else if (str == "VOID")     { word->type = TokenType_VOID;      }
    else if (str == "SYSCALL")  { word->type = TokenType_SYSCALL;   }
    else if (str == "RET")      { word->type = TokenType_RET;       }
    else if (str == "LOOP")     { word->type = TokenType_LOOP;      }
    else if (str == "IF")       { word->type = TokenType_IF;        }
    else if (str == "ELIF")     { word->type = TokenType_ELIF;      }
    else if (str == "ELSE")     { word->type = TokenType_ELSE;      }
    else if (str == "BREAK")    { word->type = TokenType_BREAK;     }
    else if (str == "CONTINUE") { word->type = TokenType_CONTINUE;  }
    else {
        word->type  = TokenType_IDENTIFIER;
        word->value = str;
    }

    return true;
}

bool Tokenizer::get_spec_char(Token *spec_char) {
    switch (this->get_curr_char()) {
        case '+': spec_char->type = TokenType_PLUS;  break;
        case '-': spec_char->type = TokenType_MINUS; break;
        case '*': spec_char->type = TokenType_TIMES; break;
        case '/': spec_char->type = TokenType_SLASH; break;
        case '=': spec_char->type = TokenType_EQL;   break;
        case '@': spec_char->type = TokenType_AT;    break;
        case '!': spec_char->type = TokenType_EXCLM; break;
        case ':': spec_char->type = TokenType_COLON; break;

        // case '{': spec_char->type = TokenType_LPAREN; break;
        // case '}': spec_char->type = TokenType_RPAREN; break;
        // case '(': spec_char->type = TokenType_LBRACE; break;
        // case ')': spec_char->type = TokenType_RBRACE; break;
        case '[': spec_char->type = TokenType_LBRACK; break;
        case ']': spec_char->type = TokenType_RBRACK; break;
        case '<': spec_char->type = TokenType_LCHEV;  break;
        case '>': spec_char->type = TokenType_RCHEV;  break;
        case ',': spec_char->type = TokenType_COMMA;  break;
        case '|': spec_char->type = TokenType_PIPE;   break;
        case '\\':spec_char->type = TokenType_BSLASH; break;
        case '_': spec_char->type = TokenType_UNDSCR; break;

        case '#': spec_char->type = TokenType_HASH;   break;
        case '\'':spec_char->type = TokenType_QUOTE;  break;
        case '"': spec_char->type = TokenType_DQOUTE; break;

        default: return false;
    }

    ++this->raw_p;
    spec_char->pos.col = this->pos.col;
    spec_char->pos.row = this->pos.row;
    ++this->pos.col;

    return true;
}

bool Tokenizer::pass_comment_space() {
    bool res = false;

    // check for white spaces
    while (this->is_white_space()) {
        ++this->raw_p;
        ++this->pos.col;
        res = true;
    }

    
    while (this->is_end_line()) {
        ++this->raw_p;
        ++this->pos.row;
        this->pos.col = 1;
        return true;
    }

    if (this->get_curr_char() == ';') {
        while (!this->is_end_line()) { ++this->raw_p; }
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
    return (c == ' ') || (c == '\t') || (c == '\f') || (c == '\v');
    // return static_cast<bool>(std::isspace(static_cast<unsigned char>(c)));
}

inline bool Tokenizer::is_end_line() {
    char c = this->get_curr_char();
    return (c == '\n') || (c == '\r');
}

inline char Tokenizer::get_curr_char() {
    return this->raw_input[this->raw_p];
}

void Tokenizer::debug_print() {
    for (Token i: this->tokens)
    {
        std::cout << i << '\n';
    }
    std::cout << '\n';
}


std::ostream &operator<<(std::ostream &os, const Token &t) { 
    os << t.pos.row << ':' << t.pos.col << ' ' << TokenTypes[t.type];
    if (t.type == TokenType_IDENTIFIER || t.type == TokenType_INTLIT) {
        os << '(' << t.value << ')';
    }

    return os;
}
