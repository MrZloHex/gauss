#include "file.h"

char *
read_file (const char *pathname)
{
	FILE *stream = fopen(pathname, "rb");
	if (stream == NULL)
	{
		fprintf(stderr, "ERROR: failed to open %s\n", pathname);
		exit(1);
	}
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
	fclose(stream);
	return buffer;
}
