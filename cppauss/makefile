CC = g++
CFLAGS =  -ggdb -O0 -Wall -Wextra -Wpedantic -std=c++20
CFLAGS += -Iinc

LDFLAGS =

TARGET = causs
INPUT = ../gauss/std/std.gfs

SRC = src
OBJ = obj
BIN = bin

SOURCES = $(wildcard $(SRC)/*.cpp)
OBJECTS = $(patsubst $(SRC)/%.cpp, $(OBJ)/%.o, $(SOURCES))


all: clean compile

compile: binary 
run: clean compile execute

binary: $(OBJECTS)
	$(CC) -o $(BIN)/$(TARGET) $^ $(LDFLAGS)

$(OBJ)/%.o: $(SRC)/%.cpp
	$(CC) -o $@ -c $< $(CFLAGS)

execute:
	./$(BIN)/$(TARGET) $(INPUT)

clean:
	-rm $(BIN)/$(TARGET)
	-rm $(OBJECTS)