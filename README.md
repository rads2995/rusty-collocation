## Description
My attempt to call the [IPOPT](https://github.com/coin-or/Ipopt/tree/stable/3.14) nonlinear optimizer using Rust's [Foreign Function Interface](https://doc..rust-lang.org/nomicon/ffi.html).

**Disclaimer:** this is a very unstable work-in-progress and everything is expected to change. 

## How to Run
To be added.

## How to Test
To be added.

## Sample Output
```
******************************************************************************
This program contains Ipopt, a library for large-scale nonlinear optimization.
 Ipopt is released as open source code under the Eclipse Public License (EPL).
         For more information visit https://github.com/coin-or/Ipopt
******************************************************************************

This is Ipopt version 3.14.17, running with linear solver MUMPS 5.7.3.

Number of nonzeros in equality constraint Jacobian...:        4
Number of nonzeros in inequality constraint Jacobian.:        4
Number of nonzeros in Lagrangian Hessian.............:       10

Total number of variables............................:        4
                     variables with only lower bounds:        0
                variables with lower and upper bounds:        4
                     variables with only upper bounds:        0
Total number of equality constraints.................:        1
Total number of inequality constraints...............:        1
        inequality constraints with only lower bounds:        1
   inequality constraints with lower and upper bounds:        0
        inequality constraints with only upper bounds:        0

iter    objective    inf_pr   inf_du lg(mu)  ||d||  lg(rg) alpha_du alpha_pr  ls
   0  1.0201000e+00 1.12e+01 5.28e-01   0.0 0.00e+00    -  0.00e+00 0.00e+00   0
   1  1.3232594e+00 7.49e-01 2.25e+01  -0.3 7.97e-01    -  3.19e-01 1.00e+00f  1
   2  1.3994049e+00 7.52e-03 4.96e+00  -0.3 5.60e-02   2.0 9.97e-01 1.00e+00h  1
   3  1.3467031e+00 4.48e-05 4.25e-01  -1.0 9.91e-01    -  9.94e-01 1.00e+00H  1
   4  1.3752187e+00 9.48e-05 5.12e-02  -1.4 2.82e-01    -  9.15e-01 1.00e+00H  1
   5  1.3780260e+00 3.37e-05 5.19e-03  -2.9 6.83e-02    -  9.81e-01 1.00e+00H  1
   6  1.3793477e+00 1.86e-04 1.17e-04  -4.7 9.69e-03    -  9.98e-01 1.00e+00h  1
   7  1.3794082e+00 2.73e-07 3.22e-07 -10.3 3.00e-04    -  9.99e-01 1.00e+00h  1

Number of Iterations....: 7

                                   (scaled)                 (unscaled)
Objective...............:   1.3794082090203641e+00    1.3794082090203641e+00
Dual infeasibility......:   3.2223909810001449e-07    3.2223909810001449e-07
Constraint violation....:   2.7280659509187899e-07    2.7280659509187899e-07
Variable bound violation:   0.0000000000000000e+00    0.0000000000000000e+00
Complementarity.........:   3.2346732764788170e-07    3.2346732764788170e-07
Overall NLP error.......:   3.2346732764788170e-07    3.2346732764788170e-07


Number of objective function evaluations             = 12
Number of objective gradient evaluations             = 8
Number of equality constraint evaluations            = 12
Number of inequality constraint evaluations          = 12
Number of equality constraint Jacobian evaluations   = 8
Number of inequality constraint Jacobian evaluations = 8
Number of Lagrangian Hessian evaluations             = 7
Total seconds in IPOPT                               = 0.012

EXIT: Optimal Solution Found.
```