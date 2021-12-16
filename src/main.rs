// ojo adicional, el nombre macro1 o macro1_derive del crate(lib) tiene que coincidir con el que definimos en el Cargo Ppal
use macro1::Trait1;  //estas 2 justamente las puedo importar asi por lo q tengo definidos los path en el Cargo
use macro1_derive::HelloMacro1;

// **VER MUY BIEN QUE IGUAL ESTOY DEFINIENDO EL TRAIT NORMALMENTE
// **LO UNICO QUE SE ESTA REEMPLAZANDO ES LA IMPLEMENTACION
// hay que agregar los path en el Cargo ppal tbm (si fueran dependencias comunes que publicaramos en crates.io las importamos asi comunmente, aca porq estan locales)
// mis path cambiaron, porq yo meti las librerias adentro de la carpeta del proyecto, para que me quedara todo junto
// hello_macro = { path = "../hello_macro" }
// hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }

/*
Our two crates are tightly related, so we create the procedural macro crate 
within the directory of our hello_macro crate. If we change the trait definition 
in hello_macro, we’ll have to change the implementation of the procedural macro 
in hello_macro_derive as well. The two crates will need to be published separately,
and programmers using these crates will need to add both as dependencies and bring 
them both into scope. We could instead have the hello_macro crate use 
hello_macro_derive as a dependency and re-export the procedural macro code. 
However, the way we’ve structured the project makes it possible for programmers 
to use hello_macro even if they don’t want the derive functionality.
*/ 

#[derive(HelloMacro1)]
struct Pancakes{
    
}


fn main() {
    Pancakes::hello_macro1();
}

// *los nombres de las funciones hello_macro1() tienen q coincidir
// *el punto donde se linkea a HelloMacro1 con Trait1 es en 'impl Trait1 for #name' del crate macro1_derive
// *de dnd tengo disponible Trait1 adentro de la funcion impl_hello_macro() ?????? 
    // si comento la linea de importacion de Trait1 aca, me da un error de que no encuentra Trait1 en el scope de HelloMacro1, asi q por ahi debe venir la mano
