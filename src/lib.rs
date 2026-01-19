use valkey_module::alloc::ValkeyAlloc;
use valkey_module::{RedisModuleCommandFilterCtx, VALKEYMODULE_CMDFILTER_NOSELF, valkey_module};

fn sleep_filter_fn(_ctx: *mut RedisModuleCommandFilterCtx) {
    std::thread::sleep(std::time::Duration::from_millis(1));
}

valkey_module! {
    name: "patience",
    version: 1,
    allocator: (ValkeyAlloc, ValkeyAlloc),
    data_types: [],
    commands: [
    ],
    filters: [
        [sleep_filter_fn, VALKEYMODULE_CMDFILTER_NOSELF],
    ]
}
