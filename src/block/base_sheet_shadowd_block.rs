/// Bloque Base de Shadowd (BaseSheetShadowdBlock).
///
/// Este bloque es el componente fundamental del sistema de archivos Shadowd Virtual File System.
/// Actúa como:
/// 1. Bloque base de todos los demás bloques en el sistema
/// 2. Gestor de datos físicos reales en disco virtual
/// 3. Coordinador para el funcionamiento de todos los demás bloques
///
/// Cada BaseSheetShadowdBlock garantiza que todo dato en el sistema tenga
/// una representación física en el disco virtual, proporcionando la capa
/// de persistencia básica.
use crate::{
    algoritm::cursors::Cursor,
    block::{
        AllocHadle,
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
        },
        shadowd_block::{
            Block,
            ShadowdBlockCore
        }
    },
    helpers::convertions::{
        be_to_size,
        size_to_be
    }
};
use std::{
    fmt,
    mem::take,
    sync::{
        Arc,
        RwLock
    }, 
    u64,
    usize
};

/// Bloque Base de Shadowd que almacena datos físicos.
///
/// Este bloque es responsable de:
/// - Almacenar los datos reales en el disco virtual
/// - Gestionar el buffer interno de datos
/// - Coordinar operaciones de lectura/escritura
/// - Implementar ofuscación de datos para seguridad
///
/// # Campos
/// - `_core`: Núcleo del bloque con información de posición y capa
/// - `_buffer`: Buffer interno protegido por RwLock para acceso concurrente
/// - `is_free`: Indica si el bloque está libre para escritura
/// - `writed_bytes`: Número de bytes escritos en el bloque
/// - `free_bytes`: Número de bytes disponibles en el bloque
/// - `_start`: Posición inicial del bloque en el disco (en bytes)
/// - `_size`: Tamaño total del bloque (en bytes)
/// - `_head_size`: Tamaño del encabezado (en bytes)
/// - `_data`: Tamaño disponible para datos (tamaño total - encabezado)
/// - `loaded`: Indica si el bloque ha sido cargado desde disco
/// - `dirty`: Indica si el bloque tiene cambios no guardados
pub struct BaseSheetShadowdBlock {
    _core: ShadowdBlockCore,
    _buffer: Arc<RwLock<Vec<u8>>>,
    is_free: bool,
    writed_bytes: usize,
    free_bytes: usize,
    _start: u64,
    _size: usize,
    _head_size: u8,
    _data: usize,
    loaded: bool,
    dirty: bool,
}

// Implementación del trait Block para BaseSheetShadowdBlock
impl Block for BaseSheetShadowdBlock {
    type Buffer = Vec<u8>;

    /// Constructor principal para crear un nuevo BaseSheetShadowdBlock.
    ///
    /// # Parámetros
    /// - `pos`: Posición del bloque en el disco (número de bloque)
    /// - `alloc`: Manejador del asignador de bloques del sistema
    /// - `disk_id`: Identificador del disco al que pertenece este bloque
    /// - `layer`: Capa lógica donde reside este bloque
    ///
    /// # Detalles
    /// Crea un bloque que representa exactamente el bloque `pos` del disco `disk_id`.
    /// Es crucial que estos valores sean correctos, ya que valores erróneos
    /// pueden causar corrupción de datos al apuntar a bloques incorrectos.
    ///
    /// # Ejemplo
    /// ```
    /// let alloc: AllocHadle = ...;
    /// let bssb = BaseSheetShadowdBlock::new(0, alloc, 0, 1);
    /// ```
    ///
    /// # Notas de seguridad
    /// - La posición y el ID del disco deben ser válidos
    /// - El bloque se inicializa como libre y no cargado
    fn new(pos: u64, alloc: AllocHadle, disk_id: usize, layer: u8) -> Self {
        Self {
            _core: ShadowdBlockCore {
                layer,
                pos,
                disk_id,
                alloc
            },
            _buffer: Arc::new(RwLock::new(Vec::new())),
            is_free: true,
            writed_bytes: 0,
            free_bytes: AllocatorBlock::get_size_block(),
            _start: AllocatorBlock::get_size_block() as u64 * pos,
            _size: AllocatorBlock::get_size_block(),
            _head_size: 4,
            loaded: false,
            dirty: false,
            _data: AllocatorBlock::get_size_block() - 4
        }
    }
    
