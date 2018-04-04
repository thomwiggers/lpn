use bit_vec::BitVec;
use oracle::LpnOracle;
use oracle::query_bits_range;
use oracle::vector_sum;
use std::ops;
use std::default::Default;
use fnv::FnvHashMap;

pub fn bkw_reduction(mut oracle: LpnOracle, a: u32, b: u32) -> LpnOracle {
    let k = oracle.k;
    assert_eq!(a * b, k, "a*b != k");

    for i in 1..a {
        let num_queries = oracle.queries.len();
        println!("BKW iteration {}, {} queries left", i, num_queries);
        let queryiter = oracle.queries.into_iter();
        // Partition into V_j
        // max j:
        let maxj = 2usize.pow(b);
        let mut vector_partitions = Vec::with_capacity(maxj);
        let query_capacity = num_queries / maxj;
        for _ in 0..maxj {
            vector_partitions.push(Vec::with_capacity(query_capacity));
        }

        let bitrange: ops::Range<usize> = ((k - (b * i)) as usize)..((k - (b * (i - 1))) as usize);
        for v in queryiter {
            let idx = query_bits_range(&v.a, bitrange.clone()) as usize;
            if vector_partitions[idx].capacity() == 0 {
                println!("Vector {} is full, will need to resize", idx);
            }
            vector_partitions[idx].push(v);
        }

        // FIXME integrate this into the sorting out loop.
        for mut partition in vector_partitions.iter_mut() {
            if let Some(first) = partition.pop() {
                for mut v in partition.iter_mut() {
                    vector_sum(&mut v.a, &first.a);
                    v.s ^= first.s;
                }
            }
        }

        oracle.queries = Vec::with_capacity(num_queries - maxj);
        for partition in vector_partitions {
            oracle.queries.extend(partition.into_iter());
        }
    }

    oracle
}

pub fn bkw_solve(oracle: &LpnOracle, b: u32) -> BitVec {
    println!("BKW Solver");
    let b = b as usize;
    println!("Selecting all queries with hw=1 from {} queries", oracle.queries.len());
    let queries = oracle
        .queries
        .iter()
        .filter(|&q| q.count_ones() == 1)
        .map(|q| (query_bits_range(&q.a, 0..(b)), q.s))
        .collect::<Vec<(u64, bool)>>();

    // allocate smaller vec
    let mut counts: FnvHashMap<u64, u64> = FnvHashMap::with_capacity_and_hasher(b, Default::default());
    let mut sums: FnvHashMap<u64, u64> = FnvHashMap::with_capacity_and_hasher(b, Default::default());

    println!("Sorting out and counting {} queries for majority selection", queries.len());
    for query in queries.into_iter() {
        debug_assert_eq!(query.0.count_ones(), 1);
        let count = counts.entry(query.0).or_insert(0);
        *count += 1;
        if query.1 {
            let sum = sums.entry(query.0).or_insert(0);
            *sum += 1;
        }
    }

    let mut result = BitVec::with_capacity(b);
    let mut i = 1 << (b-1);
    while i > 0 {
        let count = counts.get(&i).unwrap();
        if let Some(sum) = sums.get(&i) {
            result.push(*count < 2 * sum);
        } else {
            result.push(false);
        }
        i >>= 1;
    }
    result
}
