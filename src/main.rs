pub(crate) mod nlp;
mod bindings {
    pub(crate) mod blas;
    pub(crate) mod ipopt;
    pub(crate) mod lua;
}
mod math {
    pub(crate) mod gaussian_quadrature;
}

use bindings::ipopt::helper::{eval_f, eval_g, eval_grad_f, eval_h, eval_jac_g};
use bindings::ipopt::{
    ipindex, ipnumber, AddIpoptNumOption, AddIpoptStrOption, ApplicationReturnStatus,
    CreateIpoptProblem, FreeIpoptProblem, IpoptProblem, IpoptProblemInfo, IpoptReturnStatus,
    IpoptSolve,
};

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Config {
    ipopt: Ipopt,
}

#[derive(Deserialize)]
struct Ipopt {
    tol: ipnumber,
    mu_strategy: std::ffi::CString,
    output_file: std::ffi::CString,
    linear_solver: std::ffi::CString,
    index_style: u32,
    x: Vec<ipnumber>,
    x_l: Vec<ipnumber>,
    x_u: Vec<ipnumber>,
    g_l: Vec<ipnumber>,
    g_u: Vec<ipnumber>,
}

fn main() {
    let config_file: String =
        fs::read_to_string("Settings.toml").expect("Unable to read configuration file");

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
    let mut obj: [ipnumber; 1] = [0.0];

    // Create heap-allocated arrays to store IPOPT obtained values
    let mut mult_g: Vec<ipnumber> = vec![0.0; config_data.ipopt.g_l.len()];
    let mut mult_x_l: Vec<ipnumber> = vec![0.0; config_data.ipopt.x_l.len()];
    let mut mult_x_u: Vec<ipnumber> = vec![0.0; config_data.ipopt.x_u.len()];

    // Verify that all stack and heap-allocated arrays have the correct dimensions
    assert!(config_data.ipopt.x.len() == n);
    assert!(obj.len() == 1);
    assert!(mult_g.len() == m);
    assert!(mult_x_l.len() == n);
    assert!(mult_x_u.len() == n);

    // Create opaque C-style struct that will be used to store IPOPT information
    let mut user_data: IpoptProblemInfo = IpoptProblemInfo::default();

    unsafe {
        let nlp: IpoptProblem = CreateIpoptProblem(
            n as ipindex,
            config_data.ipopt.x_l.as_mut_ptr(),
            config_data.ipopt.x_u.as_mut_ptr(),
            m as ipindex,
            config_data.ipopt.g_l.as_mut_ptr(),
            config_data.ipopt.g_u.as_mut_ptr(),
            num_elem_jac as ipindex,
            num_elem_hess as ipindex,
            config_data.ipopt.index_style as ipindex,
            Some(eval_f),
            Some(eval_g),
            Some(eval_grad_f),
            Some(eval_jac_g),
            Some(eval_h),
        );

        AddIpoptNumOption(nlp, c"tol".as_ptr(), config_data.ipopt.tol);
        AddIpoptStrOption(
            nlp,
            c"mu_strategy".as_ptr(),
            config_data.ipopt.mu_strategy.as_ptr(),
        );
        AddIpoptStrOption(
            nlp,
            c"output_file".as_ptr(),
            config_data.ipopt.output_file.as_ptr(),
        );
        AddIpoptStrOption(
            nlp,
            c"linear_solver".as_ptr(),
            config_data.ipopt.linear_solver.as_ptr(),
        );

        let status: ApplicationReturnStatus = IpoptSolve(
            nlp,
            config_data.ipopt.x.as_mut_ptr(),
            core::ptr::null_mut(),
            obj.as_mut_ptr(),
            mult_g.as_mut_ptr(),
            mult_x_l.as_mut_ptr(),
            mult_x_u.as_mut_ptr(),
            &mut user_data as *mut IpoptProblemInfo,
        );

        match IpoptReturnStatus::try_from(status) {
            Ok(enum_value) => match enum_value {
                IpoptReturnStatus::SolveSucceeded => {
                    println!("Solve succeded!");
                }

                _ => println!("Return code unkown"),
            },

            Err(_) => {
                println!("Unknown return status from IPOPT: {}", status)
            }
        }

        FreeIpoptProblem(nlp);
    }
}
