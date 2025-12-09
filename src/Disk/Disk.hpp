#pragma once
#include "../Block/AllocatorBlocks.hpp"
#include "Metadata.hpp"
#include "../Files/ShadowdNode.hpp"
#include <string>
#include <sys/types.h>

class Disk {
  public:
    Disk(std::string pathVar, int blockSizeVar);
    void persist();
    bool backup();
    bool restore(int inten = 0);
  private:
    Metadata meta;
    AllocatorBlocks alloc;
    SDirectory rootDirectory; 
};