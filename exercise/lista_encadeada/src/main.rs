

use std::io::{self, BufRead};

pub fn readinput() -> String {
    let mut lines = io::stdin().lock().lines();
    let mut user_input = String::new();

    while let Some(line) = lines.next() {
        let last_input = line.unwrap();

        // stop reading
        if last_input.len() == 0 {
            break;
        }

        // add a new line once user_input starts storing user input
        if user_input.len() > 0 {
            user_input.push_str("\n");
        }

        // store user input
        user_input.push_str(&last_input);
        break;
    }

    //println!("\nMulti-line user input \n{}", user_input);

    // the lock is released after it goes out of scope
    return user_input;
}

pub fn read_linked_list(list:&mut Vec<Item>,pos: usize){
    if list.len() == 0 {return;}
    let curval:&Item = &list[pos];
    print!("{}",&curval.value);
    if curval.next==999999999usize{
        return
    } else {
        
        continue_read_linked_list(list, curval.next);
    }
}

pub fn continue_read_linked_list(list:&mut Vec<Item>,pos: usize){
    let curval:&Item = &list[pos];
    print!(", {}",curval.value);
    if curval.next==999999999usize{
        return
    } else {
        continue_read_linked_list(list, curval.next);
    }
}

pub fn get_last(list:&mut Vec<Item>,pos: usize) -> usize {
    let curval:&Item = &list[pos];
    print!(", {}",curval.value);
    if curval.next==999999999usize{
        return pos
    } else {
        return get_last(list, curval.next);
    }
}

pub fn get_from_index(list:&mut Vec<Item>,pos: usize,limit:usize) -> usize {
    let curval:&Item = &list[pos];
    print!(", {}",curval.value);
    
    if limit == 0 {
        println!("Returned 0");
        return pos;
    }
    if curval.next==999999999usize{
        println!("{},{}",curval.next,curval.value);
        return 999999999usize
    } else {
        let limit = limit-1;
        return get_from_index(list, curval.next,limit);
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Item {
    value: String,
    next: usize,
}


#[allow(unused_mut)]
#[allow(dead_code)]
fn main() {
    //let linked: () = ();
    //#[warn(while_true)]
    //while true {
    //    println!("Hello, world!");
    //    let mut val:String = String::from("");
    //    stdi
    //}
    let mut valores: Vec<Item> = Vec::new();
    let mut pi: usize = 0;
    let mut pf: usize = 0;
    

    'loopz: loop{
        print!("Valores: (");
        let mut first = true;
        for val in &valores {
            if first==false {
                print!(", {}",&val.value);
            } else {
                print!("{}",&val.value);
                first = false
            }
        } 
        println!(")");
        print!("Lista Encadeada Efetiva: (");
        read_linked_list(&mut valores,pi);
        println!(")");
        println!("1-Adicionar no fim");
        println!("2-Adicionar no inicio");
        println!("3-Adicionar no meio");
        print!("{}[2J", 27 as char);
        let action = readinput();
        if action=="1"{
            println!("Insira o valor que deve ser inserido na lista");
            if valores.len()==0{
                let valor:Item = Item {value: readinput(), next: 999999999usize};
                valores.push(valor);
                pf = 0usize;
            } else {
                let last_index = get_last(&mut valores, pi);
                let valor:Item = Item {value: readinput(), next: 999999999usize};
                valores.push(valor);
                pf = valores.len()-1usize;
                valores[last_index].next = pf;
            }
        } else if action=="2"{
            println!("Insira o valor que deve ser inserido na lista");
            if valores.len()==0{
                let valor:Item = Item {value: readinput(), next: 999999999usize};
                valores.push(valor);
            } else {
                let valor:Item = Item {value: readinput(), next: pi};
                valores.push(valor);
                pi = valores.len()-1usize;
            }
            println!("Action2");
        } else if action=="3"{
            
            let mut local:String;
            'readlocal:loop {
                //print!("{}[2J", 27 as char);
                println!("Insira o local");
                local = readinput();
                let mut index:usize;
                let parse:usize = local.parse().unwrap();
                if parse<valores.len() {
                    if parse==0usize {
                        let valor:Item = Item {value: readinput(), next: pi};
                        valores.push(valor);
                        pi = valores.len()-1usize;
                        break 'readlocal;
                    }
                    index = get_from_index(&mut valores,pi,parse);
                    if index!=999999999usize {
                        println!("Insira o valor que deve ser inserido na lista");
                        let nextnext = valores[index].next;
                        let valor:Item = Item {value: readinput(), next: nextnext};
                        valores.push(valor);
                        valores[index].next = valores.len()-1usize;
                        break 'readlocal;
                    }
                println!("{}",index);
                }
            }
            println!("Insira o valor que deve ser inserido na lista");
            if valores.len()==0{
                let valor:Item = Item {value: readinput(), next: 999999999usize};
                valores.push(valor);
            }
            println!("Action3");
        } else {
            println!("AAAAA")
        }
        print!("{}[2J", 27 as char);
        
    }

}

