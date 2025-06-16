fn main() {
    new_cnake_config()
        .build_target("Luau.Compiler")
        .build();

    let dst = new_cnake_config()
        .build_target("Luau.Require")
        .build();

    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=Luau.VM");
    println!("cargo:rustc-link-lib=static=Luau.Ast");
    println!("cargo:rustc-link-lib=static=Luau.Compiler");
    println!("cargo:rustc-link-lib=static=Luau.Config");
    println!("cargo:rustc-link-lib=static=Luau.RequireNavigator");
    println!("cargo:rustc-link-lib=static=Luau.Require");
    println!("cargo:rustc-link-lib=dylib=c++");

    bindgen::Builder::default()
        .headers([
            "../../luau/VM/include/lua.h",
            "../../luau/VM/include/lualib.h",
            "../../luau/Compiler/include/luacode.h",
        ])
        .generate()
        .unwrap()
        .write_to_file("src/luau.rs")
        .unwrap();
}

fn new_csbindgen_builder(src: &'static str) -> csbindgen::Builder {
    csbindgen::Builder::default()
        .input_bindgen_file(src)
        .rust_method_prefix("ffi_")
        .csharp_entry_point_prefix("ffi_")
        .csharp_method_prefix("")
        .csharp_namespace("Luau.Native")
        .csharp_dll_name("libluau")
        .csharp_class_accessibility("public")
        .csharp_generate_const_filter(|x| x.starts_with("LUA"))
        .csharp_use_function_pointer(false)
}

fn new_cnake_config() -> cmake::Config {
    let mut config = cmake::Config::new("../../luau");

    let target = build_target::target_triple().unwrap();
    if target == "aarch64-unknown-linux-gnu" {
        config.define("CMAKE_SYSTEM_NAME", "Linux");
        config.define("CMAKE_SYSTEM_PROCESSOR", "aarch64");
        config.define("CMAKE_C_COMPILER", "aarch64-linux-gnu-gcc");
        config.define("CMAKE_CXX_COMPILER", "aarch64-linux-gnu-g++");
    }

    return config;
}