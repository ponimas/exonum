use ::crypto::{Hash, HASH_SIZE, hash};


pub use self::error::Error;
pub use self::db::{Database, Snapshot, Fork, Patch, Change, Iter};
pub use self::leveldb::LevelDB;
pub use self::memorydb::MemoryDB;

pub use self::keys::StorageKey;
pub use self::values::StorageValue;

pub use self::base_index::{BaseIndex, BaseIndexIter};
pub use self::entry::Entry;
pub use self::map_index::MapIndex;
pub use self::list_index::ListIndex;
pub use self::key_set_index::KeySetIndex;
pub use self::value_set_index::ValueSetIndex;


pub type Result<T> = ::std::result::Result<T, Error>;

mod error;

mod db;
mod leveldb;
mod memorydb;

mod keys;
mod values;

mod base_index;
mod entry;
mod map_index;
mod list_index;
mod key_set_index;
mod value_set_index;

pub fn merkle_hash(hashes: &[Hash]) -> Hash {
    match hashes.len() {
        0 => Hash::default(),
        1 => hashes[0],
        n => {
            let (l, r) = hashes.split_at(n.next_power_of_two() / 2);
            let mut v = [0; HASH_SIZE * 2];
            v[..HASH_SIZE].copy_from_slice(merkle_hash(l).as_ref());
            v[HASH_SIZE..].copy_from_slice(merkle_hash(r).as_ref());
            hash(&v)
        }
    }
}
