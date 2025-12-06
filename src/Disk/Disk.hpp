#pragma once
#include "../Block/AllocatorBlocks.hpp"
#include "Metadata.hpp"
#include <string>

class Disk {
  public:
    Disk(std::string pathVar, int blockSizeVar);

  private:
    Metadata meta;
    int MetaFilesMax = 32;
    
};