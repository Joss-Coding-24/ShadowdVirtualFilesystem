use crate::{
    algoritm::cursors::Cursor, 
    block::{
        allocator_block::AllocatorBlock,
        insert_helpers::{
            BufferStates, 
            InsertResult, 
            InsertResultItem, 
            TransitOption, 
            TransitOptions, 
            TransitReturn, 
            TransitStates,
            TransportContext
        }, shadowd_block::{
            Block, 
            ShadowdBlockCore
        }
    }, 
    helpers::convertions::{be_to_size, size_to_be}
};
use std::{
    fmt,
    mem::take
};

pub struct BaseSheetShadowdBlock<'a>{
    _core:ShadowdBlockCore<'a>,
    _buffer:Vec<u8>,
    is_free:bool,
    writed_bytes:usize,
    free_bytes:usize,
    _start: u64,
    _size: usize,
    _head_size: u8,
    _data:usize
}

//la politica de general de bloque
impl <'a> Block<'a> for BaseSheetShadowdBlock<'a> {
    type Buffer = Vec<u8>;

    fn new(pos:u64, alloc:&'a mut AllocatorBlock, disk_id:usize)->Self {
        Self {
            _core:
                ShadowdBlockCore{
                    pos, 
                    disk_id, 
                    alloc
                },
            
            _buffer:
                Vec::new(),
            is_free: 
                true, 
            writed_bytes: 
                0, 
            free_bytes: 
                AllocatorBlock::get_size_block(), 
            _start: 
                AllocatorBlock::get_size_block() as u64 *pos,
            _size: 
                AllocatorBlock::get_size_block(),
            _head_size: 
                4,
            _data:
                AllocatorBlock::get_size_block()-4
        }
    }
    fn write_intern(&mut self) {
        if self._buffer.is_empty() {self.read_intern();}
        let buffer_size = self._buffer.len();
        let to_write = if buffer_size < self._data{buffer_size}else{self._data};
        let mut count = self._start;

        let count_writed = self.writed_bytes.min(self._data);
        let header_bytes = size_to_be(count_writed, self._head_size as usize);
        let alloc:&mut AllocatorBlock =  self._core.alloc;
        let result = &alloc.write_disk(&header_bytes, count, self._core.disk_id as u16);
        if result.is_none() {
            return
        }
        count += self._head_size as u64;

        let data_bytes= &mut self._buffer;
        data_bytes.resize(to_write, 0);
        alloc.write_disk(data_bytes, count, self._core.disk_id as u16);
        return
    }
    fn write_block(&mut self, cur:&Cursor, data:&mut Vec<u8>) -> InsertResult{
        if self._buffer.is_empty() {self.read_intern();}
        let _sym = cur;// simulamos un uso
        if self.free_bytes == 0 {
            return InsertResult {
                result: InsertResultItem::BufferIsFull,
                state: BufferStates::Full,
                remaining: data.len(),
                written: 0,
                remaining_bytes: take(data),
            };
        }
        let original_data_size = data.len();
        let remaning = self.free_bytes;
        let to_write = if data.len() < remaning {
            data.len()
        }else{
            remaning
        };
        let mut data_remaning_bytes = if to_write < data.len() {
            data.split_off(to_write)// movemos lo que no cabe 
        }else{
            Vec::new()
        };

        self._buffer.append(data);

        let remaning_bytes = if self._buffer.len() > self._data {
            let mut vec = self._buffer.split_off(self._data);
            vec.append(&mut data_remaning_bytes);
            vec
        }else{
            data_remaning_bytes
        }; // por seguridad

        self.writed_bytes += to_write;
        self.free_bytes -= to_write;

        let is_fully = if self.free_bytes == 0 {
            self.write_intern();
            true
        } else {
            false
        };
        let mut result = InsertResult {
            result: InsertResultItem::Fail,
            state: BufferStates::Empty,
            remaining: remaning_bytes.len(),
            written: 0,
            remaining_bytes: remaning_bytes,
        };

        result.written = to_write;

        result.state = if is_fully {
            BufferStates::Full
        } else if self.free_bytes == self._data {
            BufferStates::Empty
        } else {
            BufferStates::PartiallyFull
        };

        result.result = if to_write == original_data_size {
            InsertResultItem::InsertedWithoutRemaining
        } else {
            InsertResultItem::InsertedWithRemaining
        };

        result
    }
    fn read_to(&mut self, cur:&Cursor, size:usize) -> Option<&[u8]> {
        if self._buffer.is_empty() {
            self.read_intern();
        }

        let offset = cur.get_pos(1)? as usize * 11;
        let end = offset.saturating_add(size).min(self._buffer.len());
        Some(&self._buffer[offset..end])
    }
    fn clear_block_childs(&mut self)->bool {
        if self._buffer.is_empty() {self.read_intern();}
        self._buffer.fill(0);
        self.write_intern();
        self.writed_bytes = 0;
        self.free_bytes = self._data;
        self.write_intern();
        true
    }
    fn insert_to(&mut self, options:&TransitOptions)->Option<TransitReturn> {
        if self._buffer.is_empty() {
            self.read_intern();
        }
        let is_dir = options.context.is_directory();
        if is_dir { 
            let aux =  &mut Vec::new();
            aux.extend_from_slice(options.data.as_slice());
            let default = TransitReturn {
                action: TransitOption::Finalize, 
                state: TransitStates::IlegalAcction, 
                data: take(aux),
                increment_size: false,
                context: TransportContext::Directory
            };

            // Esto es algo importante, si es un dir esta accion solo es permitida a espacios libres
            let pos = options.pos.get_pos(1)?  as usize * 11;
            if self._buffer[pos] > 0 {
                // insert no tiene permitido reemplazar punteros
                // antes de lanzar esto sobre un puntero funcional
                // por favor lanzar un remove sobre el mismo puntero
                Some(default)
            }else{
                let data = aux;
                if data.len() != 11 {
                    return Some(default)
                }
                for i in 0..11{
                    self._buffer[pos+i] = data[i];
                }
                Some(
                    TransitReturn {
                        action: TransitOption::Finalize,
                        state: TransitStates::Ok, 
                        data: Vec::new(), 
                        increment_size: options.increment_size, 
                        context: TransportContext::Directory
                    }
                )
            }
        }else{ // si es archivo
            let aux =  &mut Vec::new();
            aux.extend_from_slice(options.data.as_slice());
            let op = options.option;
            match op {
                TransitOption::InsertInPos => Some(
                    insert_in_pos(
                        &mut self._buffer, 
                        aux, 
                        options.increment_size, 
                        AllocatorBlock::get_size_block(),
                        (options.pos.get_pos(1)? as usize)*11
                    )
                ),
                TransitOption::InsertBegin => Some(
                    insert_begin(
                        &mut self._buffer, 
                        aux, 
                        options.increment_size,
                        AllocatorBlock::get_size_block()
                    )
                ),
                TransitOption::InsertEnd => Some(
                    insert_end(
                        &mut self._buffer, 
                        aux, 
                        options.increment_size,
                        AllocatorBlock::get_size_block()
                    )
                ),
                _ => None
            }
        }
    }
    fn remove_to(&mut self, options:&TransitOptions)->Option<TransitReturn> {
        if self._buffer.is_empty() {
            self.read_intern();
        }
        let is_dir = options.context.is_directory();
        if is_dir {
            let defualt = TransitReturn{
                action: TransitOption::Finalize,
                state: TransitStates::Ok,
                data: Vec::new(),
                increment_size: options.increment_size,
                context: TransportContext::Directory
            };
            let pos = options.pos.get_pos(1)? as usize * 11;
            if self._buffer[pos]==0 {
                Some(defualt)
            }else{
                for i in 0..11 {
                    self._buffer[pos+i] = 0;
                }
                Some(defualt)
            }
        }else{
            let op = options.option;
            let pos = options.pos.get_pos(1)? as usize * 11;
            match op {
                TransitOption::DeletePosBytesToBegin => Some(
                    delete_pos_bytes_to_begin(
                        pos, 
                        &mut self._buffer,
                        options.increment_size
                    )
                ),
                TransitOption::DeletePosDefault => Some(
                    delete_in_range(
                        pos, 
                        8, 
                        &mut self._buffer, 
                        options.increment_size
                    )
                ),
                TransitOption::DeletePosBytesToEnd => Some(
                    delete_pos_bytes_to_end(
                        pos, 
                        &mut self._buffer, 
                        options.increment_size
                    )
                ),
                TransitOption::DeletePosToIndicator => Some(
                    delete_in_range(
                        pos, 
                        options.indicator,
                        &mut self._buffer,
                        options.increment_size
                    )
                ),
                _ => None
            }
        }
    }
}

