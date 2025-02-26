use core::panic;

/// Skupaj preverite in pokomentirajte kvize iz [učbenika](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html)

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `fib`, ki sprejme začetna člena fibbonacijevega zaporedja, število `n` in vrne `n`-ti člen zaporedja

// fn fib(a0: u32, a1: u32, n: u32) -> u32 {
//     if n == 0 {
//         return a0;
//     } else if n == 1 {
//         return a1;
//     } return fib(a1, a0 + a1, n - 1);
// }

fn fib(a0: u32, a1: u32, n: u32) -> u32 {
    let mut a = a0;
    let mut b = a1;
    for _ in 0..n {
        (a, b) = (b, a + b)
    }
    return a;
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_prestopno`, ki za podano leto preveri, ali je prestopno

fn je_prestopno(leto: u32) -> bool {
    (leto % 4 == 0 && leto % 100 != 0) || (leto % 400 == 0)
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_veljaven_datum(datum: Date) -> bool`, ki preveri, ali je datum veljaven

// Dan, mesec, leto
type Date = (u32, u32, u32);

fn je_veljaven_datum(datum: Date) -> bool {
        match datum.1 {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => {
                if datum.0 <= 31 {
                    return true;
                } else {return false}
            }
            4 | 6 | 9 | 11 => {
                if datum.0 <= 30 {
                    return true;
                } else {return false}
            }
            2 => { if je_prestopno(datum.2) == true {
                if datum.0 <= 29 {
                    return true;
                } else {return false}
            } else if datum.0 <= 28 {
                return true;
            } else {return false}
            }
            _ => {return false}
            }
        } 




/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32`, ki sprejme iteracijsko funkcijo, zaustavitveni pogoj in začetno vrednost.
/// Iteracijsko funkcijo zaporedoma uporablja, dokler za rezultat ne velja zaustavitveni pogoj, in vrne prvi rezultat, ki zadošča zaustavitvenemu pogoju.

fn iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32 {
    if cond(start) {
        return start;
    } else {return iteracija(fun(start), fun, cond)};
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo, ki izračuna ničlo zvezne funkcije s pomočjo bisekcije.
/// Postopek bisekcije je sledeč:
/// 1. Izberemo interval [a, b], kjer je f(a) * f(b) < 0
/// 2. Izračunamo sredino intervala c = (a + b) / 2
/// 3. Če je |f(c)| < prec ali je dolžina intervala manjša od določene natančnosti, vrnemo c
/// 4. Če ni, izberemo nov interval [a, b] glede na predznak f(c)
/// 5. Ponavljamo korake 2-4

fn bisekcija(mut a: f64, mut b: f64, fun: fn(f64) -> f64, prec: f64) -> f64 {
    if fun(a) == 0.0 {
        return a
    } else
    if fun(b) == 0.0 {
        return b
    } else
    if fun(a) * fun(b) < 0.0 {
        let c = (a+b)/2.0;
        if fun(c).abs() < prec {
            return c;
        } else {if fun(a) * fun(c) <= 0.0 {
            return bisekcija(a, c, fun, prec);
        } else {return bisekcija(b, c, fun, prec);}
    }
    } else {panic!("nema ništa")}
}

/// ------------------------------------------------------------------------------------------------

/// Popravite igro ugibanja iz prejšnje naloge, da bo delovala sledeče
/// Uporabnika sprašujemo po novi številki, vse dokler so števila, ki jih vpisuje del nekega aritmetičnega zaporedja
/// Če uporabnik vpiše neveljavno število to ni napaka, program za pogoj aritmetičnega zaporedja upošteva samo veljavno vpisana števila.

use std::io;

fn guessing_game() {

    // loop {
    //     println!("napiši naslednje število")

    //     let mut guess = String.new()

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

        

    // }


    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2]`, ki matriki `a` in `b` zmnoži in vrne rezultat

fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `ordered`, ki sprejme tabelo števil in vrne `true`, če so števila urejena (padajoče ali naraščajoče) in `false` sicer.

fn ordered(arr: &[u32]) -> bool {
    panic!("Not implemented");
}

fn vsebuje<T : PartialEq>(v: &Vec<T>, x : &T) -> bool {
    for y in v {
      if x == y {
        return true
      }
    }
    return false
}

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje
/// Napišite funkcijo `fn pow(mut x: u32, mut n: u32) -> u32`, ki izračuna `x` na potenco `n` v času O(log n)
/// Hitro potenciranje izgleda tako:
/// 1. Če je `n` sodo, potem je `x^n = (x^(n/2))^2`
/// 2. Če je `n` liho, potem je `x^n = (x^2)^(n/2)`
/// 3. Če je `n = 0`, potem je `x^n = 1`

/// ------------------------------------------------------------------------------------------------
/// Prepišite hitro potenciranje v iterativno obliko

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje deluje tudi, če nas zanima samo ostanek po deljenju z nekim številom `m`
/// Napišite funkcijo `fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32`, ki izračuna `x` na potenco `n` in vrne ostanek po deljenju z `m`
/// Postopek je enak, le da pri vsakem izračunu vrnemo ostanek pri deljenju z `m`

/// ------------------------------------------------------------------------------------------------
/// Urejanje z izbiranjem
/// Napišite funkcijo `fn selection_sort(arr: &mut [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

fn selection_sort(arr: &mut [u32]) {}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido višine `n` iz zvezdic

fn pyramid(n: u32) {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido črk angleške abecede višine `n`, lahkom predpostavite, da bo n največ 26.
///      A
///    A B A
///   A B C B A
/// A B C D C B A
/// Napišite funkcijo `fn selection_sort(mut arr: [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

fn main() {
    fn f(x: f64) -> f64 { return x.sin()};
    println!("{}", bisekcija(0.0, 1.0, f, 0.0000001 ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = main();
        assert_eq!(result, ());
    }

    #[test]
    fn test_fib() {
    }
}
