use flatbuffers::{FlatBufferBuilder, InvalidFlatbuffer, WIPOffset};
use solana_sdk::pubkey::Pubkey;
use rand::Rng;
use plerkle_serialization::{Pubkey as FBPubkey, CompiledInstruction, CompiledInstructionArgs, CompiledInstructionBuilder, InnerInstructionsBuilder, root_as_compiled_instruction, TransactionInfo, TransactionInfoArgs, TransactionInfoBuilder, root_as_transaction_info};


pub fn random_program() -> Pubkey {
    Pubkey::new_unique()
}

pub fn random_pubkey() -> Pubkey {
    random_program()
}

pub fn random_data(max: usize) -> Vec<u8> {
    let mut s = rand::thread_rng();
    let x = s.gen_range(1..max);
    let mut data: Vec<u8> = Vec::with_capacity(x);
    for i in 0..x {
        let d: u8 = s.gen_range(0..255);
        data.insert(i, d);
    }
    data
}

pub fn random_u8() -> u8 {
    let mut s = rand::thread_rng();
    s.gen()
}

pub fn random_u8_bound(min: u8, max: u8) -> u8 {
    let mut s = rand::thread_rng();
    s.gen_range(min..max)
}

pub fn random_list(size: usize, elem_max: u8) -> Vec<u8> {
    let mut s = rand::thread_rng();
    let mut data: Vec<u8> = Vec::with_capacity(size);
    for i in 0..size {
        let d: u8 = s.gen_range(0..elem_max);
        data.insert(i, d);
    }
    data
}

pub fn random_list_of<FN, T>(size: usize, fun: FN) -> Vec<T>
    where FN: Fn(u8) -> T
{
    let mut s = rand::thread_rng();
    let mut data: Vec<T> = Vec::with_capacity(size);
    for i in 0..size {
        data.insert(i, fun(s.gen()));
    }
    data
}

pub fn build_random_instruction<'a>(fbb: &mut FlatBufferBuilder<'a>, accounts_number_in_transaction: usize, number_of_accounts: usize) -> WIPOffset<CompiledInstruction<'a>> {
    let accounts = random_list(5, random_u8_bound(1, number_of_accounts as u8));
    let accounts = fbb.create_vector(&accounts);
    let data = random_data(10);
    let data = fbb.create_vector(&data);
    let mut s = rand::thread_rng();
    let mut builder = CompiledInstructionBuilder::new(fbb);
    builder.add_data(data);
    builder.add_program_id_index(s.gen_range(0..accounts_number_in_transaction) as u8);
    builder.add_accounts(accounts);
    builder.finish()
}

pub fn build_random_transaction(mut fbb: FlatBufferBuilder) -> FlatBufferBuilder {
    let mut s = rand::thread_rng();
    let mut outer_instructions = vec![];
    let mut inner_instructions = vec![];
    for _ in 0..s.gen_range(2..7) {
        outer_instructions.push(build_random_instruction(&mut fbb, 10, 3));

        let mut indexed_inner_instructions = vec![];
        for _ in 0..s.gen_range(2..7) {
            let ix = build_random_instruction(&mut fbb, 10, 3);
            indexed_inner_instructions.push(ix);
        }

        let indexed_inner_instructions = fbb.create_vector(&indexed_inner_instructions);
        let mut builder = InnerInstructionsBuilder::new(&mut fbb);
        builder.add_index(s.gen_range(0..7));
        builder.add_instructions(indexed_inner_instructions);
        inner_instructions.push(builder.finish());
    }

    let outer_instructions = fbb.create_vector(&outer_instructions);
    let inner_instructions = fbb.create_vector(&inner_instructions);
    let account_keys = random_list_of(10, |_| {
        FBPubkey(random_pubkey().to_bytes())
    });
    let account_keys = fbb.create_vector(&account_keys);
    let mut builder = TransactionInfoBuilder::new(&mut fbb);
    let slot = s.gen();
    builder.add_outer_instructions(outer_instructions);
    builder.add_is_vote(false);
    builder.add_inner_instructions(inner_instructions);
    builder.add_account_keys(account_keys);
    builder.add_slot(slot);
    builder.add_seen_at(s.gen());
    let builder = builder.finish();
    fbb.finish_minimal(builder);
    fbb
}

pub fn get_programs(txn_info: TransactionInfo) -> Vec<Pubkey> {
    let mut outer_keys: Vec<Pubkey> = txn_info.outer_instructions().unwrap().iter().map(|ix| {
        println!("{:?}", txn_info);
        Pubkey::new(&txn_info.account_keys().unwrap().get(ix.program_id_index() as usize).unwrap().0)
    }).collect();
    let mut inner = vec![];
    let inner_keys = txn_info.inner_instructions().unwrap().iter().fold(&mut inner, |ix, curr| {
        for p in curr.instructions().unwrap() {
            ix.push(Pubkey::new(&txn_info.account_keys().unwrap().get(p.program_id_index() as usize).unwrap().0))
        }
        ix
    });
    outer_keys.append(inner_keys);
    outer_keys.dedup();
    outer_keys
}
