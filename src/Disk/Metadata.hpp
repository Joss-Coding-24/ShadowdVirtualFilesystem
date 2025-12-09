#pragma once
#include <string>
#include "../Files/ShadowdNode.hpp"

class Metadata{
  public:
  Metadata(int blockSizeVar, std::string pathVar);
  std::string path;
  int sizeBlock;
  SFile freesFile;
  SDirectory load();
  SDirectory make();
  void persist(SDirectory dir);
  SDirectory loadOrGenerateDisk();
};