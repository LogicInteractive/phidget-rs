pub extern crate phidget22_sys;

pub use phidget22_sys::*;
use std::ffi::CStr;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct PhidgetError {
    pub return_code: Option<i32>,
}

#[no_mangle]
extern "C" fn phidget_rfid_on_tag_handler(
    _ch: PhidgetRFIDHandle,
    ctx: *mut ::std::os::raw::c_void,
    tag: *const ::std::os::raw::c_char,
    _protocol: PhidgetRFID_Protocol,
) {
    let tag = unsafe { CStr::from_ptr(tag).to_str().unwrap_or_default() };
    let phidget_rfid = unsafe { Rc::from_raw(ctx as *mut Rc<PhidgetRfid>) };
    if let Some(on_tag) = &phidget_rfid.on_tag {
        on_tag(tag.to_string());
    }
}

#[no_mangle]
extern "C" fn phidget_rfid_on_tag_lost_handler(
    _ch: PhidgetRFIDHandle,
    ctx: *mut ::std::os::raw::c_void,
    tag: *const ::std::os::raw::c_char,
    _protocol: PhidgetRFID_Protocol,
) {
    let tag = unsafe { CStr::from_ptr(tag).to_str().unwrap_or_default() };
    let phidget_rfid = unsafe { Rc::from_raw(ctx as *mut Rc<PhidgetRfid>) };
    if let Some(on_tag_lost) = &phidget_rfid.on_tag_lost {
        on_tag_lost(tag.to_string());
    }
}

#[derive(Clone)]
pub struct PhidgetRfid {
    handle: PhidgetRFIDHandle,
    on_tag: Option<Rc<dyn Fn(String)>>,
    on_tag_lost: Option<Rc<dyn Fn(String)>>,
}

impl Default for PhidgetRfid {
    fn default() -> Self {
        let handle = std::ptr::null_mut::<_PhidgetRFID>();
        PhidgetRfid {
            handle,
            on_tag: None,
            on_tag_lost: None,
        }
    }
}

#[allow(non_upper_case_globals)]
impl PhidgetRfid {
    pub fn on_tag(mut self, on_tag: impl Fn(String) + 'static) -> Self {
        self.on_tag = Some(Rc::new(on_tag));
        self
    }

    pub fn on_tag_lost(mut self, on_tag_lost: impl Fn(String) + 'static) -> Self {
        self.on_tag_lost = Some(Rc::new(on_tag_lost));
        self
    }

    pub fn build(mut self) -> Result<Rc<Self>, PhidgetError> {
        unsafe {
            match PhidgetRFID_create(&mut self.handle as *mut _ as *mut PhidgetRFIDHandle) {
                PhidgetReturnCode_EPHIDGET_OK => {
                    let phidget_rfid = Rc::new(self);
                    let phidget_rfid_raw =
                        &mut phidget_rfid.clone() as *mut _ as *mut std::os::raw::c_void;
                    PhidgetRFID_setOnTagHandler(
                        phidget_rfid.handle,
                        Some(phidget_rfid_on_tag_handler),
                        phidget_rfid_raw,
                    );
                    PhidgetRFID_setOnTagLostHandler(
                        phidget_rfid.handle,
                        Some(phidget_rfid_on_tag_lost_handler),
                        phidget_rfid_raw,
                    );
                    Phidget_openWaitForAttachment(phidget_rfid.handle as PhidgetHandle, 5000);
                    println!("Attached! {:?}", phidget_rfid.handle);
                    Ok(phidget_rfid)
                }
                return_code => Err(PhidgetError {
                    return_code: Some(return_code),
                }),
            }
        }
    }
}

#[allow(non_upper_case_globals)]
impl Drop for PhidgetRfid {
    fn drop(&mut self) {
        unsafe {
            if !self.handle.is_null() {
                match Phidget_close(self.handle as PhidgetHandle) {
                    PhidgetReturnCode_EPHIDGET_OK => {
                        println!("Phidget dropped: {:?}", &self.handle);
                        PhidgetRFID_delete(&mut self.handle as *mut _ as *mut PhidgetRFIDHandle);
                    }
                    return_code => {
                        println!("Phidget error: {}", return_code);
                    }
                }
            }
        }
    }
}
