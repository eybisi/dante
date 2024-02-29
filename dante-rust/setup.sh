#!/bin/bash
if [[ -z "${ANDROID_NDK_HOME}" ]]; then
  export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk
fi

package_name="com.KNGames.KnightUnityWorld2"
#cargo +nightly ndk -t arm64-v8a -o ./jniLibs build --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
#cargo ndk -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 -o ./jniLibs build

rebuild_imgui_sys(){
  cd imgui-sys
  cargo clean
  # gum spin -s minidot --title "Building imgui-sys" --show-error -- cargo +nightly ndk -t armeabi-v7a  -t arm64-v8a -o ./jniLibs build --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
  gum spin -s minidot --title "Building imgui-sys" --show-error -- cargo ndk -t armeabi-v7a  -t arm64-v8a -o ./jniLibs build 
  cd ..
}


build_library(){
  # cargo clean
  # gum spin -s minidot --title "Building native library" --show-error --  cargo +nightly ndk -t armeabi-v7a  -t arm64-v8a  -o ./jniLibs build --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
  gum spin -s minidot --title "Building native library" --show-error --  cargo ndk -t arm64-v8a  -o ./jniLibs build 
  RESULT=$?
  if [ $RESULT -eq 0 ]; then
    echo ':rocket: Built native library' | gum format -t emoji
  else
    echo ':x: Failed to build native library' | gum format -t emoji
    exit 1
  fi
}


replace_library(){
  path=$(adb shell pm path $package_name)
  first_path=$(echo ${path:8} | cut -f 1 -d " ")
  # first path /data/app/com.KNGames.KnightUnityWorld2-TwsAy2aMxwwYNqV88ZI8cQ==/base.apk
  # Also cut "base.apk" from end
  first_path=$(echo ${first_path::-8})
  # Append lib/arm64/libmain.so to the path
  arm64_path=$first_path/lib/arm64/libmain.so
  gum spin -s minidot --title "Pushing native library to device" --show-error -- adb push jniLibs/arm64-v8a/libdante.so $arm64_path
  # armv7=$first_path/lib/armeabi-v7a/libmain.so
  # gum spin -s minidot --title "Pushing native library to device" --show-error -- adb push jniLibs/armeabi-v7a/libdante.so $first_path
  echo ':rocket: Replaced native library' | gum format -t emoji
}

install_application(){
  cd ../
  # copy libdante from dante-rust/jniLibs/arm64-v8a/libdante.so to dantedKnightUnity/libs/arm64-v8a/libmain.so
  cp dante-rust/jniLibs/arm64-v8a/libdante.so dantedKnightUnity/lib/arm64-v8a/libmain.so
  # cp dante-rust/jniLibs/armeabi-v7a/libdante.so dantedKnightUnity/lib/armeabi-v7a/libmain.so
  # rebuild apk with apktool b
  gum spin -s minidot --title "Building apk 1/3" --show-error -- apktool b dantedKnightUnity -o danted_knight.apk
  # sign apk with ubersigner
  gum spin -s minidot --title "Signing the apk 2/3" --show-error -- uber-apk-signer -a danted_knight.apk
  # install apk to device
  gum spin -s minidot --title "Installing apk to device 3/3" --show-error -- adb install -r danted_knight-aligned-debugSigned.apk
  echo ':boom: Apk installed' | gum format -t emoji

}


start_application(){
  gum spin -s minidot --title "Stopping application.." -- adb shell am force-stop com.KNGames.KnightUnityWorld2
  gum spin -s minidot --title "Stopping application.." --  sleep 3
  gum spin -s minidot --title "Waiting to launch application.." -- adb shell am start -n com.KNGames.KnightUnityWorld2/com.unity3d.player.UnityPlayerActivity
  adb logcat --pid=$(adb shell pidof -s com.KNGames.KnightUnityWorld2) |rogcat -
}

is_installed(){
    path=$(adb shell pm path $1)
    if [ ! -z "$path" ]
    then
        return 55;
    else
        return 44;
    fi
}


is_installed $package_name
if [ $? == 55 ]
then
    #installed
    echo ':boom: Apk is already installed. After building dante.so will be replacing library in device. (Needs root)' | gum format -t emoji
    build_library
    replace_library
    start_application
else
    build_library
    install_application
    start_application
fi