use crate::mock::*;
use crate::*;

use precompile_utils::testing::*;
use precompile_utils::EvmDataWriter;
use sp_core::H160;

fn precompiles() -> TestPrecompileSet<Runtime> {
    PrecompilesValue::get()
}

#[test]
fn wrong_assets_len_or_fee_index_reverts() {
    ExtBuilder::default().build().execute_with(|| {
        precompiles()
            .prepare_test(
                TestAccount::Alice,
                PRECOMPILE_ADDRESS,
                EvmDataWriter::new_with_selector(Action::AssetsWithdraw)
                    .write(vec![Address::from(H160::repeat_byte(0xF1))])
                    .write(Vec::<U256>::new())
                    .write(H256::repeat_byte(0xF1))
                    .write(true)
                    .write(U256::from(0_u64))
                    .write(U256::from(0_u64))
                    .build(),
            )
            .expect_no_logs()
            .execute_reverts(|output| output == b"Assets resolution failure.");

        precompiles()
            .prepare_test(
                TestAccount::Alice,
                PRECOMPILE_ADDRESS,
                EvmDataWriter::new_with_selector(Action::AssetsWithdraw)
                    .write(vec![Address::from(Runtime::asset_id_to_address(1u128))])
                    .write(vec![U256::from(42000u64)])
                    .write(H256::repeat_byte(0xF1))
                    .write(true)
                    .write(U256::from(0_u64))
                    .write(U256::from(2_u64))
                    .build(),
            )
            .expect_no_logs()
            .execute_reverts(|output| output == b"Bad fee index.");
    });
}

#[test]
fn correct_arguments_works() {
    ExtBuilder::default().build().execute_with(|| {
        precompiles()
            .prepare_test(
                TestAccount::Alice,
                PRECOMPILE_ADDRESS,
                EvmDataWriter::new_with_selector(Action::AssetsWithdraw)
                    .write(vec![Address::from(Runtime::asset_id_to_address(1u128))])
                    .write(vec![U256::from(42000u64)])
                    .write(H256::repeat_byte(0xF1))
                    .write(true)
                    .write(U256::from(0_u64))
                    .write(U256::from(0_u64))
                    .build(),
            )
            .expect_no_logs()
            .execute_returns(EvmDataWriter::new().write(true).build());
    });
}
