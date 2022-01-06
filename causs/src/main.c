#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <errno.h>


char *
read_file
(
	FILE *stream
)
{
	int fsk = fseek(stream, 0, SEEK_END);
	if (fsk != 0)
	{
		fprintf(stderr, "ERROR: couldn't fetch metadata of file cause `%s`\n", strerror(errno));
		exit(1);
	}

	long length = ftell(stream);
	if (length < 0)
	{
		fprintf(stderr, "ERROR: couldn't fetch metadata of file cause `%s`\n", strerror(errno));
		exit(1);
	}

	fsk = fseek(stream, 0, SEEK_SET);
	if (fsk != 0)
	{
		fprintf(stderr, "ERROR: couldn't fetch metadata of file cause `%s`\n", strerror(errno));
		exit(1);
	}
	
	char *buffer = (char *)malloc(length);
	if (buffer == NULL)
	{
		fprintf(stderr, "ERROR: unable to allocate mem for file\n");
		exit(1);
	}

	size_t n = fread(buffer, 1, length, stream);
	if (n < length)
	{
		fprintf(stderr, "ERROR: failed to read whole file\n");
		exit(1);
	}
	return buffer;
}

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
	FILE *ifl = fopen(input_pathname, "rb");
	if (ifl == NULL)
	{
		fprintf(stderr, "ERROR: failed to open %s\n", input_pathname);
		exit(1);
	}
	char *in_txt = read_file(ifl);
	
	fclose(ifl);
	// printf("------FILE------\n%s-------END-OF-FILE------\n", in_txt);

	free(in_txt);
	return 0;
}
