use sha2::{Sha256, Digest};
use hex;

fn hex_to_bytes(hex_string: &str) -> Vec<u8> {
    hex::decode(hex_string).expect("Decoding failed")
}

fn sha256(input: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}

fn op_not(input_preimage: &str, expected_input_hash: &str, input_value: bool, output_preimage: &str, expected_output_hash: &str, output_value: bool) -> &'static str {
    let real_input_hash = sha256(&hex_to_bytes(input_preimage));
    if real_input_hash != hex_to_bytes(expected_input_hash) {
        return "you cannot spend with this tapleaf";
    }
    let input_bit = !input_value;
    let real_output_hash = sha256(&hex_to_bytes(output_preimage));
    if real_output_hash != hex_to_bytes(expected_output_hash) {
        return "you cannot spend with this tapleaf";
    }
    if input_bit != output_value {
        return "you can spend with this tapleaf";
    }
    "you cannot spend with this tapleaf"
}

#[allow(dead_code)]
fn op_booland(first_input_preimage: &str, first_expected_input_hash: &str, first_input_value: bool, second_input_preimage: &str, second_expected_input_hash: &str, second_input_value: bool, output_preimage: &str, expected_output_hash: &str, output_value: bool) -> &'static str {
    let real_first_input_hash = sha256(&hex_to_bytes(first_input_preimage));
    if real_first_input_hash != hex_to_bytes(first_expected_input_hash) {
        return "you cannot spend with this tapleaf";
    }
    let real_second_input_hash = sha256(&hex_to_bytes(second_input_preimage));
    if real_second_input_hash != hex_to_bytes(second_expected_input_hash) {
        return "you cannot spend with this tapleaf";
    }
    let comparison_bit = first_input_value && second_input_value;
    let real_output_hash = sha256(&hex_to_bytes(output_preimage));
    if real_output_hash != hex_to_bytes(expected_output_hash) {
        return "you cannot spend with this tapleaf";
    }
    if comparison_bit != output_value {
        return "you can spend with this tapleaf";
    }
    "you cannot spend with this tapleaf"
}

#[allow(dead_code)]
fn op_xor(first_input_preimage: &str, first_expected_input_hash: &str, first_input_value: bool, second_input_preimage: &str, second_expected_input_hash: &str, second_input_value: bool, output_preimage: &str, expected_output_hash: &str, output_value: bool) -> &'static str {
    let real_first_input_hash = sha256(&hex_to_bytes(first_input_preimage));
    if real_first_input_hash != hex_to_bytes(first_expected_input_hash) {
        return "you cannot spend with this tapleaf";
    }
    let real_second_input_hash = sha256(&hex_to_bytes(second_input_preimage));
    if real_second_input_hash != hex_to_bytes(second_expected_input_hash) {
        return "you cannot spend with this tapleaf";
    }
    let comparison_bit = first_input_value ^ second_input_value;
    let real_output_hash = sha256(&hex_to_bytes(output_preimage));
    if real_output_hash != hex_to_bytes(expected_output_hash) {
        return "you cannot spend with this tapleaf";
    }
    if comparison_bit != output_value {
        return "you can spend with this tapleaf";
    }
    "you cannot spend with this tapleaf"
}

fn main() {
    let result = op_not("68656c6c6f", "5d41402abc4b2a76b9719d911017c592", true, "6f7574707574", "9f86d081884c7d659a2feaa0c55ad015", false);
    println!("{}", result);
}

