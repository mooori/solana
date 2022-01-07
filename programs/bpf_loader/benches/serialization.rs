#![feature(test)]

extern crate test;

use {
    solana_bpf_loader_program::serialization::{
        serialize_parameters_aligned, serialize_parameters_unaligned,
    },
    solana_sdk::{
        account::{Account, AccountSharedData},
        bpf_loader,
        transaction_context::{InstructionAccount, TransactionContext},
    },
    test::Bencher,
};

fn create_inputs() -> TransactionContext {
    let program_id = solana_sdk::pubkey::new_rand();
    let transaction_accounts = vec![
        (
            program_id,
            AccountSharedData::from(Account {
                lamports: 0,
                data: vec![],
                owner: bpf_loader::id(),
                executable: true,
                rent_epoch: 0,
            }),
        ),
        (
            solana_sdk::pubkey::new_rand(),
            AccountSharedData::from(Account {
                lamports: 1,
                data: vec![1u8; 100000],
                owner: bpf_loader::id(),
                executable: false,
                rent_epoch: 100,
            }),
        ),
        (
            solana_sdk::pubkey::new_rand(),
            AccountSharedData::from(Account {
                lamports: 2,
                data: vec![11u8; 100000],
                owner: bpf_loader::id(),
                executable: true,
                rent_epoch: 200,
            }),
        ),
        (
            solana_sdk::pubkey::new_rand(),
            AccountSharedData::from(Account {
                lamports: 3,
                data: vec![],
                owner: bpf_loader::id(),
                executable: false,
                rent_epoch: 3100,
            }),
        ),
        (
            solana_sdk::pubkey::new_rand(),
            AccountSharedData::from(Account {
                lamports: 4,
                data: vec![1u8; 100000],
                owner: bpf_loader::id(),
                executable: false,
                rent_epoch: 100,
            }),
        ),
        (
            solana_sdk::pubkey::new_rand(),
            AccountSharedData::from(Account {
                lamports: 5,
                data: vec![11u8; 10000],
                owner: bpf_loader::id(),
                executable: true,
                rent_epoch: 200,
            }),
        ),
        (
            solana_sdk::pubkey::new_rand(),
            AccountSharedData::from(Account {
                lamports: 6,
                data: vec![],
                owner: bpf_loader::id(),
                executable: false,
                rent_epoch: 3100,
            }),
        ),
    ];
    let instruction_accounts = [1, 1, 2, 3, 4, 4, 5, 6]
        .into_iter()
        .enumerate()
        .map(
            |(index_in_instruction, index_in_transaction)| InstructionAccount {
                index_in_caller: 1usize.saturating_add(index_in_instruction),
                index_in_transaction,
                is_signer: false,
                is_writable: index_in_instruction >= 4,
            },
        )
        .collect::<Vec<_>>();
    let mut transaction_context = TransactionContext::new(transaction_accounts, 1);
    let instruction_data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    transaction_context
        .push(&[0], &instruction_accounts, &instruction_data)
        .unwrap();
    transaction_context
}

/// Creates a `TransactionContext` with one program account and the specified
/// number of instruction accounts.
///
/// Instruction accounts are unique (i.e. there are no duplicates).
fn create_inputs_with_size(number_instruction_accounts: usize) -> TransactionContext {
    let number_program_accounts = 1usize;
    let number_accounts = number_program_accounts + number_instruction_accounts;
    let program_id = solana_sdk::pubkey::new_rand();
    let mut transaction_accounts = Vec::with_capacity(number_accounts);
    transaction_accounts.push((
        program_id,
        AccountSharedData::from(Account {
            lamports: 0,
            data: vec![],
            owner: bpf_loader::id(),
            executable: true,
            rent_epoch: 0,
        }),
    ));

    for i in 0..number_instruction_accounts {
        transaction_accounts.push((
            solana_sdk::pubkey::new_rand(),
            AccountSharedData::from(Account {
                lamports: i as u64,
                data: if i % 2 == 0 {
                    vec![]
                } else {
                    vec![11u8; i * 1000]
                },
                owner: bpf_loader::id(),
                executable: i % 2 == 0,
                rent_epoch: (i as u64) * 10,
            }),
        ))
    }

    let instruction_accounts = (number_program_accounts..number_instruction_accounts)
        .enumerate()
        .map(
            |(index_in_instruction, index_in_transaction)| InstructionAccount {
                index_in_caller: number_program_accounts.saturating_add(index_in_instruction),
                index_in_transaction,
                is_signer: false,
                is_writable: index_in_instruction >= number_instruction_accounts / 2,
            },
        )
        .collect::<Vec<_>>();
    let mut transaction_context = TransactionContext::new(transaction_accounts, 1);
    let instruction_data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let program_accounts: Vec<usize> = (0..number_program_accounts).collect();
    transaction_context
        .push(
            program_accounts.as_slice(),
            &instruction_accounts,
            &instruction_data,
        )
        .unwrap();
    transaction_context
}

#[bench]
fn bench_serialize_unaligned(bencher: &mut Bencher) {
    let transaction_context = create_inputs();
    let instruction_context = transaction_context
        .get_current_instruction_context()
        .unwrap();
    bencher.iter(|| {
        let _ = serialize_parameters_unaligned(&transaction_context, instruction_context).unwrap();
    });
}

#[bench]
fn bench_serialize_unaligned_many(bencher: &mut Bencher) {
    let transaction_context = create_inputs_with_size(64);
    let instruction_context = transaction_context
        .get_current_instruction_context()
        .unwrap();
    bencher.iter(|| {
        let _ = serialize_parameters_unaligned(&transaction_context, instruction_context).unwrap();
    });
}

#[bench]
fn bench_serialize_aligned(bencher: &mut Bencher) {
    let transaction_context = create_inputs();
    let instruction_context = transaction_context
        .get_current_instruction_context()
        .unwrap();
    bencher.iter(|| {
        let _ = serialize_parameters_aligned(&transaction_context, instruction_context).unwrap();
    });
}

#[bench]
fn bench_serialize_aligned_many(bencher: &mut Bencher) {
    let transaction_context = create_inputs_with_size(64);
    let instruction_context = transaction_context
        .get_current_instruction_context()
        .unwrap();
    bencher.iter(|| {
        let _ = serialize_parameters_aligned(&transaction_context, instruction_context).unwrap();
    });
}
