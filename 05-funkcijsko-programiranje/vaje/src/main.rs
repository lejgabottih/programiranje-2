// Napišite različne funkcije, ki kot argument sprejmejo zaprtje (closure) in ga pokličejo z različnimi argumenti, ki ustrezajo sledečim ocaml tipom:
// Tip zaprtja naj bo čim bolj splošen (Fn, FnOnce, FnMut).
//  apply_int: (int -> int) -> int -> int
//  apply: ('a -> 'b) -> 'a -> 'b
//  apply2: ('a -> 'a -> 'b) -> 'a -> 'a -> 'b
//  map: ('a -> 'b) -> 'a list -> 'b list  (Uporabite Vec<T> namesto list, predpostavite, da funkcija ne spremeni elementov seznama)
//  ponavljaj: int -> ('a -> 'a) -> 'a -> 'a // Ponovi funkcijo n-krat
//  filter: ('a -> bool) -> 'a list -> 'a list // Vrne seznam elementov, ki zadoščajo pogoju - uporabite Vec<T> namesto list in že vgrajeno funkcijo filter

use std::net::UdpSocket;


fn apply_int<T>(fun: T, n: i64) -> i64
    where T: Fn(i64) -> i64
    {
        return fun(n)
    }

fn apply_int_dyn(fun: &dyn Fn(i64) -> i64, n: i64) -> i64
    {
        return fun(n)
    }

//ce impl pri fun: impl v funkciji notr lahka tut &dyn...
// potem je treba pri &dyn borrowat spremenljivko

fn apply<T, U, V>(fun: T, x: U) -> V
    where T: Fn(U) -> V
    {
            return fun(x)
    }

fn apply2<T, U, V>(fun: T, x: U, y: U) -> V
    where T: Fn(U, U) -> V
    {
        return fun(x, y)
    }

fn map<F, T, U>(fun: F, vec: Vec<T>) -> Vec<U> 
    where F: Fn(T) -> U
    {
        let mut nov = Vec::<U>::new();
        for x in vec {
            nov.push(fun(x))
        } 
        return nov
    }

fn ponavljaj<F, U>(n: i64, fun: F, a: U) -> U
    where F: Fn(U) -> U
    {
        let mut odg = a;
        for _ in 0..n {
            odg = fun(odg);
        }
        return odg
    }

fn filter<F, U>(fun: F, vec: Vec<U>) -> Vec<U>
    where F: FnMut(&U) -> bool
    {
        return vec.into_iter().filter(fun).collect();
    }




fn  main() {
    let f = |x: &i64| { *x <= 3};
    // let g = |x: i64| {x * 10};
    // let fncs = vec![f, g];
    println!("{:?}", filter(f, vec![5,4,6,3,7,2,9,1]));
}


// Vzemite zaporedja iz prejšnjih vaj in naredite nov objekt, ki sprejme zaporedje in ga naredi iterabilnega

// Iteratorji

// Napišite funkcijo, ki sprejme vektor XYZ in s pomočjo iteratorja naredi W
// števil in izpiše vsako v svojo vrstico
// nizov in izpiše njihove dolžine
// nizov in vrne vsoto njihovih dolžin
// vektor parov (i32, i32) in vrne vsoto njihovih pozitivnih produktov
// dva vektorja <i32> in vrne vektor, ki vsebuje vsote parov
// dva vektorja <i32> in vrne vsoto poparjenih pozitivni produktov s pomočjo ene izmed prejšnjih nalog
// vektor Option<T> in izpiše vse T-je
// vektor Option<T> in vrne število Some-ov
// odfiltrira števila deljena s 3

// Popravite zaporedja iz prejšnjih vaj, da bodo `iterabilna`

// Dopolnite spodnjo funkcijo, da vrne niz, kjer so vse prve črke posameznih besed velike
// ["Just,", " ", "hello", " ", "world", "!"] -> "Just, Hello World", "!"
// pub fn capitalize_words_string(words: &[&str]) -> String {
//     panic!("Not implemented");
// }
// Napišite funkcijo `fakulteta`, ki izračuna fakulteto števila n. Uporabite iteratorje (torej brez lastne for zanke, rekurzije)
// Namig: fold, reduce, `..`...

// -------------------------------------------------------------------------------------------------
// Dodatno:
// Koda vzeta iz googlvih rust vaj:
// Vse se da lepo narediti samo z iteratorji (brez indeksov, brez for zank, brez mutabilnosti)
/*
/// Calculate the differences between elements of `values` offset by `offset`,
/// wrapping around from the end of `values` to the beginning.
///
/// Element `n` of the result is `values[(n+offset)%len] - values[n]`.
fn offset_differences<N>(offset: usize, values: Vec<N>) -> Vec<N>
where
    N: Copy + std::ops::Sub<Output = N>,
{
    unimplemented!()
}

#[test]
fn test_offset_one() {
    assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
    assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
    assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
}

#[test]
fn test_larger_offsets() {
    assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
    assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
    assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
    assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
}

#[test]
fn test_custom_type() {
    assert_eq!(
        offset_differences(1, vec![1.0, 11.0, 5.0, 0.0]),
        vec![10.0, -6.0, -5.0, 1.0]
    );
}

#[test]
fn test_degenerate_cases() {
    assert_eq!(offset_differences(1, vec![0]), vec![0]);
    assert_eq!(offset_differences(1, vec![1]), vec![0]);
    let empty: Vec<i32> = vec![];
    assert_eq!(offset_differences(1, empty), vec![]);
}



*/



