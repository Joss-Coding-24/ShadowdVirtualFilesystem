use crate::helpers::convertions::{
    be_to_size,
    size_to_be
};

#[derive(Debug)]
struct CursorCore{
    pos:u8,
    disk:u16,
    type_:u8,
}
impl CursorCore {
    fn charge(v: &mut Vec<u8>) -> Option<Self> {
        if v.len() < 8 {
            return None;
        }

        // Consumimos exactamente 8 bytes
        let chunk: Vec<u8> = v.drain(..8).collect();

        // Layout:
        // [1b type][2b disk][5b pos]
        let type_v = &chunk[0..1];
        let disk_v = &chunk[1..3];
        let pos_v  = &chunk[3..8];

        Some(Self {
            type_: be_to_size(type_v, 1) as u8,
            disk:  be_to_size(disk_v, 2) as u16,
            pos:   be_to_size(pos_v, 5) as u8,
        })
    }
    fn serialize(&self)->Vec<u8>{
        let mut out: Vec<u8> = Vec::with_capacity(8);
        let type_ = size_to_be(self.type_ as usize, 1);
        let disk = size_to_be(self.disk as usize, 2);
        let pos = size_to_be(self.pos as usize, 5);
    
        out.extend_from_slice(type_.as_slice());
        out.extend_from_slice(disk.as_slice());
        out.extend_from_slice(pos.as_slice());
        out
    }
    fn serialize_into(&self, dst:&mut Vec<u8>){
        dst.extend(self.serialize());
    }
}
#[derive(Debug)]
struct Cursor1{
    core:CursorCore,
}
impl Cursor1 {
    fn charge(v: &mut Vec<u8>)-> Option<Self>{
        let core = CursorCore::charge(v)?;
        Some(
            Self { 
                core: core,
             }
        )
    }
    fn serialize_into(&self, dst:&mut Vec<u8>){
        self.core.serialize_into(dst);
    }
}

#[derive(Debug)]
struct Cursor2{
    core:CursorCore,
    cur1:Cursor1,
}
impl Cursor2 {
    fn charge(v: &mut Vec<u8>)-> Option<Self>{
        let core = CursorCore::charge(v)?;
        let cur = Cursor1::charge(v)?;
        Some(
            Self { 
                core: core,
                cur1: cur
             }
        )
    }
    fn serialize_into(&self, dst:&mut Vec<u8>){
        self.core.serialize_into(dst);
        self.cur1.serialize_into(dst);
    }
}
#[derive(Debug)]
struct Cursor3{
    core:CursorCore,
    cur2:Cursor2,
}
impl Cursor3 {
    fn charge(v: &mut Vec<u8>)-> Option<Self>{
        let core = CursorCore::charge(v)?;
        let cur = Cursor2::charge(v)?;
        Some(
            Self { 
                core: core,
                cur2: cur
             }
        )
    }
    fn serialize_into(&self, dst:&mut Vec<u8>){
        self.core.serialize_into(dst);
        self.cur2.serialize_into(dst);
    }
}
#[derive(Debug)]
struct Cursor4{
    core:CursorCore,
    cur3:Cursor3,
}
impl Cursor4 {
    fn charge(v: &mut Vec<u8>)-> Option<Self>{
        let core = CursorCore::charge(v)?;
        let cur = Cursor3::charge(v)?;
        Some(
            Self { 
                core: core,
                cur3: cur
             }
        )
    }
    fn serialize_into(&self, dst:&mut Vec<u8>){
        self.core.serialize_into(dst);
        self.cur3.serialize_into(dst);
    }
}

#[derive(Debug)]
struct Cursor5{
    core:CursorCore,
    cur4:Cursor4,
}
impl Cursor5 {
    fn charge(v: &mut Vec<u8>)-> Option<Self>{
        let core = CursorCore::charge(v)?;
        let cur = Cursor4::charge(v)?;
        Some(
            Self { 
                core: core,
                cur4: cur
             }
        )
    }
    fn serialize_into(&self, dst:&mut Vec<u8>){
        self.core.serialize_into(dst);
        self.cur4.serialize_into(dst);
    }
}
#[derive(Debug)]
pub struct Cursor{
    cur6:Cursor1,
    cur5:Cursor5,
    cur4:Cursor4,
    cur3:Cursor3,
    cur2:Cursor2,
    cur1:Cursor1,
    layer:u8,
}
impl Cursor {
    pub fn new(v:&mut Vec<u8>)->Option<Self>{
        let _6 = Cursor1::charge(v)?;
        let _5 = Cursor5::charge(v)?;
        let _4 = Cursor4::charge(v)?;
        let _3 = Cursor3::charge(v)?;
        let _2 = Cursor2::charge(v)?;
        let _1 = Cursor1::charge(v)?;

        Some(
            Self{
                cur6:_6,
                cur5:_5,
                cur4:_4,
                cur3:_3,
                cur2:_2,
                cur1:_1,
                layer:6
            }
        )
    }
    pub fn serialize_into(&self, dst:&mut Vec<u8>){
        self.cur6.serialize_into(dst);
        self.cur5.serialize_into(dst);
        self.cur4.serialize_into(dst);
        self.cur3.serialize_into(dst);
        self.cur2.serialize_into(dst);
        self.cur1.serialize_into(dst);
    }
    pub fn get_pos(&self, cn:u8) -> Option<u8>{
        self.resolve_core(cn).map(|c| c.pos)
    }
    pub fn set_pos(&mut self, cn:u8, v:u8) -> bool{
        if v >= 31 {
            false
        }else if let Some(core) = self.resolve_core_mut(cn){
            core.pos = v;
            true
        }else{
            false
        }
    }
    pub fn get_disk(&self, cn:u8) -> Option<u16>{
        self.resolve_core(cn).map(|c| c.disk)
    }
    pub fn set_disk(&mut self, cn:u8, v:u16) -> bool{
        if let Some(core) = self.resolve_core_mut(cn){
            core.disk = v;
            true
        }else{
            false
        }
    }
    pub fn get_type(&self, cn:u8) -> Option<u8>{
        self.resolve_core(cn).map(|c| c.type_)
    }
    pub fn set_type(&mut self, cn:u8, v:u8) -> bool{
        if let Some(core) = self.resolve_core_mut(cn){
            core.type_ = v;
            true
        }else{
            false
        }
    }
}

