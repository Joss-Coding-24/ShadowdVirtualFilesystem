#pragma once
#include <string>
#include "../Files/ShadowdNode.hpp"

class AllocatorBlocks;

class Metadata{
  public:
    Metadata(int blockSizeVar, std::string pathVar);
    std::string path;
    int sizeBlock;
    SDirectory load(AllocatorBlocks& alloc);
    SDirectory make(AllocatorBlocks& alloc);
    void persist(SDirectory dir);
  private:
}; 