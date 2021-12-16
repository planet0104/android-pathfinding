use anyhow::{anyhow, Result};
use bracket_pathfinding::prelude::Point;
use jni::objects::{JObject, JString};
use jni::sys::{jint, jintArray, jstring};
use jni::{objects::JClass, sys::jobjectArray};
use jni::{JNIEnv, JavaVM};
use log::{error, info};
use pathfinding::VERSION;

mod pathfinding;

#[no_mangle]
pub extern "C" fn JNI_OnLoad(_jvm: JavaVM, _reserved: *mut std::ffi::c_void) -> jint {
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Info));
    info!("pathfinding JNI_OnLoad.");
    jni::sys::JNI_VERSION_1_6
}

fn jobject_array_to_grid<'a>(env: &'a JNIEnv, grid_obj: jobjectArray) -> Result<Vec<Vec<u8>>> {
    let mut grid: Vec<Vec<u8>> = vec![];
    let height = env.get_array_length(grid_obj)? as usize;
    //检查地图宽度是否相等
    let mut total_width = 0usize;
    for i in 0..height {
        let line = env.get_object_array_element(grid_obj, i as i32)?;
        let width = env.get_array_length(*line)?;
        total_width += width as usize;
        let mut row = vec![0; width as usize];
        env.get_int_array_region(*line, 0, &mut row)?;
        grid.push(row.iter().map(|v| *v as u8).collect());
    }

    if total_width != height * grid[0].len() {
        Err(anyhow!("地图行长度不一致"))
    } else {
        Ok(grid)
    }
}

#[no_mangle]
pub extern "C" fn Java_run_ccfish_android_pathfinding_PathFinding_version<'a>(env: JNIEnv, _activity: JClass) -> jstring {
    let result = (|| -> Result<JString> {
        Ok(JString::from(env.new_string(VERSION)?))
    })();

    match result{
        Ok(ver) => ver.into_inner(),
        Err(err) => {
            error!("{:?}", &err);
            JObject::null().into_inner()
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_run_ccfish_android_pathfinding_PathFinding_loadMap<'a>(
    env: JNIEnv,
    _activity: JClass,
    grid_obj: jobjectArray,
) {
    let result = (|| -> Result<()> {
        Ok(pathfinding::load_map(jobject_array_to_grid(
            &env, grid_obj,
        )?))
    })();

    if result.is_err() {
        let err = result.err();
        error!("{:?}", &err);
        let _ = env.throw_new("java/lang/Exception", format!("{:?}", err));
    }
}

#[no_mangle]
pub extern "C" fn Java_run_ccfish_android_pathfinding_PathFinding_loadMapForKey<'a>(
    env: JNIEnv,
    _activity: JClass,
    key: JString,
    grid_obj: jobjectArray,
) {
    let result = (|| -> Result<()> {
        let grid = jobject_array_to_grid(&env, grid_obj)?;
        let key = env.get_string(key)?.to_str()?.to_string();
        Ok(pathfinding::load_map_for_key(key, grid))
    })();

    if result.is_err() {
        let err = result.err();
        error!("{:?}", &err);
        let _ = env.throw_new("java/lang/Exception", format!("{:?}", err));
    }
}

#[no_mangle]
pub extern "C" fn Java_run_ccfish_android_pathfinding_PathFinding_findPath<'a>(
    env: JNIEnv,
    _activity: JClass,
    x1: jint,
    y1: jint,
    x2: jint,
    y2: jint,
) -> jintArray {
    match (|| -> Result<JObject> {
        if x1 < 0 || y1 < 0 || x2 < 0 || y2 < 0 {
            return Err(anyhow!("坐标点不能小于0"));
        }
        match pathfinding::find_path(Point::new(x1, y1), Point::new(x2, y2))? {
            Some(path) => {
                let mut val_arr = Vec::with_capacity(path.len() * 2);
                for point in path {
                    val_arr.push(point.x);
                    val_arr.push(point.y);
                }
                let arr = env.new_int_array(val_arr.len() as i32)?;
                env.set_int_array_region(arr, 0, &val_arr)?;
                Ok(JObject::from(arr))
            }
            None => Ok(JObject::null()),
        }
    })() {
        Ok(path) => path.into_inner(),
        Err(err) => {
            error!("{:?}", &err);
            let _ = env.throw_new("java/lang/Exception", format!("{:?}", err));
            JObject::null().into_inner()
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_run_ccfish_android_pathfinding_PathFinding_findPathInMap<'a>(
    env: JNIEnv,
    _activity: JClass,
    key: JString,
    x1: jint,
    y1: jint,
    x2: jint,
    y2: jint,
) -> jintArray {
    match (|| -> Result<JObject> {
        if x1 < 0 || y1 < 0 || x2 < 0 || y2 < 0 {
            return Err(anyhow!("坐标点不能小于0"));
        }
        let key = env.get_string(key)?;
        let key = key.to_str()?;
        match pathfinding::find_path_in_map(key, Point::new(x1, y1), Point::new(x2, y2))? {
            Some(path) => {
                let mut val_arr = Vec::with_capacity(path.len() * 2);
                for point in path {
                    val_arr.push(point.x);
                    val_arr.push(point.y);
                }
                let arr = env.new_int_array(val_arr.len() as i32)?;
                env.set_int_array_region(arr, 0, &val_arr)?;
                Ok(JObject::from(arr))
            }
            None => Ok(JObject::null()),
        }
    })() {
        Ok(path) => path.into_inner(),
        Err(err) => {
            error!("{:?}", &err);
            let _ = env.throw_new("java/lang/Exception", format!("{:?}", err));
            JObject::null().into_inner()
        }
    }
}
