struct RangeId(usize);

enum MirNode {
    Range(usize, usize),
    Parallel(RangeId),
}
