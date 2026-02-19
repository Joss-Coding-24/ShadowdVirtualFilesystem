/*
* esto es basicamente una serie de
* estructuras y enumeraciones para
* indicar tanto si una insersion se dio o
* no tanto como si sobraron items y la
* cantidad de los mismos 
*/
#pragma once
#include <cstddef>
#include <cstdint>
#include <vector>
#include "../Algoritm/Cursors.hpp"

enum class InsertResultItem{
    BUFFER_IS_FULL, //buffer lleno
    INSERTED_WHITHOUT_REMAINING, // todo de inserto completamente
    INSERTED_WHITH_REMAINING, // sobraron bytes 
    FAILL
};

enum class BufferStates{
    FULL, // esta lleno
    EMPTY, // esta vacio
    PARTIALLY_FULL //punto medio
};

enum class TransitOption{
    INSERT_BEGIN, //inisertar al incio
    INSERT_END, //insertar al final
    INSERT_IN_POS, //Insertar en la posicion indicada
    DELETE_POS_BYTES_TO_BEGIN, // usa pos como indicador de cuantos bytes borrar al inicio
    DELETE_POS_BYTES_TO_END, // usa pos como.indicadoe de cusntos bytes borrar al final
    DELETE_POS_DEFAULT, // borrar desde buffer[pos] hasta buffer[pos+8]
    DELETE_POS_TO_INDICATOR, // borar desde buffer[pos] hasta buffer[pos+indicator]
    FINALIZE
};

enum class TransitStates{
    IGNORE, // Literamete, ignora lo demas
    MOVE_TO_END, //lo que retoeno ponlo al inicio del siguiente bloque, si no existe crealo, si no puedes notica
    MOVE_TO_BEGIN, //Toma del inicio del siguiente bloque y ponlo en el final del actual
    ERROR_1, //Falla menor, casi siempre es falta de bloques a los que propagar la modicacion
    ERROR_2, //Falla media, casi siempre, se intento crear un bloque pero fallo, reintenta 10 veces mas
    ERROR_3, //Falla grave, un bloque corrupto o perdida de datoa 
    OK
};

enum class LayerState{
    OBTAINED, //layer obtenida
    LOADED, // layer cargada
    GENERATED, // layer generada
    NOT_SPACE, // el disco esta lleno
    OUT_OF_RANGE //se salio del rango de un disco
};

struct InsertResult{
    InsertResultItem result;
    BufferStates state;
    size_t remaining;
    size_t writed;
};

struct TransitOptions{
    TransitOption option;
    Cursor& pos;
    size_t ind;
    bool incrementSize; // false para no insertar si supera la capacidad de capas
    std::vector<uint8_t> data;
};

struct TransitReturn{
    TransitOption action;
    TransitStates estado;
    std::vector<uint8_t> data;
    bool incrementSize;
};

template <typename block> 
struct LayerResult{
    LayerState state;
    block result;
};
