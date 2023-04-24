/*트리 */

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Default)]
struct Node<Key: Default, Type: Default> {
    children: HashMap<Key, Node<Key, Type>>,
    value: Option<Type>,
}

#[derive(Debug,Default)]
pub struct Trie<Key,Type>
where 
Key:Default+Eq+Hash,
Type:Default,
{
   root:Node<Key,Type>

}
impl<Key,Type>Trie<Key,Type>
where
  Key:Default+Eq+Hash,
  Type:Default,{

    pub fn new()->Self{

        Self { root: Node::default() }
    }
  }




