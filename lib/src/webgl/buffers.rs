use web_sys::{WebGlBuffer};
use js_sys::{WebAssembly};
use wasm_bindgen::JsCast;
use crate::errors::{Error, NativeError};
use super::{WebGlContext, Id, AttributeOptions, Attribute, WebGlRenderer, BufferTarget, BufferUsage};


pub enum BufferData<'a>{
    F32(&'a [f32]),
    U8(&'a [u8])
}

pub fn upload_buffer_direct(gl:&WebGlContext, buffer_data:BufferData, target: BufferTarget, usage:BufferUsage, webgl_buff:&WebGlBuffer) -> Result<(), Error> {

    match buffer_data {
        BufferData::F32(values) => {
            wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>()
                .map(|m:WebAssembly::Memory| {

                    let wasm_buffer = m.buffer();
                    let ptr_loc = values.as_ptr() as u32 / 4;

                    let float32 = js_sys::Float32Array::new(&wasm_buffer)
                                    .subarray(ptr_loc, ptr_loc + values.len() as u32);
            
                    //Note - WebGL2 can do less GC hits by pointing at same memory with different start/end
                    gl.buffer_data_with_array_buffer_view(target as u32, &float32, usage as u32); 
                    
                })
                .map_err(|err| Error::from(err))
        },

        BufferData::U8(values) => {
            gl.buffer_data_with_u8_array(target as u32, &values, usage as u32); 
            Ok(())
        }
    }
}

pub fn bind_buffer(gl:&WebGlContext, target:BufferTarget, buffer:&WebGlBuffer) {
    gl.bind_buffer(target as u32, Some(buffer)); 
}

impl WebGlRenderer {
    pub fn create_buffer(&mut self) -> Result<Id, Error> {
        let buffer = self.gl.create_buffer()
            .ok_or(Error::from(NativeError::NoCreateBuffer))?;

        let id = self.buffer_lookup.insert(buffer);

        Ok(id)
    }


    //only pub within the module - used elsewhere like attributes
    pub(super) fn _activate_buffer_nocheck(&self, buffer_id:Id, target: BufferTarget) -> Result<(), Error> {

        self.current_buffer_id.set(Some(buffer_id));
        self.current_buffer_target.set(Some(target));

        let buffer = self.get_current_buffer()?; 
        bind_buffer(&self.gl, target, &buffer);

        Ok(())
    }
    pub fn activate_buffer(&self, buffer_id:Id, target: BufferTarget) -> Result<(), Error> {

        if Some(buffer_id) != self.current_buffer_id.get() || Some(target) != self.current_buffer_target.get() {
            self._activate_buffer_nocheck(buffer_id, target)
        } else {
            Ok(())
        }
    }

    fn get_current_buffer(&self) -> Result<&WebGlBuffer, Error> {
        let buffer_id = self.current_buffer_id.get().ok_or(Error::from(NativeError::MissingBuffer))?;
        self.buffer_lookup.get(buffer_id).ok_or(Error::from(NativeError::MissingShaderProgram))
    }

    pub fn upload_buffer(&self, id:Id, data:BufferData, target: BufferTarget, usage:BufferUsage) -> Result<(), Error> {
        self.activate_buffer(id, target)?;

        let buffer = self.get_current_buffer()?; 

        upload_buffer_direct(&self.gl, data, target, usage, &buffer)
    }

    //Just a helper to make it simpler
    pub fn upload_buffer_to_attribute(&self, id:Id, data:BufferData, target: BufferTarget, usage:BufferUsage, attribute:&Attribute, opts:&AttributeOptions) -> Result<(), Error> {
        self.upload_buffer(id, data, target, usage)?;
        self.activate_attribute(&attribute, &opts)?;
        Ok(())
    }

}
