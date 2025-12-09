#include "AllocatorBlocks.hpp"
#include "../Helpers/BigEndianCover.hpp"

std::string AllocatorBlocks::getDiskPath(){
  return "test";
}

size_t AllocatorBlocks::max(int layer) {
    if (layer <= 0) return 0;              // evitar abismos lógicos
    if (maxis[layer] > 0) return maxis[layer];

    size_t punters = (blockSize - 4) / 8;  // punteros por bloque
    size_t numMax = 1;

    for (int l = 0; l < layer; ++l) {
        numMax *= punters;
    }

    maxis[layer] = numMax;
    return numMax;
}

size_t AllocatorBlocks::readFrees(){
  std::vector<uint8_t> end = frees.removeAndGetToLast(8);
  int size = end.size();
  if(size<=0) return 0;
  return static_cast<size_t> (beToInt(end.data(), size));
  
}
size_t AllocatorBlocks::generateBlock() {
    if (totalBlocks == std::numeric_limits<uint64_t>::max()) {
        // no hay espacio para representar un bloque más
        return 0; // o lanza excepción, o señal de error
    }

    size_t newIndex = totalBlocks;
    totalBlocks += 1;

    return newIndex;
}