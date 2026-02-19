#pragma once

#include <cstdint>
#include <cstddef>
#include <vector>
#include <type_traits>

//
// Conversión genérica (infraestructura)
//

// Convierte un entero a representación big-endian
template<typename T>
std::vector<uint8_t> toBigEndian(T value, size_t bytes);

// Convierte bytes big-endian a entero
template<typename T>
T fromBigEndian(const std::vector<uint8_t>& data, size_t bytes);


// signed
inline std::vector<uint8_t> intToBe(int32_t v, size_t b) {
    return toBigEndian<int32_t>(v, b);
}
inline int32_t beToInt(const std::vector<uint8_t>& v, size_t b) {
    return fromBigEndian<int32_t>(v, b);
}

// unsigned
inline std::vector<uint8_t> sizeToBe(size_t v, size_t b){
    return toBigEndian<size_t>(v, b);
}
inline size_t  beToSize(std::vector<uint8_t> v, size_t b){
    return fromBigEndian<size_t>(v, b);
}
inline std::vector<uint8_t> u32ToBe(uint32_t v, size_t b) {
    return toBigEndian<uint32_t>(v, b);
}
inline uint32_t beToU32(const std::vector<uint8_t>& v, size_t b) {
    return fromBigEndian<uint32_t>(v, b);
}
inline std::vector<uint8_t> u64ToBe(uint64_t v, size_t b) {
    return toBigEndian<uint64_t>(v, b);
}
inline uint64_t beToU64(const std::vector<uint8_t>& v, size_t b) {
    return fromBigEndian<uint64_t>(v, b);
}
template<typename T>
std::vector<uint8_t> toBigEndian(T value, size_t bytes) {
    static_assert(std::is_integral_v<T>, "toBigEndian solo acepta tipos enteros");

    std::vector<uint8_t> out(bytes, 0);
    for (size_t i = 0; i < bytes; ++i) {
        out[bytes - 1 - i] = static_cast<uint8_t>(value & 0xFF);
        value >>= 8;
    }
    return out;
}

template<typename T>
T fromBigEndian(const std::vector<uint8_t>& data, size_t bytes) {
    static_assert(std::is_integral_v<T>, "fromBigEndian solo acepta tipos enteros");

    T value = 0;
    for (size_t i = 0; i < bytes; ++i) {
        value = static_cast<T>((value << 8) | data[i]);
    }
    return value;
}