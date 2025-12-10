#pragma once
#include <string>
#include <vector>
#include "../Files/ShadowdNode.hpp"
#include "../Disk/Metadata.hpp"
#include "../Helpers/RandomAccessFile.hpp"

class AllocatorBlocks{
  public:
    explicit AllocatorBlocks(Metadata& metadata);
    template<typename BlockType>
    BlockType* get(size_t pos);
    void freeBlock(size_t pos);
    size_t gen();
    int blockSize = 252;
    std::string getDiskPath();
    size_t max(int layer);
    std::string getBlockName(int layer);
  private:
    uint64_t totalBlocks;
    SFile frees;
    std::vector<size_t> maxis;
    std::string path;
    size_t readFrees();
    size_t generateBlock();
};

using AllocBlock = AllocatorBlocks;
using Alloc = AllocatorBlocks;