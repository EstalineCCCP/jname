extern crate diacritics;
mod entrada;
use diacritics::*;
use std::*;
use entrada::*;
use regex::Regex;
fn repetidos(input: &str) -> String {
    let re = Regex::new(r"[\.-]{1,10}").unwrap();
    re.replace_all(input, "-").to_string()
}
fn estranhos(input: &str) -> String {
    let re = Regex::new(r"[^\w \.-]").unwrap();
    re.replace_all(input, "").to_string()
}
fn espacos(input: &str) -> String {
    let re = Regex::new(r"[ _]").unwrap();
    re.replace_all(input, "-").to_string()
}
fn pontos(input: &str) -> String { 
    let re = Regex::new(r"\.").unwrap();
    re.replace_all(input, "-").to_string()
}
fn separacao(input: &str) -> String { // conserta repeticoes de .- ou -.
    let re = Regex::new(r"[\.-][\.-]").unwrap();
    re.replace_all(input, "-").to_string()
}
fn separador_maiusculas(input: &str) -> String {
    let mut result = String::new();
    let input_vec: Vec<char> = input.chars().collect();
    result.push(input_vec[0]);
    for character in &input_vec[1..] {
        if character.is_uppercase() {
            result.push('-');
            result.push(*character);
        }else{ result.push(*character)} 
    }
    result
}
fn finaliza(input: &str) -> String { // conserta finais de .- ou -.
    input.trim_end_matches(|c: char| c.is_alphanumeric() == false).to_string()
}
fn minusculas(input: &str) -> String {
    input.to_lowercase().to_string()
}

fn main() {
    let mut linhas: Vec<String> = entrada();
    let (mut d, mut s, mut r, mut e, mut p, mut l, mut x, mut nada) = 
        (false, false, false, false, false, false, false, false); 
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {nada = true};
    if nada == false {
       if args[1].contains("d") {d = true};
       if args[1].contains("s") {s = true};
       if args[1].contains("r") {r = true};
       if args[1].contains("p") {p = true};
       if args[1].contains("e") {e = true};
       if args[1].contains("l") {l = true};
       if args[1].contains("x") {x = true};
    };
    linhas = linhas.into_iter()
                    .map(|linha| if e == true {estranhos(&linha)}else{linha})
                    .map(|linha| if s == true {espacos(&linha)}else{linha})
                    .map(|linha| if p == true {pontos(&linha)}else{linha})
                    .map(|linha| if x == true {separador_maiusculas(&linha)}else{linha})
                    .map(|linha| if r == true {repetidos(&linha)}else{linha})
                    .map(|linha| if d == true {remove_diacritics(&linha)}else{linha})
                    .map(|linha| if l == true {minusculas(&linha)}else{linha})
                    .map(|linha| separacao(&linha))
                    .map(|linha| finaliza(&linha))
                    .collect();
    for linha in linhas {
        println!("{}", linha);
    }
}
