#include "../Helpers/RandomAccessFile.hpp"
#include "BaseShadowdBlock.hpp"
#include "../Helpers/BigEndianCover.hpp"
#include <cstddef>
#include <cstdint>
#include <sstream>
#include <vector>

BaseShadowdBlock::BaseShadowdBlock(int indexVar, AllocBlock& allocVar, size_t disk_idVar): 
    alloc(allocVar), index(indexVar), size(alloc.blockSize), DATA(size-4),
    start(alloc.blockSize*indexVar), disk_id(disk_idVar)
{
    readIntern(); //Carga el estado del bloque
}
std::vector<uint8_t> BaseShadowdBlock::read(){
    return buffer;
}
void BaseShadowdBlock::writeIntern(){
    std::string path = alloc.getDiskPath(disk_id);
    RandomAccessFile raf(path);
    size_t toWrite = (buffer.size() < DATA) ? buffer.size() : DATA;
    size_t count = start;

    size_t countWrite = (writed < DATA) ? writed : DATA;
    std::vector<uint8_t> hesder = sizeToBe(countWrite, HEAD);

    raf.writeAt(hesder.data(), HEAD, count);
    count += HEAD;

    raf.writeAt(buffer.data(), toWrite, count);
    
    isFree = (countWrite == 0);
}
int BaseShadowdBlock::writeBlock(std::vector<uint8_t> data){}
std::vector<uint8_t> BaseShadowdBlock::readTo(Cursor& start, size_t size){}
void BaseShadowdBlock::readIntern(){}
std::string BaseShadowdBlock::toString() const{}