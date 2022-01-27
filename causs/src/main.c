#include <stdio.h>

#include "file.h"

int
main
(
	int argc,
	char **argv
)
{
	if (argc < 2)
	{
		fprintf(stderr, "ERROR: isn't provided input file\n");
		exit(1);
	}
	const char *input_pathname = argv[1];
	char *in_txt = read_file(input_pathname);
	
	printf("------FILE------\n%s-------END-OF-FILE------\n", in_txt);


	free(in_txt);
	return 0;
}
