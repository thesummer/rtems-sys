extern crate cc;
extern crate pkg_config;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    // The pkg-config::Config filters the flags which removes important info for RTEMS
    // So query the variables and assemble the flags manually
    let rtems_arch = pkg_config::get_variable("arm-rtems6-xilinx_zynq_a9_qemu", "RTEMS_ARCH").unwrap();
    let rtems_major = pkg_config::get_variable("arm-rtems6-xilinx_zynq_a9_qemu", "RTEMS_MAJOR").unwrap();
    let include_dir = pkg_config::get_variable("arm-rtems6-xilinx_zynq_a9_qemu", "includedir").unwrap();
    let lib_dir = pkg_config::get_variable("arm-rtems6-xilinx_zynq_a9_qemu", "libdir").unwrap();
    let abi_flags = pkg_config::get_variable("arm-rtems6-xilinx_zynq_a9_qemu", "ABI_FLAGS").unwrap();
    println!("cargo::metadata=abi_flags={abi_flags}");

    let abi_flags: Vec<&str> = abi_flags.split_whitespace().collect();
    
    // Tell Cargo that if the given file changes, to rerun this build script.
    // Use the `cc` crate to build a C file and statically link it.
    let mut build_config = cc::Build::new();
    build_config.file("src/rtemsconfig.c");
    build_config.compiler(format!("{rtems_arch}-rtems{rtems_major}-gcc"));
    build_config.include(include_dir);
    build_config.warnings(false);

    for flag in abi_flags {
        build_config.flag(flag);
    }
    build_config.compile("rtemsconfig");

    println!("cargo::rerun-if-changed=src/rtemsconfig.c"); 
    println!("cargo::rustc-link-search={lib_dir}");
}
