use crate::helpers::convertions::{
    be_to_size,
    size_to_be
};

#[derive(Debug, Clone)]
struct CursorCore{
    pos:u8,
}
impl CursorCore {
    fn new()->Self{
        Self{
            pos: 0,
        }
    }
    fn charge(v: &mut Vec<u8>) -> Option<Self> {
        if v.len() < 5 {
            return None;
        }

        // Consumimos exactamente 5 bytes
        let chunk: Vec<u8> = v.drain(..5).collect();

        // Layout:
        // [1b type][2b disk][5b pos]
        let pos_v  = &chunk[..];

        Some(Self {
            pos:   be_to_size(pos_v, 5) as u8,
        })
    }
    fn serialize(&self)->Vec<u8>{
        let mut out: Vec<u8> = Vec::with_capacity(5);
        let pos = size_to_be(self.pos as usize, 5);
    
        out.extend_from_slice(pos.as_slice());
        out
    }
    fn serialize_into(&self, dst:&mut Vec<u8>){
        dst.extend(self.serialize());
    }
    fn advance(&mut self, is_end:bool)-> u8{ //true para indicar un avance del nivrl logico siguiente
        if self.pos == 30{
            if is_end{
                return 3
            }
            self.pos = 0;
            1
        }else{
            self.pos+=1;
            2
        }
    }
    fn delay(&mut self, is_firts:bool) -> u8{
        if self.pos == 0 {
            if is_firts {
                return 3
            }
            self.pos = 30;
            1
        }else{
            self.pos-=1;
            2
        }
    }
    
    fn in_limit(&self, in_end:bool)-> bool{
        //si in_end es true retorna true si pos es 30
        //si in_end es false retorna false si pos es 
        if in_end {
            if self.pos == 30 {
                true
            }else{
                false
            }
        }else{
            if self.pos == 0 {
                true
            }else{
                false
            }
        }
    }
}
#[derive(Debug, Clone)]
struct Cursor1{
    core:CursorCore,
}
impl Cursor1 {
    fn new() -> Self{
        Self{
            core: CursorCore::new(),
        }
    }
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
    fn advance(&mut self, is_end:bool)->u8{
        self.core.advance(is_end)
    }
    fn delay(&mut self, is_firts:bool)->u8{
        self.core.delay(is_firts)
    }
    fn in_limit(&self, in_end:bool)->bool{
        self.core.in_limit(in_end)
    }
}

#[derive(Debug, Clone)]
struct Cursor2{
    core:CursorCore,
    cur1:Cursor1,
}
impl Cursor2 {
    fn new() -> Self{
        Self{
            core:CursorCore::new(),
            cur1: Cursor1::new(),
        }
    }
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
    fn advance(&mut self, is_end_:bool)->u8{
        let is_end = (self.core.pos == 30) && is_end_;
        let advance_r = self.cur1.advance(is_end);
        match advance_r {
            1 => self.core.advance(is_end),
            2 => 2,
            _ => 3
        }
    }
    fn delay(&mut self, is_firts_:bool)->u8{
        let is_firts = (self.core.pos == 30) && is_firts_;
        let delay_r = self.cur1.delay(is_firts);
        match delay_r {
            1 => self.core.delay(is_firts),
            2 => 2,
            _ => 3
        }
    }
    fn in_limit(&self, in_end:bool) -> bool{
        let core_in = self.core.in_limit(in_end);
        let cur_in = self.cur1.in_limit(in_end);
        core_in && cur_in
    }
}
#[derive(Debug, Clone)]
struct Cursor3{
    core:CursorCore,
    cur2:Cursor2,
}
impl Cursor3 {
    fn new()->Self{
        Self { 
            core: CursorCore::new(), 
            cur2: Cursor2::new()
        }
    }
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
    fn advance(&mut self, is_end_:bool)->u8{
        let is_end = (self.core.pos == 30) && is_end_;
        let advance_r = self.cur2.advance(is_end);
        match advance_r {
            1 => self.core.advance(is_end),
            2 => 2,
            _ => 3
        }
    }
    fn delay(&mut self, is_firts_:bool)->u8{
        let is_firts = (self.core.pos == 30) && is_firts_;
        let delay_r = self.cur2.delay(is_firts);
        match delay_r {
            1 => self.core.delay(is_firts),
            2 => 2,
            _ => 3
        }
    }
    fn in_limit(&self, in_end:bool) -> bool{
        let core_in = self.core.in_limit(in_end);
        let cur_in = self.cur2.in_limit(in_end);
        core_in && cur_in
    }
}
#[derive(Debug, Clone)]
struct Cursor4{
    core:CursorCore,
    cur3:Cursor3,
}
impl Cursor4 {
    fn new ()->Self{
        Self { 
            core: CursorCore::new(), 
            cur3: Cursor3::new() 
        }
    }
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
    fn advance(&mut self, is_end_:bool)->u8{
        let is_end = (self.core.pos == 30) && is_end_;
        let advance_r = self.cur3.advance(is_end);
        match advance_r {
            1 => self.core.advance(is_end),
            2 => 2,
            _ => 3
        }
    }
    fn delay(&mut self, is_firts_:bool)->u8{
        let is_firts = (self.core.pos == 30) && is_firts_;
        let delay_r = self.cur3.delay(is_firts);
        match delay_r {
            1 => self.core.delay(is_firts),
            2 => 2,
            _ => 3
        }
    }
    fn in_limit(&self, in_end:bool) -> bool{
        let core_in = self.core.in_limit(in_end);
        let cur_in = self.cur3.in_limit(in_end);
        core_in && cur_in
    }
}

