#![allow(non_camel_case_types, unused)]

#[repr(C)]
pub(crate) struct lua_Debug {
    event: core::ffi::c_int,
    name: *const core::ffi::c_char,
    namewhat: *const core::ffi::c_char,
    what: *const core::ffi::c_char,
    source: *const core::ffi::c_char,
    srclen: usize,
    currentline: core::ffi::c_int,
    linedefined: core::ffi::c_int,
    lastlinedefined: core::ffi::c_int,
    nups: core::ffi::c_uchar,
    nparams: core::ffi::c_uchar,
    isvararg: core::ffi::c_char,
    istailcall: core::ffi::c_char,
    ftransfer: core::ffi::c_ushort,
    ntransfer: core::ffi::c_ushort,
    short_src: [core::ffi::c_char; 60usize],
    i_ci: *mut CallInfo,
}

#[repr(C)]
pub(crate) struct CallInfo {
    _address: u8,
}

#[repr(C)]
pub(crate) struct lua_State {
    _unused: [u8; 0],
}

pub(crate) type lua_Number = core::ffi::c_double;

pub(crate) type lua_Integer = core::ffi::c_longlong;

pub(crate) type lua_Unsigned = core::ffi::c_ulonglong;

pub(crate) type lua_KContext = isize;

pub(crate) type lua_CFunction =
    core::option::Option<unsafe extern "C" fn(L: *mut lua_State) -> core::ffi::c_int>;

pub(crate) type lua_KFunction = core::option::Option<
    unsafe extern "C" fn(
        L: *mut lua_State,
        status: core::ffi::c_int,
        ctx: lua_KContext,
    ) -> core::ffi::c_int,
>;

pub(crate) type lua_Reader = core::option::Option<
    unsafe extern "C" fn(
        L: *mut lua_State,
        ud: *mut core::ffi::c_void,
        sz: *mut usize,
    ) -> *const core::ffi::c_char,
>;

pub(crate) type lua_Writer = core::option::Option<
    unsafe extern "C" fn(
        L: *mut lua_State,
        p: *const core::ffi::c_void,
        sz: usize,
        ud: *mut core::ffi::c_void,
    ) -> core::ffi::c_int,
>;

pub(crate) type lua_Alloc = core::option::Option<
    unsafe extern "C" fn(
        ud: *mut core::ffi::c_void,
        ptr: *mut core::ffi::c_void,
        osize: usize,
        nsize: usize,
    ) -> *mut core::ffi::c_void,
>;

pub(crate) type lua_WarnFunction = core::option::Option<
    unsafe extern "C" fn(
        ud: *mut core::ffi::c_void,
        msg: *const core::ffi::c_char,
        tocont: core::ffi::c_int,
    ),
>;

pub(crate) type lua_Hook =
    core::option::Option<unsafe extern "C" fn(L: *mut lua_State, ar: *mut lua_Debug)>;

