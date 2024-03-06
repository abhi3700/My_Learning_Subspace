//! Sectors to solution range & viceversa

type SolutionRange = u64;
const SLOT_PROBABILITY: (u64, u64) = (1, 6);
const MAX_PIECES_IN_SECTOR: u16 = 1000;
pub const NUM_CHUNKS: usize = 2_usize.pow(15);
pub const NUM_S_BUCKETS: usize = 65536;

/// Computes the following:
/// ```
/// MAX * slot_probability / (pieces_in_sector * chunks / s_buckets) / sectors
/// ```
const fn sectors_to_solution_range(sectors: u64) -> SolutionRange {
    let solution_range = SolutionRange::MAX
        // Account for slot probability
        / SLOT_PROBABILITY.1 * SLOT_PROBABILITY.0
        // Now take sector size and probability of hitting occupied s-bucket in sector into account
        / (MAX_PIECES_IN_SECTOR as u64 * NUM_CHUNKS as u64 / NUM_S_BUCKETS as u64);

    // Take number of sectors into account
    solution_range / sectors
}

/// Computes the following:
/// ```
/// MAX * slot_probability / (pieces_in_sector * chunks / s_buckets) / solution_range
/// ```
const fn solution_range_to_sectors(solution_range: SolutionRange) -> u64 {
    let sectors = SolutionRange::MAX
        // Account for slot probability
        / SLOT_PROBABILITY.1 * SLOT_PROBABILITY.0
        // Now take sector size and probability of hitting occupied s-bucket in sector into account
        / (MAX_PIECES_IN_SECTOR as u64 * NUM_CHUNKS as u64 / NUM_S_BUCKETS as u64);

    // Take solution range into account
    sectors / solution_range
}

// write test functions
fn main() {
    let sectors = 1;
    let result = SolutionRange::MAX
        .checked_mul(SLOT_PROBABILITY.0.checked_div(SLOT_PROBABILITY.1).unwrap())
        .unwrap()
        .checked_div((MAX_PIECES_IN_SECTOR as u64 * NUM_CHUNKS as u64 / NUM_S_BUCKETS as u64))
        .unwrap()
        .checked_div(sectors)
        .unwrap();
    // *(SLOT_PROBABILITY.0 / SLOT_PROBABILITY.1)
    //     * (MAX_PIECES_IN_SECTOR as u64 * NUM_CHUNKS as u64 / NUM_S_BUCKETS as u64)
    //     / sectors;

    dbg!(result);
    dbg!(sectors_to_solution_range(sectors));
    // assert_eq!(solution_range_to_sectors(1), 2);
}
