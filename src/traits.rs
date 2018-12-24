pub trait Array {
    type Element: Copy;

    ///
    /// The length of the the underlying array.
    ///
    fn len() -> usize;

    /// 
    /// Generate a pointer to the underlying array for passing a
    /// matrix or vector to the graphics hardware.
    ///
    fn as_ptr(&self) -> *const Self::Element; 

    /// 
    /// Generate a mutable pointer to the underlying array for passing a
    /// matrix or vector to the graphics hardware.
    ///
    fn as_mut_ptr(&mut self) -> *mut Self::Element; 
}