fn main() {
    new_cmake_config().build_target("Luau.Compiler").build();

    let dst = new_cmake_config().build_target("Luau.Require").build();

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
        .clang_arg(format!(
            "--target={}",
            build_target::target_triple().unwrap()
        ))
        .generate()
        .unwrap()
        .write_to_file("src/luau.rs")
        .unwrap();
}

fn new_cmake_config() -> cmake::Config {
    let mut config = cmake::Config::new("../../luau");

    let target = build_target::target_triple().unwrap();

    if target == "aarch64-unknown-linux-gnu" {
        config.define("CMAKE_SYSTEM_NAME", "Linux");
        config.define("CMAKE_SYSTEM_PROCESSOR", "aarch64");
        config.define("CMAKE_C_COMPILER", "aarch64-linux-gnu-gcc");
        config.define("CMAKE_CXX_COMPILER", "aarch64-linux-gnu-g++");
    } else if target == "i686-unknown-linux-gnu" {
        config.define("CMAKE_SYSTEM_NAME", "Linux");
        config.define("CMAKE_SYSTEM_PROCESSOR", "x86");
        config.define("CMAKE_C_COMPILER", "gcc");
        config.define("CMAKE_CXX_COMPILER", "g++");
        config.define("CMAKE_C_FLAGS", "-m32");
        config.define("CMAKE_CXX_FLAGS", "-m32");
    } else if target == "x86_64-apple-ios" {
        config.define("CMAKE_SYSTEM_NAME", "iOS");
        config.define("CMAKE_SYSTEM_PROCESSOR", "x86_64");
        config.define("CMAKE_OSX_ARCHITECTURES", "x86_64");
        config.define("CMAKE_OSX_SYSROOT", "/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator.sdk");
        config.define("CMAKE_C_FLAGS", "-fPIC -m64 --target=x86_64-apple-ios-simulator -mios-simulator-version-min=17.5");
        config.define("CMAKE_CXX_FLAGS", "-fPIC -m64 --target=x86_64-apple-ios-simulator -mios-simulator-version-min=17.5");
    } else if target == "aarch64-apple-ios" {
        config.define("CMAKE_SYSTEM_NAME", "iOS");
        config.define("CMAKE_SYSTEM_PROCESSOR", "arm64");
        config.define("CMAKE_OSX_ARCHITECTURES", "arm64");
        config.define("CMAKE_OSX_SYSROOT", "/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk");
        config.define("CMAKE_C_FLAGS", "-fPIC --target=arm64-apple-ios -miphoneos-version-min=17.5");
        config.define("CMAKE_CXX_FLAGS", "-fPIC --target=arm64-apple-ios -miphoneos-version-min=17.5");
    }
    config
}
