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
   0  1.6109693e+01 1.12e+01 5.28e-01   0.0 0.00e+00    -  0.00e+00 0.00e+00   0
   1  1.7410406e+01 7.49e-01 2.25e+01  -0.3 7.97e-01    -  3.19e-01 1.00e+00f  1
   2  1.8001613e+01 7.52e-03 4.96e+00  -0.3 5.60e-02   2.0 9.97e-01 1.00e+00h  1
   3  1.7199482e+01 4.00e-02 4.24e-01  -1.0 9.91e-01    -  9.98e-01 1.00e+00f  1
   4  1.6940955e+01 1.59e-01 4.58e-02  -1.4 2.88e-01    -  9.66e-01 1.00e+00h  1
   5  1.7003411e+01 2.16e-02 8.42e-03  -2.9 7.03e-02    -  9.68e-01 1.00e+00h  1
   6  1.7013974e+01 2.03e-04 8.65e-05  -4.5 6.22e-03    -  1.00e+00 1.00e+00h  1
   7  1.7014017e+01 2.76e-07 2.18e-07 -10.3 1.43e-04    -  9.99e-01 1.00e+00h  1

Number of Iterations....: 7

                                   (scaled)                 (unscaled)
Objective...............:   1.7014017031783709e+01    1.7014017031783709e+01
Dual infeasibility......:   2.1824331028124169e-07    2.1824331028124169e-07
Constraint violation....:   2.7626678544834249e-07    2.7626678544834249e-07
Variable bound violation:   0.0000000000000000e+00    0.0000000000000000e+00
Complementarity.........:   1.0473361552883477e-07    1.0473361552883477e-07
Overall NLP error.......:   2.7626678544834249e-07    2.7626678544834249e-07


Number of objective function evaluations             = 8
Number of objective gradient evaluations             = 8
Number of equality constraint evaluations            = 8
Number of inequality constraint evaluations          = 8
Number of equality constraint Jacobian evaluations   = 8
Number of inequality constraint Jacobian evaluations = 8
Number of Lagrangian Hessian evaluations             = 7
Total seconds in IPOPT                               = 0.003

EXIT: Optimal Solution Found.
```