/// añade funciones para hacer directo el obtener el core necesario

impl Cursor {
    fn resolve_core(&self, cn: u8) -> Option<&CursorCore> {
        if cn == 0 || cn > self.layer {
            return None;
        }

        match self.layer {
            6 => match cn {
                6 => Some(&self.cur6.core),
                5 => Some(&self.cur5.core),
                4 => Some(&self.cur5.cur4.core),
                3 => Some(&self.cur5.cur4.cur3.core),
                2 => Some(&self.cur5.cur4.cur3.cur2.core),
                1 => Some(&self.cur5.cur4.cur3.cur2.cur1.core),
                _ => None,
            },
            5 => match cn {
                5 => Some(&self.cur5.core),
                4 => Some(&self.cur5.cur4.core),
                3 => Some(&self.cur5.cur4.cur3.core),
                2 => Some(&self.cur5.cur4.cur3.cur2.core),
                1 => Some(&self.cur5.cur4.cur3.cur2.cur1.core),
                _ => None,
            },
            4 => match cn {
                4 => Some(&self.cur4.core),
                3 => Some(&self.cur4.cur3.core),
                2 => Some(&self.cur4.cur3.cur2.core),
                1 => Some(&self.cur4.cur3.cur2.cur1.core),
                _ => None,
            },
            3 => match cn {
                3 => Some(&self.cur3.core),
                2 => Some(&self.cur3.cur2.core),
                1 => Some(&self.cur3.cur2.cur1.core),
                _ => None,
            },
            2 => match cn {
                2 => Some(&self.cur2.core),
                1 => Some(&self.cur2.cur1.core),
                _ => None,
            },
            1 => match cn {
                1 => Some(&self.cur1.core),
                _ => None,
            },
            _ => None,
        }
    }
    fn resolve_core_mut(&mut self, cn: u8) -> Option<&mut CursorCore> {
        if cn == 0 || cn > self.layer {
            return None;
        }

        match self.layer {
            6 => match cn {
                6 => Some(&mut self.cur6.core),
                5 => Some(&mut self.cur5.core),
                4 => Some(&mut self.cur5.cur4.core),
                3 => Some(&mut self.cur5.cur4.cur3.core),
                2 => Some(&mut self.cur5.cur4.cur3.cur2.core),
                1 => Some(&mut self.cur5.cur4.cur3.cur2.cur1.core),
                _ => None,
            },
            5 => match cn {
                5 => Some(&mut self.cur5.core),
                4 => Some(&mut self.cur5.cur4.core),
                3 => Some(&mut self.cur5.cur4.cur3.core),
                2 => Some(&mut self.cur5.cur4.cur3.cur2.core),
                1 => Some(&mut self.cur5.cur4.cur3.cur2.cur1.core),
                _ => None,
            },
            4 => match cn {
                4 => Some(&mut self.cur4.core),
                3 => Some(&mut self.cur4.cur3.core),
                2 => Some(&mut self.cur4.cur3.cur2.core),
                1 => Some(&mut self.cur4.cur3.cur2.cur1.core),
                _ => None,
            },
            3 => match cn {
                3 => Some(&mut self.cur3.core),
                2 => Some(&mut self.cur3.cur2.core),
                1 => Some(&mut self.cur3.cur2.cur1.core),
                _ => None,
            },
            2 => match cn {
                2 => Some(&mut self.cur2.core),
                1 => Some(&mut self.cur2.cur1.core),
                _ => None,
            },
            1 => match cn {
                1 => Some(&mut self.cur1.core),
                _ => None,
            },
            _ => None,
        }
    }
}