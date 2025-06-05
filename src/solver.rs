use crate::entities::{assignment::Assignments, cnf::Cnf};

#[derive(PartialEq)]
pub enum SolveError {
    Unsat,
}

pub fn solve<'a>(cnf: &Cnf<'a>, assigns: Assignments<'a>) -> Result<Assignments<'a>, SolveError> {
    println!("--------solving--------");
    match assigns.assigns.iter().find(|a| a.value.is_none()) {
        Some(a) => match cnf.clone().assign(a.thesis, true) {
            Ok(assigned_cnf) => {
                let assigned_assigns = assigns.clone().assign(a.thesis, true);
                println!("{}", assigned_assigns.to_string());
                println!("{}", assigned_cnf.to_string());

                if assigned_cnf.is_empty() {
                    Ok(assigned_assigns)
                } else {
                    solve(&assigned_cnf, assigned_assigns)
                }
            }
            Err(_) => match cnf.clone().assign(a.thesis, false) {
                Ok(assigned_cnf) => {
                    let assigned_assigns = assigns.clone().assign(a.thesis, false);
                    println!("{}", assigned_assigns.to_string());
                    println!("{}", assigned_cnf.to_string());

                    if assigned_cnf.is_empty() {
                        Ok(assigned_assigns)
                    } else {
                        solve(&assigned_cnf, assigned_assigns)
                    }
                }
                Err(_) => Err(SolveError::Unsat),
            },
        },
        None => return Ok(assigns),
    }
}
