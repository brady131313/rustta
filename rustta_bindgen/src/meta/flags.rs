bitflags! {
    struct FuncFlags: u32 {
        const OVERLAP = 0x01000000;
        const VOLUME = 0x04000000;
        const UNSTABLE_PERIOD = 0x08000000;
        const CANDLESTICK = 0x10000000;
    }
}
