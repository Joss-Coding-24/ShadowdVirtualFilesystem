CXX = g++
CXXFLAGS = -std=gnu++17 -Wall -Wextra -Werror -O2 -Iinclude -pthread

SRC_DIR = src
BUILD_DIR = build
TARGET = app

# Busca todos los .cpp en src (recursivo)
SRCS := $(shell find $(SRC_DIR) -name "*.cpp")
# Convierte src/...cpp → build/...o
OBJS := $(SRCS:$(SRC_DIR)/%.cpp=$(BUILD_DIR)/%.o)

$(TARGET): $(OBJS)
	$(CXX) $(CXXFLAGS) -o $@ $^

# Regla general para compilar .cpp → .o (crea carpetas si faltan)
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.cpp
	@mkdir -p $(dir $@)
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Limpieza
clean:
	rm -rf $(BUILD_DIR) $(TARGET)