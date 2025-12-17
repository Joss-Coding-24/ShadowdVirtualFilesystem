#pragma once
#include <vector>
#include <cstdint>

class AllocatorBlocks;

class ShadowdNode{
  public:
    ShadowdNode(AllocatorBlocks& alloc, size_t pos);
    void AddToLast(std::vector<uint8_t> data);
    std::vector<uint8_t> removeAndGetToLast(size_t end){
      std::vector<uint8_t> last;
      for(size_t i = 0; i < end; i++) last.push_back(0);
      return last;
    }
    void persist();
    void updateTree();
};

using SFile = ShadowdNode;
using SDirectory = ShadowdNode;