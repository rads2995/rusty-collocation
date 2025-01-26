pub(crate) mod bolza {
    
    /// Rodrigues's Formula
    pub(crate) fn legendre_polynomial(l: usize, x: f64) -> f64 {
        if x.is_nan() {
            f64::NAN
        }

        else if x == 1.0 {
            1.0
        }

        else if x == -1.0 {
            if l % 2 == 1 { -1.0 } else { 1.0 }
        }

        else {
            let mut p_lm2 = 1.0;    // P_0(x)
            if l == 0 {
                return p_lm2;
            }

            let mut p_lm1 = x;  // P_1(x)

            if l == 1 {
                return p_lm1;
            }

            let mut p_l = 0.0;

            for ll in 2..=l {
                p_l = 2.0 * x * p_lm1 - p_lm2 - (x * p_lm1 - p_lm2) / ll as f64;
                p_lm2 = p_lm1;
                p_lm1 = p_l;
            }

            return p_l;

        }
    }

    // Guass-Radau Quadrature

}
