#pragma once
/**
 * template para capa 2+
 */
#include <vector>
#include <string>
#include "AllocatorBlocks.hpp"
#include "../Helpers/BigEndianCover.hpp"
#include <algorithm>
#include <sstream>
 
 template <typename typeBlock, typename entryType>
 class ShadowdBlock{
   public:
    explicit ShadowdBlock(size_t rootPos, AllocBlock* allocVal){
      alloc = allocVal;
      root.pos = rootPos;
      root.block = alloc->get<typeBlock>(rootPos);
      getIntern();
    }
    void writeIntern(){
      size_t saved = countSave;
      size_t written = countWrite;

      if (saved == written) return;

      for (size_t i = saved; i < written; i++) {
        entryType entry = entries[i];
        if(!entry.valid) {
          countWrite = i;
          return;
        }
        entry.block.writeIntern();
        countSave++;
      }
    }
    int writeBlock(std::vector<uint8_t> data){
      int layer = entryType::layer+1;
      size_t max = alloc->max(layer);
      size_t off = 0;

      while (true) {
        size_t pos = countWrite;
        std::vector < uint8_t > tmp;
        tmp.insert(tmp.end(), data.begin()+off, data.end());

        size_t count = countWrite;
        if(count > max) break;
        if(pos >= entries.size()) {
          int code = getIntern();
          if(code < 0) return code;
        }
        size_t size = tmp.size();
        if(size == 0) break;

        entryType entry = entries[pos];
        if(!entry.valid) {
          getIntern();
          continue;
        }
        typeBlock block = entry.block;
        int writed = block.writeBlock(tmp);
        if(writed == -1 && off == 0) {
          countWrite++;
          continue;
        }else {
          block.clearLoteBlock();
          continue;
        }
        off += static_cast<size_t > (writed);
        if(size <= off) break;
      }
      return static_cast<int>(off);
    }
    void clearLoteBlock(){
      int layer = entryType::layer+1;
      size_t max = static_cast<size_t > (alloc->max(layer));
      for(size_t i = 0; i < max; i++) {
        if(i > entries.size()) {
          if(getIntern(false) < 0) break;
        }
        entryType entry = entries[i];
        entry.block.clearLoteBlock();
        alloc->freeBlock(entry.pos);
      }
      root.block.clearLoteBlock();
      entries.clear();
    }
    void deleteEntry(entryType entry){
      entry.block.clearLoteBlock();
      alloc->freeBlock(entry.pos);
      entries.erase(
        std::remove_if(
          entries.begin(),
          entries.end(),
          [&](const entryType& e) {
            return e.pos == entry.pos;
          }
        ),
        entries.end()
      );
    }
    entryType getEntry(size_t pos, bool create = true){
      int layer = entryType::layer+1;
      size_t max = alloc->max(layer);
      if(pos > 0 && pos <= max){
        if(pos > entries.size()) {
          if(!create) {
            entryType type;
            type.valid = false;
            return  type;
          }
          if(getIntern(create) < 0) {
            entryType type;
            type.valid = false;
            return  type;
          }
        }
        return entries[pos];
      }else{
        entryType type;
        type.valid = false;
        return type;
      }
    }
    std::string toString(){
      int layer = entryType::layer+1;
  
      int head = (6 - layer)*2;
      int tabs = head-1;
      
      if(tabs < 0) tabs = 0;
      if(head < 1) head = 1;
      
      auto makeTabs = [&](int n){
        return std::string(n, '\t');
      };
      std::ostringstream oss;

      oss << makeTabs(tabs) << alloc->getBlockName(layer) << "(\n";
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
    }
    int next(){
      size_t count = countNext;
      if(count == entries.size()) return -2;
      int result = entries[count].next();
      if(result == -1) {
        countNext++;
        return next(); //si ya se leyo entonces avanzar
      }
      if(result <= -2) {
        return -2;
      }
      return result;
    }
  private:
    size_t countSave = 0;
    size_t countWrite = 0;
    size_t countNext = 0;
    Alloc* alloc;
    std::vector<entryType> entries;
    entryType root;
    int getIntern(bool create = true){
      int layer = entryType::layer+1;

      if (entries.size() + 1 > alloc -> max(layer)) return -2;

      uint8_t* next = root.block->readTo(entries.size() * 8);
      if (!next) {
        return create ? genIntern(): -3;
      }

      int v = beToInt(next, 8);
      loadIntern(v);
      return v;
    }
    void loadIntern(int pos){
      entryType back;
      back.valid = true;
      back.pos = pos;
      back.block = alloc->get < typeBlock > (pos);
      entries.push_back(back);
    }
    int genIntern(){
      int layer = entryType::layer+1;
      if(entries.size()+1 > alloc->max(layer)) return -2;
      size_t pos = alloc->gen();
      if(pos == 0) return -3;
      std::vector < uint8_t > ending;
      uint8_t* data = intToBe(pos, 8);
      ending.insert(ending.end(), data, data+8);
      int writed = root.block -> writeBlock(ending);
      if(writed <= 0) return -1;
      root.block -> writeIntern();
      loadIntern(pos);
      return pos;
    }
 };

template<int L, typename BlockType>
struct EntryShadowdBlock {
    size_t pos;
    BlockType block;
    static constexpr int layer = L;
    bool valid;
};