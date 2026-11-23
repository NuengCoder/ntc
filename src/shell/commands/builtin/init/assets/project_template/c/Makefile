CC ?= cc
CFLAGS ?= -Wall -Wextra -O2
TARGET := app

.PHONY: all run clean

all: $(TARGET)

$(TARGET): src/main.c
	$(CC) $(CFLAGS) -o $(TARGET) src/main.c

run: $(TARGET)
	./$(TARGET)

clean:
	rm -f $(TARGET)
