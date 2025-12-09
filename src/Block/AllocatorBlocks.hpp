#pragma once
#include <string>
#include <vector>
#include "../Files/ShadowdNode.hpp"
#include "../Disk/Metadata.hpp"

class AllocatorBlocks{
  public:
    explicit AllocatorBlocks(Metadata metadata){
      maxis.push_back(0);
      maxis.push_back(0);
      maxis.push_back(0);
      maxis.push_back(0);
      maxis.push_back(0);
      frees = metadata.freesFile;
      path = metadata.path;
      if(metadata.sizeBlock>0) blockSize = metadata.sizeBlock;
    }
    template<typename BlockType>
    BlockType* get(size_t pos){
      return new BlockType(pos, this);
    }
    void freeBlock(size_t pos);
    size_t gen(){
      size_t pos;
      pos = readFrees();
      if(pos > 0) return pos;
      pos = generateBlock();
      return pos;
    }
    int blockSize = 252;
    std::string getDiskPath();
    size_t max(int layer);
    std::string getBlockName(int layer){
      switch(layer){
        case 1: return "BaseShadowdBlock";
        case 2: return "SmallShadowdBlock";
        case 3: return "MediumShadowdBlock";
        case 4: return "GreadShadowdBlock";
        case 5: return "LargeShadowdBlock";
        default: return "UnknownShadowdBlock";
      }
    }
  private:
    uint64_t totalBlocks = 0;
    SFile frees;
    std::vector<size_t> maxis;
    std::string path;
    size_t readFrees();
    size_t generateBlock();
};

using AllocBlock = AllocatorBlocks;
using Alloc = AllocatorBlocks;