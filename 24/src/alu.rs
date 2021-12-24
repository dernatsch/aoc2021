#[derive(Debug)]
pub struct AluError{
    pub err: &'static str,
}

type AluResult = Result<(), AluError>;
type InputCallback = fn() -> Result<i64, AluError>;

pub struct ALU {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    get_input: InputCallback,
}

impl ALU {
    pub fn create(get_input: InputCallback) -> ALU {
        ALU {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
            get_input
        }
    }

    pub fn run(&mut self, instruction: &str) -> AluResult {
        let mut iter = instruction.split_whitespace();

        match iter.next() {
            Some("inp") => {
                if let (Some(var), Ok(val)) = (iter.next(), (self.get_input)()) {
                    self.set(var, val)
                } else {
                    Err(AluError{err: "given input or entered number invalid."})
                }
            },
            Some("add") => self.do_binary_operation(&mut iter, &std::ops::Add::add),
            Some("mul") => self.do_binary_operation(&mut iter, &std::ops::Mul::mul),
            Some("div") => self.do_binary_operation(&mut iter, &std::ops::Div::div),
            Some("mod") => self.do_binary_operation(&mut iter, &ALU::modulo_operation),
            Some("eql") => self.do_binary_operation(&mut iter, &ALU::equal_operation),
            _ => Err(AluError{err: "weiß auch net"})
        }
    }

    fn set(&mut self, var_name: &str, value: i64) -> AluResult {
        match var_name {
            "w" => { self.w = value; Ok(())},
            "x" => { self.x = value; Ok(())},
            "y" => { self.y = value; Ok(())},
            "z" => { self.z = value; Ok(())},
            _ =>   { Err(AluError{err: "invalid left operand"}) },
        }
    }

    fn get(&self, var_name: &str) -> Result<i64, AluError> {
        match var_name {
            "w" => Ok(self.w),
            "x" => Ok(self.x),
            "y" => Ok(self.y),
            "z" => Ok(self.z),
            s => match s.parse::<i64>() {
                Ok(num) => Ok(num),
                Err(_) => Err(AluError{err: "invalid operand"}),
            }
        }
    }

    fn do_binary_operation(&mut self, iter: &mut dyn Iterator<Item = &str>, operation: &dyn Fn(i64, i64) -> i64) -> AluResult {
        if let (Some(var1), Some(var2)) = (iter.next(), iter.next()) {
            let tmp = operation(self.get(var1)?, self.get(var2)?);
            self.set(var1, tmp)
        } else{
            Err(AluError{err: "could not identify two operands for operation"})
        }
    }

    fn modulo_operation(a: i64, b: i64) -> i64 {
        a % b
    }

    fn equal_operation(a: i64, b: i64) -> i64 {
        match a == b {
            true => 1,
            false => 0,
        }
    }

    pub fn print_state(&self) {
        println!("State: x={}, y={}, z={}, w={}", self.x, self.y, self.z, self.w);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicI64, Ordering};

    #[test]
    fn some_instructions_work() {
        static INPUT: AtomicI64 = AtomicI64::new(0);

        fn input() -> Result<i64, AluError> {
            Ok(INPUT.load(Ordering::Relaxed))
        }

        // create alu: x=0, y=0, z=0, w=0
        let mut alu = ALU::create(input);

        // store 10 in w: x=0, y=0, z=0, w=10
        INPUT.store(10, Ordering::Relaxed);
        alu.run("inp w").unwrap();
        assert_eq!(alu.w, 10);

        // store 20 in x: x=20, y=0, z=0, w=10
        INPUT.store(20, Ordering::Relaxed);
        alu.run("inp x").unwrap();
        assert_eq!(alu.x, 20);

        // store sum of x,w in x: x=30, ...
        alu.run("add x w").unwrap();
        assert_eq!(alu.x, 30);

        // store prod of x,w in w: x=30, w=300
        alu.run("mul w x").unwrap();
        assert_eq!(alu.w, 300);

        // store div of x,w in w: x=30, w=10
        alu.run("div w x").unwrap();
        assert_eq!(alu.w, 10);

        INPUT.store(20, Ordering::Relaxed);
        alu.run("inp y").unwrap();
        assert_eq!(alu.y, 20);

        // store mod of w % x in w: x=30, w=10
        alu.run("mod w y").unwrap();
        assert_eq!(alu.w, 10);

        //check w == x and store in w: w=0
        alu.run("eql w x").unwrap();
        assert_eq!(alu.w, 0);

        // check w == z and store in w
        alu.run("eql w z").unwrap();
        assert_eq!(alu.w, 1);
        
    }
}
