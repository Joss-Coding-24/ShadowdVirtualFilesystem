//! Shadowd Block.
//! Maneja los sistemas de persistencia, propagacion de errores, etc
use std::{
    cell::RefCell,
    rc::Rc};
use crate::block::allocator_block::AllocatorBlock;
/// Allocator.
/// Es el encargado de gestionar la creacion de HelperDisk
/// Ademas de proporcionar datos para la correcta construccion de los bloques
pub mod allocator_block;
///Shadowd Block
/// Es el encargado de brindar una plantilla para mantejer una similitud entre lo diferentes bloques
/// Brinda ShadowdBlockCore que es el core de bloque junto a Block que es el contrato general que deben seguir los distintos bloques
pub mod shadowd_block;
/// Base Sheed Shadowd Block.
/// No solo es el bloque base de todos los bloques, si no tambien es el encargado de gestionar los datos reales
/// Tambien es el encargado de hacer que cada uno de los demas bloques funcionen
pub mod base_sheet_shadowd_block;
/// Base Branch Shadowd Block
/// Segundo nivel de un cursor
/// Es el encargado de gestionar los BaseSheetShadowdBlock
pub mod base_branch_shadowd_block_1;
/// Insert helpers.
/// Brinda contexto en modificacion del buffer y operaciones de trasporte de datos
pub mod insert_helpers;
/// Disk Helper.
/// Representante de un DiskFile, es el encargado de escribir y leer en el
/// Debe ser usado para operaciones de I/O en disco
mod disk_helper;
/// AllocHadle
/// Es un tipo especial para contener el alloc, es usado por los bloques
pub type AllocHadle = Rc<RefCell<AllocatorBlock>>;
/// ShadowdDispatcher
/// Es una especie de sheduler para las operaciones de transporte
pub mod shadowd_dispatcher;