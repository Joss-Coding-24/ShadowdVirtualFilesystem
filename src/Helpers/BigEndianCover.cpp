#include "BigEndianCover.hpp"

uint8_t* convInt(int num, int bytes){
    uint8_t* big = new uint8_t[bytes];
    for(int i = bytes-1; i >= 0; i--){
        int mov = i * 8;
        big[i] = (uint8_t)((num >> mov) & 0xFF);
    }
    return big;
}
int convBig(uint8_t* big, int bytes){
    int num = 0;
    for(int i = bytes - 1; i >= 0; i--){
        int mov = i * 8;
        num |= ((int)big[i]) << mov;
    }
    return num;
}
uint8_t* intToBe(int num, int bytes){
  return convInt(num, bytes);
}
int beToInt(uint8_t* be, int bytes){
  return convBig(be, bytes);
}