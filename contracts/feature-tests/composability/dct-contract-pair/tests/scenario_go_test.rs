#[test]
fn init_go() {
    dharitri_sc_scenario::run_go("scenarios/init.scen.json");
}

#[ignore]
#[test]
fn reject_transfer_go() {
    dharitri_sc_scenario::run_go("scenarios/reject_transfer.scen.json");
}

#[test]
fn simple_transfer_full_go() {
    dharitri_sc_scenario::run_go("scenarios/simple_transfer_full.scen.json");
}

#[test]
fn simple_transfer_full_wrong_token_go() {
    dharitri_sc_scenario::run_go("scenarios/simple_transfer_full_wrong_token.scen.json");
}

#[test]
fn simple_transfer_half_go() {
    dharitri_sc_scenario::run_go("scenarios/simple_transfer_half.scen.json");
}
