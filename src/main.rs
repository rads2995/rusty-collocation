mod bindings {
    pub mod ipopt;
}

use crate::bindings::ipopt::*;

#[unsafe(no_mangle)]
extern "C" fn Eval_F_CB_impl(
    n: ipopt::ipindex,
    x: *mut ipopt::ipnumber,
    new_x: bool,
    obj_value: *mut ipopt::ipnumber,
    user_data: ipopt::UserDataPtr,
) -> bool {
    true
}

#[unsafe(no_mangle)]
extern "C" fn Eval_G_CB_impl(
    n: ipopt::ipindex,
    x: *mut ipopt::ipnumber,
    new_x: bool,
    m: ipopt::ipindex,
    g: *mut ipopt::ipnumber,
    user_data: ipopt::UserDataPtr,
) -> bool {
    true
}

#[unsafe(no_mangle)]
extern "C" fn Eval_Grad_F_CB_impl(
    n: ipopt::ipindex,
    x: *mut ipopt::ipnumber,
    new_x: bool,
    grad_f: *mut ipopt::ipnumber,
    user_data: ipopt::UserDataPtr,
) -> bool {
    true
}

#[unsafe(no_mangle)]
extern "C" fn Eval_Jac_G_CB_impl(
    n: ipopt::ipindex,
    x: *mut ipopt::ipnumber,
    new_x: bool,
    m: ipopt::ipindex,
    nele_jac: ipopt::ipindex,
    iRow: *mut ipopt::ipindex,
    jCol: *mut ipopt::ipindex,
    values: *mut ipopt::ipnumber,
    user_data: ipopt::UserDataPtr,
) -> bool {
    true
}

#[unsafe(no_mangle)]
extern "C" fn Eval_H_CB_impl(
    n: ipopt::ipindex,
    x: *mut ipopt::ipnumber,
    new_x: bool,
    obj_factor: ipopt::ipnumber,
    m: ipopt::ipindex,
    lambda: *mut ipopt::ipnumber,
    new_lambda: bool,
    nele_hess: ipopt::ipindex,
    iRow: *mut ipopt::ipindex,
    jCol: *mut ipopt::ipindex,
    values: *mut ipopt::ipnumber,
    user_data: ipopt::UserDataPtr,
) -> bool {
    true
}

fn main() {
    let mut n: i32 = 1;
    let mut m: i32 = 1;
    let mut nele_jac: i32 = 1;
    let mut nele_hess: i32 = 1;
    let mut index_style: i32 = 0;

    let mut x_L: Vec<f64> = vec![0.0];  // Lower bounds on x
    let mut x_U: Vec<f64> = vec![0.0];  // Upper bounds on x
    let mut g_L: Vec<f64> = vec![0.0];  // Lower bounds on g
    let mut g_U: Vec<f64> = vec![0.0];  // Upper bounds on g

    let callback: ipopt::Eval_F_CB = Some(Eval_F_CB_impl);

    unsafe {
        ipopt::CreateIpoptProblem(
            n, 
            x_L.as_mut_ptr(), 
            x_U.as_mut_ptr(), 
            m, 
            g_L.as_mut_ptr(), 
            g_U.as_mut_ptr(), 
            nele_jac, 
            nele_hess, 
            index_style, 
            Some(Eval_F_CB_impl), 
            Some(Eval_G_CB_impl), 
            Some(Eval_Grad_F_CB_impl), 
            Some(Eval_Jac_G_CB_impl), 
            Some(Eval_H_CB_impl)
        );
    } 
}
