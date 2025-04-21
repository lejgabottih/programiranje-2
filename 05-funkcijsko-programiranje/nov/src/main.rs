
enum Ali<T, U> {
    Levi(T),
    Desni(U)
}

impl <T,U> Ali<T,U> {
    
    fn uporabi<F,G,V,Z>(self, f:F,g:G)
    where
    F: Fn(T,U)-> Ali<V,Z>,
    G: Fn(T,U)-> Ali<V,Z>,
    {
        
    }

}

// fn print_length(s: String) {
//     println!("{}", s.len())
// }


fn main() {
}
    