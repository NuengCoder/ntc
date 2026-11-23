CXX ?= c++
CXXFLAGS ?= -Wall -Wextra -O2 -std=c++17
TARGET := app

.PHONY: all run clean

all: $(TARGET)

$(TARGET): src/main.cpp
	$(CXX) $(CXXFLAGS) -o $(TARGET) src/main.cpp

run: $(TARGET)
	./$(TARGET)

clean:
	rm -f $(TARGET)
