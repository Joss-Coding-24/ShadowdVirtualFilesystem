#pragma once
#include <string>
#include <sys/types.h>
#include <vector>
#include "../Files/ShadowdNode.hpp"
#include "../Disk/Metadata.hpp"

class AllocatorBlocks{
  public:
    explicit AllocatorBlocks(Metadata& metadata);
    template<typename BlockType>
    BlockType get(size_t pos);
    void freeBlock(size_t pos);
    size_t gen();
    int blockSize = 252;
    std::string getDiskPath();
    uint64_t max(int layer);
    std::string getBlockName(int layer);
    uint64_t span(int layer);
  private:
    ShadowdNode frees;
    std::vector<size_t> maxis;
    uint64_t totalBlocks = 0;
    std::string path;
    size_t readFrees();
    size_t generateBlock();
};

using AllocBlock = AllocatorBlocks;
using Alloc = AllocatorBlocks;