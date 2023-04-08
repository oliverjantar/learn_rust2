mod example_async_blocks;
mod example_async_blocks_pt2;
mod example_block_on;
mod example_long_running_computation;
mod example_spawn_async_tasks;
mod example_task_spawn;

fn main() {
    // example_block_on::call_from_sync().unwrap();
    // example_spawn_async_tasks::run();
    // example_async_blocks::run();
    // example_async_blocks_pt2::run();
    // example_task_spawn::run();
    example_long_running_computation::run();
}
