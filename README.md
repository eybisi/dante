### Template to Mod* Unity Games in Android

Weekend project to write a pgsharp like tool for any unity game. Targeted game here is called "Knight Unity" which is a pvp server for knight online. (illegal)
- dante-apk contains UI related material for the mod
- dante-rust contains native side of the mod. 


### Building

- First build dante-apk and replace app-debug.apk in dante-rust/src/payload. Also change APK_SIZE in dante-rust/build.rs according to new apk size. 
- Then build dante-rust with `cargo +nightly ndk  -t arm64-v8a  -o ./jniLibs build --release -Z build-std=std,panic_abort`
- To build armeabi-v7a version (most emulators use 32 bit android image so you need to build with this.)`cargo +nightly ndk -t armeabi-v7a  -o ./jniLibs build --release -Z build-std=std,panic_abort` but you need to change `let mut function_iter = 0u64;` 
to `let mut function_iter = 0u32;`
- Then replace libmain.so file in your target apk. (You can use apktool)
- Rebuild and sign it. You are good to go.

To do all of this you can use dante-rust/setup.sh. Connect your device with adb and it will do the rest. If target game is already installed it will replace the native library (needs root)

### Hacking

Currently this template can hook il2cpp functions via by function name (See game_hook.rs). Also hooks send/recv functions to intercepts game network (See bd.rs)
