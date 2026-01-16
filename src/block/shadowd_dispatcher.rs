
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use crate::{GLOBAL_RUNTIME, algoritm::cursors::Cursor, block::insert_helpers::{TransitOption, TransitReturn, TransitStates}};
//Nota para mi yo del futuro, cualquier espectador u otro
//Este es mi primer encuentro con cocurrencia aplica... me estoy guiando por la ducumentacion
#[derive(Clone)]
pub struct ShadowdDispatcher{
    cur: Cursor,
    sheduler: Arc<Runtime>,
    option: TransitOption,
    data: Vec<u8>,
    state: TransitStates,
}

impl ShadowdDispatcher {
    pub fn get_cur(&self)-> &Cursor{
        &self.cur
    }
    pub fn get_data(&self) -> &Vec<u8>{
        &self.data
    }
    pub fn get_state(&self) -> TransitStates {
        self.state
    }
}

impl ShadowdDispatcher {
    pub fn new(cur:Cursor, datas:TransitReturn)-> Self{
        Self { 
            cur: cur, 
            sheduler: Arc::new(Runtime::new().expect("Error al lanzar un Dispatcher")), 
            option: datas.action, 
            data: datas.data, 
            state: datas.state
        }
    }

    pub fn create_with_runtime(cur:Cursor, datas:TransitReturn, runtime:Arc<Runtime>) -> Self{
        Self{
            cur:cur,
            sheduler:runtime,
            option:datas.action,
            data:datas.data,
            state:datas.state
        }
    }

    pub fn create_with_global_runtime(cur:Cursor, datas:TransitReturn)-> Self{
        Self { 
            cur: cur, 
            sheduler: GLOBAL_RUNTIME.clone(),
            option: datas.action, 
            data: datas.data, 
            state: datas.state
        }
    }

    pub fn start(self) -> Self{
        match self.option {
            TransitOption::InsertBegin => self.init_insert_process(),
            TransitOption::InsertInPos => self.init_insert_process(),
            TransitOption::InsertEnd => self.init_insert_process(),
            TransitOption::Finalize => self,
            TransitOption::DeletePosToIndicator => todo!(),
            TransitOption::DeletePosDefault => todo!(),
            TransitOption::DeletePosBytesToEnd => todo!(),
            TransitOption::DeletePosBytesToBegin => todo!()
        }
    }

    fn init_insert_process(self) -> Self{
        let nuevo = self.clone();
        let shared = Arc::new(Mutex::new(self));
        let shared_clone = shared.clone();
        nuevo.sheduler.spawn_blocking(
            move ||{
                let mut guard = shared_clone.lock().unwrap();
                guard.insert_process();
            }
        );
        nuevo
    }
    
    fn insert_process(&mut self){
        'process_main_loop: loop{
            break 'process_main_loop;
            //aun no puedo definir esto... me adelante demaciado
        }
    }
}