    /// Escribe los datos del buffer interno al disco.
    ///
    /// # Funcionamiento
    /// 1. Verifica si el bloque está cargado; si no, lo carga primero
    /// 2. Verifica si hay cambios pendientes (dirty flag)
    /// 3. Escribe el encabezado con la cantidad de bytes escritos
    /// 4. Ofusca y escribe los datos del buffer
    /// 5. Marca el bloque como limpio (no dirty)
    ///
    /// # Retorno
    /// - `Some(())`: Escritura exitosa
    /// - `None`: Error en la escritura o en la carga previa
    ///
    /// # Ejemplo
    /// ```
    /// let mut block: BaseSheetShadowdBlock = ...;
    /// block.write_intern()?;
    /// ```
    fn write_intern(&mut self) -> Option<()> {
        // Si el bloque no está cargado, cargarlo primero
        if !self.loaded {
            self.read_intern()?;
            return Some(());
        }
        
        // Si no hay cambios, no es necesario escribir
        if !self.dirty {
            return Some(());
        }
        
        let mut count = self._start;

        // Calcular bytes a escribir (limitado por el tamaño de datos)
        let count_writed = self.writed_bytes.min(self._data);
        
        // Escribir encabezado
        let header_bytes = size_to_be(count_writed, self._head_size as usize);
        self._core.alloc.borrow_mut().write_disk(&header_bytes, count, self._core.disk_id as u16)?;
        count += self._head_size as u64;

        // Obtener, ofuscar y escribir datos
        let data = &self._buffer;
        let data_binding = data.read().ok()?;
        let data_slice = data_binding.as_slice();
        let mut data_bytes: Vec<u8> = Vec::new();
        data_bytes.extend_from_slice(data_slice);
        let data = self.ofusc(&data_bytes);
        self._core.alloc.borrow_mut().write_disk(&data, count, self._core.disk_id as u16)?;
        
        // Marcar como limpio
        self.dirty = false;
        Some(())
    }
    
