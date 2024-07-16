// TODO: Make this data structure thread-safe
mod directory_trie;
mod directory_trie_node;
pub(crate) use crate::directory_trie_node::TrieNodeRc;
pub use directory_trie::DirectoryTrie;
use directory_trie_node::DirectoryTrieNode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_one_get_one() {
        let mut trie = DirectoryTrie::new();
        println!("##INITIAL TRIE##\n{:#?}", trie);
        trie.insert("/tmp/dir1", "Hello World");
        println!(
            "##AFTER /tmp/dir1->\"Hello World\" INSERTION##\n{:#?}",
            trie
        );
        assert_eq!(
            "Hello World",
            trie.get("/tmp/dir1/dir2/textfile.txt").unwrap()
        );
    }

    #[test]
    fn insert_one_get_one_subdirs() {
        let mut trie = DirectoryTrie::new();
        trie.insert("/tmp/dir1", "Hello World");
        assert_eq!(
            "Hello World",
            trie.get("/tmp/dir1/dir2/hello/world/123/321/textfile.txt")
                .unwrap()
        );
    }

    #[test]
    fn insert_three_get_three() {
        let mut trie = DirectoryTrie::new();
        println!("Initial Trie: {:#?}", trie);
        trie.insert("/tmp", "1");
        println!("After /tmp->1 insert trie: {:#?}", trie);
        trie.insert("/tmp/dir1", "3");
        println!("After /tmp/dir1->3 insert: {:#?}", trie);
        trie.insert("/tmp/dir1/dir2", "2");
        println!("After /tmp/dir1/dir2->2 insert {:#?}", trie);
        assert_eq!("1", trie.get("/tmp/not/present/hello.txt").unwrap());
        assert_eq!(
            "2",
            trie.get("/tmp/dir1/dir2/not/present/goodbye.txt").unwrap()
        );
        assert_eq!("3", trie.get("/tmp/dir1/not/present/hello/").unwrap());
    }

    #[test]
    fn test_increasing_path_length() {
        let mut trie = DirectoryTrie::new();
        trie.insert("/tmp", "0".to_string());
        trie.insert("/tmp/1", "1".to_string());
        println!("{:#?}", trie);
        assert_eq!("0".to_string(), trie.get("/tmp").unwrap());
        assert_eq!("1".to_string(), trie.get("/tmp/1").unwrap());
    }

    #[test]
    fn test_increasing_path_length_with_root() {
        let mut trie = DirectoryTrie::new();
        trie.insert("/", "root".to_string());
        trie.insert("/tmp", "0".to_string());
        trie.insert("/tmp/1", "1".to_string());
        println!("{:#?}", trie);
        assert_eq!("root".to_string(), trie.get("/").unwrap());
        assert_eq!("0".to_string(), trie.get("/tmp").unwrap());
        assert_eq!("1".to_string(), trie.get("/tmp/1").unwrap());
    }
}
