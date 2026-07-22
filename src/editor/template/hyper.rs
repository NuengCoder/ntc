use std::path::Path;
use std::fs;

use anyhow::{Result, bail};

use super::simple::generate_header_guard;

fn guard(name: &str) -> String {
    generate_header_guard(name, "h")
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content)?;
    Ok(())
}

pub fn init_hyper_project(name: &str, lang: &str) -> Result<bool> {
    if lang != "c" {
        bail!("Unsupported language '{}'. Only 'c' is supported.", lang);
    }
    if name.contains('.') {
        bail!("Project name '{}' contains an extension. Use a name without dots.", name);
    }

    let root = Path::new(name);
    if root.exists() {
        bail!("Directory '{}' already exists.", name);
    }

    println!("Creating hyper project '{}' (C)...", name);

    for dir in &["", "dsa", "debug", "os", "header", "ntclib"] {
        fs::create_dir_all(root.join(dir))?;
    }

    let r = |p: &str| root.join(p);

    // DSA headers + sources
    write_file(&r("dsa/_arraylist.h"), &arraylist_h())?;
    write_file(&r("dsa/_arraylist.c"), arraylist_c())?;
    write_file(&r("dsa/_linkedlist.h"), &linkedlist_h())?;
    write_file(&r("dsa/_linkedlist.c"), linkedlist_c())?;
    write_file(&r("dsa/_doublylinkedlist.h"), &doublylinkedlist_h())?;
    write_file(&r("dsa/_doublylinkedlist.c"), doublylinkedlist_c())?;
    write_file(&r("dsa/_circularlinkedlist.h"), &circularlinkedlist_h())?;
    write_file(&r("dsa/_circularlinkedlist.c"), circularlinkedlist_c())?;
    write_file(&r("dsa/_stack.h"), &stack_h())?;
    write_file(&r("dsa/_stack.c"), stack_c())?;
    write_file(&r("dsa/_queue.h"), &queue_h())?;
    write_file(&r("dsa/_queue.c"), queue_c())?;
    write_file(&r("dsa/_binarytree.h"), &binarytree_h())?;
    write_file(&r("dsa/_binarytree.c"), binarytree_c())?;
    write_file(&r("dsa/_tree.h"), &tree_h())?;
    write_file(&r("dsa/_tree.c"), tree_c())?;
    write_file(&r("dsa/_hashmap.h"), &hashmap_h())?;
    write_file(&r("dsa/_hashmap.c"), hashmap_c())?;
    write_file(&r("dsa/_hashtable.h"), &hashtable_h())?;
    write_file(&r("dsa/_hashtable.c"), hashtable_c())?;
    write_file(&r("dsa/_hashset.h"), &hashset_h())?;
    write_file(&r("dsa/_hashset.c"), hashset_c())?;
    write_file(&r("dsa/_hash.h"), &hash_h())?;
    write_file(&r("dsa/_hash.c"), hash_c())?;
    write_file(&r("dsa/_heap.h"), &heap_h())?;
    write_file(&r("dsa/_heap.c"), heap_c())?;
    write_file(&r("dsa/_priorityqueue.h"), &priorityqueue_h())?;
    write_file(&r("dsa/_priorityqueue.c"), priorityqueue_c())?;
    write_file(&r("dsa/_graph.h"), &graph_h())?;
    write_file(&r("dsa/_graph.c"), graph_c())?;
    write_file(&r("dsa/_map.h"), &map_h())?;
    write_file(&r("dsa/_map.c"), map_c())?;
    write_file(&r("dsa/_set.h"), &set_h())?;
    write_file(&r("dsa/_set.c"), set_c())?;
    write_file(&r("dsa/_rope.h"), &rope_h())?;
    write_file(&r("dsa/_rope.c"), rope_c())?;
    write_file(&r("dsa/_trie.h"), &trie_h())?;
    write_file(&r("dsa/_trie.c"), trie_c())?;
    write_file(&r("dsa/_lsearch.h"), &lsearch_h())?;
    write_file(&r("dsa/_lsearch.c"), lsearch_c())?;
    write_file(&r("dsa/_bsearch.h"), &bsearch_h())?;
    write_file(&r("dsa/_bsearch.c"), bsearch_c())?;

    // Debug, OS, header, ntclib
    write_file(&r("debug/debug.h"), &debug_h())?;
    write_file(&r("debug/debug.c"), debug_c())?;
    write_file(&r("os/os.h"), &os_h())?;
    write_file(&r("os/os.c"), os_c())?;
    write_file(&r("header/ntc.h"), &ntc_h())?;
    write_file(&r("ntclib/ntc_lib.h"), &ntclib_h())?;
    write_file(&r("ntclib/ntc_lib.c"), ntclib_c())?;

    // Example files
    write_file(&r("ntc_main.c"), &ntc_main_c(name))?;
    write_file(&r("ntc_lib.c"), ntc_lib_c())?;

    // NTCRANFILE.toml
    write_file(&r("NTCRANFILE.toml"), &ntc_ranfile_toml(name))?;

    println!("Hyper project '{}' created successfully!", name);
    Ok(true)
}

