use bitcoin::Amount;

pub struct UnnecessaryInputHeuristic;

impl UnnecessaryInputHeuristic {
    /// UIH1 (Optimal change): the smallest output is likely change when it is
    /// strictly less than the smallest input. Returns `Some(min_out)` in that
    /// case. Caller is responsible for excluding unspendable outputs from
    /// `min_out` — including them (often value=0) would always trip UIH1.
    pub fn uih1_min_output_value(min_in: Amount, min_out: Amount) -> Option<Amount> {
        (min_out < min_in).then_some(min_out)
    }

    /// UIH2 (Unnecessary input): the largest output could be paid without the
    /// smallest input. Caller is responsible for excluding unspendable outputs
    /// from the sum/min and ensuring there are at least two inputs.
    pub fn is_uih2(sum_in: Amount, min_in: Amount, sum_out: Amount, min_out: Amount) -> bool {
        (sum_in - min_in) >= (sum_out - min_out)
    }
}
