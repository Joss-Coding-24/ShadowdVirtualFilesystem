#include "AllocatorBlocks.hpp"
#include "../Helpers/BigEndianCover.hpp"
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <limits>
#include "../Helpers/RandomAccessFile.hpp"

AllocatorBlocks::AllocatorBlocks(Metadata& metadata){
  frees = metadata.freesFile;
  path = metadata.path;
  if(metadata.sizeBlock>0) blockSize = metadata.sizeBlock;
  RandomAccessFile raf(path);
  size_t size = raf.size();
  totalBlocks = size/blockSize;
}
template<typename BlockType>
BlockType AllocatorBlocks::get(size_t pos){
  return BlockType(pos, this);
}
void AllocatorBlocks::freeBlock(size_t pos){
  std::vector<uint8_t> data =  sizeToBe(pos, 8);
  frees.AddToLast(data);
}
size_t AllocatorBlocks::gen(){
  size_t pos;
  pos = readFrees();
  if(pos > 0) return pos;
  pos = generateBlock();
  return pos;
}
std::string AllocatorBlocks::getDiskPath(){
  return path;
}
std::string AllocatorBlocks::getBlockName(int layer){
  switch(layer){
    case 1: return "BaseShadowdBlock";
    case 2: return "SmallShadowdBlock";
    case 3: return "MediumShadowdBlock";
    case 4: return "GreatShadowdBlock";
    case 5: return "LargeShadowdBlock";
    default: return "UnknownShadowdBlock";
  }
}
/*
* Para no volver a aolvidar que hace max lo anoto aca
* max cumple coon ver cuantos bloques caben en un root
* Capa 2 tiene un layer 1 como root, por lo que le caben 3
*
*    L1 = 31 C1
*    L2 = 31*31 C2
*    ...
*
* En resumen, una layer tiene 31^layer bloques del mismo nivel
*/
uint64_t AllocatorBlocks::max(int layer) {
  uint64_t punters = (blockSize - 4) / 8;
  uint64_t result = 1;

  for(int i = 0; i < layer; ++i) {
    if(result > UINT64_MAX / punters) return UINT64_MAX;
    result *= punters;
  }
  return result;
}
size_t AllocatorBlocks::readFrees(){
  std::vector<uint8_t> end = frees.removeAndGetToLast(8);
  int size = end.size();
  if(size < 8) return 0;
  return beToSize(end, size);
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
uint64_t AllocBlock::span(int layer){
  uint64_t punters = (blockSize - 4) / 8;
  uint64_t bytes = blockSize - 4;

  for(int i =1; i < layer; ++i){
    if(bytes > UINT64_MAX / punters) return UINT64_MAX;
    bytes *= punters;
  }
  return bytes;
}