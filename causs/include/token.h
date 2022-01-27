#ifndef GAUSS_TOKEN_H_
#define GAUSS_TOKEN_H_

#include <stdlib.h>

typedef enum TOKEN_TYPE
{
	TOKEN_BLOCK_TYPE,
	TOKEN_VAR_TYPE,
	TOKEN_VAR_SIGN,
	TOKEN_VAR_SIZE,
	TOKEN_ID,
	TOKEN_LSQPAREN,
	TOKEN_RSQPAREN,
	TOKEN_LPAREN,
	TOKEN_RPAREN,
	TOKEN_LCURLY,
	TOKEN_RCURLY,
	TOKEN_COLON,
	TOKEN_SEMICOLON,
	TOKEN_NUMBER
} token_type_T;

typedef struct TOKEN_STCT
{
	char *value;
	token_type_T type;
	
} token_T;

token_T *
token_init
(
	char *value,
	token_type_T type
);

#endif /* GAUSS_TOKEN_H_  */
