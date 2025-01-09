
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
extern "C" fn eval_grad_f(
    n: ipopt::ipindex,
    x: *mut ipopt::ipnumber,
    new_x: bool,
    grad_f: *mut ipopt::ipnumber,
    user_data: ipopt::UserDataPtr,
) -> bool {
    
    assert!(n == 4);
    assert!(!x.is_null());
    assert!(!grad_f.is_null());

    // Throw away unused variables
    let _ = new_x;
    let _ = user_data;
    
    let x_slice: &[ipopt::ipnumber] = unsafe {core::slice::from_raw_parts_mut(x, n.try_into().unwrap())};
    let grad_f_slice: &mut[ipopt::ipnumber] = unsafe {core::slice::from_raw_parts_mut(grad_f, n.try_into().unwrap())};

    grad_f_slice[0] = x_slice[0] * x_slice[3] + x_slice[3] * (x_slice[0] + x_slice[1] + x_slice[2]);
    grad_f_slice[1] = x_slice[0] * x_slice[3];
    grad_f_slice[2] = x_slice[0] * x_slice[3] + 1.0;
    grad_f_slice[3] = x_slice[0] * (x_slice[0] + x_slice[1] + x_slice[2]);  

    true
}

#[unsafe(no_mangle)]
extern "C" fn eval_jac_g(
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
    
    assert!(n == 4);
    assert!(m == 2);
    assert!(!x.is_null());
    assert!(!iRow.is_null());
    assert!(!jCol.is_null());
    
    // Throw away unused variables
    let _ = user_data;
    let _ = new_x;

    if values.is_null() {

        let iRow_slice: &mut[ipopt::ipindex] = unsafe {core::slice::from_raw_parts_mut(iRow, nele_jac.try_into().unwrap())};
        let jCol_slice: &mut[ipopt::ipindex] = unsafe {core::slice::from_raw_parts_mut(jCol, nele_jac.try_into().unwrap())};

        iRow_slice[0] = 0;
        jCol_slice[0] = 0;
        iRow_slice[1] = 0;
        jCol_slice[1] = 1;
        iRow_slice[2] = 0;
        jCol_slice[2] = 2;
        iRow_slice[3] = 0;
        jCol_slice[3] = 3;
        iRow_slice[4] = 1;
        jCol_slice[4] = 0;
        iRow_slice[5] = 1;
        jCol_slice[5] = 1;
        iRow_slice[6] = 1;
        jCol_slice[6] = 2;
        iRow_slice[7] = 1;
        jCol_slice[7] = 3; 

    } 

    else {

        let x_slice: &[ipopt::ipnumber] = unsafe {core::slice::from_raw_parts_mut(x, n.try_into().unwrap())};
        let values_slice: &mut[ipopt::ipnumber] = unsafe {core::slice::from_raw_parts_mut(values, nele_jac.try_into().unwrap())};
   
        // Return the values of the jacobian of the constraints

        values_slice[0] = x_slice[1] * x_slice[2] * x_slice[3];
        values_slice[1] = x_slice[0] * x_slice[2] * x_slice[3];
        values_slice[2] = x_slice[0] * x_slice[1] * x_slice[3];
        values_slice[3] = x_slice[0] * x_slice[1] * x_slice[2];
        values_slice[4] = 2.0 * x_slice[0];
        values_slice[5] = 2.0 * x_slice[1];
        values_slice[6] = 2.0 * x_slice[2];
        values_slice[7] = 2.0 * x_slice[3];

    }

    true
}

