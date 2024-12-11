use super::*;
use crate::parser::CloseType;
use basic_func::BasicState;

#[derive(Debug)]
pub struct StarState {
    count: u8,
}

impl BasicState for StarState {
    fn get_name(&self) -> &'static str {
        "Star"
    }

    fn get_state_return(&self) -> ReturnType {
        ReturnType::Void
    }

    fn get_child_type(&self) -> Types {
        Types::Number
    }

    fn do_first(&mut self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Star {
                locs,
                indexes: [usize::MAX; 4],
                end: End::none(),
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize, _: ReturnType) {
        if let Expr::Star { indexes, .. } = expr {
            indexes[self.count as usize] = index;
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    fn can_close(&self) -> CloseType {
        match self.count {
            0 => CloseType::Unable,
            1 => CloseType::Able,
            /*
                1-argument: width from current position
                3-argument: x and y coordinates then width
                Draw from the middle
            */
            2 => CloseType::Unable,
            3 => CloseType::Force,
            _ => unreachable!(),
        }
    }

    fn set_end(&mut self, expr: &mut Expr, index: End) {
        if let Expr::Star { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl StarState {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

#[derive(Debug)]
pub struct PolygonState {
    count: u8,
}

impl BasicState for PolygonState {
    fn get_name(&self) -> &'static str {
        "Polygon"
    }

    fn get_state_return(&self) -> ReturnType {
        ReturnType::Void
    }

    fn get_child_type(&self) -> Types {
        Types::Number
    }

    fn do_first(&mut self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Polygon {
                locs,
                indexes: [usize::MAX; 4],
                end: End::none(),
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize, _: ReturnType) {
        if let Expr::Polygon { indexes, .. } = expr {
            indexes[self.count as usize] = index;
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    fn can_close(&self) -> CloseType {
        match self.count {
            0 => CloseType::Unable,
            1 => CloseType::Unable,
            2 => CloseType::Able,
            /*
                2-argument: width sides from current position
                4-argument: x and y coordinate then width and sides
                Draw from the middle
            */
            3 => CloseType::Unable,
            4 => CloseType::Force,
            _ => unreachable!(),
        }
    }

    fn set_end(&mut self, expr: &mut Expr, index: End) {
        if let Expr::Polygon { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl PolygonState {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

#[derive(Debug)]
pub struct TriangleState {
    count: u8,
}

impl BasicState for TriangleState {
    fn get_name(&self) -> &'static str {
        "Triangle"
    }

    fn get_state_return(&self) -> ReturnType {
        ReturnType::Void
    }

    fn get_child_type(&self) -> Types {
        Types::Number
    }

    fn do_first(&mut self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Triangle {
                locs,
                indexes: [usize::MAX; 4],
                end: End::none(),
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize, _: ReturnType) {
        if let Expr::Triangle { indexes, .. } = expr {
            indexes[self.count as usize] = index;
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    fn can_close(&self) -> CloseType {
        match self.count {
            0 => CloseType::Unable,
            1 => CloseType::Able,
            /*
                1-argument: width from current position
                3-argument: x and y coordinates then width
                Draw from the middle
            */
            2 => CloseType::Unable,
            3 => CloseType::Force,
            _ => unreachable!(),
        }
    }

    fn set_end(&mut self, expr: &mut Expr, index: End) {
        if let Expr::Triangle { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl TriangleState {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

#[derive(Debug)]
pub struct HeartState {
    count: u8,
}

impl BasicState for HeartState {
    fn get_name(&self) -> &'static str {
        "Heart"
    }

    fn get_state_return(&self) -> ReturnType {
        ReturnType::Void
    }

    fn get_child_type(&self) -> Types {
        Types::Number
    }

    fn do_first(&mut self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Heart {
                locs,
                indexes: [usize::MAX; 4],
                end: End::none(),
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize, _: ReturnType) {
        if let Expr::Heart { indexes, .. } = expr {
            indexes[self.count as usize] = index;
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    fn can_close(&self) -> CloseType {
        match self.count {
            0 => CloseType::Unable,
            1..=3 => CloseType::Able,
            /*
                1-argument: width from current position
                2-argument: width and height from current position
                3-argument: x and y coordinates then width
                4-argument: x and y coordinate then width and height
                Draw from the middle
            */
            4 => CloseType::Force,
            _ => unreachable!(),
        }
    }

    fn set_end(&mut self, expr: &mut Expr, index: End) {
        if let Expr::Heart { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl HeartState {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

#[derive(Debug)]
pub struct RoundRecState {
    count: u8,
}

impl BasicState for RoundRecState {
    fn get_name(&self) -> &'static str {
        "RoundRec"
    }

    fn get_state_return(&self) -> ReturnType {
        ReturnType::Void
    }

    fn get_child_type(&self) -> Types {
        Types::Number
    }

    fn do_first(&mut self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::RoundRec {
                locs,
                indexes: [usize::MAX; 4],
                end: End::none(),
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize, _: ReturnType) {
        if let Expr::RoundRec { indexes, .. } = expr {
            indexes[self.count as usize] = index;
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    fn can_close(&self) -> CloseType {
        match self.count {
            0 => CloseType::Unable,
            1 => CloseType::Unable,
            2..=4 => CloseType::Able,
            /*
                2-argument: width from current position then radius
                3-argument: width and height from current position then radius
                4-argument: x and y coordinates then width then radius
                5-argument: x and y coordinate then width and height then radius
                Draw from the middle
            */
            5 => CloseType::Force,
            _ => unreachable!(),
        }
    }

    fn set_end(&mut self, expr: &mut Expr, index: End) {
        if let Expr::RoundRec { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl RoundRecState {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

#[derive(Debug)]
pub struct KirbyState {
    count: u8,
}

impl BasicState for KirbyState {
    fn get_name(&self) -> &'static str {
        "Kirby"
    }

    fn get_state_return(&self) -> ReturnType {
        ReturnType::Void
    }

    fn get_child_type(&self) -> Types {
        Types::Number
    }

    fn do_first(&mut self, expr: &mut Expr, locs: Vec<usize>) -> bool {
        let ret = self.count == 0;
        if ret {
            *expr = Expr::Kirby {
                locs,
                indexes: [usize::MAX; 4],
                end: End::none(),
            }
        }
        ret
    }

    fn add_child(&mut self, expr: &mut Expr, index: usize, _: ReturnType) {
        if let Expr::Kirby { indexes, .. } = expr {
            indexes[self.count as usize] = index;
            self.count += 1;
        } else {
            unreachable!()
        }
    }

    fn can_close(&self) -> CloseType {
        match self.count {
            0 => CloseType::Unable,
            1..=3 => CloseType::Able,
            /*
                1-argument: width from current position
                2-argument: width and height from current position
                3-argument: x and y coordinates then width
                4-argument: x and y coordinate then width and height
                Draw from the middle
            */
            4 => CloseType::Force,
            _ => unreachable!(),
        }
    }

    fn set_end(&mut self, expr: &mut Expr, index: End) {
        if let Expr::Kirby { end, .. } = expr {
            *end = index;
        } else {
            unreachable!()
        }
    }
}

impl KirbyState {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}
