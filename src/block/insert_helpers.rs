//! Estructuras y enumeraciones para indicar:
//! - si una inserción ocurrió o no
//! - si sobraron items
//! - cuántos sobraron

use std::vec::Vec;
use std::usize;

// Asumiendo que Cursor existe en algún módulo
use crate::algoritm::cursors::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertResultItem {
    BufferIsFull,              // buffer lleno
    InsertedWithoutRemaining,  // todo se insertó completamente
    InsertedWithRemaining,     // sobraron bytes
    Fail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferStates {
    Full,           // está lleno
    Empty,          // está vacío
    PartiallyFull,  // punto medio
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitOption {
    InsertBegin,              // insertar al inicio
    InsertEnd,                // insertar al final
    InsertInPos,              // insertar en la posición indicada
    DeletePosBytesToBegin,    // usa pos como cantidad de bytes a borrar al inicio
    DeletePosBytesToEnd,      // usa pos como cantidad de bytes a borrar al final
    DeletePosDefault,         // borrar desde buffer[pos] hasta buffer[pos + 8]
    DeletePosToIndicator,     // borrar desde buffer[pos] hasta buffer[pos + indicator]
    Finalize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitStates {
    Ignore,        // ignora lo demás
    MoveToEnd,     // lo retornado va al inicio del siguiente bloque
    MoveToBegin,   // toma del inicio del siguiente bloque y ponlo al final del actual
    Error1,        // falla menor (falta de bloques)
    Error2,        // falla media (fallo al crear bloque, reintentar)
    Error3,        // falla grave (bloque corrupto o pérdida de datos)
    Ok,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerState {
    Obtained,     // layer obtenida
    Loaded,       // layer cargada
    Generated,    // layer generada
    NotSpace,     // disco lleno
    OutOfRange,   // fuera del rango del disco
}

#[derive(Debug)]
pub struct InsertResult{
    pub result: InsertResultItem,
    pub state: BufferStates,
    pub remaining: usize,
    pub written: usize,
    pub remaining_bytes:Vec<u8>
}

#[derive(Debug)]
pub struct TransitOptions<'a> {
    pub option: TransitOption,
    pub pos: &'a mut Cursor,
    pub indicator: usize,
    pub increment_size: bool, // false para no insertar si supera la capacidad de capas
    pub data:&'a mut Vec<u8>,
}

#[derive(Debug)]
pub struct TransitReturn {
    pub action: TransitOption,
    pub state: TransitStates,
    pub data: Vec<u8>,
    pub increment_size: bool,
}

#[derive(Debug)]
pub struct LayerResult<T> {
    pub state: LayerState,
    pub result: T,
}