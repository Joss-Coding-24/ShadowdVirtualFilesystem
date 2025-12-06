#pragma once
#include "../Files/ShadowdFile.hpp"
#include <string>

class Metadata{
  public:
  Metadata(int blockSizeVar, std::string pathVar);
  std::string path;
  int sizeBlock;
  SFile freesFile;
};