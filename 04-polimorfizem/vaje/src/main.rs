// #[derive(Debug)]

// struct AritmeticnoZaporedje<T> {
//     a_0 : T,
//     d : T,
//     cur: T,
// }


use std::ops::Mul;
use std::ops::Sub;
use std::ops::Add;

// use AritmeticnoZaporedje as A;

// impl <T: Copy + std::ops::Add<Output = T>> A<T> {

//     fn next(&mut self) -> T {
//         let v = self.cur;
//         self.cur = self.d + self.cur;
//         return v
//     }

//     fn n_th(&self, n: u32) -> T {
//         let mut v = self.a_0;
//         for _ in 0..n {
//             v = self.d + v;
//         }
//         return v
//     }

//     fn sum(&self, n: u32) -> T {
//         let mut s = self.a_0;
//         for i in 1..n {
//             s = s + self.n_th(i);
//         }
//         return s
//     }

//     fn vsota(&self, seq: &A<T>) -> A<T> {
//         return A {
//             a_0: self.a_0 + seq.a_0,
//             d: self.d + seq.d,
//             cur: self.a_0 + seq.a_0
//         }
//     }
// }

// impl <T: Copy> A<T> {
//     fn reset(&self) -> A<T> {
//         return A {
//             a_0: self.a_0,
//             d: self.d,
//             cur: self.a_0,
//         }
//     }

//     fn new(first: T, diff: T) -> A<T> {
//         return A {
//             a_0: first,
//             d: diff,
//             cur: first
//         }
//     }

//     fn current(&self) -> T {
//         return self.cur
//     }
// }


// impl <T: Copy + std::ops::Add<Output = T> + Mul<Output = T>> A<T> {
//     fn produkt(&self, seq: &A<T>) -> A<T> {
//         return A::new(self.a_0 * seq.a_0, self.d * seq.d)
//     }
// }

// impl<T> PartialEq for A<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.a_0 == other.a_0 && self.d == other.d
//     }
// }



#[derive(Debug)]
enum BinOperacija {
    Plus,
    Minus,
    Times,
}
#[derive(Debug)]


enum Izraz<T> {
    Konstanta(T),
    Operacija(Box<Izraz<T>>, BinOperacija, Box<Izraz<T>>),
}


impl<T> Izraz<T>
where 
    T: Copy,
    T: std::ops::Add<Output = T>,
    T: Mul<Output = T>,
    T: std::ops::Sub<Output = T>,
{
    fn eval(&self) -> T {
        match self {
            Izraz::Operacija(a, b, c ) => 
                match b {
                    BinOperacija::Minus => a.eval() - c.eval(),
                    BinOperacija::Plus => a.eval() + c.eval(),
                    BinOperacija::Times => a.eval() * c.eval(),
                },
            Self::Konstanta(x) => *x,
        }
    }
}
impl <T: std::ops::Add<Output = T>> Izraz<T> {
    fn collect(&self) -> u32 {
        let mut count = 0;
        count +=
            match self {
                Izraz::Operacija(a,_ ,c ) => a.collect() + c.collect(),
                Self::Konstanta(_) => 1
            };
        return count
    }
}


impl <T : ToString> Izraz<T> {
    fn izpis(&self) -> String {
        let mut str = String::new();
        match self {
            Izraz::Operacija(a, b, c ) => 
                match b {
                    BinOperacija::Minus => str.push_str(&("(".to_owned() + &a.izpis().to_string() + " - " + &c.izpis().to_string() + ")")),
                    BinOperacija::Plus => str.push_str(&("(".to_owned() + &a.izpis().to_string() + " + " + &c.izpis().to_string() + ")")),
                    BinOperacija::Times =>str.push_str(&("(".to_owned() + &a.izpis().to_string() + " * " + &c.izpis().to_string() + ")")),
                },
            Self::Konstanta(x) => str.push_str(&x.to_string()),
        }
        return str
    }
}

impl<T : ToString> ToString for Izraz<T> {
    fn to_string(&self) -> String {
        return self.izpis()
    }
}



struct AritmeticnoZaporedjeAST<T> {
    a_0 : Izraz<T>,
    d : Izraz<T>,
    cur: Izraz<T>,
}

use AritmeticnoZaporedjeAST as A;

impl <T: Copy + std::ops::Add<Output = T>> A<T> where Izraz<T>: Add<Output = Izraz<T>>{

    fn next(&mut self) -> Izraz<T> {
        let v = self.cur;
        self.cur = self.d + self.cur;
        return v
    }

    fn n_th(&self, n: u32) -> Izraz<T> {
        let mut v = self.a_0;
        for _ in 0..n {
            v = Izraz::Operacija(Box<Izraz<self.d>>, BinOperacija::Plus, Box<Izraz<v>>);
        }
        return v
    }

    fn sum(&self, n: u32) -> Izraz<T> {
        let mut s = self.a_0;
        for i in 1..n {
            s = s + self.n_th(i);
        }
        return s
    }

    fn vsota(&self, seq: &A<T>) -> A<T> {
        return A {
            a_0: self.a_0 + seq.a_0,
            d: self.d + seq.d,
            cur: self.a_0 + seq.a_0
        }
    }
}

impl <T: Copy> A<T> {
    fn reset(&self) -> A<T> {
        return A {
            a_0: self.a_0,
            d: self.d,
            cur: self.a_0,
        }
    }

    fn new(first: Izraz<T>, diff: Izraz<T>) -> A<T> {
        return A {
            a_0: first,
            d: diff,
            cur: first
        }
    }

    fn current(&self) -> Izraz<T> {
        return self.cur
    }
}


impl <T: Copy + std::ops::Add<Output = T> + Mul<Output = T>> A<T> {
    fn produkt(&self, seq: &A<T>) -> A<T> {
        return A::new(self.a_0 * seq.a_0, self.d * seq.d)
    }
}

impl<T> PartialEq for A<T> {
    fn eq(&self, other: &Self) -> bool {
        self.a_0 == other.a_0 && self.d == other.d
    }
}





fn main() {
    
}



