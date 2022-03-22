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
    let phidget_rfid = unsafe { &mut *(ctx as *mut PhidgetRfid) };
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
    pub handle: PhidgetRFIDHandle,
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

    pub fn run(mut self, on_attach: impl Fn(&mut Self) + 'static) -> Result<(), PhidgetError> {
        unsafe {
            match PhidgetRFID_create(&mut self.handle as *mut _ as *mut PhidgetRFIDHandle) {
                PhidgetReturnCode_EPHIDGET_OK => {
                    let phidget_rfid_raw = &mut self as *mut _ as *mut std::os::raw::c_void;
                    PhidgetRFID_setOnTagHandler(
                        self.handle,
                        Some(phidget_rfid_on_tag_handler),
                        phidget_rfid_raw,
                    );
                    PhidgetRFID_setOnTagLostHandler(
                        self.handle,
                        Some(phidget_rfid_on_tag_lost_handler),
                        phidget_rfid_raw,
                    );
                    Phidget_openWaitForAttachment(self.handle as PhidgetHandle, 5000);

                    on_attach(&mut self);

                    match Phidget_close(self.handle as PhidgetHandle) {
                        PhidgetReturnCode_EPHIDGET_OK => {
                            println!("Phidget dropped: {:?}", &self.handle);
                            PhidgetRFID_delete(
                                &mut self.handle as *mut _ as *mut PhidgetRFIDHandle,
                            );
                        }
                        return_code => {
                            println!("Phidget error: {}", return_code);
                        }
                    }

                    Ok(())
                }
                return_code => Err(PhidgetError {
                    return_code: Some(return_code),
                }),
            }
        }
    }
}
