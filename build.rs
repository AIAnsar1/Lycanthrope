fn main() {
    let mut build = cc::Build::new();

    build
        .file("csrc/packet.c")
        .file("csrc/checksum.c")
        .file("csrc/rawsock.c")
        .include("csrc/include");

    if cfg!(target_env = "msvc") {
        build.flag("/O2");
    } else {
        build.flag("-Wall");
        build.flag("-Wextra");
        build.flag("-O2");
        build.flag("-Wno-unused-parameter");
    }

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=ws2_32");

        // Npcap SDK — укажи свой путь!
        let npcap_sdk = std::env::var("NPCAP_SDK")
            .unwrap_or_else(|_| r"D:\Program Files\Network\npcap".to_string());
        println!("cargo:rustc-link-search=native={}/Lib/x64", npcap_sdk);
    }

    build.compile("lycanthrope_core");

    println!("cargo:rerun-if-changed=csrc/");
}
