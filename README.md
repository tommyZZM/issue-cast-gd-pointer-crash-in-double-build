```
Godot Engine v4.3.beta.custom_build.5833f5978 (2024-06-08 12:11:35 UTC) - https://godotengine.org
Vulkan 1.3.278 - Forward+ - Using Device #0: NVIDIA - NVIDIA GeForce RTX 4090 D

thread '<unnamed>' panicked at C:\Users\Mehere\.cargo\git\checkouts\gdext-76630c89719e160c\7eec09c\godot-core\src\obj:
Class issue_cast_gd_pointer_crash_in_double_build::MyCustomMesh -- null instance; does the class have a Godot creator f?
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library\std\src\panicking.rs:645
   1: core::panicking::panic_fmt
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library\core\src\panicking.rs:72
   2: godot_core::obj::raw::RawGd<issue_cast_gd_pointer_crash_in_double_build::MyCustomMesh>::resolve_instance_ptr<issu>
             at C:\Users\Mehere\.cargo\git\checkouts\gdext-76630c89719e160c\7eec09c\godot-core\src\obj\raw.rs:428
   3: godot_core::obj::raw::RawGd<issue_cast_gd_pointer_crash_in_double_build::MyCustomMesh>::storage_unbounded<issue_c>
             at C:\Users\Mehere\.cargo\git\checkouts\gdext-76630c89719e160c\7eec09c\godot-core\src\obj\raw.rs:413
   4: godot_core::obj::raw::RawGd<issue_cast_gd_pointer_crash_in_double_build::MyCustomMesh>::storage<issue_cast_gd_poi>
             at C:\Users\Mehere\.cargo\git\checkouts\gdext-76630c89719e160c\7eec09c\godot-core\src\obj\raw.rs:389
   5: godot_core::obj::raw::RawGd<issue_cast_gd_pointer_crash_in_double_build::MyCustomMesh>::bind<issue_cast_gd_pointe>
             at C:\Users\Mehere\.cargo\git\checkouts\gdext-76630c89719e160c\7eec09c\godot-core\src\obj\raw.rs:370
   6: godot_core::obj::gd::Gd<issue_cast_gd_pointer_crash_in_double_build::MyCustomMesh>::bind<issue_cast_gd_pointer_cr>
             at C:\Users\Mehere\.cargo\git\checkouts\gdext-76630c89719e160c\7eec09c\godot-core\src\obj\gd.rs:160
   7: issue_cast_gd_pointer_crash_in_double_build::impl$40::parse_begin
             at H:\WorkShop\_TEST\issue-cast-gd-pointer-crash-in-double-build\rust\src\lib.rs:95
   8: issue_cast_gd_pointer_crash_in_double_build::impl$42::__virtual_call::virtual_fn::closure$0
             at H:\WorkShop\_TEST\issue-cast-gd-pointer-crash-in-double-build\rust\src\lib.rs:83
   9: core::ops::function::FnOnce::call_once<issue_cast_gd_pointer_crash_in_double_build::impl$42::__virtual_call::virt:
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs:250
  10: godot_core::meta::signature::impl$18::in_ptrcall<tuple$<>,godot_core::obj::gd::Gd<godot_core::gen::classes::objec>
             at C:\Users\Mehere\.cargo\git\checkouts\gdext-76630c89719e160c\7eec09c\godot-core\src\meta\signature.rs:9
  11: issue_cast_gd_pointer_crash_in_double_build::impl$42::__virtual_call::virtual_fn
             at H:\WorkShop\_TEST\issue-cast-gd-pointer-crash-in-double-build\rust\src\lib.rs:83
  12: embree::TaskScheduler::wait
  13: embree::TaskScheduler::wait
  14: embree::TaskScheduler::wait
  15: embree::TaskScheduler::wait
  16: embree::TaskScheduler::wait
  17: embree::TaskScheduler::wait
  18: embree::TaskScheduler::wait
  19: embree::TaskScheduler::wait
  20: embree::TaskScheduler::wait
  21: embree::TaskScheduler::wait
  22: embree::TaskScheduler::wait
  23: <unknown>
  24: embree::TaskScheduler::wait
  25: embree::TaskScheduler::wait
  26: embree::TaskScheduler::wait
  27: embree::TaskScheduler::wait
  28: embree::TaskScheduler::wait
  29: <unknown>
  30: embree::TaskScheduler::wait
  31: embree::TaskScheduler::wait
  32: embree::TaskScheduler::wait
  33: embree::TaskScheduler::wait
  34: embree::TaskScheduler::wait
  35: embree::TaskScheduler::wait
  36: embree::TaskScheduler::wait
  37: embree::TaskScheduler::wait
  38: <unknown>
  39: <unknown>
  40: embree::TaskScheduler::wait
  41: embree::TaskScheduler::wait
  42: <unknown>
  43: <unknown>
  44: <unknown>
  45: <unknown>
  46: <unknown>
  47: embree::TaskScheduler::wait
  48: BaseThreadInitThunk
  49: RtlUserThreadStart
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

================================================================
CrashHandlerException: Program crashed
Engine version: Godot Engine v4.3.beta.custom_build (5833f597865c773fae3ee09fc4e31d4a243f812d)
Dumping the backtrace. Please include this when reporting the bug to the project developer.
[0] <couldn't map PC to fn name>
[1] <couldn't map PC to fn name>
[2] <couldn't map PC to fn name>
[3] panic_unwind::__rust_start_panic (/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library\panic_unwind\src\lib.rs:1)
[4] std::panicking::rust_panic (/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library\std\src\panicking.rs:831)
[5] std::panicking::rust_panic_with_hook (/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library\std\src\panicking.rs:)
[6] std::panicking::begin_panic_handler::closure$0 (/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library\std\src\pan)
[7] std::sys_common::backtrace::__rust_end_short_backtrace<std::panicking::begin_panic_handler::closure_env$0,never$> ()
[8] std::panicking::begin_panic_handler (/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library\std\src\panicking.rs:6)
[9] core::panicking::panic_fmt (/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library\core\src\panicking.rs:72)
[10] godot_core::obj::raw::RawGd<issue_cast_gd_pointer_crash_in_double_build::MyCustomMesh>::resolve_instance_ptr<issue)
[11] godot_core::obj::raw::RawGd<issue_cast_gd_pointer_crash_in_double_build::MyCustomMesh>::storage_unbounded<issue_ca)
[12] godot_core::obj::raw::RawGd<issue_cast_gd_pointer_crash_in_double_build::MyCustomMesh>::storage<issue_cast_gd_poin)
[13] godot_core::obj::raw::RawGd<issue_cast_gd_pointer_crash_in_double_build::MyCustomMesh>::bind<issue_cast_gd_pointer)
[14] godot_core::obj::gd::Gd<issue_cast_gd_pointer_crash_in_double_build::MyCustomMesh>::bind<issue_cast_gd_pointer_cra)
[15] issue_cast_gd_pointer_crash_in_double_build::impl$40::parse_begin (H:\WorkShop\_TEST\issue-cast-gd-pointer-crash-i)
[16] issue_cast_gd_pointer_crash_in_double_build::impl$42::__virtual_call::virtual_fn::closure$0 (H:\WorkShop\_TEST\iss)
[17] core::ops::function::FnOnce::call_once<issue_cast_gd_pointer_crash_in_double_build::impl$42::__virtual_call::virtu)
[18] godot_core::meta::signature::impl$18::in_ptrcall<tuple$<>,godot_core::obj::gd::Gd<godot_core::gen::classes::object)
[19] issue_cast_gd_pointer_crash_in_double_build::impl$42::__virtual_call::virtual_fn (H:\WorkShop\_TEST\issue-cast-gd-)
[20] embree::TaskScheduler::wait
[21] embree::TaskScheduler::wait
[22] embree::TaskScheduler::wait
[23] embree::TaskScheduler::wait
[24] embree::TaskScheduler::wait
[25] embree::TaskScheduler::wait
[26] embree::TaskScheduler::wait
[27] embree::TaskScheduler::wait
[28] embree::TaskScheduler::wait
[29] embree::TaskScheduler::wait
[30] embree::TaskScheduler::wait
[31] <couldn't map PC to fn name>
[32] embree::TaskScheduler::wait
[33] embree::TaskScheduler::wait
[34] embree::TaskScheduler::wait
[35] embree::TaskScheduler::wait
[36] embree::TaskScheduler::wait
[37] <couldn't map PC to fn name>
[38] embree::TaskScheduler::wait
[39] embree::TaskScheduler::wait
[40] embree::TaskScheduler::wait
[41] embree::TaskScheduler::wait
[42] embree::TaskScheduler::wait
[43] embree::TaskScheduler::wait
[44] embree::TaskScheduler::wait
[45] embree::TaskScheduler::wait
[46] <couldn't map PC to fn name>
[47] <couldn't map PC to fn name>
[48] embree::TaskScheduler::wait
[49] embree::TaskScheduler::wait
[50] <couldn't map PC to fn name>
[51] <couldn't map PC to fn name>
[52] <couldn't map PC to fn name>
[53] <couldn't map PC to fn name>
[54] <couldn't map PC to fn name>
[55] embree::TaskScheduler::wait
[56] BaseThreadInitThunk
-- END OF BACKTRACE --
```