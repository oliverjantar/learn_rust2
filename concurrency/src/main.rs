mod example_async_blocks;
mod example_async_blocks_pt2;
mod example_block_on;
mod example_spawn_async_tasks;

fn main() {
    // example_block_on::call_from_sync().unwrap();
    // example_spawn_async_tasks::run();
    // example_async_blocks::run();
    example_async_blocks_pt2::run();
}
