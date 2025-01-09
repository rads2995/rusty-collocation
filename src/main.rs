
mod bindings {
    pub mod ipopt;
}
use crate::bindings::ipopt::*;

#[unsafe(no_mangle)]
extern "C" fn eval_f(
    n: ipopt::ipindex,
    x: *mut ipopt::ipnumber,
    new_x: bool,
    obj_value: *mut ipopt::ipnumber,
    user_data: ipopt::UserDataPtr,
) -> bool {
    
    assert!(n == 4);
    assert!(!x.is_null());
    assert!(!obj_value.is_null());
    
    // Throw away unused variables
    let _ = n;
    let _ = new_x;
    let _ = user_data;

    let x_slice: &[ipopt::ipnumber] = unsafe {core::slice::from_raw_parts_mut(x, n.try_into().unwrap())};

    unsafe {
        *obj_value = x_slice[0] * x_slice[3];
    }

    true
}

#[unsafe(no_mangle)]
extern "C" fn eval_g(
    n: ipopt::ipindex,
    x: *mut ipopt::ipnumber,
    new_x: bool,
    m: ipopt::ipindex,
    g: *mut ipopt::ipnumber,
    user_data: ipopt::UserDataPtr,
) -> bool {
    
    assert!(n == 4);
    assert!(m == 2);
    assert!(!x.is_null());
    assert!(!g.is_null());

    // Throw away unused variables
    let _ = n;
    let _ = m;
    let _ = new_x;
    let _ = user_data;
    
    let x_slice: &[ipopt::ipnumber] = unsafe {core::slice::from_raw_parts_mut(x, n.try_into().unwrap())};
    let g_slice: &mut[ipopt::ipnumber] = unsafe {core::slice::from_raw_parts_mut(g, m.try_into().unwrap())};

    g_slice[0] = x_slice[0] * x_slice[1] * x_slice[2] * x_slice[3]; 
    g_slice[1] = x_slice[0] * x_slice[0] + x_slice[1] * x_slice[1] + x_slice[2] * x_slice[2] + x_slice[3] * x_slice[3];

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
            Some(eval_f), 
            Some(eval_g), 
            Some(Eval_Grad_F_CB_impl), 
            Some(Eval_Jac_G_CB_impl), 
            Some(Eval_H_CB_impl)
        );
    } 
}
