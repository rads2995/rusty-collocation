mod bindings {
    pub mod ipopt;
}

pub mod nlp;

use crate::bindings::ipopt::*;

fn main() {
    
    // Define lower and upper bounds for decision variables
    let mut x_l: [f64; 4] = [1.0, 1.0, 1.0, 1.0];
    let mut x_u: [f64; 4] = [5.0, 5.0, 5.0, 5.0];
    
    // Set the number of decision variables
    assert!(x_l.len() == x_u.len());
    let n: usize = x_l.len();

    // Define lower and upper bounds for constraint functions
    let mut g_l: [f64; 2] = [25.0, 40.0];
    let mut g_u: [f64; 2] = [2e19, 40.0];

    // Set the number of constraints
    assert!(g_l.len() == g_u.len());
    let m: usize = g_l.len();

    // Set the number of nonzeroes in the Jacobian and Hessisan
    let num_elem_jac: i32 = 8;
    let num_elem_hess: i32 = 10;

    // Set the indexing style to C-style 
    let index_style: i32 = 0;

    // Create stack-allocated arrays to store IPOPT obtained values
    let mut obj: [f64; 1] = [0.0];
    let mut mult_g: [f64; 2] = [0.0, 0.0];
    let mut mult_x_l: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
    let mut mult_x_u: [f64; 4] = [0.0, 0.0, 0.0, 0.0];

    // Set the initial condition for the decision variables
    let mut x: [f64; 4] = [1.0, 5.0, 5.0, 1.0];
    
    // Verify that all stack-allocated arrays have correct dimensions
    assert!(x.len() == n);
    assert!(obj.len() == 1);
    assert!(mult_g.len() == m);
    assert!(mult_x_l.len() == n);
    assert!(mult_x_u.len() == n);

    // Create opaque C-style struct that will be used to store IPOPT information
    let mut user_data: ipopt::IpoptProblemInfo = ipopt::IpoptProblemInfo::default();

    unsafe {
        let nlp: ipopt::IpoptProblem = ipopt::CreateIpoptProblem(
            n as ipopt::ipindex, 
            x_l.as_mut_ptr(), 
            x_u.as_mut_ptr(), 
            m as ipopt::ipindex, 
            g_l.as_mut_ptr(), 
            g_u.as_mut_ptr(), 
            num_elem_jac as ipopt::ipindex, 
            num_elem_hess as ipopt::ipindex, 
            index_style as ipopt::ipindex, 
            Some(ipopt::helper::eval_f), 
            Some(ipopt::helper::eval_g), 
            Some(ipopt::helper::eval_grad_f), 
            Some(ipopt::helper::eval_jac_g), 
            Some(ipopt::helper::eval_h)
        );

        ipopt::AddIpoptNumOption(nlp, c"tol".as_ptr() as *const i8, 3.82e-6);
        ipopt::AddIpoptStrOption(nlp, c"mu_strategy".as_ptr() as *const i8, c"adaptive".as_ptr() as *const i8);
        ipopt::AddIpoptStrOption(nlp, c"output_file".as_ptr() as *const i8, c"ipopt.out".as_ptr() as *const i8);                 

        let status: i32 = ipopt::IpoptSolve(
            nlp, 
            x.as_mut_ptr(), 
            core::ptr::null_mut(),
            obj.as_mut_ptr(), 
            mult_g.as_mut_ptr(), 
            mult_x_l.as_mut_ptr(), 
            mult_x_u.as_mut_ptr(),
            &mut user_data as *mut ipopt::IpoptProblemInfo
        );
        
        match ipopt::IpoptReturnStatus::try_from(status) {
            Ok(enum_value) => {
                match enum_value {
                    ipopt::IpoptReturnStatus::SolveSucceeded => {
                        println!("Solve succeded!");
                    }

                    _ => println!("Other status: {:?}", enum_value),
                }
            }

            Err(_) => {
                println!("Unknown return status from IPOPT: {}", status)
            }
        }
        
        ipopt::FreeIpoptProblem(nlp);
    } 
}
