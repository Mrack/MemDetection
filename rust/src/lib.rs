/**
 * @author Mrack
 * @date 2022/12/13
 */

extern crate core;
mod check;
mod maps;

#[macro_use]
extern crate log;

#[cfg(target_os="android")]
mod android {
    macro_rules! jni_unwrap {
        ($obj:expr, $msg:expr) => {
            $obj.unwrap_or_else(|e| {
                error!("Error: {}: {}", $msg, e);
                panic!($msg);
            })
        };
    }

    use android_logger::Config;
    use jni::{JavaVM, JNIEnv};
    use jni::objects::{JClass, JString};
    use jni::sys::{jint, JNI_ERR, JNI_VERSION_1_6, jstring};
    use crate::check::{Checker};
    use crate::maps::ProcMaps;
    use log::{Level};

    #[no_mangle]
    pub extern "C" fn JNI_OnLoad(vm: JavaVM, _: *mut std::ffi::c_void) -> jint {
        let env = vm.get_env().expect("Cannot get reference to the JNIEnv");
        if env.get_java_vm().is_err() {
            return JNI_ERR;
        }
        android_logger::init_once(
            Config::default()
                .with_min_level(Level::Debug)
                .with_tag("onCreate")
        );
        JNI_VERSION_1_6
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_cn_mrack_detection_MainActivity_check(
        env: JNIEnv,
        _: JClass,
        str: JString,
    ) -> jstring {
        let f: String = jni_unwrap!(env.get_string(str),
                                  "Couldn't get java string").into();
        let maps = ProcMaps::new().unwrap();
        for m in maps {
            // if m.pathname.contains(&f) && m.perm == "r-xp" {
            if m.pathname.contains(&f) && m.perm.contains("x") {

                let result = Checker::new(m).check();
                let f = if result.is_ok() {
                    "Elf is not modified"
                } else {
                    result.err().unwrap()
                };

                let res = env
                    .new_string(f)
                    .expect("Couldn't create java string!")
                    .into_inner();
                return res;
            }
        };
        let res = env
            .new_string("detect failed")
            .expect("Couldn't create java string!")
            .into_inner();
        return res;
    }
}

