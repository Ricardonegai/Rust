 fn main(){
    let array: [i32; 6] = [10, 20, 30, 40, 50, 60];
    //Declaramos que esse let tem o nome array, é um inteiro com sinal(i32) e possui 6 items.
    //NEM SEMPRE é obrigatório especificar qual tipo de inteiro.
    println!("O segundo array é do valor: {}", array[1]);
    println!("O terceiro array é do valor: {}", array[2]);
    //O println! não é uma função e sim um macro;
    //Ponto e vírgula são obrigatórios;
    //Chamada de valor com chaves {}, seguida do nome da var/função
    //A chamada de arrays começa pelo valor 0.
}

// Inteiro com sinal: i8, i16, i32, i64, i128 e isize
// Inteiro sem sinal: u8, u16, u32, u64, u128 e usize 
// Podem ser usadas as notações: 0x, 0o, ou 0x;

// Ponto flutuante: f32, f64
// Char(unicode): a
// bool: true,false
// Arrays: [1,2,3]
// Tuples: [1, true]

// Uma variável só é mutável se conter { mut }
//Só é possível atribuir um valor de uma var em outra usando o .clone()