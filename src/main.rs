use std::{thread::sleep, time::Duration};

// This static flag is used to mimic the chain state initialization procedure.
static mut CHAIN_STATE_INITIALIZED: bool = false;

// Automatically profile this function
#[profiling::function]
fn get_chain_state() {
    unsafe {
        if !CHAIN_STATE_INITIALIZED {
            // Start a new custom scope to be shown in the trace
            profiling::scope!("Chain State Initialization scope");

            // Simulate the initialization time
            std::thread::sleep(Duration::from_millis(500));

            // Mark the initialization as completed
            // so that it is not repeated in next calls
            CHAIN_STATE_INITIALIZED = true;

            // Close the custom scope created above
            profiling::finish_frame!();
        }
    }
}

struct BlockImporter {
    pub current_block_height: u64,
}

// Automatically profile all functions in this impl block
#[profiling::all_functions]
impl BlockImporter {
    fn new() -> Self {
        // Simulate the creation time
        sleep(Duration::from_millis(100));

        BlockImporter {
            current_block_height: 0,
        }
    }

    pub fn import_block(&mut self, height: u64) {
        // Get the current chain state
        get_chain_state();
        self.verify_block(height);
    }

    fn verify_block(&self, _height: u64) {
        sleep(Duration::from_millis(80));
    }

    fn apply_transactions(&self, _height: u64) {
        sleep(Duration::from_millis(140));
    }

    fn update_state(&mut self, _height: u64) {
        sleep(Duration::from_millis(220));
        self.current_block_height = _height;
    }
}

fn main() {
    // Initialize tracing if the `tracing` feature is enabled
    #[cfg(feature = "tracing")]
    let _guard = {
        use tracing_chrome::ChromeLayerBuilder;
        use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

        let (chrome_layer, guard) = ChromeLayerBuilder::new().build();
        tracing_subscriber::registry().with(chrome_layer).init();
        guard
    };

    // Start a new custom scope to be shown in the trace
    profiling::scope!("Main Application Scope");

    // Simulate importing 5 blocks.
    // The first one should take a little longer due to the initialization step.
    for i in 0..5 {
        let mut importer = BlockImporter::new();
        importer.import_block(i);
    }

    println!(
        "Current block height: {}",
        BlockImporter::new().current_block_height
    );
}
