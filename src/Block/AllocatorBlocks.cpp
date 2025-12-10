#include "AllocatorBlocks.hpp"
#include "../Helpers/BigEndianCover.hpp"

AllocatorBlocks::AllocatorBlocks(Metadata& metadata){
  maxis.push_back(0);
  maxis.push_back(0);
  maxis.push_back(0);
  maxis.push_back(0);
  maxis.push_back(0);
  frees = metadata.freesFile;
  path = metadata.path;
  if(metadata.sizeBlock>0) blockSize = metadata.sizeBlock;
  RandomAccessFile raf(path);
  size_t size = raf.size();
  totalBlocks = size/blockSize;
}
template<typename BlockType>
BlockType* AllocatorBlocks::get(size_t pos){
  return new BlockType(pos, this);
}
void AllocatorBlocks::freeBlock(size_t pos){
  std::vector<uint8_t> data;
  uint8_t* bytes =  intToBe(pos, 8);
  data.insert(data.end(), bytes, bytes+8);
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
    case 4: return "GreadShadowdBlock";
    case 5: return "LargeShadowdBlock";
    default: return "UnknownShadowdBlock";
  }
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