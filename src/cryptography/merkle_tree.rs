use hex::encode;
use sha2::{Digest, Sha256};
use std::time::Instant;

/// 기본 해시 함수 (SHA-256)
fn hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// 두 해시를 합쳐서 새로운 해시를 만든다.
fn combine_hashes(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let combined = [left.as_slice(), right.as_slice()].concat();
    hash(&combined)
}

/// 단계별 머클 루트 계산 (디버그용)
fn build_merkle_root_verbose<T: AsRef<[u8]>>(leaves: &[T]) -> [u8; 32] {
    assert!(!leaves.is_empty(), "empty leaves not allowed");
    let mut level: Vec<[u8; 32]> = leaves.iter().map(|d| hash(d.as_ref())).collect();

    let mut depth = 0;
    println!("🌱 Building Merkle Tree");
    println!("==============================");
    println!("Level {depth} - Leaves ({} nodes)", level.len());
    for (i, h) in level.iter().enumerate() {
        println!("  [{}] {}", i, encode(h));
    }
    println!();

    while level.len() > 1 {
        depth += 1;
        let mut next = Vec::with_capacity((level.len() + 1) / 2);
        for pair in level.chunks(2) {
            let parent = if pair.len() == 2 {
                combine_hashes(&pair[0], &pair[1])
            } else {
                combine_hashes(&pair[0], &pair[0])
            };
            next.push(parent);
        }
        println!("Level {depth} - Internal ({} nodes)", next.len());
        for (i, h) in next.iter().enumerate() {
            println!("  [{}] {}", i, encode(h));
        }
        println!();
        level = next;
    }

    println!("🏆 Merkle Root: {}", encode(level[0]));
    level[0]
}

/// 머클 증명 구조체
#[derive(Debug, Clone)]
pub struct MerkleProof {
    pub index: usize,
    pub leaf: [u8; 32],
    pub siblings: Vec<([u8; 32], bool)>, // (해시, 내가 left면 true/right면 false)
    pub root: [u8; 32],
}

/// 기본 트리 구조체
#[derive(Debug, Clone)]
pub struct MerkleTree {
    leaves: Vec<[u8; 32]>,
    levels: Vec<Vec<[u8; 32]>>,
    root: [u8; 32],
}

impl MerkleTree {
    pub fn new<T: AsRef<[u8]>>(data: &[T]) -> Self {
        assert!(!data.is_empty());
        let leaves: Vec<[u8; 32]> = data.iter().map(|x| hash(x.as_ref())).collect();
        let mut levels = vec![leaves.clone()];
        let mut cur = leaves.clone();

        while cur.len() > 1 {
            let mut next = Vec::new();
            for pair in cur.chunks(2) {
                let parent = if pair.len() == 2 {
                    combine_hashes(&pair[0], &pair[1])
                } else {
                    combine_hashes(&pair[0], &pair[0])
                };
                next.push(parent);
            }
            levels.push(next.clone());
            cur = next;
        }
        Self {
            root: cur[0],
            leaves,
            levels,
        }
    }

    pub fn root(&self) -> [u8; 32] {
        self.root
    }
    pub fn leaf_count(&self) -> usize {
        self.leaves.len()
    }
    pub fn height(&self) -> usize {
        self.levels.len()
    }

    pub fn generate_proof(&self, index: usize) -> Option<MerkleProof> {
        if index >= self.leaves.len() {
            return None;
        }

        let mut siblings = Vec::new();
        let mut idx = index;

        for level in &self.levels[..self.levels.len() - 1] {
            let sib_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
            let is_left = idx % 2 == 0;
            if sib_idx < level.len() {
                siblings.push((level[sib_idx], is_left));
            } else {
                siblings.push((level[idx], is_left));
            }
            idx /= 2;
        }

        Some(MerkleProof {
            index,
            leaf: self.leaves[index],
            siblings,
            root: self.root,
        })
    }

    pub fn verify_proof(proof: &MerkleProof) -> bool {
        let mut cur = proof.leaf;
        for (sibling, is_left) in &proof.siblings {
            cur = if *is_left {
                combine_hashes(&cur, sibling)
            } else {
                combine_hashes(sibling, &cur)
            };
        }
        cur == proof.root
    }
}

/// 간단한 예제
fn basic_example() {
    let txs: &[&[u8]] = &[
        b"Alice -> Bob: 10 coins",
        b"Charlie -> Dave: 5 coins",
        b"Eve -> Frank: 8 coins",
        b"Grace -> Henry: 12 coins",
    ];
    let root = build_merkle_root_verbose(&txs);
    println!("Final Root: {}", encode(root));
}

/// 증명/검증 데모
fn proof_demo() {
    let data = vec![
        "transaction_001",
        "transaction_002",
        "transaction_003",
        "transaction_004",
        "transaction_005",
    ];
    let tree = MerkleTree::new(&data);
    println!(
        "\nTree height: {}, leaves: {}",
        tree.height(),
        tree.leaf_count()
    );
    println!("Root hash: {}", encode(tree.root()));

    let index = 2;
    if let Some(proof) = tree.generate_proof(index) {
        println!("Generated proof for index {} ({})", index, data[index]);
        println!("Siblings:");
        for (i, (sib, is_left)) in proof.siblings.iter().enumerate() {
            println!(
                "  Level {}: {} ({})",
                i,
                encode(sib),
                if *is_left { "right" } else { "left" }
            );
        }
        let ok = MerkleTree::verify_proof(&proof);
        println!("Proof verification result: {}", ok);
    }
}

/// 대용량 성능 테스트
fn performance_test() {
    let size = 50_000;
    let data: Vec<String> = (0..size).map(|i| format!("tx_{:05}", i)).collect();
    let start = Instant::now();
    let tree = MerkleTree::new(&data);
    println!("\nBuilt {} leaves in {:?}", size, start.elapsed());

    let proof_start = Instant::now();
    let proof = tree.generate_proof(size / 2).unwrap();
    println!("Proof generation: {:?}", proof_start.elapsed());
    let verify_start = Instant::now();
    let ok = MerkleTree::verify_proof(&proof);
    println!("Verify: {:?}, valid: {}", verify_start.elapsed(), ok);
}

pub fn example() {
    basic_example();
    proof_demo();
    performance_test();
}