    /// Escribe datos en el bloque siguiendo un cursor específico.
    ///
    /// # Parámetros
    /// - `data`: Datos a escribir en el bloque
    ///
    /// # Comportamiento
    /// - Si el bloque no está cargado, lo carga primero
    /// - Verifica si hay espacio disponible
    /// - Escribe la cantidad máxima posible de datos
    /// - Si el bloque se llena completamente, lo escribe a disco
    ///
    /// # Retorno
    /// - `Some(InsertResult)`: Resultado de la operación con detalles
    /// - `None`: Error durante la operación
    ///
    /// # Estados posibles del resultado:
    /// - `BufferIsFull`: El bloque está lleno
    /// - `InsertedWithoutRemaining`: Todos los datos fueron escritos
    /// - `InsertedWithRemaining`: Solo parte de los datos fueron escritos
    fn write_block(&mut self, data: &mut Vec<u8>) -> Option<InsertResult> {
        if !self.loaded {
            self.read_intern()?;
        }
        
        // Verificar si el bloque está lleno
        if self.free_bytes == 0 {
            return Some(
                InsertResult {
                    result: InsertResultItem::BufferIsFull,
                    state: BufferStates::Full,
                    remaining: data.len(),
                    written: 0,
                    remaining_bytes: take(data),
                }
            );
        }
        
        let original_data_size = data.len();
        let remaining = self.free_bytes;
        let to_write = if data.len() < remaining {
            data.len()
        } else {
            remaining
        };
        
        // Actualizar contadores
        self.writed_bytes += to_write;
        self.free_bytes -= to_write;
        self.dirty = true;
        
        // Separar datos restantes si es necesario
        let mut data_remaning_bytes = if to_write < data.len() {
            data.split_off(to_write) // Mover lo que no cabe
        } else {
            Vec::new()
        };

        // Escribir en el buffer
        let mut buff = self._buffer.write().ok()?;
        buff.append(data);
        
        // Asegurar que no excedamos el límite de datos
        let remaining_bytes = if buff.len() > self._data {
            let mut vec = buff.split_off(self._data);
            vec.append(&mut data_remaning_bytes);
            self.writed_bytes -= vec.len();
            self.free_bytes += vec.len();
            vec
        } else {
            data_remaning_bytes
        };

        // Si el bloque se llenó, escribirlo a disco
        let is_fully = if self.free_bytes == 0 {
            drop(buff);
            self.write_intern();
            true
        } else {
            false
        };
        
        // Construir resultado
        let mut result = InsertResult {
            result: InsertResultItem::Fail,
            state: BufferStates::Empty,
            remaining: remaining_bytes.len(),
            written: 0,
            remaining_bytes: remaining_bytes,
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

        Some(result)
    }
    
    /// Lee datos desde el bloque siguiendo un cursor.
    ///
    /// # Parámetros
    /// - `cur`: Cursor que indica la posición de lectura
    /// - `size`: Cantidad de bytes a leer
    ///
    /// # Retorno
    /// - `Some(Vec<u8>)`: Datos leídos
    /// - `None`: Error en la lectura o posición inválida
    ///
    /// # Notas
    /// - La posición se calcula como `pos * 11` (tamaño fijo de entrada)
    /// - Si el bloque no está cargado, se carga automáticamente
    fn read_to(&mut self, cur: &mut Cursor, size: usize) -> Option<Vec<u8>> {
        // Cargar bloque si es necesario
        if self.free_bytes > self._data {
            self.read_intern()?;
        }

        let buff = self._buffer.read().ok()?;
        let offset_ = cur.get_pos(1)? as usize;
        let offset = offset_*11;
        let end = offset.saturating_add(size).min(buff.len());
        let vec = buff[offset..end].to_vec();
        let sized = vec.len();
        let mut count = 0;
        loop {
            if sized == count {
                break;
            }
            if sized-count < 11 {
                break; // este fracmento sera leido nuevamente cuanto de llame a esta funcion mas adelante
            }
            if cur.advance() != 2 {
                return None; //out of range
            }
            count += 11;
        }
        Some(vec)
    }

    /// Limpia todos los datos del bloque y sus referencias.
    ///
    /// # Funcionamiento
    /// 1. Carga el bloque si es necesario
    /// 2. Llena el buffer con ceros
    /// 3. Escribe los cambios a disco
    /// 4. Reinicia los contadores de bytes
    ///
    /// # Retorno
    /// - `Some(true)`: Operación exitosa
    /// - `None`: Error durante la operación
    fn clear_block_childs(&mut self) -> Option<bool> {
        // Cargar bloque si es necesario
        if self.free_bytes > self._data {
            self.read_intern();
        }
        
        // Limpiar buffer
        let mut buff = self._buffer.write().ok()?;
        buff.fill(0);
        drop(buff);
        self.dirty = true;
        // Escribir cambios y reiniciar contadores
        self.write_intern();
        self.dirty = true;
        self.writed_bytes = 0;
        self.free_bytes = self._data;
        self.write_intern();
        
        Some(true)
    }

    /// Inserta datos en el bloque según las opciones especificadas.
    ///
    /// # Parámetros
    /// - `options`: Opciones de inserción (posición, tipo, contexto, etc.)
    ///
    /// # Comportamiento según el contexto:
    ///
    /// ## Directorio (is_directory == true)
    /// - Inserta una entrada de 11 bytes en posición específica
    /// - Verifica que la posición esté vacía
    /// - Retorna error si la posición está ocupada
    ///
    /// ## Archivo (is_directory == false)
    /// - Soporta diferentes modos de inserción:
    ///   - `InsertBegin`: Inserta al inicio
    ///   - `InsertEnd`: Inserta al final
    ///   - `InsertInPos`: Inserta en posición específica
    /// - Maneja overflow y redirección a otros bloques
    ///
    /// # Retorno
    /// - `Some(TransitReturn)`: Resultado de la inserción con posibles datos sobrantes
    /// - `None`: Error o operación no soportada
    fn insert_to(&mut self, options: &TransitOptions) -> Option<TransitReturn> {
        // Cargar bloque si es necesario
        if self.free_bytes > self._data {
            self.read_intern()?;
        }
        
        let mut buf = self._buffer.write().ok()?;
        let is_dir = options.context.is_directory();

        // ---- CASO DIRECTORIO ----
        if is_dir {
            let aux = &mut options.data.clone();
            let default = TransitReturn {
                action: TransitOption::Finalize,
                state: TransitStates::IlegalAcction,
                data: take(aux),
                increment_size: false,
                context: TransportContext::Directory
            };
            
            let pos = options.pos.get_pos(1)? as usize * 11;
            
            // Verificar si la posición ya está ocupada
            if buf[pos] > 0 {
                return Some(default);
            }
            
            // Verificar tamaño de datos y límites del buffer
            if aux.len() != 11 || buf.len() < pos + 11 {
                return None;
            }
            
            // Insertar datos
            for i in 0..11 {
                buf[pos + i] = aux[i];
            }
            self.dirty = true;
            return Some(TransitReturn {
                action: TransitOption::Finalize,
                state: TransitStates::Ok,
                data: Vec::new(),
                increment_size: options.increment_size,
                context: TransportContext::Directory
            });
        }

        // ---- CASO ARCHIVO ----
        let mut data = options.data.clone();
        let block_size = AllocatorBlock::get_size_block();
        let increment = options.increment_size;
        let pos = (options.pos.get_pos(1)? as usize) * 11;

        match options.option {
            TransitOption::InsertBegin => {
                // Verificar límites
                if buf.len() + data.len() >= block_size && !increment {
                    return Some(TransitReturn {
                        action: TransitOption::Finalize,
                        state: TransitStates::Error1,
                        data: take(&mut data),
                        increment_size: false,
                        context: TransportContext::File
                    });
                }
                
                // Insertar al inicio
                self.writed_bytes += data.len();
                buf.splice(0..0, data.drain(..));
                self.dirty = true;
                // Manejar overflow si hay incremento de tamaño
                if increment && buf.len() > block_size {
                    let remaining: Vec<u8> = buf.drain(block_size..).collect();
                    self.writed_bytes -= remaining.len();
                    self.free_bytes = self._data.saturating_sub(self.writed_bytes);
                    
                    return Some(TransitReturn {
                        action: TransitOption::InsertBegin,
                        state: TransitStates::MoveToEnd,
                        data: remaining,
                        increment_size: increment,
                        context: TransportContext::File
                    });
                }
                
                Some(TransitReturn {
                    action: TransitOption::Finalize,
                    state: TransitStates::Ok,
                    data: Vec::new(),
                    increment_size: increment,
                    context: TransportContext::File
                })
            }
            
            TransitOption::InsertEnd => {
                let result_error = TransitReturn {
                    action: TransitOption::Finalize,
                    state: TransitStates::Error1,
                    data: take(&mut data),
                    increment_size: false,
                    context: TransportContext::File
                };
                
                // Si el buffer está lleno
                if buf.len() == block_size {
                    if increment {
                        // Redirigir a otro bloque
                        return Some(TransitReturn {
                            action: TransitOption::InsertBegin,
                            state: TransitStates::MoveToEnd,
                            data: take(&mut data),
                            increment_size: increment,
                            context: TransportContext::File
                        });
                    } else {
                        return Some(result_error);
                    }
                }
                
                // Verificar límites sin incremento
                if buf.len() + data.len() > block_size && !increment {
                    return Some(result_error);
                }
                
                // Insertar al final
                self.writed_bytes += data.len();
                let i = buf.len();
                buf.splice(i..i, data.drain(..));
                self.dirty = true;
                // Manejar overflow
                if increment && buf.len() > block_size {
                    let remaining: Vec<u8> = buf.drain(block_size..).collect();
                    self.writed_bytes -= remaining.len();
                    self.free_bytes = self._data.saturating_sub(self.writed_bytes);
                    
                    return Some(TransitReturn {
                        action: TransitOption::InsertBegin,
                        state: TransitStates::MoveToEnd,
                        data: remaining,
                        increment_size: increment,
                        context: TransportContext::File
                    });
                }
                
                Some(TransitReturn {
                    action: TransitOption::Finalize,
                    state: TransitStates::Ok,
                    data: Vec::new(),
                    increment_size: increment,
                    context: TransportContext::File
                })
            }
            
            TransitOption::InsertInPos => {
                // Verificar límites
                if buf.len() + data.len() > block_size && !increment {
                    return Some(TransitReturn {
                        action: TransitOption::Finalize,
                        state: TransitStates::Error1,
                        data: take(&mut data),
                        increment_size: increment,
                        context: TransportContext::File
                    });
                }
                
                // Insertar en posición específica
                self.writed_bytes += data.len();
                let p = pos.min(buf.len());
                buf.splice(p..p, data.drain(..));
                self.dirty = true;
                // Manejar overflow
                if increment && buf.len() > block_size {
                    let remaining: Vec<u8> = buf.drain(block_size..).collect();
                    self.writed_bytes -= remaining.len();
                    self.free_bytes = self._data.saturating_sub(self.writed_bytes);
                    
                    return Some(TransitReturn {
                        action: TransitOption::InsertBegin,
                        state: TransitStates::MoveToEnd,
                        data: remaining,
                        increment_size: increment,
                        context: TransportContext::File
                    });
                }
                
                Some(TransitReturn {
                    action: TransitOption::Finalize,
                    state: TransitStates::Ok,
                    data: Vec::new(),
                    increment_size: increment,
                    context: TransportContext::File
                })
            }
            
            _ => None
        }
    }

    /// Elimina datos del bloque según las opciones especificadas.
    ///
    /// # Parámetros
    /// - `options`: Opciones de eliminación (posición, tipo, indicador, etc.)
    ///
    /// # Modos de eliminación soportados:
    /// - `DeletePosBytesToBegin`: Elimina desde posición hasta el inicio
    /// - `DeletePosBytesToEnd`: Elimina desde posición hasta el final
    /// - `DeletePosDefault`: Elimina 8 bytes en posición específica
    /// - `DeletePosToIndicator`: Elimina N bytes según indicador
    ///
    /// # Retorno
    /// - `Some(TransitReturn)`: Resultado con datos eliminados
    /// - `None`: Error o operación no soportada
    fn remove_to(&mut self, options: &TransitOptions) -> Option<TransitReturn> {
        // Cargar bloque si es necesario
        if self.free_bytes > self._data {
            self.read_intern()?;
        }
        
        let mut buf = self._buffer.write().ok()?;
        let pos = (options.pos.get_pos(1)? as usize) * 11;
        let dec = options.increment_size;

        // ---- CASO DIRECTORIO ----
        if options.context.is_directory() {
            let def = TransitReturn {
                action: TransitOption::Finalize,
                state: TransitStates::Ok,
                data: Vec::new(),
                increment_size: dec,
                context: TransportContext::Directory
            };
            
            // Si ya está vacío, retornar éxito
            if buf[pos] == 0 {
                return Some(def);
            }
            
            // Limpiar los 11 bytes de la entrada
            for i in 0..11 {
                buf[pos + i] = 0;
            }
            self.dirty = true;
            return Some(def);
        }

        // ---- CASO ARCHIVO ----
        match options.option {
            TransitOption::DeletePosBytesToBegin => {
                // Verificar posición válida
                if pos == 0 {
                    return Some(TransitReturn {
                        action: TransitOption::Finalize,
                        state: TransitStates::Error2,
                        data: Vec::new(),
                        increment_size: dec,
                        context: TransportContext::File
                    });
                }
                
                // Eliminar desde inicio hasta posición
                let end = pos.min(buf.len());
                let data: Vec<u8> = buf.drain(..end).collect();
                self.dirty = true;
                // Actualizar contadores
                self.writed_bytes = self.writed_bytes.saturating_sub(data.len());
                self.free_bytes = self._data.saturating_sub(self.writed_bytes);
                
                Some(TransitReturn {
                    action: TransitOption::InsertEnd,
                    state: TransitStates::MoveToBegin,
                    data,
                    increment_size: dec,
                    context: TransportContext::File
                })
            }
            
            TransitOption::DeletePosBytesToEnd => {
                // Verificar posición válida
                if pos == 0 {
                    return Some(TransitReturn {
                        action: TransitOption::Finalize,
                        state: TransitStates::Error2,
                        data: Vec::new(),
                        increment_size: dec,
                        context: TransportContext::File
                    });
                }
                
                // Eliminar desde posición hasta final
                let start = pos.min(buf.len());
                let data: Vec<u8> = buf.drain(start..).collect();
                self.dirty = true;
                // Actualizar contadores
                self.writed_bytes = self.writed_bytes.saturating_sub(data.len());
                self.free_bytes = self._data.saturating_sub(self.writed_bytes);
                
                Some(TransitReturn {
                    action: TransitOption::InsertEnd,
                    state: TransitStates::MoveToBegin,
                    data,
                    increment_size: dec,
                    context: TransportContext::File
                })
            }
            
            TransitOption::DeletePosDefault => {
                let ind = 8; // Tamaño fijo por defecto
                
                // Verificar posición y límites
                if pos == 0 || pos >= buf.len() || pos + ind > buf.len() {
                    return Some(TransitReturn {
                        action: TransitOption::Finalize,
                        state: TransitStates::Error2,
                        data: Vec::new(),
                        increment_size: dec,
                        context: TransportContext::File
                    });
                }
                
                // Eliminar 8 bytes
                let end = pos + ind;
                let data: Vec<u8> = buf.drain(pos..end).collect();
                
                // Actualizar contadores
                self.writed_bytes = self.writed_bytes.saturating_sub(data.len());
                self.free_bytes = self._data.saturating_sub(self.writed_bytes);
                self.dirty = true;
                Some(TransitReturn {
                    action: TransitOption::InsertEnd,
                    state: TransitStates::MoveToBegin,
                    data,
                    increment_size: dec,
                    context: TransportContext::File
                })
            }
            
            TransitOption::DeletePosToIndicator => {
                let ind = options.indicator;
                
                // Verificar posición y límites
                if pos == 0 || ind == 0 || pos >= buf.len() || pos + ind > buf.len() {
                    return Some(TransitReturn {
                        action: TransitOption::Finalize,
                        state: TransitStates::Error2,
                        data: Vec::new(),
                        increment_size: dec,
                        context: TransportContext::File
                    });
                }
                
                // Eliminar N bytes según indicador
                let end = pos + ind;
                let data: Vec<u8> = buf.drain(pos..end).collect();
                
                // Actualizar contadores
                self.writed_bytes = self.writed_bytes.saturating_sub(data.len());
                self.free_bytes = self._data.saturating_sub(self.writed_bytes);
                self.dirty = true;
                Some(TransitReturn {
                    action: TransitOption::InsertEnd,
                    state: TransitStates::MoveToBegin,
                    data,
                    increment_size: dec,
                    context: TransportContext::File
                })
            }
            
            _ => None
        }
    }
}

// Implementación de métodos específicos para BaseSheetShadowdBlock
impl BaseSheetShadowdBlock {
    /// Retorna el número de capa de este tipo de bloque.
    ///
    /// # Retorno
    /// - `u8`: Número de capa (siempre 1 para bloques base)
    pub fn layer() -> u8 {
        1
    }

