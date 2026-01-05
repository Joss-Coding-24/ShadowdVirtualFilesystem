use crate::helpers::convertions::be_to_size;
struct CursorCore{
    pos:usize,
    disk:u16,
    type_:u8,
}
impl CursorCore {
    fn charge(v: &mut Vec<u8>) -> Option<Self> {
        if v.len() < 11 {
            return None;
        }

        // Consumimos exactamente 11 bytes
        let chunk: Vec<u8> = v.drain(..11).collect();

        // Layout:
        // [1b type][2b disk][8b pos]
        let type_v = &chunk[0..1];
        let disk_v = &chunk[1..3];
        let pos_v  = &chunk[3..11];

        Some(Self {
            type_: be_to_size(type_v, 1) as u8,
            disk:  be_to_size(disk_v, 2) as u16,
            pos:   be_to_size(pos_v, 8),
        })
    }

}
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
}
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
}
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
}
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

}
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
}
#[derive(Debug)]
pub struct Cursor{}