//la politica interna del bloque 
impl BaseSheetShadowdBlock<'_> {
    pub fn layer()->u8{
        1
    }
    pub fn is_free(&self) -> bool{
        self.is_free
    }
    pub fn writed_bytes(&self) -> usize{
        self.writed_bytes
    }
    pub fn free_bytes(&self) -> usize{
        self.free_bytes
    }
    fn read_intern(&mut self) -> Option<()>{
        let mut count = self._start;
        let head = self._core.alloc.read_disk(self._head_size as usize, count, self._core.disk_id as u16)?;
        count += self._head_size as u64;

        let writed = be_to_size(&head, self._head_size as usize);
        let to_read = writed.min(self._data);
        self.writed_bytes = to_read;
        if to_read > 0 {
            self.free_bytes = self._data - to_read;
            self.is_free = false;
        }else{
            self.free_bytes = self._data;
            self.is_free = true;
        }
        let mut data = self._core.alloc.read_disk(to_read, count, self._core.disk_id as u16)?;
        self._buffer.splice(.., data.drain(..));
        Some(())
    }
}
impl fmt::Display for BaseSheetShadowdBlock<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let make_tabs = |n: usize| "\t".repeat(n);

        writeln!(f, "{}BaseShadowdBlock(", make_tabs(16))?;
        writeln!(f, "{}index={}", make_tabs(17), self._core.pos)?;
        writeln!(f, "{}disk={}", make_tabs(17), self._core.disk_id)?;
        writeln!(f, "{}writed={}", make_tabs(17), self.writed_bytes)?;
        writeln!(f, "{}free={}", make_tabs(17), self.free_bytes)?;
        writeln!(f, "{}is free={}", make_tabs(17), self.is_free)?;
        writeln!(f, "{})", make_tabs(16))?;

        Ok(())
    }
}
fn insert_begin(buff: &mut Vec<u8>, data:&mut Vec<u8>, increment: bool, block_size:usize) -> TransitReturn{
    if buff.len() + data.len() >= block_size && !increment{
        return TransitReturn{
            action: TransitOption::Finalize,
            state: TransitStates::Error1,
            data: take(data),
            increment_size: false, // si fuera true no estaria aqui,
            context: TransportContext::File // si fuera dir no estaria aqui
        }
    }

    buff.splice(0..0, data.drain(..));
    
    if increment { // yo tambien pense que estas dos condiciones estan al revez, pero no, arriba decidimos que si increment era verdaero o la suma de buff + data no superara block size, asi quje esto esta bien, que si no se pudiera incrementar no podfia padar del tamaño del bloque
        if buff.len() > block_size {
            let remaining = buff.drain(block_size..).collect();
            return TransitReturn { 
                action: TransitOption::InsertBegin, 
                state: TransitStates::MoveToEnd, 
                data: remaining,
                increment_size: increment, 
                context: TransportContext::File // aca solo se llega en file
            }
        }
    }

    TransitReturn {
        action: TransitOption::Finalize, 
        state: TransitStates::Ok, 
        data: Vec::new(), 
        increment_size: increment, 
        context: TransportContext::File
    }
}
fn insert_end(buff: &mut Vec<u8>, data:&mut Vec<u8>, increment: bool, block_size:usize)-> TransitReturn{
    let result_error = TransitReturn{
        action: TransitOption::Finalize,
        state: TransitStates::Error1,
        data: take(data),
        increment_size: false, // si fuera true no estaria aqui,
        context: TransportContext::File // si fuera dir no estaria aqui
    };
    if buff.len() == block_size{
        if increment { // si es permitido el incremento
            return TransitReturn {
                action: TransitOption::InsertBegin ,
                state: TransitStates::MoveToEnd, 
                data: take(data),
                increment_size: increment,
                context: TransportContext::File 
            }
        }else{ // no se permite incremento
            return result_error;
        }
    }
    if buff.len() + data.len() > block_size && !increment{
        return result_error
    }
    let i = buff.len();
    let range = i..i;
    buff.splice(range, data.drain(..));
    if increment {
        if buff.len() > block_size {
            let remaining = buff.drain(block_size..).collect();
            return TransitReturn { 
                action: TransitOption::InsertBegin, 
                state: TransitStates::MoveToEnd, 
                data: remaining, 
                increment_size: increment, 
                context: TransportContext::File
            }
        }
    }
    TransitReturn {
        action: TransitOption::Finalize,
        state: TransitStates::Ok, 
        data: Vec::new(), 
        increment_size: increment, 
        context: TransportContext::File
    }

}
fn insert_in_pos(buff: &mut Vec<u8>, data:&mut Vec<u8>, increment: bool, block_size:usize, pos:usize)->TransitReturn{
    if buff.len() + data.len() > block_size && !increment {
        return TransitReturn { 
            action: TransitOption::Finalize, 
            state: TransitStates::Error1,
            data: take(data),
            increment_size: increment, 
            context: TransportContext::File
        }
    }
    let pos_ = pos.min(buff.len());
    let range = pos_..pos_;
    buff.splice(range, data.drain(..));
    if increment{
        if buff.len() > block_size {
            let remaining = buff.drain(block_size..).collect();
            return TransitReturn {
                action: TransitOption::InsertBegin, 
                state: TransitStates::MoveToEnd,
                data: remaining, 
                increment_size: increment, 
                context: TransportContext::File }
        }
    }
    TransitReturn { 
        action: TransitOption::Finalize, 
        state: TransitStates::Ok, 
        data: Vec::new(), 
        increment_size: increment, 
        context: TransportContext::File
    }
}
fn delete_pos_bytes_to_begin(pos:usize, buff: &mut Vec<u8>, decrement:bool)->TransitReturn{
    if pos == 0 {
        return TransitReturn { 
            action: TransitOption::Finalize, 
            state: TransitStates::Error2, 
            data: Vec::new(), 
            increment_size: decrement, 
            context: TransportContext::File
        }
    }
    let end = pos.min(buff.len());
    let data = buff.drain(..end).collect();
    TransitReturn { 
        action: TransitOption::InsertEnd, 
        state: TransitStates::MoveToBegin, 
        data, 
        increment_size: decrement, 
        context: TransportContext::File
    }
}
fn delete_pos_bytes_to_end(pos:usize, buff: &mut Vec<u8>, decrement:bool)->TransitReturn{
    if pos == 0 {
        return TransitReturn { 
            action: TransitOption::Finalize, 
            state: TransitStates::Error2, 
            data: Vec::new(), 
            increment_size: decrement, 
            context: TransportContext::File
        }
    }
    let start = pos.min(buff.len());
    let data = buff.drain(start..).collect();
    TransitReturn { 
        action: TransitOption::InsertEnd, 
        state: TransitStates::MoveToBegin, 
        data, 
        increment_size: decrement, 
        context: TransportContext::File
    }
}
fn delete_in_range(pos:usize, ind:usize, buff: &mut Vec<u8>, decrement:bool)->TransitReturn{
    if pos == 0 || ind == 0 || pos >= buff.len() || pos+ind > buff.len(){
        return TransitReturn { 
            action: TransitOption::Finalize, 
            state: TransitStates::Error2, 
            data: Vec::new(), 
            increment_size: decrement, 
            context: TransportContext::File
        }
    }

    let end = pos + ind;
    let data = buff.drain(pos..end).collect();
    TransitReturn { 
        action: TransitOption::InsertEnd, 
        state: TransitStates::MoveToBegin, 
        data, 
        increment_size: decrement, 
        context: TransportContext::File
    }
}
pub struct EntrySheetShadowdBlock<'a>{
    pos:usize,
    bs_sb:BaseSheetShadowdBlock<'a>,
    valid:bool,
}
