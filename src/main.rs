mod bindings {
    pub mod ipopt;
}

use crate::bindings::ipopt::*;

fn main() {
    
    // Set the number of variables
    let n: i32 = 4;
    let mut x_L: [f64; 4] = [1.0, 1.0, 1.0, 1.0];
    let mut x_U: [f64; 4] = [5.0, 5.0, 5.0, 5.0];
    
    // Set the number of constraints
    let m: i32 = 2;
    let mut g_L: [f64; 2] = [25.0, 40.0];
    let mut g_U: [f64; 2] = [2e19, 40.0];

    // Set the number of nonzeroes in the Jacobian and Hessisan
    let nele_jac: i32 = 8;
    let nele_hess: i32 = 10;

    // Set the indexing style to C-style 
    let index_style: i32 = 0;

    let mut obj: [f64; 1] = [0.0];
    let mut mult_g: [f64; 2] = [0.0, 0.0];
    let mut mult_x_L: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
    let mut mult_x_U: [f64; 4] = [0.0, 0.0, 0.0, 0.0];

    // Set the initial point and the values
    let mut x: [f64; 4] = [1.0, 5.0, 5.0, 1.0];

    let mut user_data: ipopt::IpoptProblemInfo = ipopt::IpoptProblemInfo::default();

    unsafe {
        let nlp: ipopt::IpoptProblem = ipopt::CreateIpoptProblem(
            n as ipopt::ipindex, 
            x_L.as_mut_ptr(), 
            x_U.as_mut_ptr(), 
            m as ipopt::ipindex, 
            g_L.as_mut_ptr(), 
            g_U.as_mut_ptr(), 
            nele_jac as ipopt::ipindex, 
            nele_hess as ipopt::ipindex, 
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
            mult_x_L.as_mut_ptr(), 
            mult_x_U.as_mut_ptr(),
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
