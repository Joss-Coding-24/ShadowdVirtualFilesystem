#pragma once
#include "AllocatorBlocks.hpp"
#include <vector>
#include <string>

/**
 * bloque de capa 1
 * este es el bloque base de toda la infraestructura
 */

class BaseShadowdBlock{
  public:
    explicit BaseShadowdBlock(int indexVar, AllocBlock& allocVar);
    bool isFree;
    size_t freeBytes;
    size_t writed;
    std::vector<uint8_t> read();
    void writeIntern();
    int writeBlock(std::vector<uint8_t> data);
    void clearLoteBlock(bool clearData=true);
    bool removeToLast(int end = 8);
    std::vector<uint8_t> removeAngGetToLast(int end = 8);
    std::vector<uint8_t> readTo(size_t start, size_t end);
    std::vector<uint8_t> readTo(size_t start);
    int next();
  private:
    size_t countNext = 0;
    int index;
    Alloc alloc; // alias de AllocatorBlock
    void readIntern();
    long start;
    long size;
    uint8_t HEAD;
    int DATA;
    std::vector<uint8_t> buffer;
    std::string toString() const;
};

using bSB = BaseShadowdBlock;

struct EntryBaseShadowdBlock{
  size_t pos; //Position in disk
  bSB block;
  const int layer = 1;
  bool valid;
};

using EbSB = EntryBaseShadowdBlock;