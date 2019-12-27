/// Converts a java class name
/// 
/// # Examples
/// 
/// * **input:**    com.maulingmonkey.jni_bindgen.example_jni_java.RustJniGlueAdder$Inner
/// * **output:**   com_maulingmonkey_jni_1bindgen_example_1jni_1java_RustJniGlueAdder_00024Inner
/// 
/// # Does *not* handle:
/// 
/// * Prefixing "Java_" to C methods
/// * Java keywords
/// * Wrapping identifiers in L...; for arguments
pub fn java_fqn_class_name_to_c_identifier(java: &str) -> String {
    let mut out = String::new();

    for ch in java.chars() {
        match ch {
            '_' => out.push_str("_1"),
            ';' => out.push_str("_2"),
            '[' => out.push_str("_3"),
            '.' => out.push_str("_"),
            '$' => out.push_str("_00024"),
            ch @ '0'..='9' => out.push(ch),
            ch @ 'a'..='z' => out.push(ch),
            ch @ 'A'..='Z' => out.push(ch),
            ch => out.push_str(&format!("_0{:04x}", ch as usize)),
        }
    }

    out
}
