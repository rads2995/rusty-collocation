#![allow(non_camel_case_types, non_snake_case, unused)]

/// Type of all indices of vectors, matrices, etc.
///
/// Requires IPOPT version >= 3.14.0.
pub(crate) type ipindex = core::ffi::c_int;

/// Type for floating-point numbers.
///
/// Requires IPOPT version >= 3.14.0.
pub(crate) type ipnumber = core::ffi::c_double;

/// Opaque structure collecting all information about the problem definition and solve statistics, etc.
#[repr(C)]
pub(crate) struct IpoptProblemInfo {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl IpoptProblemInfo {
    pub(crate) fn default() -> Self {
        IpoptProblemInfo {
            _data: [],
            _marker: core::marker::PhantomData,
        }
    }
}

/// Pointer to an Ipopt Problem.
pub(crate) type IpoptProblem = *mut IpoptProblemInfo;

/// A pointer for anything that is to be passed between the called and individual callback function.
pub(crate) type UserDataPtr = *mut IpoptProblemInfo;

/// Return codes for the Optimize call for an application.
pub(crate) type ApplicationReturnStatus = core::ffi::c_int;

/// Safe wrapper for IPOPT return codes for the Optimize call for an application.
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum IpoptReturnStatus {
    SolveSucceeded = 0,
    SolvedToAcceptableLevel = 1,
    InfeasibleProblemDetected = 2,
    SearchDirectionBecomesTooSmall = 3,
    DivergingIterates = 4,
    UserRequestedStop = 5,
    FeasiblePointFound = 6,
    MaximumIterationsExceeded = -1,
    RestorationFailed = -2,
    ErrorInStepComputation = -3,
    MaximumCpuTimeExceeded = -4,
    MaximumWallTimeExceeded = -5,
    NotEnoughDegreesOfFreedom = -10,
    InvalidProblemDefinition = -11,
    InvalidOption = -12,
    InvalidNumberDetected = -13,
    UnrecoverableException = -100,
    NonIpoptExceptionThrown = -101,
    InsufficientMemory = -102,
    InternalError = -199,
}

impl core::convert::TryFrom<ApplicationReturnStatus> for IpoptReturnStatus {
    type Error = ();

