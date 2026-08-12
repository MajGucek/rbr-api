fn main() {
    cc::Build::new()
        .cpp(true)
        .file("cpp/plugin.cpp")
        .include("cpp")
        .flag_if_supported("/EHsc")
        .compile("rbr_plugin_cpp");

    println!("cargo:rerun-if-changed=cpp/IPlugin.h");
    println!("cargo:rerun-if-changed=cpp/plugin.cpp");
}