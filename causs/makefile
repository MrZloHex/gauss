CC = gcc 
CFLAGS =  -g -O0 -Wall -Wextra -Wpedantic -std=c11 -Wstrict-aliasing
CFLAGS += -Wno-old-style-declaration -Wno-implicit-fallthrough
CFLAGS += -Iinc

LDFLAGS =

TARGET = causs
INPUT = std.gts

SRC = src
OBJ = obj
BIN = bin

SOURCES = $(wildcard $(SRC)/*.c)
OBJECTS = $(patsubst $(SRC)/%.c, $(OBJ)/%.o, $(SOURCES))


all: clean compile

compile: binary 
run: clean compile execute

binary: $(OBJECTS)
	$(CC) -o $(BIN)/$(TARGET) $^ $(LDFLAGS)

$(OBJ)/%.o: $(SRC)/%.c
	$(CC) -o $@ -c $< $(CFLAGS)

execute:
	./$(BIN)/$(TARGET) ./$(BIN)/$(INPUT)

clean:
	-rm $(BIN)/$(TARGET)
	-rm $(OBJECTS)