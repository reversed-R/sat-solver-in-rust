mod entities;
mod solver;

use entities::{
    assignment::{Assignment, Assignments},
    clause::Clause,
    cnf::Cnf,
    thesis::{AtomicThesis, Thesis},
};
use solver::solve;

fn main() {
    let a_p = AtomicThesis::new("P".to_string());
    let a_q = AtomicThesis::new("Q".to_string());
    let a_r = AtomicThesis::new("R".to_string());
    let a_s = AtomicThesis::new("S".to_string());

    let p = Thesis::Positive(&a_p);
    let q = Thesis::Positive(&a_q);
    let r = Thesis::Positive(&a_r);
    let s = Thesis::Positive(&a_s);

    let not_p = p.not();
    let not_q = q.not();
    let not_r = r.not();
    let not_s = s.not();

    let cnf = Cnf::new(vec![
        Clause::new(vec![&not_p, &q, &r]),
        Clause::new(vec![&p, &r, &s]),
        Clause::new(vec![&p, &r, &not_s]),
        Clause::new(vec![&p, &not_r, &s]),
        Clause::new(vec![&p, &not_r, &not_s]),
        Clause::new(vec![&not_q, &not_r, &s]),
        Clause::new(vec![&not_p, &q, &not_r]),
        Clause::new(vec![&not_p, &not_q, &r]),
    ]);

    println!("cnf(original): ");
    println!("{}", cnf.to_string());

    let res = solve(
        &cnf,
        Assignments {
            assigns: vec![
                Assignment::new(&a_p, None),
                Assignment::new(&a_q, None),
                Assignment::new(&a_r, None),
                Assignment::new(&a_s, None),
            ],
        },
    );

    println!("--------solved!--------");
    match res {
        Ok(ass) => {
            println!("Satisfiable: {}", ass.to_string());
        }
        Err(_) => println!("Unsatisfiable!"),
    }
}
