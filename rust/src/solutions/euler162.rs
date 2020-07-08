fn of_length(n: u32) -> u64 {
    // Count all numbers of length n starting with 1:
    // all suffixes - suffixes without A - suffixes without 0 + suffixes without A and 0
    let case_1 = 16u64.pow(n - 1) + 14u64.pow(n - 1) - 2 * 15u64.pow(n - 1);
    // Count all numbers of length n starting with A:
    // all suffixes - suffixes without 1 - suffixes without 0 + suffixes without 1 and 0
    let case_a = case_1;
    // Count all numbers of length n starting with neither A nor 1:
    // all suffixes - suffixes without 1 - suffixes without 0 - suffixes without A
    // + suffixes without 1 and 0 + suffixes without 1 and A + suffixes without A and 0
    // - suffixes without 1, A and 0
    let case_other =
        13 * (16u64.pow(n - 1) + 3 * 14u64.pow(n - 1) - 3 * 15u64.pow(n - 1) - 13u64.pow(n - 1));
    case_1 + case_a + case_other
}

pub fn euler162() -> String {
    format!("{:X}", (3..=16).map(of_length).sum::<u64>())
}
