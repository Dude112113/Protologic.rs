use crate::lowlevel::{self};
use crate::lowlevel::actions::*;

pub fn self_destruct()
{
    unsafe
    {
        lowlevel::actions::ship_self_destruct();
    }
}

pub fn wheel_set_torque(x: f32, y: f32, z: f32)
{
    unsafe
    {
        lowlevel::actions::wheel_set_torque(x, y, z);
    }
}

pub fn engine_set_throttle(throttle: f32)
{
    unsafe
    {
        lowlevel::actions::engine_set_throttle(throttle);
    }
}

pub fn radar_configure(angle: f32, bearing: f32, elevation: f32, trigger: bool)
{
    radar_set_angle(angle);
    radar_set_bearing(bearing);
    radar_set_elevation(elevation);

    if trigger {
        radar_trigger();
    }
}

/// Set the radar arc angle.
/// Min angle can be retrieved with `const_get_shipradarminangle()`.
/// Max angle can be retrieved with `const_get_shipradarmaxangle()`.
pub fn radar_set_angle(angle: f32)
{
    unsafe
    {
        lowlevel::actions::radar_set_angle(angle);
    }
}

/// Set the radar bearing (0..360)
pub fn radar_set_bearing(bearing: f32)
{
    unsafe
    {
        lowlevel::actions::radar_set_bearing(bearing);
    }
}

/// Set the radar elevation (degrees)
pub fn radar_set_elevation(elevation: f32)
{
    unsafe
    {
        lowlevel::actions::radar_set_elevation(elevation);
    }
}

/// Trigger the radar to scan for targets and return the results in the next frame
pub fn radar_trigger()
{
    unsafe
    {
        lowlevel::actions::radar_trigger();
    }
}

pub fn gun_configure(index: i32, bearing: f32, elevation: f32, fuse: f32)
{
    gun_set_bearing(index, bearing);
    gun_set_elevation(index, elevation);
    gun_set_fuse(index, fuse);
}

/// Set the bearing the gun with the given index should begin turning to
pub fn gun_set_bearing(index: i32, bearing: f32)
{
    unsafe
    {
        match index
        {
            0 => gun0_set_bearing(bearing),
            1 => gun1_set_bearing(bearing),
            2 => gun2_set_bearing(bearing),
            3 => gun3_set_bearing(bearing),
            _ => panic!("Unknown gun index: {}", index),
        }
    }
}

/// Set the elevation the gun with the given index should begin turning to.
/// Maximum elevation can be retrieved with `const_get_turretmaxelevation()`.
pub fn gun_set_elevation(index: i32, elevation: f32)
{
    unsafe
    {
        match index
        {
            0 => gun0_set_elevation(elevation),
            1 => gun1_set_elevation(elevation),
            2 => gun2_set_elevation(elevation),
            3 => gun3_set_elevation(elevation),
            _ => panic!("Unknown gun index: {}", index),
        }
    }
}

/// Set the fuse for shells fired from the given gun
pub fn gun_set_fuse(index: i32, fuse: f32)
{
    unsafe
    {
        match index
        {
            0 => gun0_set_fuse(fuse),
            1 => gun1_set_fuse(fuse),
            2 => gun2_set_fuse(fuse),
            3 => gun3_set_fuse(fuse),
            _ => panic!("Unknown gun index: {}", index),
        }
    }
}

/// Set a flag to fire the gun with the given index once it is ready to fire
pub fn gun_trigger(index: i32)
{
    unsafe
    {
        match index
        {
            0 => gun0_trigger(),
            1 => gun1_trigger(),
            2 => gun2_trigger(),
            3 => gun3_trigger(),
            _ => panic!("Unknown gun index: {}", index),
        }
    }
}