#pragma once
#include "../Block/AllocatorBlocks.hpp"
#include "Metadata.hpp"
#include "../Block/BaseShadowdBlock.hpp"
#include "../Files/ShadowdNode.hpp"
#include <map>
#include <string>
#include <sys/types.h>

class Disk {
  public:
    Disk(std::string pathVar, int blockSizeVar);

  private:
    Metadata meta;
    AllocatorBlocks alloc;
    int MetaFilesMax = 32;
    std::map<int, BaseShadowdBlock> SBTotals; // ShadowdBlocks totals, is referent of the actual active basic blcks
    SDirectory rootDirectory; 
};