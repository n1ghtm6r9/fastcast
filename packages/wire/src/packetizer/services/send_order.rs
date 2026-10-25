use crate::packetizer::BlockPlan;

pub fn send_order(plans: &[BlockPlan]) -> impl Iterator<Item = (usize, usize)> + '_ {
    let rounds = plans.iter().map(BlockPlan::total_shards).max().unwrap_or(0);

    (0..rounds).flat_map(move |index| {
        plans
            .iter()
            .enumerate()
            .filter(move |(_, plan)| index < plan.total_shards())
            .map(move |(block, _)| (block, index))
    })
}