#[unsafe(no_mangle)]
extern "C" fn eval_h(
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
    
    assert!(n == 4);
    assert!(m == 2);
    assert!(!x.is_null());
    assert!(!lambda.is_null());
    assert!(!iRow.is_null());
    assert!(!jCol.is_null());
    
    if values.is_null() {

        let iRow_slice: &mut[ipopt::ipindex] = unsafe {core::slice::from_raw_parts_mut(iRow, nele_hess.try_into().unwrap())};
        let jCol_slice: &mut[ipopt::ipindex] = unsafe {core::slice::from_raw_parts_mut(jCol, nele_hess.try_into().unwrap())};

        iRow_slice[0] = 0;
        jCol_slice[0] = 0;
        iRow_slice[1] = 1;
        jCol_slice[1] = 0;
        iRow_slice[2] = 1;
        jCol_slice[2] = 1;
        iRow_slice[3] = 2;
        jCol_slice[3] = 0;
        iRow_slice[4] = 2;
        jCol_slice[4] = 1;
        iRow_slice[5] = 2;
        jCol_slice[5] = 2;
        iRow_slice[6] = 3;
        jCol_slice[6] = 0;
        iRow_slice[7] = 3;
        jCol_slice[7] = 1; 
        iRow_slice[8] = 3;
        jCol_slice[8] = 2; 
        iRow_slice[9] = 3;
        jCol_slice[9] = 3;

    }

    else {

        let x_slice: &[ipopt::ipnumber] = unsafe {core::slice::from_raw_parts_mut(x, n.try_into().unwrap())};
        let values_slice: &mut[ipopt::ipnumber] = unsafe {core::slice::from_raw_parts_mut(values, nele_hess.try_into().unwrap())};
        let lambda_slice: &mut[ipopt::ipnumber] = unsafe {core::slice::from_raw_parts_mut(lambda, m.try_into().unwrap())};


      /* return the values. This is a symmetric matrix, fill the lower left
       * triangle only */

      /* fill the objective portion */
      values_slice[0] = obj_factor * (2.0 * x_slice[3]);
      
      values_slice[1] = obj_factor * (x_slice[3]);
      values_slice[2] = 0.0;

      values_slice[3] = obj_factor * (x_slice[3]);
      values_slice[4] = 0.0;
      values_slice[5] = 0.0;

      values_slice[6] = obj_factor * (2.0 * x_slice[0] + x_slice[1] + x_slice[2]);
      values_slice[7] = obj_factor * (x_slice[0]);
      values_slice[8] = obj_factor * (x_slice[0]);
      values_slice[9] = 0.0;

      /* add the portion for the first constraint */
      values_slice[1] += lambda_slice[0] * (x_slice[2] * x_slice[3]);

      values_slice[3] += lambda_slice[0] * (x_slice[1] * x_slice[3]);
      values_slice[4] += lambda_slice[0] * (x_slice[0] * x_slice[3]);

      values_slice[6] += lambda_slice[0] * (x_slice[1] * x_slice[2]);
      values_slice[7] += lambda_slice[0] * (x_slice[0] * x_slice[2]);
      values_slice[8] += lambda_slice[0] * (x_slice[0] * x_slice[1]);

      /* add the portion for the second constraint */
      values_slice[0] += lambda_slice[1] * 2.0; 

      values_slice[2] += lambda_slice[1] * 2.0; 

      values_slice[5] += lambda_slice[1] * 2.0; 

      values_slice[9] += lambda_slice[1] * 2.0; 

    }

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

    let mut obj: f64 = 0.0;

    // Set the initial point and the values
    let mut x: [f64; 4] = [1.0, 5.0, 5.0, 1.0];

    let tol_msg: std::ffi::CString = std::ffi::CString::new("tol").unwrap();
        
    let mu_strategy_msg: std::ffi::CString = std::ffi::CString::new("mu_strategy").unwrap();
    let mu_strategy_val: std::ffi::CString = std::ffi::CString::new("adaptive").unwrap();
    
    let output_file_msg: std::ffi::CString = std::ffi::CString::new("output_file").unwrap();
    let output_file_val: std::ffi::CString = std::ffi::CString::new("ipopt.out").unwrap();
    
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
            Some(eval_grad_f), 
            Some(eval_jac_g), 
            Some(eval_h)
        );

        ipopt::AddIpoptNumOption(nlp, tol_msg.into_raw(), 3.82e-6);
        ipopt::AddIpoptStrOption(nlp, mu_strategy_msg.into_raw(), mu_strategy_val.into_raw());
        ipopt::AddIpoptStrOption(nlp, output_file_msg.into_raw(), output_file_val.into_raw());
    
    //     ipopt::IpoptSolve(nlp, 
    //         x.as_mut_ptr(), 
    //         core::ptr::null_mut(),
    //         obj as *mut f64, 
    //         mult_g, 
    //         mult_x_L, 
    //         mult_x_U, 
    //         user_data);
    
    } 
}
