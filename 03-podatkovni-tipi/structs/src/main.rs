#[derive(Debug)]

struct AritmeticnoZaporedje {
    a_0 : i32,
    d : i32,
    cur: i32,
}

use AritmeticnoZaporedje as A;

impl A {
    fn new(first: i32, diff: i32) -> A {
        return A {
            a_0: first,
            d: diff,
            cur: first
        }
    }

    fn next(&mut self) -> i32 {
        let v = self.cur;
        self.cur += self.d;
        return v
    }

    fn n_th(&self, n: i32) -> i32 {
        return self.a_0 + n * self.d
    }

    fn reset(&self) -> A {
        return A {
            a_0: self.a_0,
            d: self.d,
            cur: self.a_0,
        }
    }

    fn current(&self) -> i32 {
        return self.cur
    }

    fn sum(&self, n: i32) -> i32 {
        let mut s = 0;
        for i in 0..n {
            s += self.a_0 + i * self.d;
        }
        return s
    }

    fn vsota(&self, seq: A) -> A {
        return A {
            a_0: self.a_0 + seq.a_0,
            d: self.d + seq.d,
            cur: self.cur + seq.cur
        }
    }


}

fn main() {
    let new = A::new(3, 5);
    let new1 = A::new(4, 6);

    println!("{:#?}", new.vsota(new1));
}

