use std::{string, iter::repeat, result};

fn main() {
    //println!("Hello, world!");
    //2x(sin(x^2)^2)
    //x = coefficient_variable, coefficient_constant =2, exponent = 1, internal_term = sin(x^2)^2
    //sin(x^2)^2 = coefficient_variable = None, coefficient_constant = 1, exponent = 2, internal_term = sin(x^2)
    //sin(x^2) = coefficient_variable = None, coefficient_constant = 1, exponent = 1, internal_term = x^2
    //x^2 = coefficient_variable = Some(x), coefficient_constant = 1, exponent = 2, internal_term = None
    //applicable rules = chain rule, power rule, multiplication rule, sin rule

    //take input
    //let input = "2x(sin(x))";
    //result = 2xcos(x) + 2sin(x)
    //parse input
    //let parsed_input = parse_input(input.to_string());
    let input = 
    
    term{
        coefficient: term_internal::constant(term_number::int(2)),
        exponent: term_internal::constant(term_number::int(3)),
        internal:tInVar(),
        modifier: modifier::NoMod,
        denom: tInVar(),
    };
    
    // term{
    //     coefficient: term_internal::constant(term_number::int(2)),
    //     exponent: term_internal::constant(term_number::int(1)),
    //     // internal: term_internal::var,
    //     internal: tInTerm(
    //         term { 
    //             coefficient: tInCon(2),
    //             exponent: tInCon(1), 
    //             internal: tInTerm(term{
    //                 coefficient: term_internal::constant(term_number::int(2)),
    //                 exponent: term_internal::constant(term_number::int(1)),
    //                 internal: tInTerm(
    //                     term { 
    //                         coefficient: tInCon(2),
    //                         exponent: tInCon(1), 
    //                         internal: tInVar(), 
    //                         modifier: noMod(), 
    //                         denom: tInCon(1),
    //                     }),
    //                 modifier: modifier::sin,
    //                 denom: term_internal::constant(term_number::int(1)),
    //                 }), 
    //             modifier: modifier::sin, 
    //             denom: tInCon(1),
    //         }),
    //     modifier: modifier::sin,
    //     denom: term_internal::constant(term_number::int(1)),
    //     };

    let result = simplify(&inTermOut(derive_start(&input)));
    println!("Original: ");
    printTerm(&input);
    println!("\nresult: ");
    printTerm(&result);
    println!();


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


#[derive(Debug,PartialEq, Clone)]
struct term{
    coefficient: term_internal,
    exponent: term_internal,
    internal: term_internal,
    modifier: modifier,
    denom:term_internal,
}

#[derive(Debug,PartialEq, Clone)]
enum term_internal{
    var,
    neg_var,
    constant(term_number),
    term(Box<term>),
    terms(Vec<term>, Vec<operators>),

}
#[derive(Debug,PartialEq, Clone)]
enum term_number{
    e,
    pi,
    int(i32),
}

#[derive(Debug,PartialEq, Clone)]
enum operators{
    add,
    subtract,
    multiply,
    divide,
    power,
    none,
}

#[derive(Debug,PartialEq, Clone)]
enum modifier{
    sin,
    cos,
    csc,
    sec,
    tan,
    cotan,
    arctan,
    arccos,
    arcsec,
    arcsin,
    arccotan,
    arccsc,
    ln,
    log(i32),
    NoMod
}

impl term{
    fn new() -> term{
        term{
            coefficient: term_internal::constant(term_number::int(1)),
            exponent: term_internal::constant(term_number::int(1)),
            internal: term_internal::var,
            modifier: modifier::NoMod,
            denom: term_internal::constant(term_number::int(1)),
        }
    }
}

fn seperation(s:String, seperation:Vec<(usize, usize)>) -> Vec<String>{
let mut cutin = s.clone();
let mut cutouts = vec![];
for i in seperation{
    let mut thing = "".to_string();
    for p in i.0+1..i.1{
        thing.push(cutin.chars().nth(p).unwrap());
    }
    let mut ping = "".to_string();
    for k in i.0..=i.1{
        ping.push('!');
    }
    cutin.replace_range(i.0..=i.1, &ping);
    cutouts.push(thing);
}

let mut builder = "".to_string();


for p in 0..cutin.len(){
if let Some(x) = cutin.chars().nth(p){
if x!='!'{
    builder.push(x);
}else{
    cutouts.push(builder);
    builder = "".to_string();
}
}

}




cutouts.iter().filter(|x| x.len()>0).map(|x| x.to_string()).collect()
}

#[test]
fn sep_test(){
    let input = "2x(sin(x^2)^2)".to_string();
    let a = find_all_parenthesis(input.clone());
    println!("{:?}", a);
    let b = no_overlap(a);
    println!("{:?}", b);

    println!("Seperating...");
    let g = seperation(input.to_string(), b);
    println!("Seperated...");
    println!("{:?}", g);


}

#[test]
fn sep_test2(){
    let input = "2x(sin(x^2)^2)+cos(5x)+x(x^2)/(16/x+1)".to_string();
    let a = find_all_parenthesis(input.clone());
    println!("{:?}", a);
    let b = no_overlap(a);
    println!("{:?}", b);

    println!("Seperating...");
    let mut g = seperation(input.clone().to_string(), b);
    println!("Seperated...");
    println!("{:?}", g);
    re_sort(input.clone().to_string(), &mut g);
    println!("{:?}", g);
}

fn re_sort(s:String, stringVec:&mut Vec<String>){
    let mut s = s.clone();
    let mut beginDex = 0;
    let mut finalVal = vec![];
    while finalVal.len() < stringVec.len(){
        for i in stringVec.clone(){
            if let Some(x) = s.find(&*i){

                if x==beginDex{
                    finalVal.push(i.clone());
                    let replacer = repeat('!').take(i.len()).collect::<String>();
                    s.replace_range(x..x+i.len(), &replacer);
                    //stringVec[beginDex] = "!".to_string();
                    break;
                }
            }
        }
        beginDex+=1;
    }

    for i in 0..finalVal.len(){
        stringVec[i]=finalVal[i].clone();
    }

    
}

#[test]
fn test_sorting(){
let mut s = "asdf".to_string();
let mut stringVec = vec!["s".to_string(),"d".to_string(),"f".to_string(),"a".to_string()];
re_sort(s, &mut stringVec);
//loop through until 0, if no 0 index+=1;
println!("{:?}", stringVec);
}

#[test]
fn derive_test(){
    let input = "sin(x)".to_string();

    let parsedTerm = term{
        coefficient: term_internal::constant(term_number::int(1)),
        exponent: term_internal::constant(term_number::int(1)),
        internal: term_internal::var,
        modifier: modifier::sin,
        denom: term_internal::constant(term_number::int(1)),
    };



}

#[test]
fn test_derive_start(){
    let test_term_constant = term{
        coefficient: term_internal::constant(term_number::int(1)),
        exponent: term_internal::constant(term_number::int(1)),
        internal: term_internal::constant(term_number::int(1)),
        modifier: modifier::NoMod,
        denom: term_internal::constant(term_number::int(1)),
    };
    assert!(derive_start(&test_term_constant)==tInTerm(term{
        coefficient: term_internal::constant(term_number::int(0)),
        exponent: term_internal::constant(term_number::int(0)),
        internal: term_internal::constant(term_number::int(0)),
        modifier: modifier::NoMod,
        denom: term_internal::constant(term_number::int(1)),
    }));

}


fn derive_start(t:&term)->term_internal{
//check internal - if it is a number or a variable

println!("Deriving: ");
printTerm(&t);
println!();

if term_is_constant(&t){
    return tInTerm(term{
        coefficient: term_internal::constant(term_number::int(0)),
        exponent: term_internal::constant(term_number::int(0)),
        internal: term_internal::constant(term_number::int(0)),
        modifier: modifier::NoMod,
        denom: term_internal::constant(term_number::int(1)),
    })
}

let mut internal_constant = false;
let mut coeff_constant = true;
let mut denom_constant = true;
let mut exponent_constant = true;

if is_constant(&t.internal){
    //THINK
    internal_constant = true;
}
//check for multiplication rule - check coeff
if !is_constant(&t.coefficient){
    //TODO: multiplication rule
    coeff_constant = false;
}

//check for division rule - is denom not a constant
if !is_constant(&t.denom){
    //TODO: division rule
    denom_constant = false;
}

//check for power rule - is exponent not a constant, if it is do power rule if needed
if !is_constant(&t.exponent){
    //TODO: power rule
    exponent_constant = false;
}
let mut exponent_is_one = false;
let mut exponent_is_zero = false;
if let term_internal::constant(term_number::int(x)) = t.exponent{
    if x == 1{
        exponent_is_one = true;
    }
    if x==0{
        exponent_is_zero = true;
    }
}
let mut coeff_is_one = false;
if let term_internal::constant(term_number::int(x)) = t.coefficient{
    if x == 1{
        coeff_is_one = true;
    }
}

//basic derive vvv internal not constant, coeff const, denom const, exponent const, exponent is 1
if !internal_constant&&coeff_constant&&denom_constant&&exponent_constant&&exponent_is_one{
    println!("Basic Derive: ");
    //printTerm(&t);
    print!("\nResult: ");
    printTerm(&basic_derive(t));
    //println!("\nReturn");
    return tInTerm(basic_derive(t));
}
//internal is not constant, coeff is not constant, denom is constant, exponent is constant, exponent is 1
if !internal_constant&&!coeff_constant&&denom_constant&&exponent_constant&&exponent_is_one{
    println!("Multiplication rule: ");
    //printTerm(&t);
    println!("\nResult: ");
    let a = derive_multiplication_rule(t);
    return tInTerms(a.0, a.1);
}
//interal is not constant, coeff and constant, denom is not constant, exponent is constant, exponent is 1
if !internal_constant&&coeff_constant&&!denom_constant&&exponent_constant{
    println!("Division rule: ");
    println!("Result: ");
    return tInTerm(derive_division_rule(t));
}
//if exponent is not 1 and not 0
//if coeff is constant, if denom is constant, if exponent is constant, if internal is not constant
if !internal_constant&&coeff_constant&&denom_constant&&exponent_constant&&!exponent_is_one&&!exponent_is_zero{
    println!("Power rule: ");
    println!("Result: ");

    return tInTerm(power_rule(t));
}


panic!("end");
tInTerm(term::new())
}


///basic derive requirements - internal is not constant, coeff is constant, denom is constant, exponent is constant and is 1
fn basic_derive(t:&term)->term{
    //derive internal first == look for chain
    //coeff is constant
    //denom is constant
    //exponent is constant
    //internal is not constant, check modifier
    //Basically = coeff * derived internal

    println!();
    printTerm(&t);
    println!();
    

    let mut return_term = t.clone();
    let internal_derived = derive(&t.internal);


    println!("      Derived Internal: {:?}", internal_derived);
    //printTerm(&inTermOut(internal_derived.clone()));
    println!();
    let mut chains = vec![];
    // if the derived internal is constant, 
    if is_constant(&internal_derived){
    //if let term_internal::constant(term_number::int(x)) = internal_derived{
        let x = get_constant(internal_derived.clone());
        if t.modifier == modifier::NoMod{
            //make the return term, internal is x, need to chain if internal derived is not one
        return_term = term{
            coefficient: term_internal::constant(term_number::int(1)),
            exponent: term_internal::constant(term_number::int(1)),
            internal: term_internal::constant(term_number::int(x)),
            modifier: modifier::NoMod,
            denom: term_internal::constant(term_number::int(1)),
        };
        if let term_internal::constant(term_number::int(x)) = t.coefficient{
            return_term.coefficient = term_internal::constant(term_number::int(x));
        }
        }else{
            let powerMod= derive_modifier(&t.modifier);
            return_term = term{
                coefficient: term_internal::constant(term_number::int(1)),
                exponent: term_internal::constant(term_number::int(1)),
                internal: t.internal.clone(),
                modifier: modifier::NoMod,
                denom: term_internal::constant(term_number::int(1)),
            };
            if let term_internal::constant(term_number::int(x)) = t.coefficient{
                //make sure to Chain!
                return_term.coefficient = multiply_terms_coeff(&term_internal::constant(term_number::int(x)), &internal_derived);
            }
            normalize_power_mod(&mut return_term, powerMod);
            println!("      Return Term: {:?}", return_term);

        //}

        //println!("return");
        return return_term;
    }

}else{
        //chain rule
        //TODO: multiply terms
        //if t internal is term
        if let term_internal::term(x) = &t.internal{
            print!("          Chaining: ");
            printTerm(x);
            println!("\n");
            //println!("Chains -3: {:?}", chains);
            let derivedTemp = derive_start(&**x);
            print!("          Derived Chain: ");
            printTerm(&inTermOut(derivedTemp.clone()));
            println!("\n");
            chains.push(derivedTemp);
            //println!("Chains -2: {:?}", chains);
        }else if let term_internal::terms(x, o) = &t.internal{
            //if t internal is terms
            let mut new_terms = vec![];
            for i in 0..x.len(){
                let termermer = inTermOut(derive_start(&x[i]));
                //println!("\n\n\n\n\nChain:? ");
                printTerm(&termermer);
                println!();
                new_terms.push(termermer);
            }
            //println!("Chains -1: {:?}", chains);
            chains.push(tInTerms(new_terms, o.clone()));
            //println!("Chains 0: {:?}", chains);
        }
    }
    //println!("Chains 1: {:?}", chains);
    //derive modifier
    let new_mod = derive_modifier(&t.modifier);
    match new_mod{
        powerModifier::power(modif, pow, sign) => {
            return_term.modifier = modif;
            return_term.exponent = term_internal::constant(term_number::int(pow));
            if let term_internal::constant(term_number::int(x)) = t.coefficient{
            return_term.coefficient = term_internal::constant(term_number::int(sign*x));
            }
        }
        _=>panic!("oopsies"),

    };

    //println!("Chains 2: {:?}", chains);
    //println!("Chain len: {}", chains.len());
    for i in &chains{
        println!();
        printTerm(&inTermOut(i.clone()));
    }

    for i in chains{
        println!("\n  Chain Term: ");
        printTerm(&inTermOut(i.clone()));
        println!("\n");
        //println!();
        let new = inTermOut(multiply_terms_coeff(&tInTerm(return_term), &i));
        //println!("       Chain Term new: ");
        //printTerm(&new);
        //println!();
        return_term = new;

    }
    println!("Return Term");
    printTerm(&return_term);
    println!("\nReturn");
    //let return_term = term::new();
    return return_term;

}

fn inTermOut(t:term_internal)->term{
    return term{
        coefficient: term_internal::constant(term_number::int(1)),
        exponent: term_internal::constant(term_number::int(1)),
        internal: t,
        modifier: modifier::NoMod,
        denom: term_internal::constant(term_number::int(1)),
    }
}

fn get_constant(t:term_internal)->i32{
    let mut internal = 1;
    match t.clone(){
        term_internal::constant(term_number::int(x)) => internal = x,
        term_internal::constant(term_number::e) => todo!("Make e"),
        term_internal::constant(term_number::pi) => todo!("Make pi"),
        term_internal::var => panic!("no vars allowed"),
        term_internal::neg_var => panic!("no vars allowed"),
        term_internal::term(x) => internal = i32::pow(get_constant(x.internal), get_constant(x.exponent) as u32)*get_constant(x.coefficient)/get_constant(x.denom),
        term_internal::terms(x, o) => {
            todo!("terms constant");
        }
    }
    //println!("\n\nget_constant: {}\n\n", internal);
internal
}



//division reqs: - internal not con, coef is con, denom not con, exponent con & is 1
fn derive_division_rule(t:&term)->term{
    //derive internal first == look for chain
    //coeff is constant
    //denom is constant
    //exponent is constant
    //internal is not constant, check modifier

    //split the term, top and bottom, get the things, recombine
    let og_coeff = get_constant(t.coefficient.clone());
    let og_exponent = get_constant(t.exponent.clone());
    let denom = &t.denom;
    let numer = term{
        coefficient: tInCon(1),
        exponent: tInCon(og_exponent),
        internal: t.internal.clone(),
        modifier: t.modifier.clone(),
        denom: tInCon(1),
    };

    let derived_numer = derive(&tInTerm(numer.clone()));
    let derived_denom = derive(&denom);

    let new_top1 = multiply_terms_coeff(&denom, &derived_numer);
    let new_top2 = multiply_terms_coeff(&tInTerm(numer), &derived_denom);
    let operator = vec![operators::subtract];
    let new_top_final = tInTerms(vec![inTermOut(new_top1), inTermOut(new_top2)], operator);
    let new_bottom = term{
        coefficient: tInCon(1),
        exponent: tInCon(2),
        internal: denom.clone(),
        modifier: modifier::NoMod,
        denom: tInCon(1),
    };

    let return_term = term{
        coefficient: tInCon(og_coeff),
        exponent: tInCon(1),
        internal: new_top_final,
        denom: tInTerm(new_bottom),
        modifier: modifier::NoMod,
    };

    //println!("return term: {:?}", return_term);
    return return_term;
    panic!("OH NOOO");
    }

//if coeff is constant, if denom is constant, if exponent is constant, if internal is not constant, check modifier
fn power_rule(t:&term)->term{

    if t.modifier == modifier::NoMod{
        let mut og_coeff = 1;
        if is_constant(&t.coefficient){
            og_coeff = get_constant(t.coefficient.clone());
        }
        let mut og_exponent = get_constant(t.exponent.clone());

    let mut return_term = term{

        coefficient: multiply_terms_coeff(&term_internal::constant(term_number::int(og_coeff*og_exponent)), &derive(&t.internal)),
        exponent: term_internal::constant(term_number::int(og_exponent-1)),
        internal: t.internal.clone(),
        modifier: modifier::NoMod,
        denom: term_internal::constant(term_number::int(1)),

    };
    //}
    return return_term;
}else{
    let mut og_coeff = 1;
    if is_constant(&t.coefficient){
        og_coeff = get_constant(t.coefficient.clone());
    }
    let mut og_exponent = get_constant(t.exponent.clone());

// let mut return_term = term{

//     coefficient: multiply_terms_coeff(&term_internal::constant(term_number::int(og_coeff*og_exponent)), &derive(&t.internal)),
//     exponent: term_internal::constant(term_number::int(og_exponent-1)),
//     internal: t.internal.clone(),
//     modifier: modifier::NoMod,
//     denom: term_internal::constant(term_number::int(1)),

// };

let derivable = term{
    coefficient: tInCon(1),
    exponent: tInCon(1),
    internal: t.internal.clone(),
    modifier: t.modifier.clone(),
    denom: tInCon(1),
};

let mut wrapper_term = term{
    coefficient: multiply_terms_coeff(&tInCon(og_exponent*og_coeff), &derive(&tInTerm(derivable))),
    exponent: term_internal::constant(term_number::int(og_exponent-1)),
    internal: t.internal.clone(),
    modifier: t.modifier.clone(),
    denom: term_internal::constant(term_number::int(1)),
};

return wrapper_term;

}

panic!("f");

}



//return a multiplied term_internal coefficient
fn multiply_terms_coeff(t1:&term_internal, t2:&term_internal) -> term_internal{
    let new_term = term{
        coefficient:t2.clone(),
        exponent:tInCon(1),
        internal:t1.clone(),
        modifier:modifier::NoMod,
        denom:tInCon(1),
    };

    return tInTerm(new_term);

    //panic!("OH NO, wrong multiplication, {:?}, {:?}", t1, t2);
    //return term_internal::constant(term_number::int(0));
}


fn derive_modifier(m:&modifier)->powerModifier{
//TODO handle log and ln
match m{
    modifier::sin => powerModifier::power(modifier::cos, 1 , 1),
    modifier::cos => powerModifier::power(modifier::sin, 1, -1),
    modifier::tan => powerModifier::power(modifier::sec, 2, 1),
    modifier::cotan => powerModifier::power(modifier::csc, 2, -1),
    modifier::sec => powerModifier::complex(modifier::sec, 1, modifier::tan, 1, 1),
    modifier::csc => powerModifier::complex(modifier::csc, 1, modifier::cotan, 1, -1),
    modifier::arcsin => powerModifier::complicated,
    modifier::arccos => powerModifier::complicated,
    modifier::arctan => powerModifier::complicated,
    modifier::log(_)=>powerModifier::isLog,
    modifier::ln=>powerModifier::isLog,
    _=>powerModifier::power(modifier::NoMod, 1, 1)


}
}

#[derive(Debug,PartialEq, Clone)]
enum powerModifier{
    //modifer, power, sign
    power(modifier, i32, i32),
    //modifier, power, modifier, power, sign
    complex(modifier, i32, modifier, i32, i32),
    complicated,
    isLog
}



#[test]
fn derive_test_2(){
    let test_term = term{
        coefficient: term_internal::constant(term_number::int(1)),
        exponent: term_internal::constant(term_number::int(1)),
        internal: term_internal::var,
        modifier: modifier::NoMod,
        denom: term_internal::constant(term_number::int(1)),
    };
    assert!(derive(&test_term.internal)==term_internal::constant(term_number::int(1)));

}
#[test]
fn derive_test_start(){

    let test_term = term{
        coefficient: term_internal::constant(term_number::int(1)),
        exponent: term_internal::constant(term_number::int(1)),
        internal: term_internal::var,
        modifier: modifier::sin,
        denom: term_internal::constant(term_number::int(1)),
    };

    let result = derive_start(&test_term);
    let correct = tInTerm(term{
        coefficient: term_internal::constant(term_number::int(1)),
        exponent: term_internal::constant(term_number::int(1)),
        internal: term_internal::var,
        modifier: modifier::cos,
        denom: term_internal::constant(term_number::int(1)),
    });
    println!("Result: {:?}\nCorrect: {:?} \n\n", result, correct);
    assert!(result==correct);

}




fn derive(t:&term_internal)->term_internal{
    return match t{
        term_internal::constant(_x) => derive_constant(),
        term_internal::var => derive_var(),
        term_internal::term(x) => derive_term(&**x),
        term_internal::neg_var => todo!("OH NO"),
        term_internal::terms(x, y) => {
            println!("      multi term derive");
            let mut vec1 = vec![];
            for i in 0..x.len(){
                let termer = derive_start(&x[i]);
                vec1.push(term{
                    coefficient: term_internal::constant(term_number::int(1)),
                    exponent: term_internal::constant(term_number::int(1)),
                    internal: termer,
                    modifier: modifier::NoMod,
                    denom: term_internal::constant(term_number::int(1)),
                });
            }
            term_internal::terms(vec1, y.clone())
        },
        _=>panic!("OH NO"),
    }

}
fn derive_constant()->term_internal{
    term_internal::constant(term_number::int(0))
}
fn derive_var()->term_internal{
    term_internal::constant(term_number::int(1))
}
fn derive_term(t:&term)->term_internal{
    
        return derive_start(&t);
}


fn is_constant(term:&term_internal) -> bool{
    match term{
        term_internal::constant(_) => true,
        term_internal::term(x)=> term_is_constant(x),
        term_internal::terms(x, _)=> {
            let mut is_const = true;
            for i in x{
                if !term_is_constant(i){
                    is_const = false;
                }
            }
            is_const
        },
        _ => false,
    }
}

fn term_is_constant(term:&term)->bool{
    if is_constant(&term.internal)&&is_constant(&term.coefficient)&&is_constant(&term.exponent)&&is_constant(&term.denom){
        return true;
    }
    false
}
#[test]
fn test_is_constant(){

let testTerm = term{
    coefficient: term_internal::constant(term_number::int(1)),
    exponent: term_internal::term(Box::new(term{
        coefficient: term_internal::constant(term_number::int(1)),
        exponent: term_internal::constant(term_number::int(1)),
        internal: term_internal::var,
        modifier: modifier::NoMod,
        denom: term_internal::constant(term_number::int(1)),

    })),
    internal: term_internal::var,
    modifier: modifier::NoMod,
    denom: term_internal::constant(term_number::int(1)),
};
assert!(!is_constant(&testTerm.internal));
assert!(!is_constant(&testTerm.exponent));
assert!(is_constant(&testTerm.coefficient));
assert!(is_constant(&testTerm.denom));


}
#[test]
fn test_term_constant(){
    let testTerm = term{
        coefficient: term_internal::constant(term_number::int(1)),
        exponent: term_internal::term(Box::new(term{
            coefficient: term_internal::constant(term_number::int(1)),
            exponent: term_internal::constant(term_number::int(1)),
            internal: term_internal::var,
            modifier: modifier::NoMod,
            denom: term_internal::constant(term_number::int(1)),
    
        })),
        internal: term_internal::var,
        modifier: modifier::NoMod,
        denom: term_internal::constant(term_number::int(1)),
    };
    assert!(!term_is_constant(&testTerm));

}


//reqs: internal is not constant, coeff is constant, denom is constant, exponent is constant and is 1
fn derive_multiplication_rule(t:&term)->(Vec<term>, Vec<operators>){
    let mut t = t.clone();
    //println!("Derive Multiplication Rule: ");
    //printTerm(&t);
    println!("\nCoefficient: ");
    printTerm(&inTermOut(t.coefficient.clone()));
    println!("\nInternal: ");
    printTerm(&inTermOut(t.internal.clone()));
    println!("\n");

    //get coeff and internal
    let mut internal_var = false;
    let mut coeff_var = false;
    if let term_internal::var = t.internal{
        if t.modifier == modifier::NoMod{
            internal_var = true;
        }else{
            t.internal = tInTerm(term {
                coefficient: term_internal::constant(term_number::int(1)),
                exponent: term_internal::constant(term_number::int(1)),
                internal: term_internal::var,
                modifier: t.modifier.clone(),
                denom: term_internal::constant(term_number::int(1)),
            });
            t.modifier = modifier::NoMod;
        }
        //internal_var = true;
    }
    if let term_internal::var = t.coefficient{
        coeff_var = true;
    }

if !coeff_var&&!internal_var{
    if let term_internal::term(x) = &t.internal{
        if let term_internal::term(y) = &t.coefficient{
            //chain rule! Chain when modifier is not NoMod, exponent must be 1
            let derived_internal = derive(&t.internal);
            let derived_coefficient = derive(&t.coefficient);
            let newMod = derive_modifier(&t.modifier);
            let newMod2 = derive_modifier(&y.modifier);

            let mut returnTerms = vec![];
            let mut operators = vec![];
            //if newMod == powerModifier::

            let mut help1= multiply_terms_coeff(&derived_internal, &t.coefficient);
            let mut help2 = multiply_terms_coeff(&t.internal, &derived_coefficient);

            if t.modifier!=modifier::NoMod{
                let mut newThing = term{
                    coefficient: derived_internal.clone(),
                    exponent: term_internal::constant(term_number::int(1)),
                    internal: t.internal.clone(),
                    modifier: modifier::NoMod,
                    denom: term_internal::constant(term_number::int(1)),
                };
                normalize_power_mod(&mut newThing, newMod);
                //help1 = tInTerm(newThing);
                returnTerms.push(newThing);
            }else{
                returnTerms.push(inTermOut(help1));
            }

            if y.modifier!=modifier::NoMod{
                let mut newThing = term{
                    coefficient: derived_coefficient.clone(),
                    exponent: term_internal::constant(term_number::int(1)),
                    internal: y.internal.clone(),
                    modifier: modifier::NoMod,
                    denom: term_internal::constant(term_number::int(1)),
                };
                normalize_power_mod(&mut newThing, newMod2);
                //help2 = tInTerm(newThing);
                returnTerms.push(newThing);
            }else{
                returnTerms.push(inTermOut(help2));
            }

 


            
            operators.push(operators::add);



            return (returnTerms, operators);

        }else{
            panic!("OH NO COEFFICIENT IS CONSTANT OR SOMETHING: MULTIPLICATION RULE");
        }
    }else{
        printTerm(&t);
        println!("\n{:?}\n{:?}\n", t.internal, t.coefficient);

        panic!("OH NO INTERNAL IS CONSTANT OR SOMETHING: MULTIPLICATION RULE");
    }

}else if !internal_var&&coeff_var{
    println!("else 1");
//////
if let term_internal::term(x) = &t.internal{
//         let derived_internal = derive(&t.internal);

//         let newMod = derive_modifier(&t.modifier);
//         //let newMod2 = derive_modifier(&y.modifier);
//         let derived_coefficient = derive(&t.coefficient);
//         let help1= multiply_terms_coeff(&derived_internal, &t.coefficient);
//         let help2 = multiply_terms_coeff(&t.internal, &derived_coefficient);
//         let mut returnTerms = vec![];
//         let mut operators = vec![];
//         if let term_internal::term(xy) = help1{
//             //xy.modifier = newMod;
//             let mut xy = xy.clone();
//             normalize_power_mod(&mut *xy, newMod);
//             returnTerms.push(*xy);
//         }
//         if let term_internal::term(xy) = help2{
//             //let mut xy = xy.clone();
//             //normalize_power_mod(&mut *xy, newMod2);
//             returnTerms.push(*xy);
//         }
//         operators.push(operators::add);
        
//         return (returnTerms, operators);

            //chain rule! Chain when modifier is not NoMod, exponent must be 1
            let derived_internal = derive(&t.internal);
            let derived_coefficient = derive(&t.coefficient);
            let newMod = derive_modifier(&t.modifier);
            //let newMod2 = derive_modifier(&y.modifier);

            let mut returnTerms = vec![];
            let mut operators = vec![];
            //if newMod == powerModifier::

            let mut help1= multiply_terms_coeff(&derived_internal, &t.coefficient);
            let mut help2 = multiply_terms_coeff(&t.internal, &derived_coefficient);

            if t.modifier!=modifier::NoMod{
                let mut newThing = term{
                    coefficient: derived_internal.clone(),
                    exponent: term_internal::constant(term_number::int(1)),
                    internal: t.internal.clone(),
                    modifier: modifier::NoMod,
                    denom: term_internal::constant(term_number::int(1)),
                };
                normalize_power_mod(&mut newThing, newMod);
                //help1 = tInTerm(newThing);
                returnTerms.push(newThing);
            }else{
                returnTerms.push(inTermOut(help1));
            }

            // if y.modifier!=modifier::NoMod{
            //     let mut newThing = term{
            //         coefficient: derived_coefficient.clone(),
            //         exponent: term_internal::constant(term_number::int(1)),
            //         internal: y.internal.clone(),
            //         modifier: modifier::NoMod,
            //         denom: term_internal::constant(term_number::int(1)),
            //     };
            //     normalize_power_mod(&mut newThing, newMod2);
            //     //help2 = tInTerm(newThing);
            //     returnTerms.push(newThing);
            // }else{
                returnTerms.push(inTermOut(help2));
                operators.push(operators::add);



                return (returnTerms, operators);
// }

}else{
    printTerm(&t);
    println!("\n{:?}\n{:?}\n", t.internal, t.coefficient);
    panic!("OH NO INTERNAL IS CONSTANT OR SOMETHING: MULTIPLICATION RULE");
}

//////

}else if internal_var&&!coeff_var{
    println!("Else 2");
    //////
    if let term_internal::term(y) = &t.coefficient{
            println!("t internal: {:?}", t.internal);
            let derived_internal = derive(&t.internal);
            //let newMod = derive_modifier(&x.modifier);
            let newMod2 = derive_modifier(&y.modifier);
            let derived_coefficient = derive(&t.coefficient);
            let help1= multiply_terms_coeff(&derived_internal, &t.coefficient);
            let help2 = multiply_terms_coeff(&t.internal, &derived_coefficient);
            let mut returnTerms = vec![];
            let mut operators = vec![];
            if let term_internal::term(xy) = help1{
                //xy.modifier = newMod;
                //let mut xy = xy.clone();
                //normalize_power_mod(&mut *xy, newMod);
                returnTerms.push(*xy);
            }
            if let term_internal::term(xy) = help2{
                let mut xy = xy.clone();
                normalize_power_mod(&mut *xy, newMod2);
                returnTerms.push(*xy);
            }
            operators.push(operators::add);
            println!("\nreturn terms: {:?}\n", returnTerms);
            printTermIn(&tInTerms(returnTerms.clone(), operators.clone()));
            println!();
            return (returnTerms, operators);
    
    }else{
        printTerm(&t);
        println!("\n{:?}\n{:?}\n", t.internal, t.coefficient);
    
        panic!("OH NO COEFF IS CONSTANT OR SOMETHING: MULTIPLICATION RULE");
    }
    
    //////
    }

    //term::new()
    panic!("END");
}

fn normalize_power_mod(t:&mut term, p:powerModifier){
    if let powerModifier::power(modif, pow, sign) = p{
        println!("normalizing power mod {:?}, {}, {}", modif, pow, sign);
        t.modifier = modif;
        t.exponent = multiply_terms_coeff(&tInCon(pow), &t.exponent);
        println!("t.exponent: {:?}", t.exponent);
        t.coefficient = multiply_terms_coeff(&tInCon(sign), &t.coefficient);
    }else if let powerModifier::complex(modif1, pow1, modif2, pow2, sign) = p{
        todo!("More complex modifiers");
    }

}



#[test]
fn multiplication_test(){
    //constant*constant
    let test1 = term_internal::constant(term_number::int(2));
    let test2 = term_internal::constant(term_number::int(3));
    let result = multiply_terms_coeff(&test1, &test2);
    println!("{:?}",result);
    assert!(result==term_internal::constant(term_number::int(6)));


    //constant*var
    let test3 = term_internal::constant(term_number::int(2));
    let test4 = term_internal::var;
    let result = multiply_terms_coeff(&test3, &test4);

    println!("\n{:?}\n", result);
    assert!(result==term_internal::term(Box::new(term{
        coefficient: term_internal::constant(term_number::int(2)),
        exponent: term_internal::constant(term_number::int(1)),
        modifier: modifier::NoMod,
        internal: term_internal::var,
        denom: term_internal::constant(term_number::int(1)),
    })));

    //var*var
    let test5 = term_internal::var;
    let test6 = term_internal::var;
    let result = multiply_terms_coeff(&test5, &test6);
    println!("\n{:?}\n", result);


    assert!(result==term_internal::term(Box::new(term{
        coefficient: term_internal::var,
        exponent: term_internal::constant(term_number::int(1)),
        modifier: modifier::NoMod,
        internal: term_internal::var,
        denom: term_internal::constant(term_number::int(1)),
    })));

    //var*term
    //x
    let test7 = term_internal::var;
    //2x^2
    let test8 = term_internal::term(Box::new(term{
        coefficient: term_internal::constant(term_number::int(2)),
        exponent: term_internal::constant(term_number::int(2)),
        modifier: modifier::NoMod,
        internal: term_internal::var,
        denom: term_internal::constant(term_number::int(1)),
    }));
    let result = multiply_terms_coeff(&test7, &test8);
    println!("\n{:?}\n", result);
    //2x(x^2)
    assert!(result==term_internal::term(Box::new(term{
        coefficient: term_internal::term(
            Box::new(term{
                coefficient: term_internal::constant(term_number::int(2)),
                exponent: term_internal::constant(term_number::int(1)),
                modifier: modifier::NoMod,
                internal: term_internal::var,
                denom: term_internal::constant(term_number::int(1)),
            })
        ),
        exponent: term_internal::constant(term_number::int(2)),
        modifier: modifier::NoMod,
        internal: term_internal::var,
        denom: term_internal::constant(term_number::int(1)),
    })));

    //term*term
    //2x^2 * 2x^2
    //4x^2 * x^2
    //multiply coefficients and stuff
    let test9 = term_internal::term(Box::new(term{
        coefficient: term_internal::constant(term_number::int(2)),
        exponent: term_internal::constant(term_number::int(2)),
        modifier: modifier::NoMod,
        internal: term_internal::var,
        denom: term_internal::constant(term_number::int(1)),
    }));

    let result = multiply_terms_coeff(&test9, &test9);
    println!("\n{:?}\n", result);
    assert!(result==term_internal::term(Box::new(term { 
            coefficient: term_internal::term(Box::new(term{
            coefficient: term_internal::constant(term_number::int(2)),
            exponent: term_internal::constant(term_number::int(1)),
            modifier: modifier::NoMod,
            internal: term_internal::term(Box::new(term { 
                coefficient: term_internal::constant(term_number::int(2)), 
                exponent: term_internal::constant(term_number::int(2)), 
                internal: term_internal::var, 
                modifier: modifier::NoMod, 
                denom: term_internal::constant(term_number::int(1)),
            })),
            denom: term_internal::constant(term_number::int(1)),
        })),
        exponent: term_internal::constant(term_number::int(2)), 
        internal: term_internal::var, 
        modifier: modifier::NoMod, 
        denom: term_internal::constant(term_number::int(1)),
        })));


}

//utility functions
fn tnum(i:i32)->term_number{
    term_number::int(i)
}
fn noMod()->modifier{
    modifier::NoMod
}
fn tInCon(i:i32)->term_internal{
    term_internal::constant(tnum(i))
}
fn tInVar()->term_internal{
    term_internal::var
}
fn tInTerm(t:term)->term_internal{
    term_internal::term(Box::new(t))
}
fn tInTerms(t:Vec<term>, o:Vec<operators>)->term_internal{
    term_internal::terms(t, o)
}


fn printTerm(t:&term){
    //println!("\n\nPrinting term: {:?}\n", t);
    //let t = simplify(t);
    //print coefficient
    print!("(");
    print!("(");
    // if let term_internal::constant(term_number::int(x)) = t.coefficient{
    //     //println!("\n\n{}\n\n", get_constant(t.coefficient.clone()));
    //     if x!=1{
    //         print!("{}", x);
    //     }
    if is_constant(&t.coefficient){
        if get_constant(t.coefficient.clone())!=1{
            let x = get_constant(t.coefficient.clone());
            print!("{}", x);
            //}
        }
    }else{
        printTermIn(&t.coefficient);
    }
    //print modifier  
    printMod(&t.modifier);
    print!("(");
    //print internal
    printTermIn(&t.internal);
    print!(")");
    //print exponent
    if is_constant(&t.exponent){
        if get_constant(t.exponent.clone())!=1{
            let x = get_constant(t.exponent.clone());
            print!("^({})", x);
        
        }else{
            // if let term_internal::constant(term_number::int(x)) = t.exponent{
            //     print!(" ^({})", x);
            // }else{
            //     panic!("Not costant");
            // }
        }

    }else{
    print!("^(");
    printTermIn(&t.exponent);
    print!(")");
    }

    print!(")");
    //print denom
    if is_constant(&t.denom){
        if get_constant(t.denom.clone())!=1{
            let x = get_constant(t.denom.clone());
            print!("  / ({})", x);
            
        }
    }else{
    print!("/(");
    printTermIn(&t.denom);
    print!(")");
    }

    print!(")");

}

fn printTermIn(t:&term_internal){

    match t{
        term_internal::constant(term_number::int(x)) => print!("{}", x),
        term_internal::constant(term_number::e) => print!("e"),
        term_internal::constant(term_number::pi) => print!("pi"),
        term_internal::term(x) => printTerm(&*x),
        term_internal::neg_var => print!("(-x)"),
        term_internal::var=>print!("x"),
        term_internal::terms(x, o)=>{
            print!("(");
            for i in 0..x.len(){
                printTerm(&x[i]);
                if i<o.len(){
                match o[i]{
                    operators::add => print!("+"),
                    operators::subtract => print!("-"),
                    operators::multiply => print!("*"),
                    operators::divide => print!("/"),
                    operators::power => print!("^"),
                    operators::none => print!(""),
                }
            }

            }
            print!(")");

        },
        //_=>panic!("OH NO"),
    }
}

fn printMod(m:&modifier){
    match m{
        modifier::sin => print!("sin"),
        modifier::cos => print!("cos"),
        modifier::tan => print!("tan"),
        modifier::cotan => print!("cotan"),
        modifier::sec => print!("sec"),
        modifier::csc => print!("csc"),
        modifier::arcsin => print!("arcsin"),
        modifier::arccos => print!("arccos"),
        modifier::arctan => print!("arctan"),
        modifier::arccotan => print!("arccotan"),
        modifier::arcsec => print!("arcsec"),
        modifier::arccsc => print!("arccsc"),
        modifier::ln => print!("ln"),
        modifier::log(x) => print!("log(Base{})", x),
        modifier::NoMod => print!(""),
        //_=>panic!("OH NO"),
    }
}

#[test]
fn testPrint(){

let i = term { 
    coefficient: term_internal::term(Box::new(term{
    coefficient: term_internal::constant(term_number::int(2)),
    exponent: term_internal::constant(term_number::int(1)),
    modifier: modifier::NoMod,
    internal: term_internal::term(Box::new(term { 
        coefficient: term_internal::constant(term_number::int(2)), 
        exponent: term_internal::constant(term_number::int(2)), 
        internal: term_internal::var, 
        modifier: modifier::NoMod, 
        denom: term_internal::constant(term_number::int(1)),
    })),
    denom: term_internal::constant(term_number::int(1)),
})),
exponent: term_internal::constant(term_number::int(2)), 
internal: term_internal::var, 
modifier: modifier::NoMod, 
denom: term_internal::constant(term_number::int(1)),
};
printTerm(&i);

}

#[test]
fn advanced_multiplication_test(){

//2x(x)
//result = 2x+2x
let test1 = 
inTermOut(term_internal::term(Box::new(term{
    coefficient: tInTerm(term{
        coefficient: tInCon(2),
        exponent: tInCon(1),
        internal: tInVar(),
        modifier: noMod(),
        denom: tInCon(1),
    }),
    exponent: tInCon(1),
    modifier: noMod(),
    internal: tInVar(),
    denom: tInCon(1),
})));

printTerm(&test1);
println!("\n{:?}", test1);
let a = derive_start(&test1);
println!("\nFinal Derived Output:");
printTerm(&strip_outer_layers(&inTermOut(a)));
println!();
//assert!(false);
}

fn is_term(t:&term_internal)->bool{
    if let term_internal::term(_) = &t{
        true
    }else{
        false
    }
}

fn get_term(t:term_internal)->term{
    if let term_internal::term(x) = t{
        *x
    }else{
        panic!("Not a term");
    }
}

fn strip_outer_layers(t:&term)->term{
    //println!("\n\nstart\n\n");
    //printTerm(&t);
    //println!("\n\n");
    let mut return_term = t.clone();
    //printTerm(&t);
    //println!("{:?}", t);

    if is_term(&t.internal){
        let x = get_term(t.internal.clone());
        //println!("In term: \n");
        //printTerm(&x);
        //println!("\n");
        if is_constant(&t.coefficient) && get_constant(t.coefficient.clone())==1{
            if is_constant(&t.denom) && get_constant(t.denom.clone())==1{
                if is_constant(&t.exponent) && get_constant(t.exponent.clone())==1{
                    if modifier::NoMod == t.modifier{
                        return_term = strip_outer_layers(&x);
                    }else{
                        //println!("Modifier not NoMod");
                    }
                }else{
                    //println!("Exponent not 1");
                }
            }else{
                //println!("Denom not 1");
            }
        }else{
            //println!("Coeff not 1");
        }
        //return_term = strip_outer_layers(&*x);
    }else if let term_internal::terms(x, o) = &t.internal{
        if is_constant(&t.coefficient) && get_constant(t.coefficient.clone())==1{
            if is_constant(&t.denom) && get_constant(t.denom.clone())==1{
                if is_constant(&t.exponent) && get_constant(t.exponent.clone())==1{
                    if modifier::NoMod == t.modifier{
        let mut stripped = vec![];
        for i in x{
            stripped.push(strip_outer_layers(i));
        }
        return_term = inTermOut(tInTerms(stripped, o.to_vec()));}}}}
    }
    else{
        //println!("term internal not term, {:?}", t.internal);
    }
    //println!("\nReturn Term:\n");
    //printTerm(&return_term);
    //println!("\n");

    return_term
}


fn simplify(t:&term)->term{
    //println!("\nSimplifying: {:?}\n", t);

    let mut t= t.clone();
    t = strip_outer_layers(&t);

    if let term_internal::term(x) = t.internal{
        t.internal = tInTerm(strip_outer_layers(&*x));
    }
    if let term_internal::term(x) = t.coefficient{
        t.coefficient = tInTerm(strip_outer_layers(&*x));
    }
    if let term_internal::term(x) = t.exponent{
        t.exponent = tInTerm(strip_outer_layers(&*x));
    }
    if let term_internal::term(x) = t.denom{
        t.denom = tInTerm(strip_outer_layers(&*x));
    }


    if let term_internal::terms(x, o) = t.internal{
        let mut stripped = vec![];
        for i in x{
            stripped.push(strip_outer_layers(&i));
        }
        t.internal = tInTerms(stripped, o.to_vec());
    }
    if let term_internal::terms(x, o) = t.coefficient{
        let mut stripped = vec![];
        for i in x{
            stripped.push(strip_outer_layers(&i));
        }
        t.coefficient = tInTerms(stripped, o.to_vec());
    }
    if let term_internal::terms(x, o) = t.exponent{
        let mut stripped = vec![];
        for i in x{
            stripped.push(strip_outer_layers(&i));
        }
        t.exponent = tInTerms(stripped, o.to_vec());
    }
    if let term_internal::terms(x, o) = t.denom{
        let mut stripped = vec![];
        for i in x{
            stripped.push(strip_outer_layers(&i));
        }
        t.denom = tInTerms(stripped, o.to_vec());
    }


    t


    //panic!("oopsie");
}

#[test]
fn get_constant_test(){
    let constant = term{
        coefficient: term_internal::constant(term_number::int(2)),
        exponent: term_internal::constant(term_number::int(1)),
        internal: tInTerm(term{
            coefficient: term_internal::constant(term_number::int(2)),
            exponent: term_internal::constant(term_number::int(1)),
            internal: tInTerm(term{
                coefficient: tInTerm(term{
                    coefficient: term_internal::constant(term_number::int(2)),
                    exponent: term_internal::constant(term_number::int(1)),
                    internal: tInCon(1),
                    modifier: modifier::NoMod,
                    denom: term_internal::constant(term_number::int(1)),
                }),
                exponent: term_internal::constant(term_number::int(1)),
                internal: tInCon(1),
                modifier: modifier::NoMod,
                denom: term_internal::constant(term_number::int(1)),
            }),
            modifier: modifier::NoMod,
            denom: term_internal::constant(term_number::int(1)),
        }),
        modifier: modifier::NoMod,
        denom: term_internal::constant(term_number::int(1)),
    };

let con =get_constant(tInTerm(constant));
println!("{:?}", con);
assert!(con==8);
}