use sled::{Batch, Db, Tree};

use serde::de::Error as DeError;
use serde::de::Unexpected;
use serde::de::{DeserializeSeed, Visitor};

const MAX_BATCH_SIZE: usize = 4096;

pub struct DbDeserializer<'a>(pub &'a Db);
pub struct TreeDeserializer<'a>(pub &'a Tree);

impl<'de, 'a> DeserializeSeed<'de> for DbDeserializer<'a> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

impl<'de, 'a> Visitor<'de> for DbDeserializer<'a> {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "expecting a map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        while let Some(tree_name) = map.next_key()? {
            let tree_name: String = tree_name;
            let tree_name = hex::decode(&tree_name).map_err(|_| {
                DeError::invalid_value(Unexpected::Str(&tree_name), &"A hex-encoded byte string")
            })?;
            let tree: Tree = self.0.open_tree(tree_name).map_err(DeError::custom)?;

            let () = map.next_value_seed(TreeDeserializer(&tree))?;
        }
        Ok(())
    }
}

impl<'de, 'a> DeserializeSeed<'de> for TreeDeserializer<'a> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

impl<'de, 'a> Visitor<'de> for TreeDeserializer<'a> {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "expecting a map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut batch = Batch::default();
        let mut counter: usize = 0;
        while let Some(key) = map.next_key()? {
            let key: String = key;
            let key = hex::decode(&key).map_err(|_| {
                DeError::invalid_value(Unexpected::Str(&key), &"A hex-encoded byte string")
            })?;

            let value: String = map.next_value()?;
            let value = hex::decode(&value).map_err(|_| {
                DeError::invalid_value(Unexpected::Str(&value), &"A hex-encoded byte string")
            })?;

            batch.insert(key, value);

            counter += 1;

            if counter >= MAX_BATCH_SIZE {
                self.0.apply_batch(batch).map_err(DeError::custom)?;
                counter = 0;
                batch = Batch::default();
            }
        }
        self.0.apply_batch(batch).map_err(DeError::custom)?;
        Ok(())
    }
}
