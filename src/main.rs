



use libloading::{Library, Symbol};

fn main() {
    unsafe {
        let lib = Library::new("/ssd1/evm-arbitrage-bot/target/release/libevm_arbitrage_bot.so").unwrap();

        let init: Symbol<unsafe extern "C" fn()> = lib.get(b"init_arb").unwrap();

        init();

        // lib 必须在函数调用及动态库代码执行期间保持存活。
        // 如果 init_arb 启动的后台线程仍在执行动态库代码，
        // 就不能在这些线程退出前释放 lib。
        std::mem::forget(lib);
    }

}
// git remote add arb_run https://github.com/zhouwei0192/arb_run.git