// ── ArrayList ─────────────────────────────────────────────────────────────────

fn arraylist_h() -> String {
    let g = guard("_arraylist_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_arraylist.h"))
}

fn arraylist_c() -> &'static str {
    include_str!("hyper_project/dsa/_arraylist.c")
}

// ── LinkedList ────────────────────────────────────────────────────────────────

fn linkedlist_h() -> String {
    let g = guard("_linkedlist_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_linkedlist.h"))
}

fn linkedlist_c() -> &'static str {
    include_str!("hyper_project/dsa/_linkedlist.c")
}

// ── DoublyLinkedList ──────────────────────────────────────────────────────────

fn doublylinkedlist_h() -> String {
    let g = guard("_doublylinkedlist_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_doublylinkedlist.h"))
}

fn doublylinkedlist_c() -> &'static str {
    include_str!("hyper_project/dsa/_doublylinkedlist.c")
}

// ── CircularLinkedList ────────────────────────────────────────────────────────

fn circularlinkedlist_h() -> String {
    let g = guard("_circularlinkedlist_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_circularlinkedlist.h"))
}

fn circularlinkedlist_c() -> &'static str {
    include_str!("hyper_project/dsa/_circularlinkedlist.c")
}

// ── Stack ─────────────────────────────────────────────────────────────────────

fn stack_h() -> String {
    let g = guard("_stack_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_stack.h"))
}

fn stack_c() -> &'static str {
    include_str!("hyper_project/dsa/_stack.c")
}

// ── Queue ─────────────────────────────────────────────────────────────────────

fn queue_h() -> String {
    let g = guard("_queue_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_queue.h"))
}

fn queue_c() -> &'static str {
    include_str!("hyper_project/dsa/_queue.c")
}

// ── BinaryTree ────────────────────────────────────────────────────────────────

fn binarytree_h() -> String {
    let g = guard("_binarytree_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_binarytree.h"))
}

fn binarytree_c() -> &'static str {
    include_str!("hyper_project/dsa/_binarytree.c")
}

// ── Tree ──────────────────────────────────────────────────────────────────────

fn tree_h() -> String {
    let g = guard("_tree_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_tree.h"))
}

fn tree_c() -> &'static str {
    include_str!("hyper_project/dsa/_tree.c")
}

// ── HashMap ───────────────────────────────────────────────────────────────────

fn hashmap_h() -> String {
    let g = guard("_hashmap_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_hashmap.h"))
}

fn hashmap_c() -> &'static str {
    include_str!("hyper_project/dsa/_hashmap.c")
}

// ── HashTable ─────────────────────────────────────────────────────────────────

fn hashtable_h() -> String {
    let g = guard("_hashtable_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_hashtable.h"))
}

fn hashtable_c() -> &'static str {
    include_str!("hyper_project/dsa/_hashtable.c")
}

// ── HashSet ───────────────────────────────────────────────────────────────────

fn hashset_h() -> String {
    let g = guard("_hashset_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_hashset.h"))
}

fn hashset_c() -> &'static str {
    include_str!("hyper_project/dsa/_hashset.c")
}

// ── Hash ──────────────────────────────────────────────────────────────────────

fn hash_h() -> String {
    let g = guard("_hash_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_hash.h"))
}

