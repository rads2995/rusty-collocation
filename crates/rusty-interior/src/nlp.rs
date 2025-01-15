pub mod nlp {

    use crate::bindings::ipopt::*;
    
    pub fn objective_function(x: &[ipopt::ipnumber]) -> ipopt::ipnumber {

        x[0] * x[3] * (x[0] + x[1] + x[2]) + x[2]
    }

    pub fn constraint_functions(x: &[ipopt::ipnumber], g: &mut [ipopt::ipnumber]) -> () {

        g[0] = x[0] * x[1] * x[2] * x[3]; 
        g[1] = x[0] * x[0] + x[1] * x[1] + x[2] * x[2] + x[3] * x[3];
    }

    pub fn gradient_objective_function(x: &[ipopt::ipnumber], grad_f: &mut [ipopt::ipnumber]) -> () {
        
        grad_f[0] = x[0] * x[3] + x[3] * (x[0] + x[1] + x[2]);
        grad_f[1] = x[0] * x[3];
        grad_f[2] = x[0] * x[3] + 1.0;
        grad_f[3] = x[0] * (x[0] + x[1] + x[2]);  
    }

    pub fn jacobian_constraint_elements(i_row: &mut [ipopt::ipindex], j_col: &mut [ipopt::ipindex]) -> () {

        i_row[0] = 0;
        j_col[0] = 0;
        i_row[1] = 0;
        j_col[1] = 1;
        i_row[2] = 0;
        j_col[2] = 2;
        i_row[3] = 0;
        j_col[3] = 3;
        i_row[4] = 1;
        j_col[4] = 0;
        i_row[5] = 1;
        j_col[5] = 1;
        i_row[6] = 1;
        j_col[6] = 2;
        i_row[7] = 1;
        j_col[7] = 3; 
    }
    
    pub fn jacobian_constraint_function(x: &[ipopt::ipnumber], jac_g: &mut [ipopt::ipnumber]) -> () {

        jac_g[0] = x[1] * x[2] * x[3];
        jac_g[1] = x[0] * x[2] * x[3];
        jac_g[2] = x[0] * x[1] * x[3];
        jac_g[3] = x[0] * x[1] * x[2];
        jac_g[4] = 2.0 * x[0];
        jac_g[5] = 2.0 * x[1];
        jac_g[6] = 2.0 * x[2];
        jac_g[7] = 2.0 * x[3];
    }

    pub fn hessian_elements(i_row: &mut [ipopt::ipindex], j_col: &mut [ipopt::ipindex]) -> () {

        i_row[0] = 0;
        j_col[0] = 0;
        i_row[1] = 1;
        j_col[1] = 0;
        i_row[2] = 1;
        j_col[2] = 1;
        i_row[3] = 2;
        j_col[3] = 0;
        i_row[4] = 2;
        j_col[4] = 1;
        i_row[5] = 2;
        j_col[5] = 2;
        i_row[6] = 3;
        j_col[6] = 0;
        i_row[7] = 3;
        j_col[7] = 1; 
        i_row[8] = 3;
        j_col[8] = 2; 
        i_row[9] = 3;
        j_col[9] = 3;
    }

    pub fn hessian_lagrangian_function(x: &[ipopt::ipnumber], lambda: &[ipopt::ipnumber], obj_factor: ipopt::ipnumber, hessian: &mut [ipopt::ipnumber]) -> () {
        
        // Fill the objective portion
        hessian[0] = obj_factor * (2.0 * x[3]);
        
        hessian[1] = obj_factor * (x[3]);
        hessian[2] = 0.0;
    
        hessian[3] = obj_factor * (x[3]);
        hessian[4] = 0.0;
        hessian[5] = 0.0;
    
        hessian[6] = obj_factor * (2.0 * x[0] + x[1] + x[2]);
        hessian[7] = obj_factor * (x[0]);
        hessian[8] = obj_factor * (x[0]);
        hessian[9] = 0.0;

        // Add the portion for the first constraint
        hessian[1] += lambda[0] * (x[2] * x[3]);
    
        hessian[3] += lambda[0] * (x[1] * x[3]);
        hessian[4] += lambda[0] * (x[0] * x[3]);
    
        hessian[6] += lambda[0] * (x[1] * x[2]);
        hessian[7] += lambda[0] * (x[0] * x[2]);
        hessian[8] += lambda[0] * (x[0] * x[1]);
    
        // Add the portion for the second constraint
        hessian[0] += lambda[1] * 2.0; 
    
        hessian[2] += lambda[1] * 2.0; 
    
        hessian[5] += lambda[1] * 2.0; 
    
        hessian[9] += lambda[1] * 2.0;
    }
}