    fn try_from(code: ApplicationReturnStatus) -> Result<Self, Self::Error> {
        match code {
            0 => Ok(Self::SolveSucceeded),
            1 => Ok(Self::SolvedToAcceptableLevel),
            2 => Ok(Self::InfeasibleProblemDetected),
            3 => Ok(Self::SearchDirectionBecomesTooSmall),
            4 => Ok(Self::DivergingIterates),
            5 => Ok(Self::UserRequestedStop),
            6 => Ok(Self::FeasiblePointFound),
            -1 => Ok(Self::MaximumIterationsExceeded),
            -2 => Ok(Self::RestorationFailed),
            -3 => Ok(Self::ErrorInStepComputation),
            -4 => Ok(Self::MaximumCpuTimeExceeded),
            -5 => Ok(Self::MaximumWallTimeExceeded),
            -10 => Ok(Self::NotEnoughDegreesOfFreedom),
            -11 => Ok(Self::InvalidProblemDefinition),
            -12 => Ok(Self::InvalidOption),
            -13 => Ok(Self::InvalidNumberDetected),
            -100 => Ok(Self::UnrecoverableException),
            -101 => Ok(Self::NonIpoptExceptionThrown),
            -102 => Ok(Self::InsufficientMemory),
            -199 => Ok(Self::InternalError),
            _ => Err(()),
        }
    }
}

/// Type defining the callback function for evaluating the value of the objective function.
///
/// Return value should be set to false if there was a problem doing the evaluation.
///
/// See also `Ipopt::TNLP::eval_f`.
type Eval_F_CB = core::option::Option<
    unsafe extern "C" fn(
        n: ipindex,
        x: *mut ipnumber,
        new_x: bool,
        obj_value: *mut ipnumber,
        user_data: UserDataPtr,
    ) -> bool,
>;

/// Type defining the callback function for evaluating the gradient of the objective function.
///
/// Return value should be set to false if there was a problem doing the evaluation.
///
/// See also `Ipopt::TNLP::eval_grad_f`.
type Eval_Grad_F_CB = core::option::Option<
    unsafe extern "C" fn(
        n: ipindex,
        x: *mut ipnumber,
        new_x: bool,
        grad_f: *mut ipnumber,
        user_data: UserDataPtr,
    ) -> bool,
>;

/// Type defining the callback function for evaluating the value of the constraint functions.
///
/// Return value should be set to false if there was a problem doing the evaluation.
///
/// See also `Ipopt::TNLP::eval_g`.
type Eval_G_CB = core::option::Option<
    unsafe extern "C" fn(
        n: ipindex,
        x: *mut ipnumber,
        new_x: bool,
        m: ipindex,
        g: *mut ipnumber,
        user_data: UserDataPtr,
    ) -> bool,
>;

/// Type defining the callback function for evaluating the Jacobian of the constrant functions.
///
/// Return value should be set to false if there was a problem doing the evaluation.
///
/// See also `Ipopt::TNLP::eval_jac_g`.
type Eval_Jac_G_CB = core::option::Option<
    unsafe extern "C" fn(
        n: ipindex,
        x: *mut ipnumber,
        new_x: bool,
        m: ipindex,
        nele_jac: ipindex,
        iRow: *mut ipindex,
        jCol: *mut ipindex,
        values: *mut ipnumber,
        user_data: UserDataPtr,
    ) -> bool,
>;

/// Type defining the callback function for evaluating the Hessian of the Lagrangian function.
///
/// Return value should be set to false if there was a problem doing the evaluation.
///
/// See also `Ipopt::TNLP::eval_h`.
type Eval_H_CB = core::option::Option<
    unsafe extern "C" fn(
        n: ipindex,
        x: *mut ipnumber,
        new_x: bool,
        obj_factor: ipnumber,
        m: ipindex,
        lambda: *mut ipnumber,
        new_lambda: bool,
        nele_hess: ipindex,
        iRow: *mut ipindex,
        jCol: *mut ipindex,
        values: *mut ipnumber,
        user_data: UserDataPtr,
    ) -> bool,
>;

/// Type defining the callback function for giving intermediate execution control to the user.
///
/// If set, it is called once per iteration, providing the user\
/// with some information on the state of the optimization.\
/// This can be used to print some user-defined output.\
/// It also gives the user a way to terminate the optimization prematurely.\
/// If this method returns false, Ipopt will terminate the optimization.
///
/// See also `Ipopt::TNLP::intermediate_callback`.
type Intermediate_CB = core::option::Option<
    unsafe extern "C" fn(
        alg_mod: ipindex,
        iter_count: ipindex,
        obj_value: ipnumber,
        inf_pr: ipnumber,
        inf_du: ipnumber,
        mu: ipnumber,
        d_norm: ipnumber,
        regularization_size: ipnumber,
        alpha_du: ipnumber,
        alpha_pr: ipnumber,
        ls_trials: ipindex,
        user_data: UserDataPtr,
    ) -> bool,
>;

// Here, we assume that we are linking to IPOPT as a dynamic-library.
#[link(name = "ipopt", kind = "dylib")]
unsafe extern "C" {

    /// Function for creating a new Ipopt Problem object.
    ///
    /// This function returns an object that can be passed to the IpoptSolve call.\
    /// It contains the basic definition of the optimization problem, such\
    /// as number of variables and constraints, bounds on variables and\
    /// constraints, information about the derivatives, and the callback\
    /// function for the computation of the optimization problem\
    /// functions and derivatives. During this call, the options file\
    /// `PARAMS.DAT` is read as well.
    ///
    /// If `NULL` is returned, there was a problem with one of the inputs\
    /// or reading the options file.
    ///
    /// See also `Ipopt::TNLP::get_nlp_info` and `Ipopt::TNLP::get_bounds_info`.
    pub(crate) unsafe fn CreateIpoptProblem(
        n: ipindex,
        x_L: *mut ipnumber,
        x_U: *mut ipnumber,
        m: ipindex,
        g_L: *mut ipnumber,
        g_U: *mut ipnumber,
        nele_jac: ipindex,
        nele_hess: ipindex,
        index_style: ipindex,
        eval_f: Eval_F_CB,
        eval_g: Eval_G_CB,
        eval_grad_f: Eval_Grad_F_CB,
        eval_jac_g: Eval_Jac_G_CB,
        eval_h: Eval_H_CB,
    ) -> IpoptProblem;

    /// Method for freeing a previously created IpoptProblem.
    ///
    /// After freeing an IpoptProblem, it cannot be used anymore.
    pub(crate) unsafe fn FreeIpoptProblem(ipopt_problem: IpoptProblem);

    /// Function for adding a string option.
    ///
    /// Return false, if the option could not be set (e.g., if keyword is unknown).
    pub(crate) unsafe fn AddIpoptStrOption(
        ipopt_problem: IpoptProblem,
        keyword: *const core::ffi::c_char,
        val: *const core::ffi::c_char,
    ) -> bool;

    /// Function for adding a Number option.
    ///
    /// Return false, if the option could not be set (e.g., if keyword is unknown).
    pub(crate) unsafe fn AddIpoptNumOption(
        ipopt_problem: IpoptProblem,
        keyword: *const core::ffi::c_char,
        val: ipnumber,
    ) -> bool;

    /// Function for adding an Integer option.
    ///
    /// Return false, if the option could not be set (e.g., if keyword is unknown).
    pub(crate) unsafe fn AddIpoptIntOption(
        ipopt_problem: IpoptProblem,
        keyword: *const core::ffi::c_char,
        val: ipindex,
    ) -> bool;

    /// Function for opening an output file for a given name with given printlevel.
    ///
    /// Return false, if there was a problem opening the file.
    pub(crate) unsafe fn OpenIpoptOutputFile(
        ipopt_problem: IpoptProblem,
        file_name: *const core::ffi::c_char,
        print_level: core::ffi::c_int,
    ) -> bool;

    /// Optional function for setting scaling parameter for the NLP.
    ///
    /// This corresponds to the `TNLP::get_scaling_parameters` method.\
    /// If the pointers `x_scaling` or `g_scaling` are `NULL`, then no scaling\
    /// for x resp. g is done.
    pub(crate) unsafe fn SetIpoptProblemScaling(
        ipopt_problem: IpoptProblem,
        obj_scaling: ipnumber,
        x_scaling: *mut ipnumber,
        g_scaling: *mut ipnumber,
    ) -> bool;

    /// Setting a callback function for the "intermediate callback" method in the TNLP.
    ///
    /// This gives control back to the user once\
    /// per iteration. If set, it provides the user with some\
    /// information on the state of the optimization. This can be used\
    /// to print some user-defined output. It also gives the user a way\
    /// to terminate the optimization prematurely. If the callback\
    /// method returns false, Ipopt will terminate the optimization.\
    /// Calling this set method to set the CB pointer to `NULL` disables\
    /// the intermediate callback functionality.
    pub(crate) unsafe fn SetIntermediateCallback(
        ipopt_problem: IpoptProblem,
        intermediate_cb: Intermediate_CB,
    ) -> bool;

    /// Function calling the Ipopt optimization algorithm for a problem\
    /// previously defined with `CreateIpoptProblem`.
    ///
    /// Return outcome of the optimization procedure (e.g., success, failure, etc.).
    pub(crate) unsafe fn IpoptSolve(
        ipopt_problem: IpoptProblem,
        x: *mut ipnumber,
        g: *mut ipnumber,
        obj_val: *mut ipnumber,
        mult_g: *mut ipnumber,
        mult_x_L: *mut ipnumber,
        mult_x_U: *mut ipnumber,
        user_data: UserDataPtr,
    ) -> ApplicationReturnStatus;

    /// Get primal and dual variable values of the current iterate.
    ///
    /// This method can be used to get the values of the current iterate during the intermediate callback set by `SetIntermediateCallback()`.\
    /// The method expects the number of variables (dimension of x), number of constraints (dimension of g(x)),\
    /// and allocated arrays of appropriate lengths as input.
    ///
    /// The method translates the x(), c(), d(), y_c(), y_d(), z_L(), and z_U() vectors from `Ipopt::IpoptData::curr()`\
    /// of the internal NLP representation into the form used by the TNLP.\
    /// For the correspondence between scaled and unscaled solutions, see the detailed description of `Ipopt::OrigIpoptNLP`.\
    /// If IPOPT is in restoration mode, it maps the current iterate of restoration NLP (see `Ipopt::RestoIpoptNLP`) back to the original TNLP.
    ///
    /// If there are fixed variables and `fixed_variable_treatment=make_parameter`, then requesting z_L and z_U can trigger a reevaluation of\
    /// the Gradient of the objective function and the Jacobian of the constraint functions.
    pub(crate) unsafe fn GetIpoptCurrentIterate(
        ipopt_problem: IpoptProblem,
        scaled: bool,
        n: ipindex,
        x: *mut ipnumber,
        z_L: *mut ipnumber,
        z_U: *mut ipnumber,
        m: ipindex,
        g: *mut ipnumber,
        lambda: *mut ipnumber,
    ) -> bool;

    /// Get primal and dual infeasibility of the current iterate.
    ///
    /// This method can be used to get the violations of constraints and optimality conditions\
    /// at the current iterate during the intermediate callback set by `SetIntermediateCallback()`.\
    /// The method expects the number of variables (dimension of x), number of constraints (dimension of g(x)),\
    /// and allocated arrays of appropriate lengths as input.
    ///
    /// The method makes the vectors behind `(unscaled_)curr_orig_bounds_violation()`, `(unscaled_)curr_nlp_constraint_violation()`, `(unscaled_)curr_dual_infeasibility()`,\
    /// `(unscaled_)curr_complementarity()` from `Ipopt::IpoptCalculatedQuantities` of the internal NLP representation available into the form used by the TNLP.\
    /// If IPOPT is in restoration mode, it maps the current iterate of restoration NLP (see `Ipopt::RestoIpoptNLP`) back to the original TNLP.
    ///
    /// **Note:** If in restoration phase, then requesting `grad_lag_x` can trigger a call to `Eval_F_CB`.
    ///
    /// **Note:** Ipopt by default relaxes variable bounds (option `bound_relax_factor` > 0.0).\
    /// `x_L_violation` and `x_U_violation` report the violation of a solution w.r.t. the original unrelaxed bounds.\
    /// However, `compl_x_L` and `compl_x_U` use the relaxed variable bounds to calculate the complementarity.
    pub(crate) unsafe fn GetIpoptCurrentViolations(
        ipopt_problem: IpoptProblem,
        scaled: bool,
        n: ipindex,
        x_L_violation: *mut ipnumber,
        x_U_violation: *mut ipnumber,
        compl_x_L: *mut ipnumber,
        compl_x_U: *mut ipnumber,
        grad_lag_x: *mut ipnumber,
        m: ipindex,
        nlp_constraint_violation: *mut ipnumber,
        compl_g: *mut ipnumber,
    ) -> bool;
}

pub(crate) mod helper {

