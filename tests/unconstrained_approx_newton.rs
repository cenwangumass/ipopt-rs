extern crate ipopt;
#[macro_use] extern crate approx;

use ipopt::*;

struct NLP {
}

impl BasicProblem for NLP {
    fn num_variables(&self) -> usize { 2 }
    fn bounds(&self) -> (Vec<Number>, Vec<Number>) { (vec![-1e20; 2], vec![1e20; 2]) }
    fn initial_point(&self) -> Vec<Number> { vec![0.0, 0.0] }
    fn objective(&mut self, x: &[Number], obj: &mut Number) -> bool {
        *obj = (x[0] - 1.0)*(x[0] - 1.0) + (x[1] - 1.0)*(x[1] - 1.0);
        true
    }
    fn objective_grad(&mut self, x: &[Number], grad_f: &mut [Number]) -> bool {
        grad_f[0] = 2.0*(x[0] - 1.0);
        grad_f[1] = 2.0*(x[1] - 1.0);
        true
    }
}

#[test]
fn quadratic_test() {
    let nlp = NLP { };
    let mut ipopt = Ipopt::new_unconstrained(nlp);
    ipopt.set_option("tol", 1e-9);
    ipopt.set_option("mu_strategy", "adaptive");
    ipopt.set_option("sb", "yes"); // suppress license message
    ipopt.set_option("print_level", 0); // suppress debug output
    let (r, obj) = ipopt.solve();
    {
        let x = ipopt.solution();
        assert_eq!(r, ReturnStatus::SolveSucceeded);
        assert_relative_eq!(x[0], 1.0, epsilon = 1e-10);
        assert_relative_eq!(x[1], 1.0, epsilon = 1e-10);
        assert_relative_eq!(obj, 0.0, epsilon = 1e-10);
    }
}