#pragma once
#include <cstdint>

uint8_t* intToBe(int num, int bytes);
int beToInt(uint8_t* be, int bytes);