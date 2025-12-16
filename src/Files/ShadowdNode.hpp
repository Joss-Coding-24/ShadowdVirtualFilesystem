#pragma once
#include <vector>
#include <cstdint>

class ShadowdNode{
  public:
    void AddToLast(std::vector<uint8_t> data);
    std::vector<uint8_t> removeAndGetToLast(size_t end){
      std::vector<uint8_t> last;
      for(size_t i = 0; i < end; i++) last.push_back(0);
      return last;
    }
};

using SFile = ShadowdNode;
using SDirectory = ShadowdNode;