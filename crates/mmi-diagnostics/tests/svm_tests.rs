use mmi_diagnostics::{
    LoopbackSimulator, SvmSolver, SVM_CHANNEL_15_XOR_CIPHER, SVM_CHANNEL_15_XOR_KEY,
};

#[test]
fn test_svm_03276_xor_cipher_math() {
    assert_eq!(SVM_CHANNEL_15_XOR_KEY, 51666);
    assert_eq!(SVM_CHANNEL_15_XOR_CIPHER, 51666);

    // Reference test vector 1: 51620 -> 118
    assert_eq!(SvmSolver::calculate_channel_15_xor(51620), 118);
    assert_eq!(SvmSolver::solve_svm_03276(51620), 118);

    // Reference test vector 2: 24581 -> 43479 (0x6005 ^ 0xC9D2 = 0xA9D7)
    assert_eq!(SvmSolver::calculate_channel_15_xor(24581), 43479);
    assert_eq!(SvmSolver::solve_svm_03276(24581), 43479);

    // Identity with 0
    assert_eq!(SvmSolver::calculate_channel_15_xor(0), 51666);

    // Involutory bijection: f(f(x)) == x
    for &val in &[0u16, 1, 42, 118, 24581, 43479, 51620, 65535] {
        let transformed = SvmSolver::calculate_channel_15_xor(val);
        let restored = SvmSolver::calculate_channel_15_xor(transformed);
        assert_eq!(restored, val);
    }
}

#[test]
fn test_svm_03175_rehash_math() {
    let (s1, s2, match_orig) = SvmSolver::simulate_svm_03175_rehash(5);
    assert_eq!(s1, 6);
    assert_eq!(s2, 5);
    assert!(match_orig);

    let (s1, s2, match_orig) = SvmSolver::simulate_svm_03175_rehash(42);
    assert_eq!(s1, 43);
    assert_eq!(s2, 42);
    assert!(match_orig);
}

#[test]
fn test_svm_03276_end_to_end_uds_resolution() {
    let mut sim = LoopbackSimulator::new();
    let initial_challenge = 24581u16;
    sim.set_channel_15(initial_challenge);
    assert!(sim.is_dtc_present(0x03276));

    let report = SvmSolver::resolve_svm(&mut sim, true).unwrap();

    assert!(report.connected);
    assert_eq!(report.original_channel_15, initial_challenge);
    assert_eq!(report.updated_channel_15, initial_challenge ^ SVM_CHANNEL_15_XOR_KEY);
    assert!(report.dtc_03276_cleared);
    assert!(report.gem_unlocked);

    // Verify in simulator state
    assert_eq!(sim.get_channel_15(), 43479);
    assert!(sim.is_gem_unlocked());
    assert!(!sim.is_dtc_present(0x03276));
}

#[test]
fn test_svm_03175_end_to_end_uds_resolution() {
    let mut sim = LoopbackSimulator::new();
    assert_eq!(sim.get_car_menu_setting(), 5);
    assert!(sim.is_dtc_present(0x03175));

    let report = SvmSolver::resolve_svm_03175(&mut sim).unwrap();

    assert!(report.rehash_executed);
    assert_eq!(report.initial_value, 5);
    assert_eq!(report.toggled_value, 6);
    assert_eq!(report.restored_value, 5);
    assert!(report.dtc_03175_cleared);

    // Verify in simulator state
    assert_eq!(sim.get_car_menu_setting(), 5);
    assert!(!sim.is_dtc_present(0x03175));
}