    use super::{ipindex, ipnumber, UserDataPtr};

    use crate::nlp::ipopt::{
        constraint_functions, gradient_objective_function, hessian_elements,
        hessian_lagrangian_function, jacobian_constraint_elements, jacobian_constraint_function,
        objective_function,
    };

    /// Implementation of callback function for evaluating the value of the objective function.
    ///
    /// This function converts the objective function definition into something that IPOPT understands.
    #[unsafe(no_mangle)]
    pub(crate) extern "C" fn eval_f(
        n: ipindex,
        x: *mut ipnumber,
        new_x: bool,
        obj_value: *mut ipnumber,
        user_data: UserDataPtr,
    ) -> bool {
        assert!(!x.is_null());
        assert!(!obj_value.is_null());

        // These variables are not yet used
        let _ = new_x;
        let _ = user_data;

        let x_slice: &[ipnumber] =
            unsafe { core::slice::from_raw_parts_mut(x, n.try_into().unwrap()) };

        assert!(x_slice.len() == n.try_into().unwrap());

        let objective_value: ipnumber = objective_function(x_slice);

        unsafe {
            *obj_value = objective_value;
        }

        true
    }

    /// Implementation of callback function for evaluating the value of the constraint functions.
    ///
    /// This function converts the constraint functions definitions into something that IPOPT understands.
    #[unsafe(no_mangle)]
    pub(crate) extern "C" fn eval_g(
        n: ipindex,
        x: *mut ipnumber,
        new_x: bool,
        m: ipindex,
        g: *mut ipnumber,
        user_data: UserDataPtr,
    ) -> bool {
        assert!(!x.is_null());
        assert!(!g.is_null());

        // These variables are not yet used
        let _ = n;
        let _ = m;
        let _ = new_x;
        let _ = user_data;

        let x_slice: &[ipnumber] =
            unsafe { core::slice::from_raw_parts_mut(x, n.try_into().unwrap()) };
        let g_slice: &mut [ipnumber] =
            unsafe { core::slice::from_raw_parts_mut(g, m.try_into().unwrap()) };

        assert!(x_slice.len() == n.try_into().unwrap());
        assert!(g_slice.len() == m.try_into().unwrap());

        constraint_functions(x_slice, g_slice);

        true
    }

