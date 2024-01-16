//! # box_any
//!
//! A wrap Box without generic type that allows dynamic dispatch / downcast.
//!
//! Allows dynamic dispatch like Box<dyn Any> but without using fat pointer.
//! Provides a dynamic box type `BoxAny`, which contains a Box<T> value.
//! `BoxAny' is similar to Box<dyn Any> from `std::any::Any` but it doesn't use
//! fat pointer to dynamic dispatch.


use std::any::TypeId;
use std::convert::From;
use std::ffi::c_void;

#[allow(non_camel_case_types)]
type c_void_mut_ptr = *mut c_void;

/// # Examples
///
/// ```
/// use std::any:: TypeId;
/// use box_any::BoxAny;
///
/// let box_u32 = Box::new(032);
/// let box_string = Box::new(String::from("Hello World"));
/// let box_any_u32: BoxAny = box_u32.clone().into();
/// let box_any_string: BoxAny = box_string.clone().into();
/// let box_any_vec: Vec<BoxAny> = vec![box_any_u32, box_any_string];
/// assert!(box_any_vec[1].is::<String>());
/// let string_2: &String = box_any_vec[1].downcast_ref::<String>().unwrap();
/// assert_eq!(string_2, box_string.as_ref());
/// ```
pub struct BoxAny {
    ptr: c_void_mut_ptr,
    type_id: TypeId,
    f_drop: fn(c_void_mut_ptr) -> (),
}

impl BoxAny {
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    pub fn is<T: 'static>(&self) -> bool {
        self.type_id == TypeId::of::<T>()
    }

    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        if self.is::<T>() {
            return Some(unsafe { self.downcast_ref_unchecked() });
        }
        None
    }

    pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            return Some(unsafe { self.downcast_mut_unchecked() });
        }
        None
    }

    pub unsafe fn downcast_ref_unchecked<T: 'static>(&self) -> &T {
        &*(self.ptr as *const T)
    }

    pub unsafe fn downcast_mut_unchecked<T: 'static>(&mut self) -> &mut T {
        &mut *(self.ptr as *mut T)
    }

    pub fn into_inner<T: 'static>(self) -> Option<Box<T>> {
        if self.is::<T>() {
            return Some(self.into_inner_unchecked());
        }
        None        
    }

    pub fn into_inner_unchecked<T: 'static>(mut self) -> Box<T> {
        let inner = unsafe { Box::from_raw(self.ptr as *mut T) };
        self.ptr = std::ptr::null_mut();
        inner
    }
}

impl<T: 'static> From<Box<T>> for BoxAny {
    /// Consume and convert a Box<T> to a `BoxAny` that allows dynamic dispatch without using fat pointer.
    /// # Examples
    ///
    /// ```
    /// use std::any:: TypeId;
    /// use box_any::BoxAny;
    ///
    /// let box_u32 = Box::new(032);
    /// let box_string = Box::new(String::from("Hello World"));
    /// let box_any_u32: BoxAny = box_u32.clone().into();
    /// let box_any_string: BoxAny = box_string.clone().into();
    /// let box_any_vec: Vec<BoxAny> = vec![box_any_u32, box_any_string];
    /// let string_2: &String = box_any_vec[1].downcast_ref::<String>().unwrap();
    /// assert!(box_any_vec[1].is::<String>());
    /// assert_eq!(string_2, box_string.as_ref());
    /// ```
    fn from(v: Box<T>) -> Self {
        let ptr = Box::into_raw(v) as c_void_mut_ptr;
        let f_drop = drop_ptr::<T>;
        BoxAny {
            ptr,
            type_id: TypeId::of::<T>(),
            f_drop,
        }
    }
}

fn drop_ptr<T>(ptr: c_void_mut_ptr) {
    if !ptr.is_null() {
        unsafe { let _ = Box::from_raw(ptr as *mut T); };
    }
}

impl Drop for BoxAny {
    fn drop(&mut self) {
        (self.f_drop)(self.ptr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let c_usize: BoxAny = Box::new(Vec::<usize>::new()).into();
        let c_string: BoxAny = Box::new(Vec::<String>::new()).into();
        let mut containers: Vec<BoxAny> = vec![c_usize, c_string];
        let c_usize_option: Option<&mut Vec<usize>> = containers[0].downcast_mut();
        if let Some(c_usize_mut) = c_usize_option {
            c_usize_mut.push(0);
            assert_eq!(c_usize_mut, &vec![0usize]);
        } else {
            panic!("Error downcasting mut c_usize");
        };
        let none: Option<&String> = containers[0].downcast_ref();
        assert_eq!(none, None);
        if none.is_some() {
            panic!("Error downcasting c_usize");
        }
        let c_string_option: Option<&mut Vec<String>> = containers[1].downcast_mut();
        if let Some(c_string_mut) = c_string_option {
            c_string_mut.push(String::from(""));
            assert_eq!(c_string_mut, &vec![String::from("")]);
        } else {
            panic!("Error downcasting mut c_string");
        };
        let none: Option<&u32> = containers[1].downcast_ref();
        assert_eq!(none, None);
        if none.is_some() {
            panic!("Error downcasting c_string");
        }        
    }
}
