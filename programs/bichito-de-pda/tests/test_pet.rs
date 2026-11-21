
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

fn send_instruction(svm: &mut LiteSVM, instruction: Instruction, signer: &Keypair) -> bool {
    svm.expire_blockhash();

    let blockhash = svm.latest_blockhash();
    let message = Message::new_with_blockhash(
        &[instruction],
        Some(&signer.pubkey()),
        &blockhash,
    );
    let transaction = VersionedTransaction::try_new(VersionedMessage::Legacy(message), &[signer])
        .unwrap();

    let result = svm.send_transaction(transaction);
    if let Err(error) = &result {
        println!("LiteSVM transaction error: {error:?}");
    }
    result.is_ok()
}

fn pet_account_data(svm: &LiteSVM, pet: &Pubkey) -> bichito_de_pda::state::Pet {
    let account = svm.get_account(pet).unwrap();
    let mut data: &[u8] = &account.data;
    bichito_de_pda::state::Pet::try_deserialize(&mut data).unwrap()
}

#[test]
fn test_pet_registration_care_permissions_limit_and_close() {
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

    assert!(send_instruction(&mut svm, instruction, &payer));

    let pet_state = pet_account_data(&svm, &pet);
    assert_eq!(pet_state.name, name);
    assert_eq!(pet_state.care_count, 0);
    assert_eq!(pet_state.authority, payer.pubkey());

    let unauthorized = Keypair::new();
    svm.airdrop(&unauthorized.pubkey(), 1_000_000_000).unwrap();
    let unauthorized_care = Instruction::new_with_bytes(
        program_id,
        &bichito_de_pda::instruction::CareForPet {}.data(),
        bichito_de_pda::accounts::CareForPet {
            pet,
            authority: unauthorized.pubkey(),
        }
        .to_account_metas(None),
    );
    assert!(!send_instruction(&mut svm, unauthorized_care, &unauthorized));
    assert_eq!(pet_account_data(&svm, &pet).care_count, 0);

    let early_close = Instruction::new_with_bytes(
        program_id,
        &bichito_de_pda::instruction::ClosePet {}.data(),
        bichito_de_pda::accounts::ClosePet {
            pet,
            authority: payer.pubkey(),
        }
        .to_account_metas(None),
    );
    assert!(!send_instruction(&mut svm, early_close, &payer));
    assert_eq!(pet_account_data(&svm, &pet).care_count, 0);

    let unauthorized_close = Instruction::new_with_bytes(
        program_id,
        &bichito_de_pda::instruction::ClosePet {}.data(),
        bichito_de_pda::accounts::ClosePet {
            pet,
            authority: unauthorized.pubkey(),
        }
        .to_account_metas(None),
    );
    assert!(!send_instruction(
        &mut svm,
        unauthorized_close,
        &unauthorized
    ));
    assert_eq!(pet_account_data(&svm, &pet).care_count, 0);

    for expected_count in 1..=bichito_de_pda::constants::MAX_CARE_COUNT {
        let care = Instruction::new_with_bytes(
            program_id,
            &bichito_de_pda::instruction::CareForPet {}.data(),
            bichito_de_pda::accounts::CareForPet {
                pet,
                authority: payer.pubkey(),
            }
            .to_account_metas(None),
        );
        assert!(send_instruction(&mut svm, care, &payer));
        assert_eq!(pet_account_data(&svm, &pet).care_count, expected_count);
    }

    let extra_care = Instruction::new_with_bytes(
        program_id,
        &bichito_de_pda::instruction::CareForPet {}.data(),
        bichito_de_pda::accounts::CareForPet {
            pet,
            authority: payer.pubkey(),
        }
        .to_account_metas(None),
    );
    assert!(!send_instruction(&mut svm, extra_care, &payer));
    assert_eq!(
        pet_account_data(&svm, &pet).care_count,
        bichito_de_pda::constants::MAX_CARE_COUNT
    );

    let balance_before_close = svm.get_balance(&payer.pubkey()).unwrap();
    let close = Instruction::new_with_bytes(
        program_id,
        &bichito_de_pda::instruction::ClosePet {}.data(),
        bichito_de_pda::accounts::ClosePet {
            pet,
            authority: payer.pubkey(),
        }
        .to_account_metas(None),
    );
    assert!(send_instruction(&mut svm, close, &payer));
    assert!(svm.get_account(&pet).is_none());
    assert!(svm.get_balance(&payer.pubkey()).unwrap() > balance_before_close);
}