    /// Implementation of callback function for evaluating the gradient of the objective function.
    ///
    /// This function converts the gradient of the objective function definition into something that IPOPT understands.
    #[unsafe(no_mangle)]
    pub(crate) extern "C" fn eval_grad_f(
        n: ipindex,
        x: *mut ipnumber,
        new_x: bool,
        grad_f: *mut ipnumber,
        user_data: UserDataPtr,
    ) -> bool {
        assert!(!x.is_null());
        assert!(!grad_f.is_null());

        // These variables are not yet used
        let _ = new_x;
        let _ = user_data;

        let x_slice: &[ipnumber] =
            unsafe { core::slice::from_raw_parts_mut(x, n.try_into().unwrap()) };
        let grad_f_slice: &mut [ipnumber] =
            unsafe { core::slice::from_raw_parts_mut(grad_f, n.try_into().unwrap()) };

        assert!(x_slice.len() == n.try_into().unwrap());
        assert!(grad_f_slice.len() == n.try_into().unwrap());

        gradient_objective_function(x_slice, grad_f_slice);

        true
    }

    /// Implementation of callback function for evaluating the Jacobian of the constrant functions.
    ///
    /// This function converts the Jacobian of the constrant functions definitions into something that IPOPT understands.
    #[unsafe(no_mangle)]
    pub(crate) extern "C" fn eval_jac_g(
        n: ipindex,
        x: *mut ipnumber,
        new_x: bool,
        m: ipindex,
        nele_jac: ipindex,
        iRow: *mut ipindex,
        jCol: *mut ipindex,
        values: *mut ipnumber,
        user_data: UserDataPtr,
    ) -> bool {
        // Throw away unused variables
        let _ = user_data;
        let _ = new_x;
        let _ = m;

        if values.is_null() {
            assert!(x.is_null());
            assert!(values.is_null());

            assert!(!iRow.is_null());
            assert!(!jCol.is_null());

            let i_row_slice: &mut [ipindex] =
                unsafe { core::slice::from_raw_parts_mut(iRow, nele_jac.try_into().unwrap()) };
            let j_col_slice: &mut [ipindex] =
                unsafe { core::slice::from_raw_parts_mut(jCol, nele_jac.try_into().unwrap()) };

            assert!(i_row_slice.len() == nele_jac.try_into().unwrap());
            assert!(j_col_slice.len() == nele_jac.try_into().unwrap());

            jacobian_constraint_elements(i_row_slice, j_col_slice);
        } else {
            assert!(!x.is_null());
            assert!(!values.is_null());

            assert!(iRow.is_null());
            assert!(jCol.is_null());

            let x_slice: &[ipnumber] =
                unsafe { core::slice::from_raw_parts_mut(x, n.try_into().unwrap()) };
            let values_slice: &mut [ipnumber] =
                unsafe { core::slice::from_raw_parts_mut(values, nele_jac.try_into().unwrap()) };

            assert!(x_slice.len() == n.try_into().unwrap());
            assert!(values_slice.len() == nele_jac.try_into().unwrap());

            jacobian_constraint_function(x_slice, values_slice);
        }

        true
    }

