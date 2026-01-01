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
            entries.resize(31);
            root.pos = indexVar;
            root.block = allocVar.get<typeBlock>(indexVar);
            getIntern(0);
        }
        void writeIntern()
        {
            size_t saved = countSave;
            size_t writed = countWrite;
            if(saved == writed) return;

            for(size_t i = saved; i < writed; i++){
                entryType entry = entries[i];
                if(!entry.valid){
                    countWrite = i;
                    return;
                }
                entry.block.writeIntern();
                countSave++;
            }
        }
        InsertResult writeBlock(std::vector<uint8_t> data)
        {
            int layer = entryType::layer+1;
            size_t offset = 0;
            size_t remaning = data.size(); //checksum primitivo
            while(true){
                size_t i = countWrite;

                if(data.size()<offset) break; // revisar si aun quedan buffers

                std::vector<uint8_t> buff;
                buff.insert(buff.end(), data.begin()+offset, data.end());

                if(entries.size() <= i)
                    if(LayerState::OBTAINED!=getIntern(i).state)
                        break; //no se pudo obtener

                if(buff.size()==0) break; //por si acaso reviso otra vez

               entryType& entry = entries[i];
                if(!entry.valid){
                    if(genIntern(i).state != LayerState::GENERATED){
                        break;
                    }else {
                        continue;
                    };
                }

                typeBlock& block = entry.block;
                InsertResult result = block.writeBlock(buff);
                if(result.result == InsertResultItem::FAILL)
                    return {InsertResultItem::FAILL};

                if(result.result == InsertResultItem::INSERTED_WHITHOUT_REMAINING)
                    break; // ya termino
                offset += result.writed;
                if(remaning == result.remaining+result.writed){
                    //realizo bien el trabajo
                    remaning = result.remaining;
                }else{
                    //algo fallo... malio sal el sistema xd
                    return {
                        InsertResultItem::FAILL
                    };
                }
            }
            InsertResult result;

            if(remaning > 0) {
                result.result = InsertResultItem::INSERTED_WHITH_REMAINING;
                result.remaining = remaning;
            }else{
                result.result = InsertResultItem::INSERTED_WHITHOUT_REMAINING;
            }
            result.writed = offset;
            return result;
        }
        bool clearLoteBlock()
        {
            int layer = entryType::layer+1;
            size_t max = 31;
            for(size_t i = 0; i < max; i++) {
                if(!entries[i].valid) {
                    if(getIntern(i, false) < 0) break;
                }
                entryType& entry = entries[i];
                entry.block.clearLoteBlock();
                alloc.freeBlock(entry.pos);
            }
            root.block.clearLoteBlock();
            entries.clear();
            countSave = 0;
            countWrite = 0;
        }
        void deleteEntry(entryType entry)
        {
            auto it = std::find_if(
                entries.begin(),
                entries.end(),
                [&](const entryType& e){
                return e.pos == entry.pos;
                }
            );
            if(it == entries.end()) return;

            for(auto jt = it; jt != entries.end(); ++jt){
                jt->block.clearLoteBlock();
                alloc.freeBlock(jt->pos);
            }

            entries.erase(it, entries.end());

            //Actualizamos root
            root.block.clearLoteBlock();
            root.block.writeBlock(entries.data());
            root.block.writeIntern();
            //no uso removeYo porque desconosco el cursor ademas, al ser una eliminacion completa, asi esta mejor
        }
        entryType getEntry(size_t pos, bool create = true)
        {
            if(!entries[pos].valid){
                if(create){
                    if(getIntern(pos).state != LayerState::GENERATED){
                        return {.valid = false};
                    }
                }else{
                    return {.valid = false};
                }
            }
            return entries[pos];

        }
        std::string toString()
        {
            int layer = entryType::layer+1;
        
            int head = (6 - layer)*2;
            int tabs = head-1;
            
            if(tabs < 0) tabs = 0;
            if(head < 1) head = 1;
            
            auto makeTabs = [&](int n){
                return std::string(n, '\t');
            };
            std::ostringstream oss;

            oss << makeTabs(tabs) << alloc.getBlockName(layer) << "(\n";
            oss << makeTabs(head) << "Root[\n";
            oss << root.block.toString();
            oss << makeTabs(head) << "]\n";
            oss << makeTabs(head) << "Datas[\n";
            for(size_t i = 0; i<entries.size(); i++){
                if(!entries[i].valid) continue;
                oss << entries[i].block.toString();
            }
            oss << makeTabs(head) << "]\n";
            oss << makeTabs(tabs) << ")\n";
            return oss.str();
        };
        std::vector<uint8_t> readTo(Cursor& startPos, size_t size){}
        std::vector<uint8_t> readTo(Cursor& startPos){
            return readTo(startPos, 9);
        }
        TransitReturn removeTo(TransitOptions& options){}
        TransitReturn insertTo(TransitOptions& options){}
    private:
        size_t countSave = 0;
        size_t countWrite = 0;
        size_t disk_id;
        Alloc& alloc;
        std::vector<entryType> entries;
        entryType root;
        LayerResult<typeBlock> getIntern(size_t index, bool create = true){}
        LayerResult<typeBlock> loadIntern(size_t pos){}
        LayerResult<typeBlock> genIntern(size_t index){}

};
template<int L, typename BlockType>
struct EntryShadowdBlock {
    size_t pos;
    BlockType block;
    static constexpr int layer = L;
    bool valid = false;
};