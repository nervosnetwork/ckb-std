use blake2b_rs::Blake2bBuilder;
use ckb_mock_tx_types::{MockCellDep, MockInfo, MockInput, MockTransaction, ReprMockTransaction};
use ckb_testtool::ckb_types::core;
use ckb_testtool::context::Context;
use ckb_x64_simulator::RunningSetup;
use serde_json::to_string_pretty;
use std::fs;

pub fn dump_mock_tx(
    test_case_name: &str,
    tx: &core::TransactionView,
    context: &Context,
    setup: &RunningSetup,
) {
    let mock_tx = build_mock_tx(&tx, &context);
    let repr_tx: ReprMockTransaction = mock_tx.into();
    let tx_json = to_string_pretty(&repr_tx).expect("serialize to json");
    fs::write(
        format!("simulator/data/{}-mock-tx.json", test_case_name),
        tx_json,
    )
    .expect("write tx to local file");
    let setup_json = to_string_pretty(setup).expect("serialize to json");
    fs::write(
        format!("simulator/data/{}-setup.json", test_case_name),
        setup_json,
    )
    .expect("write setup to local file");
}

fn build_mock_tx(tx: &core::TransactionView, context: &Context) -> MockTransaction {
    let mock_inputs = tx
        .inputs()
        .into_iter()
        .map(|input| {
            let (output, data) = context
                .get_cell(&input.previous_output())
                .expect("get cell");
            MockInput {
                input,
                output,
                data,
                header: None,
            }
        })
        .collect();
    let mock_cell_deps = tx
        .cell_deps()
        .into_iter()
        .map(|cell_dep| {
            if cell_dep.dep_type() == core::DepType::DepGroup.into() {
                panic!("Implement dep group support later!");
            }
            let (output, data) = context.get_cell(&cell_dep.out_point()).expect("get cell");
            MockCellDep {
                cell_dep,
                output,
                data,
                header: None,
            }
        })
        .collect();
    let mock_info = MockInfo {
        inputs: mock_inputs,
        cell_deps: mock_cell_deps,
        header_deps: vec![],
        extensions: Default::default(),
    };
    MockTransaction {
        mock_info,
        tx: tx.data(),
    }
}

pub fn blake2b_256<T: AsRef<[u8]>>(s: T) -> [u8; 32] {
    let mut result = [0u8; 32];
    let mut blake2b = Blake2bBuilder::new(32)
        .personal(b"ckb-default-hash")
        .build();
    blake2b.update(s.as_ref());
    blake2b.finalize(&mut result);
    result
}
