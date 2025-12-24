#include "../Helpers/RandomAccessFile.hpp"
#include "BaseShadowdBlock.hpp"
#include "../Helpers/BigEndianCover.hpp"
#include <cstddef>
#include <sstream>
#include <vector>

BaseShadowdBlock::BaseShadowdBlock(int indexVar, AllocBlock& allocVar): alloc(allocVar){
  index = indexVar;
  size = alloc.blockSize;
  HEAD = 4;
  DATA = size-4;
  start = (size*index)+4; // el tamaño de bloque se guarda en 4b
  readIntern();
  writed = 0;
  isFree = true;
  freeBytes = DATA;
}
std::vector < uint8_t > BaseShadowdBlock::read() {
  return buffer;
}
void BaseShadowdBlock::writeIntern() {
  std::string path = alloc.getDiskPath();
  RandomAccessFile raf(path);

  size_t toWrite = (buffer.size() < DATA) ? buffer.size(): DATA;
  size_t count = start;

  std::vector<uint8_t> header = sizeToBe(writed, HEAD);

  raf.writeAt(header.data(), HEAD, count);
  count += HEAD;

  raf.writeAt(buffer.data(), toWrite, count);

  isFree = (writed == 0);
}
int BaseShadowdBlock::writeBlock(std::vector < uint8_t > data) {
  if (freeBytes == 0) return -1;

  size_t remaining = freeBytes;

  // el operador ternario estaba mal escrito
  size_t toWrite = (data.size() < remaining) ? data.size(): remaining;

  // insertamos solo la parte útil de 'data'
  buffer.insert(
    buffer.end(),
    data.begin(),
    data.begin() + toWrite
  );

  writed += toWrite;
  freeBytes -= toWrite;
  
  if(freeBytes == 0){
    writeIntern();
  }
  
  return toWrite;
}
void BaseShadowdBlock::clearLoteBlock(bool clearData) {
  writed = 0;
  freeBytes = DATA;
  if(clearData) {
    buffer.clear();
    writeIntern();
  }
  isFree = true;
}
bool BaseShadowdBlock::removeToLast(int end) {
  if (end < 0) {
    return false; // no queremos rangos negativos
  }

  size_t sz = buffer.size();
  if (sz < static_cast<size_t> (end)) {
    return false; // no hay suficientes bytes
  }

  size_t start = sz - end;
  buffer.erase(buffer.begin() + start, buffer.end());

  return true;
}
std::vector < uint8_t > BaseShadowdBlock::readTo(size_t start, size_t end) {
  std::vector < uint8_t > out;

  // comprobación básica de rangos
  if (start > end) return out; // rango invertido
  if (end > buffer.size()) return out; // intenta leer más allá
  if (start >= buffer.size()) return out; // empieza fuera del buffer

  // copiamos el rango deseado
  out.insert(out.end(), buffer.begin() + start, buffer.begin() + end);

  return out;
}
std::vector < uint8_t > BaseShadowdBlock::readTo(size_t start) {
  // si quedan menos de 4 bytes desde 'start', no se puede leer un bloque completo
  if (start + 8 > buffer.size()) {
    return {}; // vacío
  }

  return readTo(start, start + 8);
}
void BaseShadowdBlock::readIntern() {
  buffer.clear();
  size_t count = start;
  std::string path = alloc.getDiskPath();
  RandomAccessFile raf(path);

  uint8_t head[HEAD];
  raf.readAt(head, HEAD, count);
  count += HEAD;
  std::vector<uint8_t> headBytes;
  headBytes.insert(headBytes.end(), head, head+HEAD);

  writed = beToSize(headBytes, HEAD);
  if(writed>DATA) writed = DATA;
  if(writed > 0) {
    freeBytes = DATA - writed;
    isFree = false;
  }else {
    freeBytes = DATA;
    isFree = true;
  }

  std::vector < uint8_t > buff(writed);
  raf.readAt(buff.data(), writed, count);

  buffer.insert(buffer.end(), buff.begin(), buff.end());
}
std::string BaseShadowdBlock::toString() const {
  std::ostringstream oss;

  auto makeTabs = [&](int n){
    return std::string(n, '\t');
  };
  
  oss << makeTabs(9) << "BaseShadowdBlock(\n";
  oss << makeTabs(10) << "index=" << index << '\n';
  oss << makeTabs(10) << "start=" << start << '\n';
  oss << makeTabs(10) << "writed=" << writed << '\n';
  oss << makeTabs(10) << "free=" << freeBytes << '\n';
  oss << makeTabs(9) << ")" << '\n';

  return oss.str();
}
std::vector<uint8_t> BaseShadowdBlock::removeAngGetToLast(int end) {
    if (end <= 0) {
        return {}; // no hay nada que cortar
    }

    const int size = static_cast<int>(buffer.size());
    if (end >= size) {
        // Se corta absolutamente todo
        std::vector<uint8_t> all = buffer;
        buffer.clear();
        return all;
    }

    // rango de los bytes finales
    std::vector<uint8_t> tail(buffer.end() - end, buffer.end());

    // eliminar los últimos 'end' bytes
    buffer.erase(buffer.end() - end, buffer.end());

    return tail;
}
size_t BaseShadowdBlock::next(){
  if (countNext + 8 > writed) return -1;
  auto v = beToSize(readTo(countNext), 8);
  countNext += 8;
  return v;
}