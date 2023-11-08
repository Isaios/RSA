mod primagen;
mod rsa;
mod parse;
mod files;

fn main(){
    parse::parse().unwrap();
}
