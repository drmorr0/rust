use crate::spec::{LinkerFlavor, LldFlavor, Target};

pub fn target() -> Target {
    let mut base = super::windows_msvc_base::opts();
    base.cpu = "pentium4".to_string();
    base.max_atomic_width = Some(64);

    let pre_link_args_msvc = vec![
        // Mark all dynamic libraries and executables as compatible with the larger 4GiB address
        // space available to x86 Windows binaries on x86_64.
        "/LARGEADDRESSAWARE".to_string(),
        // Ensure the linker will only produce an image if it can also produce a table of
        // the image's safe exception handlers.
        // https://docs.microsoft.com/en-us/cpp/build/reference/safeseh-image-has-safe-exception-handlers
        "/SAFESEH".to_string(),
    ];
    base.pre_link_args.entry(LinkerFlavor::Msvc).or_default().extend(pre_link_args_msvc.clone());
    base.pre_link_args
        .entry(LinkerFlavor::Lld(LldFlavor::Link))
        .or_default()
        .extend(pre_link_args_msvc);
    // Workaround for #95429
    base.has_thread_local = false;

    Target {
        llvm_target: "i686-pc-windows-msvc".to_string(),
        pointer_width: 32,
        data_layout: "e-m:x-p:32:32-p270:32:32-p271:32:32-p272:64:64-\
            i64:64-f80:128-n8:16:32-a:0:32-S32"
            .to_string(),
        arch: "x86".to_string(),
        options: base,
    }
}
