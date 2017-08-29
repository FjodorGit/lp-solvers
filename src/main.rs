extern crate lp_modeler;

use lp_modeler::operations::LpOperations;
use lp_modeler::problem::{LpProblem, LpObjective, LpFileFormat};
use lp_modeler::solvers::*;
use lp_modeler::variables::{LpInteger};

fn main() {


    let ref a = LpInteger::new("a");
    let ref b = LpInteger::new("b");
    let ref c = LpInteger::new("c");

    let mut problem = LpProblem::new("Problem", LpObjective::Maximize);

    problem += 10.0 * a + 20.0 * b;

    problem += (300*(a-b)).ge(100);
    problem += (300*(-a+b)).le(100);
    problem += (a+b).le(10);
    let coucou = ((2+a) * (2+b)).to_lp_file_format();
    println!("{}", coucou);

   // problem += ((a+b)*300).le(100);


    // Suboptimal instead of unfeasible
    // problem += (a+b).equal(10);

    // let solver = GurobiSolver
    // solver <<= BaseDirectory("/opt/gurobi1.2/...")
    // solver <<= Config().arg("-thread 2").arg("...")

    let _ = problem.write_lp("toto.lp");

    let solver = GurobiSolver::new();

    match solver.run(&problem) {
        Ok((status, res)) => {
            println!("Status {:?}", status);
            for (name, value) in res.iter() {
                println!("value of {} = {}", name, value);
            }
        },
        Err(msg) => println!("{}", msg),
    }

}
