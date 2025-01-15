use std::fs;
use serde::Deserialize;

mod bindings {
    pub mod ipopt;
}

pub mod nlp;

use crate::bindings::ipopt::*;

#[derive(Deserialize)]
struct Config {
    ipopt: Ipopt,
}

#[derive(Deserialize)]
struct Ipopt {
    tol: ipopt::ipnumber,
    mu_strategy: std::ffi::CString,
    output_file: std::ffi::CString,
    linear_solver: std::ffi::CString,
    index_style: u32,
    x: Vec<ipopt::ipnumber>,
    x_l: Vec<ipopt::ipnumber>,
    x_u: Vec<ipopt::ipnumber>,
    g_l: Vec<ipopt::ipnumber>,
    g_u: Vec<ipopt::ipnumber>
}

fn main() {

    let config_file: String = fs::read_to_string("config.toml").expect("Unable to read configuration file");

    let mut config_data: Config = toml::from_str(&config_file).unwrap();
    
    // Set the number of decision variables
    assert!(config_data.ipopt.x_l.len() == config_data.ipopt.x_u.len());
    let n: usize = config_data.ipopt.x_l.len();

    // Set the number of constraints
    assert!(config_data.ipopt.g_l.len() == config_data.ipopt.g_u.len());
    let m: usize = config_data.ipopt.g_l.len();

    // Set the number of nonzeroes in the Jacobian and Hessisan
    let num_elem_jac: i32 = 8;
    let num_elem_hess: i32 = 10;

    // Create stack-allocated arrays to store IPOPT obtained values
    let mut obj: [ipopt::ipnumber; 1] = [0.0];
    
    // Create heap-allocated arrays to store IPOPT obtained values
    let mut mult_g: Vec<ipopt::ipnumber> = vec![0.0; config_data.ipopt.g_l.len()];
    let mut mult_x_l: Vec<ipopt::ipnumber> = vec![0.0; config_data.ipopt.x_l.len()];
    let mut mult_x_u: Vec<ipopt::ipnumber> = vec![0.0; config_data.ipopt.x_u.len()];
    
    // Verify that all stack and heap-allocated arrays have the correct dimensions
    assert!(config_data.ipopt.x.len() == n);
    assert!(obj.len() == 1);
    assert!(mult_g.len() == m);
    assert!(mult_x_l.len() == n);
    assert!(mult_x_u.len() == n);

    // Create opaque C-style struct that will be used to store IPOPT information
    let mut user_data: ipopt::IpoptProblemInfo = ipopt::IpoptProblemInfo::default();

    unsafe {
        let nlp: ipopt::IpoptProblem = ipopt::CreateIpoptProblem(
            n as ipopt::ipindex, 
            config_data.ipopt.x_l.as_mut_ptr(), 
            config_data.ipopt.x_u.as_mut_ptr(), 
            m as ipopt::ipindex, 
            config_data.ipopt.g_l.as_mut_ptr(), 
            config_data.ipopt.g_u.as_mut_ptr(), 
            num_elem_jac as ipopt::ipindex, 
            num_elem_hess as ipopt::ipindex, 
            config_data.ipopt.index_style as ipopt::ipindex, 
            Some(ipopt::helper::eval_f), 
            Some(ipopt::helper::eval_g), 
            Some(ipopt::helper::eval_grad_f), 
            Some(ipopt::helper::eval_jac_g), 
            Some(ipopt::helper::eval_h)
        );

        ipopt::AddIpoptNumOption(nlp, c"tol".as_ptr() as *const i8, config_data.ipopt.tol);
        ipopt::AddIpoptStrOption(nlp, c"mu_strategy".as_ptr() as *const i8, config_data.ipopt.mu_strategy.as_ptr() as *const i8);
        ipopt::AddIpoptStrOption(nlp, c"output_file".as_ptr() as *const i8, config_data.ipopt.output_file.as_ptr() as *const i8);      
        ipopt::AddIpoptStrOption(nlp, c"linear_solver".as_ptr() as *const i8, config_data.ipopt.linear_solver.as_ptr() as *const i8);                            

        let status: i32 = ipopt::IpoptSolve(
            nlp, 
            config_data.ipopt.x.as_mut_ptr(), 
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
