use std::fmt::Show;

fn modify_by_ref(y: &mut int) {
    let new_y = 7i;
    println!("changing from {} to {}", *y, new_y);
    *y = new_y;
}

/// join all values of an iterator to a <sep> seperated
/// String
///
/// the elements of the iterator have to implement the Show trait
fn join_into_string<V: Show, I: Iterator<V>>(iter: &mut I, sep: &str) -> String {
    let mut s = "".to_string();
    while let Some(v) = iter.next() {
        if !s.is_empty() {
            s.push_str(sep);
        }
        s.push_str(format!("{}", v).as_slice());
    }
    s
}


fn main() {
    let mut x = 5i;
    println!("initial x: {}", x)

    modify_by_ref(&mut x);
    println!("modify_by_ref x: {}", x)

    // block
    {
        let x = 2i;
        println!("inside block x: {}", x)
    }
    println!("outside block x: {}", x)


    // join iterator into string
    let mut s = "".to_string();
    for x in range(0i, x) {
        if !s.is_empty() {
            s = s + ", ";
        }
        s = s + format!("{}", x).as_slice();
    }
    println!("joining in loop: s = {}", s);


    // generic variant of joining interator into string
    let mut iter = range(0i, x);
    println!("joining using generics: s = {}", join_into_string(&mut iter, ", "));


    // closure
    let fnc = |txt:&str, v:&int| { 
        println!("{}: v = {}", txt, v);
    };
    let ai = [20i, 45i, 67i].iter().map(|&v| {v*2}).collect::<Vec<int>>();
    for v in ai.iter() {
        fnc("closure", v);
    }
    for v in ai.iter() {
        let v2 = v.clone();
        spawn(move || {
            println!("spawned closure: {}", v2);
        });
    }

    // channels
    let (tx1, rx1) = channel();
    spawn(move || {
        loop {
            match rx1.recv_opt() {
                Ok(v) => {
                    println!("recieved on channel: {}", v);
                },
                Err(()) => {
                    println!("Could not receive - channel closed");
                    break
                }
            };
        }     
    });

    for v in ai.iter() {
        tx1.send(v.clone());
    }


}
