#![no_std]
extern crate alloc;

use soroban_sdk::{contractimpl, symbol, Address, Bytes, Env, Map, Symbol};

#[derive(Clone)]
pub struct Roast {
    pub id: u64,
    pub author: Address,
    pub target: Bytes,
    pub content: Bytes,
    pub upvotes: u32,
}

impl Roast {
    fn to_map(&self, env: &Env) -> Map<Symbol, soroban_sdk::Value> {
        let mut m = Map::new(env);
        m.set(&symbol!("id"), self.id.into());
        m.set(&symbol!("author"), self.author.clone().into());
        m.set(&symbol!("target"), self.target.clone().into());
        m.set(&symbol!("content"), self.content.clone().into());
        m.set(&symbol!("upvotes"), (self.upvotes as u64).into());
        m
    }
}

pub struct RoasterContract;

#[contractimpl]
impl RoasterContract {
    pub fn init(env: Env) {
        env.storage().set(&symbol!("counter"), 0u64);
    }

    pub fn post_roast(env: Env, author: Address, target: Bytes, content: Bytes) -> u64 {
        let max_len: u32 = 280;
        if content.len() > max_len as usize {
            panic!("content too long");
        }

        let mut counter: u64 = env.storage().get(&symbol!("counter")).unwrap_or(0u64);
        counter += 1;
        let id = counter;

        let roast = Roast {
            id,
            author: author.clone(),
            target,
            content,
            upvotes: 0,
        };

        env.storage().set(&symbol!(format!("roast:{}", id)), roast.to_map(&env));
        env.storage().set(&symbol!("counter"), counter);

        id
    }

    pub fn get_roast(env: Env, id: u64) -> Map<Symbol, soroban_sdk::Value> {
        env.storage()
            .get(&symbol!(format!("roast:{}", id)))
            .unwrap()
    }

    pub fn get_roast_count(env: Env) -> u64 {
        env.storage().get(&symbol!("counter")).unwrap_or(0u64)
    }

    pub fn upvote_roast(env: Env, voter: Address, id: u64) {
        let key = symbol!(format!("roast:{}", id));
        let mut m: Map<Symbol, soroban_sdk::Value> =
            env.storage().get(&key).expect("roast not found");

        let author: Address = m.get(&symbol!("author")).unwrap().try_into().unwrap();
        if author == voter {
            panic!("author cannot upvote their own roast");
        }

        let up: u64 = m.get(&symbol!("upvotes")).unwrap().try_into().unwrap();
        m.set(&symbol!("upvotes"), (up + 1).into());

        env.storage().set(&key, m);
    }
}
