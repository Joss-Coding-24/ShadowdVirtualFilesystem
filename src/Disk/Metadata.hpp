#pragma once
#include "../Files/ShadowdFile.hpp"
#include <string>
#include "../Files/ShadowdDirectory.hpp"

class Metadata{
  public:
  Metadata(int blockSizeVar, std::string pathVar);
  std::string path;
  int sizeBlock;
  SFile freesFile;
  SDirectory load();
  SDirectory make();
};