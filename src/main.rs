pub(crate) mod nlp;
mod bindings {
    pub(crate) mod blas;
    pub(crate) mod ipopt;
    pub(crate) mod lua;
}

use bindings::ipopt::helper::{eval_f, eval_g, eval_grad_f, eval_h, eval_jac_g};
use bindings::ipopt::{
    ipindex, ipnumber, AddIpoptNumOption, AddIpoptStrOption, ApplicationReturnStatus,
    CreateIpoptProblem, FreeIpoptProblem, IpoptProblem, IpoptProblemInfo, IpoptReturnStatus,
    IpoptSolve,
};

fn main() {
    let mut x_l: Vec<f64> = std::vec!(1.0, 1.0, 1.0, 1.0);
    let mut x_u: Vec<f64> = std::vec!(5.0, 5.0, 5.0, 5.0);
    let mut g_l: Vec<f64> = std::vec!(25.0, 40.0);
    let mut g_u: Vec<f64> = std::vec!(2e19, 40.0);
    let mut x: Vec<f64> = std::vec!(1.0, 5.0, 5.0, 1.0);

    // Set the number of decision variables
    assert!(x_l.len() == x_u.len());
    let n: usize = x_l.len();

    // Set the number of constraints
    assert!(g_l.len() == g_u.len());
    let m: usize = g_l.len();

    // Set the number of nonzeroes in the Jacobian and Hessisan
    let num_elem_jac: i32 = 8;
    let num_elem_hess: i32 = 10;

    // Create stack-allocated arrays to store IPOPT obtained values
    let mut obj: [ipnumber; 1] = [0.0];

    // Create heap-allocated arrays to store IPOPT obtained values
    let mut mult_g: Vec<ipnumber> = vec![0.0; g_l.len()];
    let mut mult_x_l: Vec<ipnumber> = vec![0.0; x_l.len()];
    let mut mult_x_u: Vec<ipnumber> = vec![0.0; x_u.len()];

    // Verify that all stack and heap-allocated arrays have the correct dimensions
    assert!(x.len() == n);
    assert!(obj.len() == 1);
    assert!(mult_g.len() == m);
    assert!(mult_x_l.len() == n);
    assert!(mult_x_u.len() == n);

    // Create opaque C-style struct that will be used to store IPOPT information
    let mut user_data: IpoptProblemInfo = IpoptProblemInfo::default();

    unsafe {
        let nlp: IpoptProblem = CreateIpoptProblem(
            n as ipindex,
            x_l.as_mut_ptr(),
            x_u.as_mut_ptr(),
            m as ipindex,
            g_l.as_mut_ptr(),
            g_u.as_mut_ptr(),
            num_elem_jac as ipindex,
            num_elem_hess as ipindex,
            0 as ipindex,
            Some(eval_f),
            Some(eval_g),
            Some(eval_grad_f),
            Some(eval_jac_g),
            Some(eval_h),
        );

        AddIpoptNumOption(nlp, c"tol".as_ptr(), 3.82e-6);
        AddIpoptStrOption(nlp, c"mu_strategy".as_ptr(), c"adaptive".as_ptr());
        AddIpoptStrOption(nlp, c"linear_solver".as_ptr(), c"mumps".as_ptr());

        let status: ApplicationReturnStatus = IpoptSolve(
            nlp,
            x.as_mut_ptr(),
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

        // Lua shenanigans
        let lua_state: *mut bindings::lua::lua_State = bindings::lua::luaL_newstate();

        if lua_state.is_null() {
            panic!("Failed to create Lua state!");
        }

        bindings::lua::luaL_openlibs(lua_state);

        let script: std::ffi::CString = std::ffi::CString::new("return 42").unwrap();
        bindings::lua::luaL_loadstring(lua_state, script.as_ptr());

        bindings::lua::lua_pcallk(lua_state, 0, 1, 0, 0, None);

        let mut len: usize = 0;

        let str_ptr: *const i8 = bindings::lua::luaL_tolstring(lua_state, -1, &mut len);

        let c_str: &std::ffi::CStr = std::ffi::CStr::from_ptr(str_ptr);

        let rust_str: String = c_str.to_string_lossy().into_owned();

        dbg!(rust_str);

        bindings::lua::lua_settop(lua_state, 1);

        bindings::lua::lua_close(lua_state);
    }
}