    /// Verifica si el bloque está libre para escritura.
    ///
    /// # Retorno
    /// - `true`: El bloque está libre
    /// - `false`: El bloque está ocupado
    pub fn is_free(&self) -> bool {
        self.is_free
    }

    /// Obtiene la cantidad de bytes escritos en el bloque.
    ///
    /// # Retorno
    /// - `usize`: Número de bytes escritos
    pub fn writed_bytes(&self) -> usize {
        self.writed_bytes
    }

    /// Obtiene la cantidad de bytes disponibles en el bloque.
    ///
    /// # Retorno
    /// - `usize`: Número de bytes libres
    pub fn free_bytes(&self) -> usize {
        self.free_bytes
    }

    /// Carga los datos del bloque desde el disco.
    ///
    /// # Funcionamiento
    /// 1. Lee el encabezado para obtener la cantidad de datos
    /// 2. Calcula los bytes escritos y libres
    /// 3. Lee los datos del disco
    /// 4. Desofusca los datos
    /// 5. Almacena los datos en el buffer interno
    ///
    /// # Retorno
    /// - `Some(())`: Carga exitosa
    /// - `None`: Error en la lectura del disco
    ///
    /// # Notas
    /// - Este método se llama automáticamente cuando se necesita
    ///   acceder a datos no cargados
    /// - Marca el bloque como cargado (`loaded = true`)
    fn read_intern(&mut self) -> Option<()> {
        let mut count = self._start;
        
        // Leer encabezado
        let head = self._core.alloc.borrow_mut().read_disk(
            self._head_size as usize,
            count,
            self._core.disk_id as u16
        )?;
        count += self._head_size as u64;

        // Si no hay encabezado, bloque vacío
        if head.is_empty() {
            self.writed_bytes = 0;
            self.free_bytes = self._data;
            self.is_free = true;
            self.loaded = true;
            return Some(());
        }

        // Obtener cantidad de datos del encabezado
        let writed = be_to_size(&head, self._head_size as usize);
        let to_read = writed.min(self._data);
        self.writed_bytes = to_read;
        
        // Actualizar estados
        if to_read > 0 {
            self.free_bytes = self._data - to_read;
            self.is_free = false;
        } else {
            self.free_bytes = self._data;
            self.is_free = true;
        }

        // Leer y desofuscar datos
        let mut ofusc_data = self._core.alloc.borrow_mut().read_disk(
            to_read,
            count,
            self._core.disk_id as u16
        )?;
        let mut data = self.ofusc(&mut ofusc_data);
        
        // Almacenar en buffer
        self._buffer.write().ok()?.splice(.., data.drain(..));
        self.loaded = true;
        
        Some(())
    }

