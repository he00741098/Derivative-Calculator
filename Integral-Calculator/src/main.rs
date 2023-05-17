fn main() {
    println!("Hello, world!");
    //2x(sin(x^2)^2)
    //x = coefficient_variable, coefficient_constant =2, exponent = 1, internal_term = sin(x^2)^2
    //sin(x^2)^2 = coefficient_variable = None, coefficient_constant = 1, exponent = 2, internal_term = sin(x^2)
    //sin(x^2) = coefficient_variable = None, coefficient_constant = 1, exponent = 1, internal_term = x^2
    //x^2 = coefficient_variable = Some(x), coefficient_constant = 1, exponent = 2, internal_term = None
    //applicable rules = chain rule, power rule, multiplication rule, sin rule

    //take input
    let input = "2x(sin(x^2)^2)";
    //parse input
    let parsed_input = parse_input(input.to_string());
}

fn parse_input(input:String) -> term{
    //parse input into terms
    let mut terms: Vec<term> = Vec::new();
    
    //find keywords/symbols
    let keywords = ["sin", "cos", "tan", "cot", "sec", "csc", "ln", "log", "e", "pi", "sqrt", "(", ")", "^", "*", "/", "+", "-"];
    let mut keyword_indices: Vec<usize> = Vec::new();
    //find addition and subtraction
    let parenthesis = find_all_parenthesis(input.clone().to_string());

    let no_overlap_parenthesis = no_overlap(parenthesis.clone());

    if parenthesis.len() != no_overlap_parenthesis.len(){
        //panic!("Parenthesis Overlap");
        
    }
    //send things inside parenthesis to new parse_input            
    


    //default
    return term::new()
}



fn find_parenthesis(s:String, starting_index:usize) -> (usize, usize){

    let charVec:Vec<char> = s.chars().collect();
    let mut count = 1;
    for i in (starting_index+1)..s.len(){  
        if charVec[i] == '('{
            count += 1;
        }
        if charVec[i]==')'{
            count-=1;
        }
        if count == 0{
            return (starting_index, i);
        }
    }

    return (starting_index,0)

}

fn no_overlap(s:Vec<(usize,usize)>) -> Vec<(usize, usize)>{
    let mut returner = vec![];
    //find max
    let mut max = 0;
    s.iter().for_each(|x| if x.1 > max {max = x.1});
    let mut check = vec![false; max+1];
    
    for i in 0..s.len(){
        if !is_overlapping(s[i], &check){
            for i in s[i].0..s[i].1{
                check[i] = true;
            }
            returner.push(s[i]);
        }
    }

    returner
}

fn is_overlapping(s:(usize, usize), vector:&Vec<bool>)->bool{

if vector[s.0] || vector[s.1]{
    return true;
}
false
}


#[test]
fn testParenthesis(){

    let testString1 = "()(())()";
    let thing = find_parenthesis(testString1.clone().to_string(), 0);
    let thing2 = find_parenthesis(testString1.clone().to_string(), 2);
    let thing3 = find_parenthesis(testString1.clone().to_string(), 3);
    let thing4 = find_parenthesis(testString1.clone().to_string(), 6);
    println!("{}, {}", thing.0, thing.1);
    println!("{}, {}", thing2.0, thing2.1);
    println!("{}, {}", thing3.0, thing3.1);
    println!("{}, {}", thing4.0, thing4.1);

    assert!(thing==(0,1));
    assert!(thing2==(2,5));
    assert!(thing3==(3,4));
    assert!(thing4==(6,7));
    let vecy = vec![thing, thing2, thing3, thing4];
    let vecy2 = no_overlap(vecy);
    println!("{:?}", vecy2);
    assert!(vecy2 == vec![(0,1), (2,5), (6,7)]);

}

#[test]
fn parenthesisTest2(){
let testString1 = "()(())()";
let vec = find_all_parenthesis(testString1.clone().to_string());
let vec2 = find_all_parenthesis("".to_string());
println!("{:?}", vec);
println!("{:?}", vec2);
assert!(vec == vec![(0,1), (2,5), (3,4), (6,7)]);
assert!(vec2 == vec![]);
}

fn find_all_parenthesis(s:String) ->Vec<(usize, usize)>{
    let mut parenthesis:Vec<(usize, usize)> = Vec::new();
    //find all parenthesis () (())+()
    let mut front_parenthesis:Vec<usize> = Vec::new(); 
    let charVec:Vec<char> = s.chars().collect();
    for i in 0..s.len(){
        if charVec[i] == '('{
            front_parenthesis.push(i);
        }

    }
    //match parenthesis
    
   for i in front_parenthesis{
        parenthesis.push(find_parenthesis(s.clone().to_string(), i));
    }

    parenthesis
}


struct term{
    coefficient_constant: i64,
    coefficient_variable: Option<Box<term>>,
    exponent: i64,
    internal_term: Option<Box<term>>,
    is_variable: bool,
}

impl term{
    fn new() -> term{
        term{
            coefficient_constant: 0,
            coefficient_variable: None,
            exponent: 0,
            internal_term: None,
            is_variable: false,
        }
    }
}

fn seperation(s:String, seperation:Vec<(usize, usize)>) -> Vec<String>{
let mut cutin = s.clone();
let mut cutouts = vec![];
for i in seperation{
    let mut thing = "".to_string();
    for p in i.0..i.1{
        thing.push(cutin.chars().nth(p).unwrap());
        cutin.replace_range(i.0..i.1, "");
    }
    cutouts.push(thing);
}



cutouts
}
