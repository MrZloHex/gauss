#include "tokenizer/parser.hpp"


void Parser::parse() {
    this->tok_p = 0;

    AST_Node node;
    while (this->next_node(&node)) {
        this->AST.push_back(node);
    }

    this->debug_print();
}

bool Parser::next_node(AST_Node *node) {
    Token tok = this->get_token();
    
    if (tok.is_var_size() && this->get_token(1).type == TokenType_IDENTIFIER && this->get_token(2).type == TokenType_COLON) {
        AST_FuncDecl nd = {
            ._nt = NT_FUNCDECL,
            .type = this->token2type(tok),
            .name = this->get_token(1).value,
        };
        *node = nd;
        ++this->tok_p;
        return true;
    }

    return false;
}


void Parser::debug_print() {
    for (AST_Node node: this->AST) {
        std::visit(AST_Print(), node);
        std::cout << '\n';
    }
}




AST_Types Parser::token2type(Token token) {
    return static_cast<AST_Types>(token.type - TokenType_BYTE);
}


inline Token Parser::get_token() {
    return this->tokens[this->tok_p];
}

inline Token Parser::get_token(std::size_t i) {
    return this->tokens[this->tok_p + i];
}