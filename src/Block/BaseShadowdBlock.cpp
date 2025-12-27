#include "../Helpers/RandomAccessFile.hpp"
#include "BaseShadowdBlock.hpp"
#include "../Helpers/BigEndianCover.hpp"
#include "InsertsHelpers.hpp"
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
InsertResult BaseShadowdBlock::writeBlock(std::vector<uint8_t> data){
    if(freeBytes==0)return {InsertResultItem::BUFFER_IS_FULL};
    size_t remaining = freeBytes;

    size_t toWrite = (data.size() < remaining) ? data.size() : remaining;

    buffer.insert(
        buffer.end(),
        data.begin(),
        data.begin()+toWrite
    );

    writed += toWrite;
    freeBytes -= toWrite;
    bool isFully = false;
    if(freeBytes == 0){
        isFully = true;
        writeIntern();
    }
    InsertResult result;
    result.writed = toWrite;
    if(isFully){
        result.state = BufferStates::FULL;
    }else if(freeBytes == DATA){
        result.state = BufferStates::EMPTY;
    }else{
        result.state = BufferStates::PARTIALLY_FULL;
    }
    if(toWrite == data.size()){
        result.result =InsertResultItem::INSERTED_WHITHOUT_REMAINING;
        result.remaining = 0;
    }else {
        result.result =InsertResultItem::INSERTED_WHITH_REMAINING;
        result.remaining = data.size()-toWrite;
    }
    return result;
}
bool BaseShadowdBlock::clearLoteBlock(){
    writed = 0;
    freeBytes = DATA;
    buffer.clear();
    writeIntern();
    isFree = true;
    return true;
}
std::vector<uint8_t> BaseShadowdBlock::readTo(Cursor& cur, size_t size){
    std::vector<uint8_t> out;
    size_t start = cur.getActuallyC1Pos();
    size_t end = start + size;

    if(end > buffer.size()) return out;
    if(start >= buffer.size()) return out;
    out.insert(out.end(), buffer.begin() + start, buffer.begin() + end);
    return out;
}
void BaseShadowdBlock::readIntern(){}
TransitReturn removeBegin(Cursor& cur, std::vector<uint8_t>& buffer, bool incrementSize){
    size_t pos = cur.getActuallyC1Pos() ;
    if( pos == 0) return {.estado = TransitStates::ERROR_1};
    std::vector<uint8_t> data;
    if(buffer.size() < pos) {
        data.insert(data.end(), buffer.begin(), buffer.end());
        buffer.erase(buffer.begin(), buffer.end());
    }else {
        data.insert(data.end(), buffer.begin(), buffer.begin()+pos);
        buffer.erase(buffer.begin(), buffer.begin()+pos);
    }
    return {
        TransitOption::INSERT_END,
        TransitStates::MOVE_TO_BEGIN,
        data,
        incrementSize
    };
}
TransitReturn removeEnd(Cursor& cur, std::vector<uint8_t>& buffer, bool incrementSize){
    size_t pos = cur.getActuallyC1Pos();
    if(pos == 0) return {.estado = TransitStates::ERROR_1};
    std::vector<uint8_t> data;
    if(buffer.size() < pos){
        data.insert(data.end(), buffer.begin(), buffer.end());
        buffer.erase(buffer.begin(), buffer.end());
    }else{
        size_t sstart = buffer.size()-pos;
        data.insert(data.end(), buffer.begin()+sstart, buffer.end());
        buffer.erase(buffer.begin()+sstart, buffer.end());
    }
    return {
        TransitOption::INSERT_END,
        TransitStates::MOVE_TO_BEGIN,
        data,
        incrementSize
    };    
}
TransitReturn removeInRange(Cursor& cur, size_t ind, std::vector<uint8_t> buffer, bool incrementSize){
    size_t pos = cur.getActuallyC1Pos();
    size_t finish = pos + ind;
    std::vector<uint8_t> data;
    data.insert(data.end(), buffer.begin()+pos, buffer.begin()+finish);
    buffer.erase(buffer.begin()+pos, buffer.begin()+finish);
    return {
        TransitOption::INSERT_END,
        TransitStates::MOVE_TO_BEGIN,
        data,
        incrementSize
    };
}
TransitReturn BaseShadowdBlock::removeTo(TransitOptions& options){
    // extramos las variables que vamos a usar
    bool incrementSize = options.incrementSize;
    size_t indicador = options.ind;
    TransitOption option = options.option;
    Cursor& cur = options.pos;

    //vemos que opcion debemos de elegir
    switch (option) {
        case TransitOption::DELETE_POS_BYTES_TO_BEGIN: return removeBegin(cur, buffer, incrementSize);
        case TransitOption::DELETE_POS_BYTES_TO_END: return removeEnd(cur, buffer, incrementSize);
        case TransitOption::DELETE_POS_DEFAULT: return removeInRange(cur, 8, buffer, incrementSize);
        case TransitOption::DELETE_POS_TO_INDICATOR: return removeInRange(cur, indicador, buffer, incrementSize);;
        default: return {.estado = TransitStates::IGNORE};
    }
}
TransitReturn insertTo(TransitOptions& options){}
std::string BaseShadowdBlock::toString() const{}