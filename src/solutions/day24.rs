use std::collections::HashSet;

pub fn problem1(input: &str) -> String {
    let program = parser::parse(input).unwrap().1;
    dfs_model_number(&program, true)
}

pub fn problem2(input: &str) -> String {
    let program = parser::parse(input).unwrap().1;
    dfs_model_number(&program, false)
}

fn dfs_model_number(program: &[Instruction], max: bool) -> String {
    let mut path = Vec::new();
    dfs_model_number_rec(
        max,
        program,
        &mut path,
        &mut HashSet::new(),
        RegisterState::default(),
    );

    String::from_iter(
        path.iter()
            .map(|x| char::from_digit(*x as u32, 10).unwrap()),
    )
}

fn dfs_model_number_rec(
    max: bool,
    program: &[Instruction],
    path: &mut Vec<i64>,
    seen: &mut HashSet<RegisterState>,
    state: RegisterState,
) -> bool {
    if seen.contains(&state) {
        return false;
    }

    let nums: Box<dyn Iterator<Item = i64>> = match max {
        true => Box::new((1..10).rev()),
        false => Box::new(1..10),
    };

    for i in nums {
        path.push(i);
        match run_machine(program, state, i) {
            InterruptState::InputRequred(s) => {
                let ok = dfs_model_number_rec(max, program, path, seen, s);
                seen.insert(s);
                if ok {
                    return true;
                }
            }
            InterruptState::Crashed => (),
            InterruptState::Complete(s) => {
                if s.z == 0 {
                    return true;
                }
            }
        }
        path.pop();
    }

    false
}

fn run_machine(program: &[Instruction], state: RegisterState, input: i64) -> InterruptState {
    let mut m = Machine {
        program: program,
        state: state,
        next_input: Some(input),
    };

    match m.run() {
        Interrupt::InputRequred => InterruptState::InputRequred(m.state),
        Interrupt::Crashed => InterruptState::Crashed,
        Interrupt::Complete => InterruptState::Complete(m.state),
    }
}

#[derive(Clone, Copy, Debug)]
enum InterruptState {
    InputRequred(RegisterState),
    Crashed,
    Complete(RegisterState),
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Inp(Register),
    Add(Register, Operand),
    Mul(Register, Operand),
    Div(Register, Operand),
    Mod(Register, Operand),
    Eql(Register, Operand),
}

#[derive(Clone, Copy, Debug)]
pub enum Operand {
    Register(Register),
    Value(i64),
}

impl Operand {
    fn value(&self, m: RegisterState) -> i64 {
        match self {
            Self::Register(r) => *m.register(*r),
            Self::Value(v) => *v,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Register {
    W,
    X,
    Y,
    Z,
}

#[derive(Clone, Debug)]
struct Machine<'a> {
    program: &'a [Instruction],
    state: RegisterState,
    next_input: Option<i64>,
}

impl<'a> Machine<'a> {
    fn run(&mut self) -> Interrupt {
        loop {
            if let Some(interrupt) = self.step() {
                return interrupt;
            }
        }
    }

    fn step(&mut self) -> Option<Interrupt> {
        let Some(inst) = self.program.get(self.state.pc) else {return Some(Interrupt::Complete)};

        match inst {
            Instruction::Inp(r) => {
                let Some(input) = self.next_input.take() else {return Some(Interrupt::InputRequred)};
                self.state.set_register(*r, input)
            }
            Instruction::Add(a, b) => *self.state.register_mut(*a) += b.value(self.state),
            Instruction::Mul(a, b) => *self.state.register_mut(*a) *= b.value(self.state),
            Instruction::Div(a, b) => {
                let bv = b.value(self.state);
                if bv == 0 {
                    return Some(Interrupt::Crashed);
                }
                *self.state.register_mut(*a) /= bv
            }
            Instruction::Mod(a, b) => {
                let bv = b.value(self.state);
                let r = self.state.register_mut(*a);
                if *r < 0 || bv <= 0 {
                    return Some(Interrupt::Crashed);
                }
                *r %= bv
            }
            Instruction::Eql(a, b) => {
                let bv = b.value(self.state);
                let r = self.state.register_mut(*a);
                *r = if *r == bv { 1 } else { 0 }
            }
        };

        self.state.pc += 1;

        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Interrupt {
    InputRequred,
    Crashed,
    Complete,
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct RegisterState {
    w: i64,
    x: i64,
    y: i64,
    z: i64,

    // program counter
    pc: usize,
}

impl RegisterState {
    fn register(&self, r: Register) -> &i64 {
        match r {
            Register::W => &self.w,
            Register::X => &self.x,
            Register::Y => &self.y,
            Register::Z => &self.z,
        }
    }

    fn register_mut(&mut self, r: Register) -> &mut i64 {
        match r {
            Register::W => &mut self.w,
            Register::X => &mut self.x,
            Register::Y => &mut self.y,
            Register::Z => &mut self.z,
        }
    }

    fn set_register(&mut self, r: Register, v: i64) {
        match r {
            Register::W => self.w = v,
            Register::X => self.x = v,
            Register::Y => self.y = v,
            Register::Z => self.z = v,
        }
    }
}

mod parser {
    use super::*;
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
        let parser = separated_list1(line_ending, instruction);
        complete(parser)(input)
    }

    fn register(input: &str) -> IResult<&str, Register> {
        let mut parser = alt((
            value(Register::W, tag("w")),
            value(Register::X, tag("x")),
            value(Register::Y, tag("y")),
            value(Register::Z, tag("z")),
        ));
        parser(input)
    }

    fn operand(input: &str) -> IResult<&str, Operand> {
        let mut parser = alt((
            map(register, |r| Operand::Register(r)),
            map(int, |v| Operand::Value(v)),
        ));
        parser(input)
    }

    fn instruction(input: &str) -> IResult<&str, Instruction> {
        let binary_params = || separated_pair(register, space1, operand);

        let mut parser = alt((
            inst("inp", register, |r| Instruction::Inp(r)),
            inst("add", binary_params(), |(a, b)| Instruction::Add(a, b)),
            inst("mul", binary_params(), |(a, b)| Instruction::Mul(a, b)),
            inst("div", binary_params(), |(a, b)| Instruction::Div(a, b)),
            inst("mod", binary_params(), |(a, b)| Instruction::Mod(a, b)),
            inst("eql", binary_params(), |(a, b)| Instruction::Eql(a, b)),
        ));

        parser(input)
    }

    fn inst<'a, P, OP, O, E, F>(
        op: &'a str,
        parameters: P,
        f: F,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        E: ParseError<&'a str>,
        F: FnMut(OP) -> O,
        P: Parser<&'a str, OP, E>,
    {
        let params = preceded(pair(tag(op), space1), parameters);
        map(params, f)
    }
}
