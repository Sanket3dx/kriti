mod token;
mod parser;
// use crate::token::Token;
// use crate::parser::parse;

fn main() {
    let code = "
   
    karya add(int32 a , int32 b) -> int32 {
        int32 a;
        a = 5 + 3;
        if x > 10 {
            print(\"x is greater than 10\");
        } else {
            print(\"x is less than or equal to 10\");
        }
        
        loop a > b {
            
        }

        loop arr -> a {

        }

        loop a = 0 ; a <= 10; a ++ {

        }
    }

    ";

    let tokens = token::tokenize(code);
    println!("Tokens: {:?}", tokens);

    let _ = parser::parse(&tokens); // Basic parsing call
}
