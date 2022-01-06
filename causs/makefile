CC = gcc
C_FLAGS = -Wall -pedantic -Iinclude -O3

SRC_DIR = src

TARGET  = causs
SRCS    = $(wildcard $(SRC_DIR)/*.c)

IF = /home/zs/gauss/tests/variables/var.gis

.PHONY: all	
all: clean $(TARGET) run 

$(TARGET): $(SRCS)
	$(CC) $^ $(C_FLAGS) -o $@


run: 
	./$(TARGET) $(IF)

.PHONY: clean
clean:
	-rm -f $(TARGET)