unsafe extern "C" {
    pub(crate) unsafe fn lua_newstate(f: lua_Alloc, ud: *mut core::ffi::c_void) -> *mut lua_State;

    pub(crate) unsafe fn lua_close(L: *mut lua_State);

    pub(crate) unsafe fn lua_newthread(L: *mut lua_State) -> *mut lua_State;

    pub(crate) unsafe fn lua_closethread(
        L: *mut lua_State,
        from: *mut lua_State,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_resetthread(L: *mut lua_State) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction;

    pub(crate) unsafe fn lua_version(L: *mut lua_State) -> lua_Number;

    pub(crate) unsafe fn lua_absindex(L: *mut lua_State, idx: core::ffi::c_int)
        -> core::ffi::c_int;

    pub(crate) unsafe fn lua_gettop(L: *mut lua_State) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_settop(L: *mut lua_State, idx: core::ffi::c_int);

    pub(crate) unsafe fn lua_pushvalue(L: *mut lua_State, idx: core::ffi::c_int);

    pub(crate) unsafe fn lua_rotate(L: *mut lua_State, idx: core::ffi::c_int, n: core::ffi::c_int);

    pub(crate) unsafe fn lua_copy(
        L: *mut lua_State,
        fromidx: core::ffi::c_int,
        toidx: core::ffi::c_int,
    );

    pub(crate) unsafe fn lua_checkstack(L: *mut lua_State, n: core::ffi::c_int)
        -> core::ffi::c_int;

    pub(crate) unsafe fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: core::ffi::c_int);

    pub(crate) unsafe fn lua_isnumber(L: *mut lua_State, idx: core::ffi::c_int)
        -> core::ffi::c_int;

    pub(crate) unsafe fn lua_isstring(L: *mut lua_State, idx: core::ffi::c_int)
        -> core::ffi::c_int;

    pub(crate) unsafe fn lua_iscfunction(
        L: *mut lua_State,
        idx: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_isinteger(
        L: *mut lua_State,
        idx: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_isuserdata(
        L: *mut lua_State,
        idx: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_type(L: *mut lua_State, idx: core::ffi::c_int) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_typename(
        L: *mut lua_State,
        tp: core::ffi::c_int,
    ) -> *const core::ffi::c_char;

    pub(crate) unsafe fn lua_tonumberx(
        L: *mut lua_State,
        idx: core::ffi::c_int,
        isnum: *mut core::ffi::c_int,
    ) -> lua_Number;

    pub(crate) unsafe fn lua_tointegerx(
        L: *mut lua_State,
        idx: core::ffi::c_int,
        isnum: *mut core::ffi::c_int,
    ) -> lua_Integer;

    pub(crate) unsafe fn lua_toboolean(
        L: *mut lua_State,
        idx: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_tolstring(
        L: *mut lua_State,
        idx: core::ffi::c_int,
        len: *mut usize,
    ) -> *const core::ffi::c_char;

    pub(crate) unsafe fn lua_rawlen(L: *mut lua_State, idx: core::ffi::c_int) -> lua_Unsigned;

    pub(crate) unsafe fn lua_tocfunction(L: *mut lua_State, idx: core::ffi::c_int)
        -> lua_CFunction;

    pub(crate) unsafe fn lua_touserdata(
        L: *mut lua_State,
        idx: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;

    pub(crate) unsafe fn lua_tothread(L: *mut lua_State, idx: core::ffi::c_int) -> *mut lua_State;

    pub(crate) unsafe fn lua_topointer(
        L: *mut lua_State,
        idx: core::ffi::c_int,
    ) -> *const core::ffi::c_void;

    pub(crate) unsafe fn lua_arith(L: *mut lua_State, op: core::ffi::c_int);

    pub(crate) unsafe fn lua_rawequal(
        L: *mut lua_State,
        idx1: core::ffi::c_int,
        idx2: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_compare(
        L: *mut lua_State,
        idx1: core::ffi::c_int,
        idx2: core::ffi::c_int,
        op: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_pushnil(L: *mut lua_State);

    pub(crate) unsafe fn lua_pushnumber(L: *mut lua_State, n: lua_Number);

    pub(crate) unsafe fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);

    pub(crate) unsafe fn lua_pushlstring(
        L: *mut lua_State,
        s: *const core::ffi::c_char,
        len: usize,
    ) -> *const core::ffi::c_char;

    pub(crate) unsafe fn lua_pushstring(
        L: *mut lua_State,
        s: *const core::ffi::c_char,
    ) -> *const core::ffi::c_char;

    pub(crate) unsafe fn lua_pushvfstring(
        L: *mut lua_State,
        fmt: *const core::ffi::c_char,
        argp: *mut core::ffi::c_void,
    ) -> *const core::ffi::c_char;

    pub(crate) unsafe fn lua_pushfstring(
        L: *mut lua_State,
        fmt: *const core::ffi::c_char,
        ...
    ) -> *const core::ffi::c_char;

    pub(crate) unsafe fn lua_pushcclosure(
        L: *mut lua_State,
        fn_: lua_CFunction,
        n: core::ffi::c_int,
    );

    pub(crate) unsafe fn lua_pushboolean(L: *mut lua_State, b: core::ffi::c_int);

    pub(crate) unsafe fn lua_pushlightuserdata(L: *mut lua_State, p: *mut core::ffi::c_void);

    pub(crate) unsafe fn lua_pushthread(L: *mut lua_State) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_getglobal(
        L: *mut lua_State,
        name: *const core::ffi::c_char,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_gettable(L: *mut lua_State, idx: core::ffi::c_int)
        -> core::ffi::c_int;

    pub(crate) unsafe fn lua_getfield(
        L: *mut lua_State,
        idx: core::ffi::c_int,
        k: *const core::ffi::c_char,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_geti(
        L: *mut lua_State,
        idx: core::ffi::c_int,
        n: lua_Integer,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_rawget(L: *mut lua_State, idx: core::ffi::c_int) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_rawgeti(
        L: *mut lua_State,
        idx: core::ffi::c_int,
        n: lua_Integer,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_rawgetp(
        L: *mut lua_State,
        idx: core::ffi::c_int,
        p: *const core::ffi::c_void,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_createtable(
        L: *mut lua_State,
        narr: core::ffi::c_int,
        nrec: core::ffi::c_int,
    );

    pub(crate) unsafe fn lua_newuserdatauv(
        L: *mut lua_State,
        sz: usize,
        nuvalue: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;

    pub(crate) unsafe fn lua_getmetatable(
        L: *mut lua_State,
        objindex: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_getiuservalue(
        L: *mut lua_State,
        idx: core::ffi::c_int,
        n: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_setglobal(L: *mut lua_State, name: *const core::ffi::c_char);

    pub(crate) unsafe fn lua_settable(L: *mut lua_State, idx: core::ffi::c_int);

    pub(crate) unsafe fn lua_setfield(
        L: *mut lua_State,
        idx: core::ffi::c_int,
        k: *const core::ffi::c_char,
    );

    pub(crate) unsafe fn lua_seti(L: *mut lua_State, idx: core::ffi::c_int, n: lua_Integer);

    pub(crate) unsafe fn lua_rawset(L: *mut lua_State, idx: core::ffi::c_int);

    pub(crate) unsafe fn lua_rawseti(L: *mut lua_State, idx: core::ffi::c_int, n: lua_Integer);

    pub(crate) unsafe fn lua_rawsetp(
        L: *mut lua_State,
        idx: core::ffi::c_int,
        p: *const core::ffi::c_void,
    );

    pub(crate) unsafe fn lua_setmetatable(
        L: *mut lua_State,
        objindex: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_setiuservalue(
        L: *mut lua_State,
        idx: core::ffi::c_int,
        n: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_callk(
        L: *mut lua_State,
        nargs: core::ffi::c_int,
        nresults: core::ffi::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    );

    pub(crate) unsafe fn lua_pcallk(
        L: *mut lua_State,
        nargs: core::ffi::c_int,
        nresults: core::ffi::c_int,
        errfunc: core::ffi::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_load(
        L: *mut lua_State,
        reader: lua_Reader,
        dt: *mut core::ffi::c_void,
        chunkname: *const core::ffi::c_char,
        mode: *const core::ffi::c_char,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_dump(
        L: *mut lua_State,
        writer: lua_Writer,
        data: *mut core::ffi::c_void,
        strip: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_yieldk(
        L: *mut lua_State,
        nresults: core::ffi::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_resume(
        L: *mut lua_State,
        from: *mut lua_State,
        narg: core::ffi::c_int,
        nres: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_status(L: *mut lua_State) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_isyieldable(L: *mut lua_State) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_setwarnf(
        L: *mut lua_State,
        f: lua_WarnFunction,
        ud: *mut core::ffi::c_void,
    );

    pub(crate) unsafe fn lua_warning(
        L: *mut lua_State,
        msg: *const core::ffi::c_char,
        tocont: core::ffi::c_int,
    );

    pub(crate) unsafe fn lua_gc(L: *mut lua_State, what: core::ffi::c_int, ...)
        -> core::ffi::c_int;

    pub(crate) unsafe fn lua_error(L: *mut lua_State) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_next(L: *mut lua_State, idx: core::ffi::c_int) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_concat(L: *mut lua_State, n: core::ffi::c_int);

    pub(crate) unsafe fn lua_len(L: *mut lua_State, idx: core::ffi::c_int);

    pub(crate) unsafe fn lua_stringtonumber(
        L: *mut lua_State,
        s: *const core::ffi::c_char,
    ) -> usize;

    pub(crate) unsafe fn lua_getallocf(
        L: *mut lua_State,
        ud: *mut *mut core::ffi::c_void,
    ) -> lua_Alloc;

    pub(crate) unsafe fn lua_setallocf(L: *mut lua_State, f: lua_Alloc, ud: *mut core::ffi::c_void);

    pub(crate) unsafe fn lua_toclose(L: *mut lua_State, idx: core::ffi::c_int);

    pub(crate) unsafe fn lua_closeslot(L: *mut lua_State, idx: core::ffi::c_int);

    pub(crate) unsafe fn lua_getstack(
        L: *mut lua_State,
        level: core::ffi::c_int,
        ar: *mut lua_Debug,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_getinfo(
        L: *mut lua_State,
        what: *const core::ffi::c_char,
        ar: *mut lua_Debug,
    ) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_getlocal(
        L: *mut lua_State,
        ar: *const lua_Debug,
        n: core::ffi::c_int,
    ) -> *const core::ffi::c_char;

    pub(crate) unsafe fn lua_setlocal(
        L: *mut lua_State,
        ar: *const lua_Debug,
        n: core::ffi::c_int,
    ) -> *const core::ffi::c_char;

    pub(crate) unsafe fn lua_getupvalue(
        L: *mut lua_State,
        funcindex: core::ffi::c_int,
        n: core::ffi::c_int,
    ) -> *const core::ffi::c_char;

    pub(crate) unsafe fn lua_setupvalue(
        L: *mut lua_State,
        funcindex: core::ffi::c_int,
        n: core::ffi::c_int,
    ) -> *const core::ffi::c_char;

    pub(crate) unsafe fn lua_upvalueid(
        L: *mut lua_State,
        fidx: core::ffi::c_int,
        n: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;

    pub(crate) unsafe fn lua_upvaluejoin(
        L: *mut lua_State,
        fidx1: core::ffi::c_int,
        n1: core::ffi::c_int,
        fidx2: core::ffi::c_int,
        n2: core::ffi::c_int,
    );

    pub(crate) unsafe fn lua_sethook(
        L: *mut lua_State,
        func: lua_Hook,
        mask: core::ffi::c_int,
        count: core::ffi::c_int,
    );

    pub(crate) unsafe fn lua_gethook(L: *mut lua_State) -> lua_Hook;

    pub(crate) unsafe fn lua_gethookmask(L: *mut lua_State) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_gethookcount(L: *mut lua_State) -> core::ffi::c_int;

    pub(crate) unsafe fn lua_setcstacklimit(
        L: *mut lua_State,
        limit: core::ffi::c_uint,
    ) -> core::ffi::c_int;
}
