#include "token.h"

token_T *
token_init
(
	char *value,
	token_type_T type
)
{
	token_T *token = calloc(1, sizeof(struct TOKEN_STCT));
	token->value = value;
	token->type = type;

	return token;
}
