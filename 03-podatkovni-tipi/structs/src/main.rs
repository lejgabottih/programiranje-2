// #[derive(Debug)]

// struct AritmeticnoZaporedje {
//     a_0 : i32,
//     d : i32,
//     cur: i32,
// }
// #[derive(Debug)]
// struct GeometrijskoZaporedje {
//     a_0 : i32,
//     q : i32,
//     cur : i32,
// }

// use AritmeticnoZaporedje as A;
// use GeometrijskoZaporedje as G;

// impl A {
//     fn new(first: i32, diff: i32) -> A {
//         return A {
//             a_0: first,
//             d: diff,
//             cur: first
//         }
//     }

//     fn next(&mut self) -> i32 {
//         let v = self.cur;
//         self.cur += self.d;
//         return v
//     }

//     fn n_th(&self, n: i32) -> i32 {
//         return self.a_0 + n * self.d
//     }

//     fn reset(&self) -> A {
//         return A {
//             a_0: self.a_0,
//             d: self.d,
//             cur: self.a_0,
//         }
//     }

//     fn current(&self) -> i32 {
//         return self.cur
//     }

//     fn sum(&self, n: i32) -> i32 {
//         let mut s = 0;
//         for i in 0..n {
//             s += self.a_0 + i * self.d;
//         }
//         return s
//     }

//     fn vsota(&self, seq: &A) -> A {
//         return A {
//             a_0: self.a_0 + seq.a_0,
//             d: self.d + seq.d,
//             cur: self.a_0 + seq.a_0
//         }
//     }

//     fn produkt(&self, seq: &A) -> String {
//         let mut str = String::from("");
//         for i in 0..10 {
//             str.push_str(&(
//                 self.a_0 * seq.a_0 + self.a_0 * i * seq.d + seq.a_0 * i * self.d + i * i * self.d * seq.d
//             ).to_string());
//             str.push_str(" ");
//         }
//         str.push_str("...");
//         return str;
//     }

// }


// impl G {
//     fn new(first: i32, quo: i32) -> G {
//         return G {
//             a_0: first,
//             q: quo,
//             cur: first,
//         }
//     }

//     fn next(&mut self) -> i32 {
//         let v = self.cur;
//         self.cur *= self.q;
//         return v
//     }

//     fn n_th(&self, n: u32) -> i32 {
//         return self.a_0 * self.q.pow(n)
//     }

//     fn reset(&self) -> G {
//         return G {
//             a_0: self.a_0,
//             q: self.q,
//             cur: self.a_0,
//         }
//     }

//     fn current(&self) -> i32 {
//         return self.cur
//     }

//     fn sum(&self, n: u32) -> i32 {
//         let mut s = 0;
//         for i in 0..n {
//             s += self.a_0 * self.q.pow(i);
//         }
//         return s
//     }

//     fn vsota(&self, seq: &G) -> String {
//         let mut str = String::from("");
//         for i in 0..10 {
//             str.push_str(&(
//                 self.a_0 * self.q.pow(i) + seq.a_0 * seq.q.pow(i)
//             ).to_string());
//             str.push_str(" ");
//         }
//         str.push_str("...");
//         return str;
//     }

//     fn produkt(&self, seq: &G) -> G {
//         return G {
//             a_0: self.a_0 * seq.a_0,
//             q: self.q * seq.q,
//             cur: self.a_0 * seq.a_0
//         }
//     }
// }


#[derive(Debug)]
enum BinOperacija {
    Plus,
    Minus,
    Times,
}
#[derive(Debug)]
enum Izraz {
    Konstanta(u32),
    Operacija(Box<Izraz>, BinOperacija, Box<Izraz>),
}


impl Izraz {

    fn eval(&self) -> i32 {
        // let mut v = 0;
        match self {
            Izraz::Operacija(a, b, c ) => 
                match b {
                    BinOperacija::Minus => a.eval() - c.eval(),
                    BinOperacija::Plus => a.eval() + c.eval(),
                    BinOperacija::Times => a.eval() * c.eval(),
                },
            Self::Konstanta(x) => (*x).try_into().unwrap(),
        }
    }

    fn collect(&self) -> u32 {
        let mut count = 0;
        count +=
            match self {
                Izraz::Operacija(a,_ ,c ) => a.collect() + c.collect(),
                Self::Konstanta(_) => 1
            };
        return count
    }

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


fn main() {
    let new: Izraz = 
        Izraz::Operacija(
            Box::new(Izraz::Konstanta(3)), 
                BinOperacija::Times, 
            Box::new(
                Izraz::Operacija(
                Box::new(Izraz::Konstanta(2)), 
                    BinOperacija::Minus, 
                Box::new(Izraz::Konstanta(10))
        )
    ))
    ;

    println!("{:#?}", new.eval());
}