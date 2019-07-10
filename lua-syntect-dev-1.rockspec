rockspec_format = "3.0"
package = "lua-syntect"
version = "dev-1"
source = {
   url = "git://github.com/hishamhm/lua-syntect"
}
description = {
   summary = "Minimal Lua binding for syntect, a syntax highlighting library",
   detailed = [[
      This is a minimal Lua binding for syntect, a syntax highlighting library
      written in Rust.
   ]],
   homepage = "https://github.com/hishamhm/lua-syntect",
   license = "MIT"
}
build_dependencies = {
   "luarocks-build-rust",
}
build = {
   type = "rust",
   modules = {
      "syntect"
   }
}
