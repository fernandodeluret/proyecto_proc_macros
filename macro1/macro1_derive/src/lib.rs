extern crate proc_macro;

use proc_macro::TokenStream;
// quote sirve para convertir las estructuras de syn en codigo rust
use quote::quote;  //este
use syn;  //y este hay q agregarlos en la parte de [dependencies] del Cargo, porq no vienen por defecto con rust

#[proc_macro_derive(HelloMacro1)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // println!("{}",input);
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();  //la libreria syn sirve para convertir el input q recibimos(que lo recibimos como un string) en estructuras sobre las que podemos operar
    // por ej en ete caso parse nos va a devolver un Struct de Clase DeriveInput con los datos del input que recibio (q en este caso a su vez tbm va a ser el Struct Pancakes)
    // el unwrap() cumple la funcion de un expect() o panic!(), o sea devuelve error si el parse no se puede hacer bn

    // Build the trait implementation
    impl_hello_macro(&ast)
}


fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // println!("{}",name);
    
    let gen = quote! {  //aca vemos el uso de la libreria quote, supuestamente lo q le pasemos a quote va a ser dps generado como codigo rust
        impl Trait1 for #name {  //el # es una funcion que nos da la libreria quote, y en este caso sirve para q se asigne a #name el valor q tenia la variable name(q no entiendo bien q concha quiere decir)
                                     //PERO SIRVE TBM POR EJ PARA HACER REPETICIONES (ver doc de quote)
            fn hello_macro1() {  //****ESTE VA A SER EL NOMBRE DE LA FUNCION QUE VAMOS A ESTAR APPENDEANDO AL STRUCT Q NOS PASEN
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()  //into() convierte el codigo de rust que nos da quote en un TokenStream
}