    /// Implementation of callback function for evaluating the Hessian of the Lagrangian function.
    ///
    /// This function converts the Hessian of the Lagrangian function definition into something that IPOPT understands.
    #[unsafe(no_mangle)]
    pub(crate) extern "C" fn eval_h(
        n: ipindex,
        x: *mut ipnumber,
        new_x: bool,
        obj_factor: ipnumber,
        m: ipindex,
        lambda: *mut ipnumber,
        new_lambda: bool,
        nele_hess: ipindex,
        iRow: *mut ipindex,
        jCol: *mut ipindex,
        values: *mut ipnumber,
        user_data: UserDataPtr,
    ) -> bool {
        // These variables are not yet used
        let _ = new_x;
        let _ = new_lambda;
        let _ = user_data;

        if values.is_null() {
            assert!(x.is_null());
            assert!(values.is_null());

            assert!(!lambda.is_null());
            assert!(!iRow.is_null());
            assert!(!jCol.is_null());

            let i_row_slice: &mut [ipindex] =
                unsafe { core::slice::from_raw_parts_mut(iRow, nele_hess.try_into().unwrap()) };
            let j_col_slice: &mut [ipindex] =
                unsafe { core::slice::from_raw_parts_mut(jCol, nele_hess.try_into().unwrap()) };

            assert!(i_row_slice.len() == nele_hess.try_into().unwrap());
            assert!(j_col_slice.len() == nele_hess.try_into().unwrap());

            hessian_elements(i_row_slice, j_col_slice);
        } else {
            assert!(!x.is_null());
            assert!(!values.is_null());
            assert!(!lambda.is_null());

            assert!(iRow.is_null());
            assert!(jCol.is_null());

            let x_slice: &[ipnumber] =
                unsafe { core::slice::from_raw_parts_mut(x, n.try_into().unwrap()) };
            let values_slice: &mut [ipnumber] =
                unsafe { core::slice::from_raw_parts_mut(values, nele_hess.try_into().unwrap()) };
            let lambda_slice: &mut [ipnumber] =
                unsafe { core::slice::from_raw_parts_mut(lambda, m.try_into().unwrap()) };

            assert!(x_slice.len() == n.try_into().unwrap());
            assert!(values_slice.len() == nele_hess.try_into().unwrap());
            assert!(lambda_slice.len() == m.try_into().unwrap());

            hessian_lagrangian_function(x_slice, lambda_slice, obj_factor, values_slice);
        }

        true
    }
}
