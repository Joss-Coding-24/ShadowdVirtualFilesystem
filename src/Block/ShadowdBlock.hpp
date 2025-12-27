/*
* El codigo de la version anterior se
* encuentra em el .txt con el mismo nombre
* que este archivo
*/

#pragma once
/**
 * template para capa 2+
 */
#include <cstddef>
#include <cstdint>
#include <vector>
#include <string>
#include "AllocatorBlocks.hpp"
#include "../Helpers/BigEndianCover.hpp" 
#include "../Algoritm/Cursors.hpp"
#include "InsertsHelpers.hpp"
#include <algorithm>
#include <sstream>

template <typename typeBlock, typename entryType>
class ShadowdBlock{
    public:
        explicit ShadowdBlock(uint64_t indexVar, AllocBlock& allocVar, size_t disk_idVar):
            alloc(allocVar), disk_id(disk_idVar)
            {
                root.pos = indexVar;
                root.block = allocVar.get<typeBlock>(indexVar);
                getIntern();
            }
            void writeIntern(){}
            InsertResult writeBlock(std::vector<uint8_t> data){}
            bool clearLoteBlock(){}
            void deleteEntry(entryType entry){}
            entryType getEntry(size_t pos, bool create = true){}
            std::string toString(){};
            std::vector<uint8_t> readTo(Cursor& startPos, size_t size);
            std::vector<uint8_t> readTo(Cursor& startPos){
                return readTo(startPos, 9);
            }
            TransitReturn removeTo(TransitOptions& options);
            TransitReturn insertTo(TransitOptions& options);
        private:
            size_t countSave = 0;
            size_t countWrite = 0;
            size_t disk_id;
            Alloc& alloc;
            std::vector<entryType> entries;
            entryType root;
            void getIntern(bool create = true){}
            void loadIntern(size_t pos);
            void genIntern();

};
template<int L, typename BlockType>
struct EntryShadowdBlock {
    size_t pos;
    BlockType block;
    static constexpr int layer = L;
    bool valid;
};