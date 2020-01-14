#[cfg(test)]
mod tests {
    use ckb_contract_tool::{bytes::Bytes, Context, TxBuilder};
    use std::fs::File;
    use std::io::Read;

    #[test] fn it_works() {
        let mut code = Vec::new();
        File::open("contract/target/riscv64imac-unknown-none-elf/release/contract")
            .unwrap()
            .read_to_end(&mut code)
            .expect("read code");
        let code = Bytes::from(code);
        let mut context = Context::default();
        context.deploy_contract(code.clone());
        let tx = TxBuilder::default()
            .lock_bin(code)
            .inject_and_build(&mut context)
            .expect("build tx");
        println!("{:#x}", tx.hash());
        let verify_result = context.verify_tx(&tx, 500000000u64);
        verify_result.expect("pass test");
    }
}
