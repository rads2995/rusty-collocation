#![allow(non_camel_case_types, non_snake_case, unused)]

pub(crate) mod cblas_ffi {

    #[repr(C)]
    pub(crate) enum CBLAS_ORDER {
        CblasRowMajor=101, 
        CblasColMajor=102,
    }

    #[repr(C)]
    pub(crate) enum CBLAS_TRANSPOSE {
        CblasNoTrans=111, 
        CblasTrans=112, 
        CblasConjTrans=113,
    }
    
    #[repr(C)]
    pub(crate) enum CBLAS_UPLO {
        CblasUpper=121, 
        CblasLower=122
    }
    
    #[repr(C)]
    pub(crate) enum CBLAS_DIAG {
        CblasNonUnit=131, 
        CblasUnit=132,
    }
    
    #[repr(C)]
    pub(crate) enum CBLAS_SIDE {
        CblasLeft=141, 
        CblasRight=142
    }

    #[link(name = "blas", kind = "dylib")]
    unsafe extern "C" {
        // Prototypes for level 1 BLAS functions (complex are recast as routines)
        pub(crate) unsafe fn cblas_sdsdot(
            N: core::ffi::c_int,
            alpha: core::ffi::c_float,
            X: *const core::ffi::c_float,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_float,
            incY: core::ffi::c_int,
        ) -> core::ffi::c_float;
    
        pub(crate) unsafe fn cblas_dsdot(
            N: core::ffi::c_int,
            X: *const core::ffi::c_float,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_float,
            incY: core::ffi::c_int,
        ) -> core::ffi::c_double;

        pub(crate) unsafe fn cblas_sdot(
            N: core::ffi::c_int,
            X: *const core::ffi::c_float,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_float,
            incY: core::ffi::c_int,
        ) -> core::ffi::c_float;

        pub(crate) unsafe fn cblas_ddot(
            N: core::ffi::c_int,
            X: *const core::ffi::c_double,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_double,
            incY: core::ffi::c_int,
        ) -> core::ffi::c_double;

        // Functions having prefixes Z and C only
        pub(crate) unsafe fn cblas_cdotu_sub(
            N: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_void,
            incY: core::ffi::c_int,
            dotu: *mut core::ffi::c_void,
        );

        pub(crate) unsafe fn cblas_cdotc_sub(
            N: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_void,
            incY: core::ffi::c_int,
            dotc: *mut core::ffi::c_void,
        );

        pub(crate) unsafe fn cblas_zdotu_sub(
            N: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_void,
            incY: core::ffi::c_int,
            dotu: *mut core::ffi::c_void,
        );

        pub(crate) unsafe fn cblas_zdotc_sub(
            N: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_void,
            incY: core::ffi::c_int,
            dotc: *mut core::ffi::c_void,
        );
    
        // Functions having prefixes S D SC DZ
        pub(crate) unsafe fn cblas_snrm2(
            N: core::ffi::c_int, 
            X: *const core::ffi::c_float, 
            incX: core::ffi::c_int
        ) -> core::ffi::c_float;

        pub(crate) unsafe fn cblas_sasum(
            N: core::ffi::c_int, 
            X: *const core::ffi::c_float, 
            incX: core::ffi::c_int
        ) -> core::ffi::c_float;
        
        pub(crate) unsafe fn cblas_dnrm2(
            N: core::ffi::c_int, 
            X: *const core::ffi::c_double, 
            incX: core::ffi::c_int
        ) -> core::ffi::c_double;

        pub(crate) unsafe fn cblas_dasum(
            N: core::ffi::c_int, 
            X: *const core::ffi::c_double, 
            incX: core::ffi::c_int
        ) -> core::ffi::c_double;

        pub(crate) unsafe fn cblas_scnrm2(
            N: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
        ) -> core::ffi::c_float;

        pub(crate) unsafe fn cblas_scasum(
            N: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
        ) -> core::ffi::c_float;

        pub(crate) unsafe fn cblas_dznrm2(
            N: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
        ) -> core::ffi::c_double;

        pub(crate) unsafe fn cblas_dzasum(
            N: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
        ) -> core::ffi::c_double;

        // Functions having standard 4 prefixes (S D C Z)
        pub(crate) unsafe fn cblas_isamax(
            N: core::ffi::c_int,
            X: *const core::ffi::c_float,
            incX: core::ffi::c_int,
        ) -> usize;

        pub(crate) unsafe fn cblas_idamax(
            N: core::ffi::c_int,
            X: *const core::ffi::c_double,
            incX: core::ffi::c_int,
        ) -> usize;

        pub(crate) unsafe fn cblas_icamax(
            N: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
        ) -> usize;

        pub(crate) unsafe fn cblas_izamax(
            N: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
        ) -> usize;    
    
        // Prototypes for level 1 BLAS routines
    
        // Routines with standard 4 prefixes (s, d, c, z)
        pub(crate) unsafe fn cblas_sswap(
            N: core::ffi::c_int,
            X: *mut core::ffi::c_float,
            incX: core::ffi::c_int,
            Y: *mut core::ffi::c_float,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_scopy(
            N: core::ffi::c_int,
            X: *const core::ffi::c_float,
            incX: core::ffi::c_int,
            Y: *mut core::ffi::c_float,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_saxpy(
            N: core::ffi::c_int,
            alpha: core::ffi::c_float,
            X: *const core::ffi::c_float,
            incX: core::ffi::c_int,
            Y: *mut core::ffi::c_float,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dswap(
            N: core::ffi::c_int,
            X: *mut core::ffi::c_double,
            incX: core::ffi::c_int,
            Y: *mut core::ffi::c_double,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dcopy(
            N: core::ffi::c_int,
            X: *const core::ffi::c_double,
            incX: core::ffi::c_int,
            Y: *mut core::ffi::c_double,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_daxpy(
            N: core::ffi::c_int,
            alpha: core::ffi::c_double,
            X: *const core::ffi::c_double,
            incX: core::ffi::c_int,
            Y: *mut core::ffi::c_double,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_cswap(
            N: core::ffi::c_int,
            X: *mut core::ffi::c_void,
            incX: core::ffi::c_int,
            Y: *mut core::ffi::c_void,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ccopy(
            N: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            Y: *mut core::ffi::c_void,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_caxpy(
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            Y: *mut core::ffi::c_void,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zswap(
            N: core::ffi::c_int,
            X: *mut core::ffi::c_void,
            incX: core::ffi::c_int,
            Y: *mut core::ffi::c_void,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zcopy(
            N: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            Y: *mut core::ffi::c_void,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zaxpy(
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            Y: *mut core::ffi::c_void,
            incY: core::ffi::c_int,
        );

        // Routines with S and D prefix only
        pub(crate) unsafe fn cblas_srotg(
            a: *mut core::ffi::c_float, 
            b: *mut core::ffi::c_float, 
            c: *mut core::ffi::c_float, 
            s: *mut core::ffi::c_float
        );

        pub(crate) unsafe fn cblas_srotmg(
            d1: *mut core::ffi::c_float, 
            d2: *mut core::ffi::c_float, 
            b1: *mut core::ffi::c_float, 
            b2: core::ffi::c_float, 
            P: *mut core::ffi::c_float
        );

        pub(crate) unsafe fn cblas_srot(
            N: core::ffi::c_int,
            X: *mut core::ffi::c_float,
            incX: core::ffi::c_int,
            Y: *mut core::ffi::c_float,
            incY: core::ffi::c_int,
            c: core::ffi::c_float,
            s: core::ffi::c_float,
        );

        pub(crate) unsafe fn cblas_srotm(
            N: core::ffi::c_int,
            X: *mut core::ffi::c_float,
            incX: core::ffi::c_int,
            Y: *mut core::ffi::c_float,
            incY: core::ffi::c_int,
            P: *const core::ffi::c_float,
        );

        pub(crate) unsafe fn cblas_drotg(
            a: *mut core::ffi::c_double, 
            b: *mut core::ffi::c_double, 
            c: *mut core::ffi::c_double, 
            s: *mut core::ffi::c_double
        );

        pub(crate) unsafe fn cblas_drotmg(
            d1: *mut core::ffi::c_double, 
            d2: *mut core::ffi::c_double, 
            b1: *mut core::ffi::c_double, 
            b2: core::ffi::c_double, 
            P: *mut core::ffi::c_double
        );

        pub(crate) unsafe fn cblas_drot(
            N: core::ffi::c_int,
            X: *mut core::ffi::c_double,
            incX: core::ffi::c_int,
            Y: *mut core::ffi::c_double,
            incY: core::ffi::c_int,
            c: core::ffi::c_double,
            s: core::ffi::c_double,
        );

        pub(crate) unsafe fn cblas_drotm(
            N: core::ffi::c_int,
            X: *mut core::ffi::c_double,
            incX: core::ffi::c_int,
            Y: *mut core::ffi::c_double,
            incY: core::ffi::c_int,
            P: *const core::ffi::c_double,
        );

        pub(crate) unsafe fn cblas_sscal(
            N: core::ffi::c_int,
            alpha: core::ffi::c_float,
            X: *mut core::ffi::c_float,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dscal(
            N: core::ffi::c_int,
            alpha: core::ffi::c_double,
            X: *mut core::ffi::c_double,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_cscal(
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            X: *mut core::ffi::c_void,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zscal(
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            X: *mut core::ffi::c_void,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_csscal(
            N: core::ffi::c_int,
            alpha: core::ffi::c_float,
            X: *mut core::ffi::c_void,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zdscal(
            N: core::ffi::c_int,
            alpha: core::ffi::c_double,
            X: *mut core::ffi::c_void,
            incX: core::ffi::c_int,
        );

        // Prototypes for level 2 BLAS
        // Routines with standard 4 prefixes (S, D, C, Z)
        pub(crate) unsafe fn cblas_sgemv(
            order: CBLAS_ORDER,
            TransA: CBLAS_TRANSPOSE,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: core::ffi::c_float,
            A: *const core::ffi::c_float,
            lda: core::ffi::c_int,
            X: *const core::ffi::c_float,
            incX: core::ffi::c_int,
            beta: core::ffi::c_float,
            Y: *mut core::ffi::c_float,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_sgbmv(
            order: CBLAS_ORDER,
            TransA: CBLAS_TRANSPOSE,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            KL: core::ffi::c_int,
            KU: core::ffi::c_int,
            alpha: core::ffi::c_float,
            A: *const core::ffi::c_float,
            lda: core::ffi::c_int,
            X: *const core::ffi::c_float,
            incX: core::ffi::c_int,
            beta: core::ffi::c_float,
            Y: *mut core::ffi::c_float,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_strmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            A: *const core::ffi::c_float,
            lda: core::ffi::c_int,
            X: *mut core::ffi::c_float,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_stbmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            A: *const core::ffi::c_float,
            lda: core::ffi::c_int,
            X: *mut core::ffi::c_float,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_stpmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            Ap: *const core::ffi::c_float,
            X: *mut core::ffi::c_float,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_strsv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            A: *const core::ffi::c_float,
            lda: core::ffi::c_int,
            X: *mut core::ffi::c_float,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_stbsv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            A: *const core::ffi::c_float,
            lda: core::ffi::c_int,
            X: *mut core::ffi::c_float,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_stpsv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            Ap: *const core::ffi::c_float,
            X: *mut core::ffi::c_float,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dgemv(
            order: CBLAS_ORDER,
            TransA: CBLAS_TRANSPOSE,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: core::ffi::c_double,
            A: *const core::ffi::c_double,
            lda: core::ffi::c_int,
            X: *const core::ffi::c_double,
            incX: core::ffi::c_int,
            beta: core::ffi::c_double,
            Y: *mut core::ffi::c_double,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dgbmv(
            order: CBLAS_ORDER,
            TransA: CBLAS_TRANSPOSE,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            KL: core::ffi::c_int,
            KU: core::ffi::c_int,
            alpha: core::ffi::c_double,
            A: *const core::ffi::c_double,
            lda: core::ffi::c_int,
            X: *const core::ffi::c_double,
            incX: core::ffi::c_int,
            beta: core::ffi::c_double,
            Y: *mut core::ffi::c_double,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dtrmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            A: *const core::ffi::c_double,
            lda: core::ffi::c_int,
            X: *mut core::ffi::c_double,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dtbmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            A: *const core::ffi::c_double,
            lda: core::ffi::c_int,
            X: *mut core::ffi::c_double,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dtpmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            Ap: *const core::ffi::c_double,
            X: *mut core::ffi::c_double,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dtrsv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            A: *const core::ffi::c_double,
            lda: core::ffi::c_int,
            X: *mut core::ffi::c_double,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dtbsv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            A: *const core::ffi::c_double,
            lda: core::ffi::c_int,
            X: *mut core::ffi::c_double,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dtpsv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            Ap: *const core::ffi::c_double,
            X: *mut core::ffi::c_double,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_cgemv(
            order: CBLAS_ORDER,
            TransA: CBLAS_TRANSPOSE,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            Y: *mut core::ffi::c_void,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_cgbmv(
            order: CBLAS_ORDER,
            TransA: CBLAS_TRANSPOSE,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            KL: core::ffi::c_int,
            KU: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            Y: *mut core::ffi::c_void,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ctrmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            X: *mut core::ffi::c_void,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ctbmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            X: *mut core::ffi::c_void,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ctpmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            Ap: *const core::ffi::c_void,
            X: *mut core::ffi::c_void,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ctrsv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            X: *mut core::ffi::c_void,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ctbsv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            X: *mut core::ffi::c_void,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ctpsv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            Ap: *const core::ffi::c_void,
            X: *mut core::ffi::c_void,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zgemv(
            order: CBLAS_ORDER,
            TransA: CBLAS_TRANSPOSE,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            Y: *mut core::ffi::c_void,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zgbmv(
            order: CBLAS_ORDER,
            TransA: CBLAS_TRANSPOSE,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            KL: core::ffi::c_int,
            KU: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            Y: *mut core::ffi::c_void,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ztrmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            X: *mut core::ffi::c_void,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ztbmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            X: *mut core::ffi::c_void,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ztpmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            Ap: *const core::ffi::c_void,
            X: *mut core::ffi::c_void,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ztrsv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            X: *mut core::ffi::c_void,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ztbsv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            X: *mut core::ffi::c_void,
            incX: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ztpsv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            N: core::ffi::c_int,
            Ap: *const core::ffi::c_void,
            X: *mut core::ffi::c_void,
            incX: core::ffi::c_int,
        );

        // Routines with S and D prefixes only
        pub(crate) unsafe fn cblas_ssymv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: core::ffi::c_float,
            A: *const core::ffi::c_float,
            lda: core::ffi::c_int,
            X: *const core::ffi::c_float,
            incX: core::ffi::c_int,
            beta: core::ffi::c_float,
            Y: *mut core::ffi::c_float,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ssbmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: core::ffi::c_float,
            A: *const core::ffi::c_float,
            lda: core::ffi::c_int,
            X: *const core::ffi::c_float,
            incX: core::ffi::c_int,
            beta: core::ffi::c_float,
            Y: *mut core::ffi::c_float,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_sspmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: core::ffi::c_float,
            Ap: *const core::ffi::c_float,
            X: *const core::ffi::c_float,
            incX: core::ffi::c_int,
            beta: core::ffi::c_float,
            Y: *mut core::ffi::c_float,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_sger(
            order: CBLAS_ORDER,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: core::ffi::c_float,
            X: *const core::ffi::c_float,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_float,
            incY: core::ffi::c_int,
            A: *mut core::ffi::c_float,
            lda: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ssyr(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: core::ffi::c_float,
            X: *const core::ffi::c_float,
            incX: core::ffi::c_int,
            A: *mut core::ffi::c_float,
            lda: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_sspr(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: core::ffi::c_float,
            X: *const core::ffi::c_float,
            incX: core::ffi::c_int,
            Ap: *mut core::ffi::c_float,
        );

        pub(crate) unsafe fn cblas_ssyr2(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: core::ffi::c_float,
            X: *const core::ffi::c_float,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_float,
            incY: core::ffi::c_int,
            A: *mut core::ffi::c_float,
            lda: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_sspr2(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: core::ffi::c_float,
            X: *const core::ffi::c_float,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_float,
            incY: core::ffi::c_int,
            A: *mut core::ffi::c_float,
        );

        pub(crate) unsafe fn cblas_dsymv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: core::ffi::c_double,
            A: *const core::ffi::c_double,
            lda: core::ffi::c_int,
            X: *const core::ffi::c_double,
            incX: core::ffi::c_int,
            beta: core::ffi::c_double,
            Y: *mut core::ffi::c_double,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dsbmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: core::ffi::c_double,
            A: *const core::ffi::c_double,
            lda: core::ffi::c_int,
            X: *const core::ffi::c_double,
            incX: core::ffi::c_int,
            beta: core::ffi::c_double,
            Y: *mut core::ffi::c_double,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dspmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: core::ffi::c_double,
            Ap: *const core::ffi::c_double,
            X: *const core::ffi::c_double,
            incX: core::ffi::c_int,
            beta: core::ffi::c_double,
            Y: *mut core::ffi::c_double,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dger(
            order: CBLAS_ORDER,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: core::ffi::c_double,
            X: *const core::ffi::c_double,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_double,
            incY: core::ffi::c_int,
            A: *mut core::ffi::c_double,
            lda: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dsyr(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: core::ffi::c_double,
            X: *const core::ffi::c_double,
            incX: core::ffi::c_int,
            A: *mut core::ffi::c_double,
            lda: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dspr(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: core::ffi::c_double,
            X: *const core::ffi::c_double,
            incX: core::ffi::c_int,
            Ap: *mut core::ffi::c_double,
        );

        pub(crate) unsafe fn cblas_dsyr2(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: core::ffi::c_double,
            X: *const core::ffi::c_double,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_double,
            incY: core::ffi::c_int,
            A: *mut core::ffi::c_double,
            lda: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dspr2(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: core::ffi::c_double,
            X: *const core::ffi::c_double,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_double,
            incY: core::ffi::c_int,
            A: *mut core::ffi::c_double,
        );

        // Routines with C and Z prefixes only
        pub(crate) unsafe fn cblas_chemv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            Y: *mut core::ffi::c_void,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_chbmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            Y: *mut core::ffi::c_void,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_chpmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            Ap: *const core::ffi::c_void,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            Y: *mut core::ffi::c_void,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_cgeru(
            order: CBLAS_ORDER,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_void,
            incY: core::ffi::c_int,
            A: *mut core::ffi::c_void,
            lda: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_cgerc(
            order: CBLAS_ORDER,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_void,
            incY: core::ffi::c_int,
            A: *mut core::ffi::c_void,
            lda: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_cher(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: core::ffi::c_float,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            A: *mut core::ffi::c_void,
            lda: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_chpr(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: core::ffi::c_float,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            A: *mut core::ffi::c_void,
        );

        pub(crate) unsafe fn cblas_cher2(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_void,
            incY: core::ffi::c_int,
            A: *mut core::ffi::c_void,
            lda: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_chpr2(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_void,
            incY: core::ffi::c_int,
            Ap: *mut core::ffi::c_void,
        );

        pub(crate) unsafe fn cblas_zhemv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            Y: *mut core::ffi::c_void,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zhbmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            Y: *mut core::ffi::c_void,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zhpmv(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            Ap: *const core::ffi::c_void,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            Y: *mut core::ffi::c_void,
            incY: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zgeru(
            order: CBLAS_ORDER,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_void,
            incY: core::ffi::c_int,
            A: *mut core::ffi::c_void,
            lda: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zgerc(
            order: CBLAS_ORDER,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_void,
            incY: core::ffi::c_int,
            A: *mut core::ffi::c_void,
            lda: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zher(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: core::ffi::c_double,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            A: *mut core::ffi::c_void,
            lda: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zhpr(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: core::ffi::c_double,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            A: *mut core::ffi::c_void,
        );

        pub(crate) unsafe fn cblas_zher2(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_void,
            incY: core::ffi::c_int,
            A: *mut core::ffi::c_void,
            lda: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zhpr2(
            order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            X: *const core::ffi::c_void,
            incX: core::ffi::c_int,
            Y: *const core::ffi::c_void,
            incY: core::ffi::c_int,
            Ap: *mut core::ffi::c_void,
        );

        // Prototypes for level 3 BLAS
        // Routines with standard 4 prefixes (S, D, C, Z)
        pub(crate) unsafe fn cblas_sgemm(
            Order: CBLAS_ORDER,
            TransA: CBLAS_TRANSPOSE,
            TransB: CBLAS_TRANSPOSE,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: core::ffi::c_float,
            A: *const core::ffi::c_float,
            lda: core::ffi::c_int,
            B: *const core::ffi::c_float,
            ldb: core::ffi::c_int,
            beta: core::ffi::c_float,
            C: *mut core::ffi::c_float,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ssymm(
            Order: CBLAS_ORDER,
            Side: CBLAS_SIDE,
            Uplo: CBLAS_UPLO,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: core::ffi::c_float,
            A: *const core::ffi::c_float,
            lda: core::ffi::c_int,
            B: *const core::ffi::c_float,
            ldb: core::ffi::c_int,
            beta: core::ffi::c_float,
            C: *mut core::ffi::c_float,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ssyrk(
            Order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            Trans: CBLAS_TRANSPOSE,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: core::ffi::c_float,
            A: *const core::ffi::c_float,
            lda: core::ffi::c_int,
            beta: core::ffi::c_float,
            C: *mut core::ffi::c_float,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ssyr2k(
            Order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            Trans: CBLAS_TRANSPOSE,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: core::ffi::c_float,
            A: *const core::ffi::c_float,
            lda: core::ffi::c_int,
            B: *const core::ffi::c_float,
            ldb: core::ffi::c_int,
            beta: core::ffi::c_float,
            C: *mut core::ffi::c_float,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_strmm(
            Order: CBLAS_ORDER,
            Side: CBLAS_SIDE,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: core::ffi::c_float,
            A: *const core::ffi::c_float,
            lda: core::ffi::c_int,
            B: *mut core::ffi::c_float,
            ldb: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_strsm(
            Order: CBLAS_ORDER,
            Side: CBLAS_SIDE,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: core::ffi::c_float,
            A: *const core::ffi::c_float,
            lda: core::ffi::c_int,
            B: *mut core::ffi::c_float,
            ldb: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dgemm(
            Order: CBLAS_ORDER,
            TransA: CBLAS_TRANSPOSE,
            TransB: CBLAS_TRANSPOSE,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: core::ffi::c_double,
            A: *const core::ffi::c_double,
            lda: core::ffi::c_int,
            B: *const core::ffi::c_double,
            ldb: core::ffi::c_int,
            beta: core::ffi::c_double,
            C: *mut core::ffi::c_double,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dsymm(
            Order: CBLAS_ORDER,
            Side: CBLAS_SIDE,
            Uplo: CBLAS_UPLO,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: core::ffi::c_double,
            A: *const core::ffi::c_double,
            lda: core::ffi::c_int,
            B: *const core::ffi::c_double,
            ldb: core::ffi::c_int,
            beta: core::ffi::c_double,
            C: *mut core::ffi::c_double,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dsyrk(
            Order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            Trans: CBLAS_TRANSPOSE,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: core::ffi::c_double,
            A: *const core::ffi::c_double,
            lda: core::ffi::c_int,
            beta: core::ffi::c_double,
            C: *mut core::ffi::c_double,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dsyr2k(
            Order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            Trans: CBLAS_TRANSPOSE,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: core::ffi::c_double,
            A: *const core::ffi::c_double,
            lda: core::ffi::c_int,
            B: *const core::ffi::c_double,
            ldb: core::ffi::c_int,
            beta: core::ffi::c_double,
            C: *mut core::ffi::c_double,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dtrmm(
            Order: CBLAS_ORDER,
            Side: CBLAS_SIDE,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: core::ffi::c_double,
            A: *const core::ffi::c_double,
            lda: core::ffi::c_int,
            B: *mut core::ffi::c_double,
            ldb: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_dtrsm(
            Order: CBLAS_ORDER,
            Side: CBLAS_SIDE,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: core::ffi::c_double,
            A: *const core::ffi::c_double,
            lda: core::ffi::c_int,
            B: *mut core::ffi::c_double,
            ldb: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_cgemm(
            Order: CBLAS_ORDER,
            TransA: CBLAS_TRANSPOSE,
            TransB: CBLAS_TRANSPOSE,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            B: *const core::ffi::c_void,
            ldb: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            C: *mut core::ffi::c_void,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_csymm(
            Order: CBLAS_ORDER,
            Side: CBLAS_SIDE,
            Uplo: CBLAS_UPLO,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            B: *const core::ffi::c_void,
            ldb: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            C: *mut core::ffi::c_void,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_csyrk(
            Order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            Trans: CBLAS_TRANSPOSE,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            C: *mut core::ffi::c_void,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_csyr2k(
            Order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            Trans: CBLAS_TRANSPOSE,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            B: *const core::ffi::c_void,
            ldb: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            C: *mut core::ffi::c_void,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ctrmm(
            Order: CBLAS_ORDER,
            Side: CBLAS_SIDE,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            B: *mut core::ffi::c_void,
            ldb: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ctrsm(
            Order: CBLAS_ORDER,
            Side: CBLAS_SIDE,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            B: *mut core::ffi::c_void,
            ldb: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zgemm(
            Order: CBLAS_ORDER,
            TransA: CBLAS_TRANSPOSE,
            TransB: CBLAS_TRANSPOSE,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            B: *const core::ffi::c_void,
            ldb: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            C: *mut core::ffi::c_void,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zsymm(
            Order: CBLAS_ORDER,
            Side: CBLAS_SIDE,
            Uplo: CBLAS_UPLO,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            B: *const core::ffi::c_void,
            ldb: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            C: *mut core::ffi::c_void,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zsyrk(
            Order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            Trans: CBLAS_TRANSPOSE,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            C: *mut core::ffi::c_void,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zsyr2k(
            Order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            Trans: CBLAS_TRANSPOSE,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            B: *const core::ffi::c_void,
            ldb: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            C: *mut core::ffi::c_void,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ztrmm(
            Order: CBLAS_ORDER,
            Side: CBLAS_SIDE,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            B: *mut core::ffi::c_void,
            ldb: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_ztrsm(
            Order: CBLAS_ORDER,
            Side: CBLAS_SIDE,
            Uplo: CBLAS_UPLO,
            TransA: CBLAS_TRANSPOSE,
            Diag: CBLAS_DIAG,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            B: *mut core::ffi::c_void,
            ldb: core::ffi::c_int,
        );

        // Routines with prefixes C and Z only
        pub(crate) unsafe fn cblas_chemm(
            Order: CBLAS_ORDER,
            Side: CBLAS_SIDE,
            Uplo: CBLAS_UPLO,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            B: *const core::ffi::c_void,
            ldb: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            C: *mut core::ffi::c_void,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_cherk(
            Order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            Trans: CBLAS_TRANSPOSE,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: core::ffi::c_float,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            beta: core::ffi::c_float,
            C: *mut core::ffi::c_void,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_cher2k(
            Order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            Trans: CBLAS_TRANSPOSE,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            B: *const core::ffi::c_void,
            ldb: core::ffi::c_int,
            beta: core::ffi::c_float,
            C: *mut core::ffi::c_void,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zhemm(
            Order: CBLAS_ORDER,
            Side: CBLAS_SIDE,
            Uplo: CBLAS_UPLO,
            M: core::ffi::c_int,
            N: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            B: *const core::ffi::c_void,
            ldb: core::ffi::c_int,
            beta: *const core::ffi::c_void,
            C: *mut core::ffi::c_void,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zherk(
            Order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            Trans: CBLAS_TRANSPOSE,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: core::ffi::c_double,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            beta: core::ffi::c_double,
            C: *mut core::ffi::c_void,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_zher2k(
            Order: CBLAS_ORDER,
            Uplo: CBLAS_UPLO,
            Trans: CBLAS_TRANSPOSE,
            N: core::ffi::c_int,
            K: core::ffi::c_int,
            alpha: *const core::ffi::c_void,
            A: *const core::ffi::c_void,
            lda: core::ffi::c_int,
            B: *const core::ffi::c_void,
            ldb: core::ffi::c_int,
            beta: core::ffi::c_double,
            C: *mut core::ffi::c_void,
            ldc: core::ffi::c_int,
        );

        pub(crate) unsafe fn cblas_xerbla(
            p: core::ffi::c_int,
            rout: *const core::ffi::c_char,
            form: *const core::ffi::c_char,
            ...
        );
    }   
}
