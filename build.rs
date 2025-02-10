use std::path::Path;
use cc;

fn main() {

    let lua_path: &Path = Path::new("third-party/lua-5.4.7");

    cc::Build::new()
    .file("third-party/lua-5.4.7/lapi.c")
    .file("third-party/lua-5.4.7/lauxlib.c")
    .file("third-party/lua-5.4.7/lbaselib.c")
    .file("third-party/lua-5.4.7/lcode.c")
    .file("third-party/lua-5.4.7/lcorolib.c")
    .file("third-party/lua-5.4.7/lctype.c")
    .file("third-party/lua-5.4.7/ldblib.c")
    .file("third-party/lua-5.4.7/ldebug.c")
    .file("third-party/lua-5.4.7/ldo.c")
    .file("third-party/lua-5.4.7/ldump.c")
    .file("third-party/lua-5.4.7/lfunc.c")
    .file("third-party/lua-5.4.7/lgc.c")
    .file("third-party/lua-5.4.7/linit.c")
    .file("third-party/lua-5.4.7/liolib.c")
    .file("third-party/lua-5.4.7/llex.c")
    .file("third-party/lua-5.4.7/lmathlib.c")
    .file("third-party/lua-5.4.7/lmem.c")
    .file("third-party/lua-5.4.7/loadlib.c")
    .file("third-party/lua-5.4.7/lobject.c")
    .file("third-party/lua-5.4.7/lopcodes.c")
    .file("third-party/lua-5.4.7/loslib.c")
    .file("third-party/lua-5.4.7/lparser.c")
    .file("third-party/lua-5.4.7/lstate.c")
    .file("third-party/lua-5.4.7/lstring.c")
    .file("third-party/lua-5.4.7/lstrlib.c")
    .file("third-party/lua-5.4.7/ltable.c")
    .file("third-party/lua-5.4.7/ltablib.c")
    .file("third-party/lua-5.4.7/ltm.c")
    .file("third-party/lua-5.4.7/lua.c")
    .file("third-party/lua-5.4.7/luac.c")
    .file("third-party/lua-5.4.7/lundump.c")
    .file("third-party/lua-5.4.7/lutf8lib.c")
    .file("third-party/lua-5.4.7/lvm.c")
    .file("third-party/lua-5.4.7/lzio.c")
    .include(lua_path)
    .compile("lua");
}
