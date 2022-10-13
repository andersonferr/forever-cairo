use ::libc;
pub type cairo_list_t = _cairo_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_list {
    pub next: *mut _cairo_list,
    pub prev: *mut _cairo_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_observer {
    pub link: cairo_list_t,
    pub callback: Option::<
        unsafe extern "C" fn(*mut cairo_observer_t, *mut libc::c_void) -> (),
    >,
}
pub type cairo_observer_t = _cairo_observer;
#[no_mangle]
pub unsafe extern "C" fn _cairo_observers_notify(
    mut observers: *mut cairo_list_t,
    mut arg: *mut libc::c_void,
) {
    let mut obs: *mut cairo_observer_t = 0 as *mut cairo_observer_t;
    let mut next: *mut cairo_observer_t = 0 as *mut cairo_observer_t;
    obs = ({
        let mut mptr__: *const cairo_list_t = (*observers).next;
        (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut cairo_observer_t
    });
    next = ({
        let mut mptr__: *const cairo_list_t = (*obs).link.next;
        (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut cairo_observer_t
    });
    while &mut (*obs).link as *mut cairo_list_t != observers {
        ((*obs).callback).expect("non-null function pointer")(obs, arg);
        obs = next;
        next = ({
            let mut mptr__: *const cairo_list_t = (*next).link.next;
            (mptr__ as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
                as *mut cairo_observer_t
        });
    }
}
