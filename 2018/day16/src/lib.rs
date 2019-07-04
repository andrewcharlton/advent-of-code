#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Op {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

pub fn all_ops() -> Vec<Op> {
    use self::Op::*;
    vec![
        Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri,
        Eqrr,
    ]
}

pub fn apply_op(op: &Op, r: &[usize; 4], a: usize, b: usize) -> usize {
    match op {
        Op::Addr => r[a] + r[b],
        Op::Addi => r[a] + b,
        Op::Mulr => r[a] * r[b],
        Op::Muli => r[a] * b,
        Op::Banr => r[a] & r[b],
        Op::Bani => r[a] & b,
        Op::Borr => r[a] | r[b],
        Op::Bori => r[a] | b,
        Op::Setr => r[a],
        Op::Seti => a,
        Op::Gtir => {
            if a > r[b] {
                1
            } else {
                0
            }
        }
        Op::Gtri => {
            if r[a] > b {
                1
            } else {
                0
            }
        }
        Op::Gtrr => {
            if r[a] > r[b] {
                1
            } else {
                0
            }
        }
        Op::Eqir => {
            if a == r[b] {
                1
            } else {
                0
            }
        }
        Op::Eqri => {
            if r[a] == b {
                1
            } else {
                0
            }
        }
        Op::Eqrr => {
            if r[a] == r[b] {
                1
            } else {
                0
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ops_test() {
        assert_eq!(5, apply_op(&Op::Addr, &[1, 3, 2, 5], 1, 2));
        assert_eq!(8, apply_op(&Op::Addi, &[1, 3, 2, 5], 1, 5));
    }
}
