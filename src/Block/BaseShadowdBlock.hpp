/*
* Para ver la versuon de bloque anterior revisar el archivo
* .txt con nombre similar a este 
*/

#pragma once
#include "AllocatorBlocks.hpp"
#include <cstddef>
#include <cstdint>
#include <sys/types.h>
#include <vector>
#include <string>
#include "../Algoritm/Cursors.hpp"
#include "InsertsHelpers.hpp"
/*
* bloque de capa 
* bloque base de toda la infraestructura
* Funcion: Hojas
*/
struct EntryBaseShadowdBlock;

class BaseShadowdBlock{
    public:
        explicit BaseShadowdBlock(int indexVar, AllocBlock& allocVar, size_t disk_idVar);
        bool isFree;
        size_t freeBytes;
        size_t writed;
        std::vector<uint8_t> read();
        void writeIntern();
        InsertResult writeBlock(std::vector<uint8_t> data);
        std::vector<uint8_t> readTo(Cursor& startPos, size_t size);
        std::vector<uint8_t> readTo(Cursor& startPos){
            return readTo(startPos, 9);
        }
        bool clearLoteBlock();
        TransitReturn removeTo(TransitOptions& options);
        TransitReturn insertTo(TransitOptions& options);
    private: 
        size_t index;
        AllocBlock& alloc; // alias de AllocatorBlock
        void readIntern();
        size_t disk_id;
        uint64_t start;
        uint64_t size;
        int HEAD = 4;
        size_t DATA;
        std::vector<uint8_t> buffer;
        std::string toString() const;
};

using bSB = BaseShadowdBlock;

struct EntryBaseShadowdBlock{
  size_t pos; //Position in disk
  bSB& block;
  const int layer = 1;
  bool valid;
};

using EbSB = EntryBaseShadowdBlock;