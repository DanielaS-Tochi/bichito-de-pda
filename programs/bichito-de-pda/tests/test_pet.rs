
use {
    anchor_lang::{
        prelude::Pubkey,
        solana_program::{instruction::Instruction, system_program},
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_initialize_pet() {
    let program_id = bichito_de_pda::id();
    let payer = Keypair::new();
    let pet = Pubkey::find_program_address(
        &[bichito_de_pda::constants::PET_SEED, payer.pubkey().as_ref()],
        &program_id,
    )
    .0;
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/bichito_de_pda.so"
    ));
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let name = "Pompon".to_string();
    let instruction = Instruction::new_with_bytes(
        program_id,
        &bichito_de_pda::instruction::InitializePet {
            name: name.clone(),
        }
        .data(),
        bichito_de_pda::accounts::InitializePet {
            authority: payer.pubkey(),
            pet,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    let pet_account = svm.get_account(&pet).unwrap();
    let mut data: &[u8] = &pet_account.data;
    let pet_state = bichito_de_pda::state::Pet::try_deserialize(&mut data).unwrap();
    assert_eq!(pet_state.name, name);
    assert_eq!(pet_state.care_count, 0);
    assert_eq!(pet_state.authority, payer.pubkey());
}