fn hash_c() -> &'static str {
    include_str!("hyper_project/dsa/_hash.c")
}

// ── Heap ──────────────────────────────────────────────────────────────────────

fn heap_h() -> String {
    let g = guard("_heap_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_heap.h"))
}

fn heap_c() -> &'static str {
    include_str!("hyper_project/dsa/_heap.c")
}

// ── PriorityQueue ─────────────────────────────────────────────────────────────

fn priorityqueue_h() -> String {
    let g = guard("_priorityqueue_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_priorityqueue.h"))
}

fn priorityqueue_c() -> &'static str {
    include_str!("hyper_project/dsa/_priorityqueue.c")
}

// ── Graph ─────────────────────────────────────────────────────────────────────

fn graph_h() -> String {
    let g = guard("_graph_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_graph.h"))
}

fn graph_c() -> &'static str {
    include_str!("hyper_project/dsa/_graph.c")
}

// ── Map ───────────────────────────────────────────────────────────────────────

fn map_h() -> String {
    let g = guard("_map_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_map.h"))
}

fn map_c() -> &'static str {
    include_str!("hyper_project/dsa/_map.c")
}

// ── Set ───────────────────────────────────────────────────────────────────────

fn set_h() -> String {
    let g = guard("_set_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_set.h"))
}

fn set_c() -> &'static str {
    include_str!("hyper_project/dsa/_set.c")
}

// ── Rope ──────────────────────────────────────────────────────────────────────

fn rope_h() -> String {
    let g = guard("_rope_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_rope.h"))
}

fn rope_c() -> &'static str {
    include_str!("hyper_project/dsa/_rope.c")
}

// ── Trie ──────────────────────────────────────────────────────────────────────

fn trie_h() -> String {
    let g = guard("_trie_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_trie.h"))
}

fn trie_c() -> &'static str {
    include_str!("hyper_project/dsa/_trie.c")
}

// ── Linear Search ─────────────────────────────────────────────────────────────

fn lsearch_h() -> String {
    let g = guard("_lsearch_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_lsearch.h"))
}

fn lsearch_c() -> &'static str {
    include_str!("hyper_project/dsa/_lsearch.c")
}

// ── Binary Search ─────────────────────────────────────────────────────────────

fn bsearch_h() -> String {
    let g = guard("_bsearch_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/dsa/_bsearch.h"))
}

fn bsearch_c() -> &'static str {
    include_str!("hyper_project/dsa/_bsearch.c")
}

// ── Debug ─────────────────────────────────────────────────────────────────────

fn debug_h() -> String {
    let g = guard("debug_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/debug/debug.h"))
}

fn debug_c() -> &'static str {
    include_str!("hyper_project/debug/debug.c")
}

// ── OS ────────────────────────────────────────────────────────────────────────

fn os_h() -> String {
    let g = guard("os_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/os/os.h"))
}

fn os_c() -> &'static str {
    include_str!("hyper_project/os/os.c")
}

// ── ntc header ────────────────────────────────────────────────────────────────

fn ntc_h() -> String {
    let g = guard("ntc_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/header/ntc.h"))
}

// ── ntclib ────────────────────────────────────────────────────────────────────

fn ntclib_h() -> String {
    let g = guard("ntclib_h");
    format!("#ifndef {g}\n#define {g}\n\n{}\n\n#endif /* {g} */\n", include_str!("hyper_project/ntclib/ntc_lib.h"))
}

fn ntclib_c() -> &'static str {
    include_str!("hyper_project/ntclib/ntc_lib.c")
}

// ── ntc_lib.c (alternative lib implementation) ────────────────────────────────

fn ntc_lib_c() -> &'static str {
    include_str!("hyper_project/ntc_lib.c")
}

// ── ntc_main.c (example main file) ────────────────────────────────────────────

fn ntc_main_c(name: &str) -> String {
    let g = guard("ntc_h");
    include_str!("hyper_project/ntc_main.c.in")
        .replace("{name}", name)
        .replace("{g}", &g)
}

// ── NTCRANFILE.toml ───────────────────────────────────────────────────────────

fn ntc_ranfile_toml(name: &str) -> String {
    include_str!("hyper_project/NTCRANFILE.toml.in").replace("{name}", name)
}
