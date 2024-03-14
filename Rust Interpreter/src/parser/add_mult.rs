use super::*;

#[derive(Debug)]
pub enum BiFunctionType{
    Add,
    Mult
}

#[derive(Debug)]
pub struct BiFuncState{
    children:u8,
    fn_type:BiFunctionType
}
impl ParseState for BiFuncState{
    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {
        todo!()
    }

    fn step_match(
        &mut self,
        env: &mut Enviroment,
        did_child_match: bool,
        word: &Slice,
        rest: &Slice,
    ) -> MatchResult {
        todo!()
    }

    fn get_name(&self) -> &'static str {
        todo!()
    }
    fn do_replace(&self) -> bool {
        false
    }
}
impl BiFuncState{
    fn new(fn_type:BiFunctionType)->Self{
        BiFuncState{
            children:0,
            fn_type
        }
    }

    pub fn new_add()->Self{
        Self::new(BiFunctionType::Add)
    }

    pub fn new_mult()->Self{
        Self::new(BiFunctionType::Mult)
    }
}