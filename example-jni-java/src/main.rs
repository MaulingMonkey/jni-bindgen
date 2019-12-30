#[path = "RustJniGlueAdder.rs"] mod jni_glue_adder;
#[path = "RustJniSysAdder.rs"]  mod jni_sys_adder;

pub fn main() {
	jni_glue_adder::do_test().unwrap();
	jni_sys_adder::do_test().unwrap();
}
