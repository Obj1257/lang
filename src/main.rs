
 #[derive(PartialEq, Debug, Clone)]
enum Prog {
    Bool(bool),
    Int(u16),
    Add(Box<Prog>, Box<Prog>),
    Lt(Box<Prog>,Box<Prog>),
    If(Box<Prog>,Box<Prog>,Box<Prog>)
}

 #[derive(PartialEq, Debug)]
enum Typ {
    TBool,
    TInt,
}

fn infer(prog:&Prog) -> Result<Typ, String>{
    match prog {
        Prog::Bool(_) => Ok(Typ::TBool),
        Prog::Int(_) => Ok(Typ::TInt),
        Prog::Add(p1, p2 ) => {
            let p1_type = infer(&*p1)?;
            let p2_type = infer(&*p2)?;
            match (p1_type, p2_type) {
                (Typ::TInt, Typ::TInt) => Ok(Typ::TInt),
                _ => Err("Il y a une addition qui ne fait pas intervenir deux entiers".to_string())
            }

        }
        Prog::Lt(p1, p2 ) => {
            let p1_type = infer(&*p1)?;
            let p2_type = infer(&*p2)?;
            match (p1_type, p2_type) {
                (Typ::TInt, Typ::TInt) => Ok(Typ::TBool),
                _ => Err("Il y a une comparaison qui ne fait pas intervenir deux entiers".to_string())
            }

        }
        Prog::If(p,q,r) => {
            let p_type = infer(&*p)?;
            let q_type = infer(&*q)?;
            let r_type = infer(&*r)?;
            
            
            if p_type == Typ::TInt{
                return Err("La condition pour un if n'est pas un bool".to_string());
            }
            match (q_type, r_type) {
                (Typ::TBool, Typ::TBool) => Ok(Typ::TBool),
                (Typ::TInt, Typ::TInt) => Ok(Typ::TInt),
                _ => Err("Les branches d'un if n'ont pas le même type".to_string()),

            }

        }
    }
}

fn typable(prog: &Prog) -> bool {
    match infer(&prog) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn reduce(prog: &Prog) -> Option<Prog> {
    match prog {
        Prog::Add(p1, p2) => {
            match (p1.as_ref(), p2.as_ref()) {
                (Prog::Int(n1), Prog::Int(n2)) => Some(Prog::Int(n1 + n2)),
                _ => {
                    let reduced_p1: Prog = reduce(p1).unwrap_or_else(|| *p1.clone());
                    let reduced_p2: Prog = reduce(p2).unwrap_or_else(|| *p2.clone());
                    Some(Prog::Add(Box::new(reduced_p1), Box::new(reduced_p2)))
                }
            }

        },
        Prog::If(cond, p, q) => match &**cond {
            Prog::Bool(true) => Some(*p.clone()),
            Prog::Bool(false) => Some(*q.clone()),
            _ => {
                let reduced_cond = reduce(cond).unwrap_or_else(|| *cond.clone());
                Some(Prog::If(Box::new(reduced_cond), Box::new(*p.clone()), Box::new(*q.clone())))
            }
        },
        Prog::Lt(p1, p2) => {
            match (p1.as_ref(), p2.as_ref()) {
                (Prog::Int(n1), Prog::Int(n2)) => Some(Prog::Bool(n1 < n2)),
                _ => {
                    let reduced_p1: Prog = reduce(p1).unwrap_or_else(|| *p1.clone());
                    let reduced_p2: Prog = reduce(p2).unwrap_or_else(|| *p2.clone());
                    Some(Prog::Lt(Box::new(reduced_p1), Box::new(reduced_p2)))
                }
            }

        },
        _ => None
    }
}

fn normalize(prog : &Prog) -> Result<Prog, String> {
    if typable(prog){
        let reduced = reduce(prog);
        match reduced {
            Some(a) => Ok(normalize(&a)?),
            None => Ok(prog.clone()),
        }
    }
    else {
        Err("Le programme n'est pas typable".to_string())
    }

}



fn main() {
    //if 1+(2+3) < 4 then 6 else 5
    let test = Prog::If(
        Box::new(Prog::Lt(
            Box::new(Prog::Add(
                Box::new(Prog::Int(1)),
                Box::new(Prog::Add(Box::new(Prog::Int(2)), Box::new(Prog::Int(3))))
            )),
            Box::new(Prog::Int(4))
        )),
        Box::new(Prog::Int(6)),
        Box::new(Prog::Int(5))
    );
    
    println!("\nTypable: {}", typable(&test));
    
    let normalized = normalize(&test);
    println!("{:?}", normalized);
    

}