    /// Ofusca/Desofusca datos usando una clave derivada de metadatos.
    ///
    /// # Parámetros
    /// - `data`: Datos a procesar (ofuscar o desofuscar)
    ///
    /// # Funcionamiento
    /// - Usa XOR con una clave generada a partir de:
    ///   - Capa del bloque
    ///   - ID del disco
    ///   - Posición del bloque
    /// - El algoritmo es simétrico (misma función para ofuscar y desofuscar)
    ///
    /// # Retorno
    /// - `Vec<u8>`: Datos procesados
    ///
    /// # Notas de seguridad
    /// - Esta es una ofuscación básica, no criptografía fuerte
    /// - La clave se deriva de metadatos conocidos
    fn ofusc(&self, data: &Vec<u8>) -> Vec<u8> {
        // Generar clave única para este bloque
        let key = AllocatorBlock::get_ofusc_key(
            self._core.layer as u64,
            self._core.disk_id as u64,
            self._core.pos,
        );

        let mut out = Vec::with_capacity(data.len());

        // Aplicar XOR byte por byte
        for (i, &b) in data.iter().enumerate() {
            let shift = (i % 8) * 8;
            let k = ((key >> shift) & 0xFF) as u8;
            out.push(b ^ k);
        }

        out
    }
}

// Implementación del trait Display para depuración
impl fmt::Display for BaseSheetShadowdBlock {
    /// Formatea el bloque para mostrar información de depuración.
    ///
    /// # Formato
    /// ```
    /// BaseShadowdBlock(
    ///     index=0
    ///     disk=0
    ///     writed=1024
    ///     free=3072
    ///     is free=false
    /// )
    /// ```
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

/// Bloque de entrada que referencia a un BaseSheetShadowdBlock.
///
/// Esta estructura actúa como un wrapper que contiene:
/// - Un BaseSheetShadowdBlock real
/// - Información de validez de la entrada
/// - Posición lógica dentro de una estructura mayor
///
/// # Campos
/// - `pos`: Posición lógica de esta entrada
/// - `bs_sb`: Bloque base real que contiene los datos
pub struct EntrySheetShadowdBlock {
    pos: usize,
    bs_sb: BaseSheetShadowdBlock,
    is_valid:bool,
}

impl EntrySheetShadowdBlock {
    pub fn new_invalid(pos:usize, bs_sb:BaseSheetShadowdBlock) -> Self{
        Self { pos, bs_sb, is_valid: false }
    }
    pub fn new_valid(pos:usize, bs_sb:BaseSheetShadowdBlock) -> Self{
        Self { pos, bs_sb, is_valid: true }
    }
    pub fn get_pos(&self) -> usize{
        self.pos
    }
    pub fn is_valid(&self) -> bool{
        self.is_valid
    }
    pub fn get_bs(&mut self)-> Option<&mut BaseSheetShadowdBlock>{
        if self.is_valid {
            Some(&mut self.bs_sb)
        }else{
            None
        }
    }
}