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

    let target = build_target::target_triple().unwrap();
    if target == "x86_64-pc-windows-gnu" {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else {
        println!("cargo:rustc-link-lib=dylib=c++");
    }

    bindgen::Builder::default()
        .headers([
            "../../luau/VM/include/lua.h",
            "../../luau/VM/include/lualib.h",
            "../../luau/Compiler/include/luacode.h",
        ])
        .clang_arg(format!(
            "--target={}",
            target)
        )
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
        config.define("CMAKE_C_FLAGS", "-ffunction-sections -fdata-sections -fPIC");
        config.define(
            "CMAKE_CXX_FLAGS",
            "-ffunction-sections -fdata-sections -fPIC",
        );
        if let Ok(cc) = std::env::var("CC") {
            if !cc.is_empty() {
                config.define("CMAKE_C_COMPILER", cc);
            }
        }
        if let Ok(cxx) = std::env::var("CXX") {
            if !cxx.is_empty() {
                config.define("CMAKE_CXX_COMPILER", cxx);
            }
        }
    } else if target == "x86_64-apple-ios" {
        config.define("CMAKE_SYSTEM_NAME", "iOS");
        config.define("CMAKE_SYSTEM_PROCESSOR", "x86_64");
        config.define("CMAKE_OSX_ARCHITECTURES", "x86_64");
        config.define("CMAKE_OSX_SYSROOT", "/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator.sdk");
        config.define(
            "CMAKE_C_FLAGS",
            "-fPIC -m64 --target=x86_64-apple-ios-simulator -mios-simulator-version-min=17.5",
        );
        config.define(
            "CMAKE_CXX_FLAGS",
            "-fPIC -m64 --target=x86_64-apple-ios-simulator -mios-simulator-version-min=17.5",
        );
    } else if target == "aarch64-apple-ios" {
        config.define("CMAKE_SYSTEM_NAME", "iOS");
        config.define("CMAKE_SYSTEM_PROCESSOR", "arm64");
        config.define("CMAKE_OSX_ARCHITECTURES", "arm64");
        config.define("CMAKE_OSX_SYSROOT", "/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk");
        config.define(
            "CMAKE_C_FLAGS",
            "-fPIC --target=arm64-apple-ios -miphoneos-version-min=17.5",
        );
        config.define(
            "CMAKE_CXX_FLAGS",
            "-fPIC --target=arm64-apple-ios -miphoneos-version-min=17.5",
        );
    } else if target == "wasm32-unknown-unknown" {
        config.define("CMAKE_SYSTEM_NAME", "Emscripten");
        config.define("CMAKE_C_COMPILER", "emcc");
        config.define("CMAKE_CXX_COMPILER", "em++");
        config.define("CMAKE_AR", "emar");
        config.define("CMAKE_RANLIB", "emranlib");
        config.define("CMAKE_C_FLAGS", "-fPIC");
        config.define("CMAKE_CXX_FLAGS", "-fPIC");
    } else if target == "aarch64-linux-android" {
        let ndk_home = std::env::var("ANDROID_NDK_HOME").unwrap();
        let ndk_bin = format!("{}/toolchains/llvm/prebuilt/linux-x86_64/bin", ndk_home);
        config.define("CMAKE_SYSTEM_NAME", "Android");
        config.define("CMAKE_SYSTEM_PROCESSOR", "aarch64");
        config.define("CMAKE_ANDROID_ARCH_ABI", "arm64-v8a");
        config.define("CMAKE_ANDROID_NDK", &ndk_home);
        config.define("CMAKE_ANDROID_STL_TYPE", "c++_static");
        config.define("CMAKE_ANDROID_API", "26");
        config.define(
            "CMAKE_C_COMPILER",
            format!("{}/aarch64-linux-android26-clang", ndk_bin),
        );
        config.define(
            "CMAKE_CXX_COMPILER",
            format!("{}/aarch64-linux-android26-clang++", ndk_bin),
        );
        config.define(
            "CMAKE_C_FLAGS",
            "-DANDROID -ffunction-sections -fdata-sections -fPIC",
        );
        config.define(
            "CMAKE_CXX_FLAGS",
            "-DANDROID -ffunction-sections -fdata-sections -fPIC",
        );
    } else if target == "x86_64-linux-android" {
        let ndk_home = std::env::var("ANDROID_NDK_HOME").unwrap();
        let ndk_bin = format!("{}/toolchains/llvm/prebuilt/linux-x86_64/bin", ndk_home);
        config.define("CMAKE_SYSTEM_NAME", "Android");
        config.define("CMAKE_SYSTEM_PROCESSOR", "x86_64");
        config.define("CMAKE_ANDROID_ARCH_ABI", "x86_64");
        config.define("CMAKE_ANDROID_NDK", &ndk_home);
        config.define("CMAKE_ANDROID_STL_TYPE", "c++_static");
        config.define("CMAKE_ANDROID_API", "26");
        config.define(
            "CMAKE_C_COMPILER",
            format!("{}/x86_64-linux-android26-clang", ndk_bin),
        );
        config.define(
            "CMAKE_CXX_COMPILER",
            format!("{}/x86_64-linux-android26-clang++", ndk_bin),
        );
        config.define(
            "CMAKE_C_FLAGS",
            "-DANDROID -ffunction-sections -fdata-sections -fPIC -m64",
        );
        config.define(
            "CMAKE_CXX_FLAGS",
            "-DANDROID -ffunction-sections -fdata-sections -fPIC -m64",
        );
    }

    config
}
