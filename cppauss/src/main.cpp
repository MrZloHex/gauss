#include <iostream>
#include <fstream>
#include <string>

#include "tokenizer/tokenizer.hpp"


int main (int argc, char **argv) {
	if (argc < 2) {
		std::clog << "ERROR: isn't provided input file\n";
		exit(1);
	}
	
	const std::string input_pathname(argv[1]);
	std::ifstream ifl(input_pathname, std::ios::binary | std::ios::ate);
	if (!ifl.is_open()) {
		std::clog << "ERROR: failed to open " << input_pathname << '\n';
		exit(1);
	}

	std::streampos size = ifl.tellg();
	std::string raw_input(size, '\0');
	ifl.seekg(0);
	ifl.read(&raw_input[0], size);
	ifl.close();


	
	Tokenizer tknz(raw_input);
	tknz.tokenize();
	

	return 0;
}