#[derive(Debug, Clone)]
struct Cursor5{
    core:CursorCore,
    cur4:Cursor4,
}
impl Cursor5 {
    fn new()->Self{
        Self { 
            core: CursorCore::new(), 
            cur4: Cursor4::new()
        }
    }
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
    fn advance(&mut self, is_end_:bool)->u8{
        let is_end = (self.core.pos == 30) && is_end_;
        let advance_r = self.cur4.advance(is_end);
        match advance_r {
            1 => self.core.advance(is_end),
            2 => 2,
            _ => 3
        }
    }
    fn delay(&mut self, is_firts_:bool)->u8{
        let is_firts = (self.core.pos == 30) && is_firts_;
        let delay_r = self.cur4.delay(is_firts);
        match delay_r {
            1 => self.core.delay(is_firts),
            2 => 2,
            _ => 3
        }
    }
    fn in_limit(&self, in_end:bool) -> bool{
        let core_in = self.core.in_limit(in_end);
        let cur_in = self.cur4.in_limit(in_end);
        core_in && cur_in
    }
}
#[derive(Debug, Clone)]
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
    pub fn new()->Self{
        Self { 
            cur6: Cursor1::new(), 
            cur5: Cursor5::new(), 
            cur4: Cursor4::new(),
            cur3: Cursor3::new(), 
            cur2: Cursor2::new(), 
            cur1: Cursor1::new(), 
            layer: 6
        }
    }
    pub fn charge(v:&mut Vec<u8>)->Option<Self>{
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
    fn in_limit(&self, in_end:bool)->bool{
        let c1 = self.cur1.in_limit(in_end);
        let c2 = self.cur2.in_limit(in_end);
        let c3 = self.cur3.in_limit(in_end);
        let c4 = self.cur4.in_limit(in_end);
        let c5 = self.cur5.in_limit(in_end);
        let c6 = self.cur6.in_limit(in_end);
        c1 && c2 && c3 && c4 && c5 && c6
    }
    pub fn advance(&mut self)->u8{
        let is_end = self.in_limit(true);

        let r1 = self.cur1.advance(is_end);

        let r2 = if r1 == 1 {
            self.cur2.advance(is_end)
        }else{
            return r1;
        };
        let is_end = self.in_limit(true);

        let r3 = if r2 == 1{
            self.cur3.advance(is_end)
        }else{
            return r2;
        };
        let is_end = self.in_limit(true);
       
        let r4 = if r3 == 1{
            self.cur4.advance(is_end)
        }else{
            return r3;
        };
        let is_end = self.in_limit(true);

        let r5 = if r4 == 1{
            self.cur5.advance(is_end)
        }else{
            return r4;
        };
        let is_end = self.in_limit(true);

        let r6 = if r5 == 1{
            self.cur6.advance(is_end)
        }else{
            return r5;
        };
        r6
    }
    pub fn delay(&mut self)-> u8{
        let is_firts = self.in_limit(false);
        let r1 = self.cur1.delay(is_firts);
        let is_firts =self.in_limit(false);
        
        let r2 = if r1 == 1 {
            self.cur2.delay(is_firts)
        }else{
            return r1;
        };
        let is_firts =self.in_limit(false);

        let r3 = if r2 == 1{
            self.cur3.delay(is_firts)
        }else{
            return r2;
        };
        let is_firts = self.in_limit(false);

        let r4 = if r3 == 1 {
            self.cur4.delay(is_firts)
        }else{
            return r3;
        };
        let is_firts = self.in_limit(false);

        let r5 = if r4 == 1 {
            self.cur5.delay(is_firts)
        }else{
            return r4;
        };
        let is_firts = self.in_limit(false);

        let r6 = if r5 == 1 {
            self.cur6.delay(is_firts)
        }else{
            return r5;
        };

        r6
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
