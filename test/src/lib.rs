#[cfg(test)]
mod tests {
    use ckb_tool::{
        bytes::Bytes,
        ckb_types::{
            core::ScriptHashType,
            packed::{CellOutput, Script},
            prelude::*,
        },
        testtool::{context::Context, tx_builder::TxBuilder},
    };
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn it_works() {
        let mut code = Vec::new();
        File::open("contract/target/riscv64imac-unknown-none-elf/release/contract")
            .unwrap()
            .read_to_end(&mut code)
            .expect("read code");
        let code = Bytes::from(code);
        let mut context = Context::default();
        context.deploy_contract(code.clone());
        let lock_code_hash = CellOutput::calc_data_hash(&code);
        let tx = TxBuilder::default()
            .lock_script(
                Script::new_builder()
                    .code_hash(lock_code_hash)
                    .hash_type(ScriptHashType::Data.into())
                    .build()
                    .as_bytes(),
            )
            .inject_and_build(&mut context)
            .expect("build tx");
        println!("{:#x}", tx.hash());
        let verify_result = context.verify_tx(&tx, 500000000u64);
        verify_result.expect("pass test");
    }
}
