#pragma once

#include "tokenizer/tokenizer.hpp"

#include <iostream>
#include <variant>


static constexpr std::string_view NodeTypes[] = {
    "FUNCDECL", "VARDECL", "BLOCK", "ASSIGN",
    "FUNCCALL", "VAR", "RETURN", "INTLIT"
};

enum AST_NodeType {
    NT_FUNCDECL,
    NT_VARDECL,
    NT_BLOCK,
    NT_ASSIGN,
    NT_FUNCCALL,
    NT_VAR,
    NT_RETURN,
    NT_INTLIT    
};

static constexpr std::string_view VarSize[] = {
    "BYTE", "WORD", "DWORD", "QWORD", "UNRET", "NULL"
};

enum AST_Types {
    AT_BYTE, AT_WORD,
    AT_DWORD, AT_QWORD,
    AT_UNRET, AT_NULL
};

struct AST_VarDecl {
    const AST_NodeType _nt = NT_VARDECL;
    AST_Types type;
    std::string name;

    void operator=(const AST_VarDecl& F) { 
        type = F.type; 
        name = F.name; 
    } 
};

struct AST_FuncDecl {
    AST_NodeType _nt = NT_FUNCDECL;
    AST_Types type;
    std::string name;
    std::vector<AST_VarDecl> params;

    void operator=(const AST_FuncDecl& F) { 
        type = F.type; 
        name = F.name; 
        params = F.params;
    } 
  
};

struct AST_Print {
    void operator()(const AST_FuncDecl &f) { std::cout << NodeTypes[f._nt] << ' ' << VarSize[f.type] << ' ' << f.name; }
    void operator()(const AST_VarDecl  &v) { std::cout << NodeTypes[v._nt] << ' ' << VarSize[v.type] << ' ' << v.name; }
};

typedef std::variant<AST_FuncDecl, AST_VarDecl> AST_Node;

class Parser {
    std::vector<Token> tokens;
    std::size_t tok_p;

    std::vector<AST_Node> AST;

    
    bool next_node(AST_Node *node);

    inline Token get_token();
    inline Token get_token(std::size_t i);

    AST_Types token2type(Token token);

    void debug_print();

    public:
        explicit Parser(std::vector<Token> tokens) : tokens(tokens) {}

        void parse();
};