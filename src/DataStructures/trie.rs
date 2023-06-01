/*트리 
최소 신장트리,허프만 코드에 사용

*/

use std::collections::HashMap;
use std::hash::Hash;


/*Node */
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




