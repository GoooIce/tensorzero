use anyhow::{anyhow, Result, Context};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing::{debug, error, info, instrument};
use wasmtime::*;

// Constants for Wasm function names (based on wasm-bindgen conventions)
const WASM_SIGN_FN: &str = "sign";
const WASM_MALLOC_FN: &str = "__wbindgen_malloc";
// const WASM_REALLOC_FN: &str = "__wbindgen_realloc";
const WASM_FREE_FN: &str = "__wbindgen_free";
const WASM_MEMORY: &str = "memory";

// Statically include the WASM binary
const WASM_BINARY: &[u8] = include_bytes!("../sign_bg.wasm");

struct WasmSignerInner {
    store: Store<()>, 
    // instance: Instance,
    memory: Memory,
    sign_func: TypedFunc<(i32, i32, i32, i32, i32, i32, i32, i32, i32), ()>,
    malloc_func: TypedFunc<(i32, i32), i32>,
    // realloc_func: Option<TypedFunc<(i32, i32, i32, i32), i32>>, // Realloc might not be strictly needed if we pre-allocate enough
    free_func: TypedFunc<(i32, i32, i32), ()>,
}

pub struct WasmSigner {
    inner: Mutex<WasmSignerInner>,
}

// Lazy static initialization for the Wasm environment
static WASM_SIGNER: Lazy<Result<WasmSigner>> = Lazy::new(|| {
    info!("Initializing WasmSigner...");
    WasmSigner::new()
});

impl WasmSignerInner {
    #[instrument(skip_all, name = "wasm_inner_new")]
    fn new() -> Result<Self> {
        debug!("Creating Wasmtime engine...");
        let engine = Engine::default();
        let mut store = Store::new(&engine, ());

        debug!("Loading WASM module from embedded binary...");
        let module = Module::from_binary(&engine, WASM_BINARY)
            .map_err(|e| anyhow!("Failed to load WASM module from embedded binary: {}", e))?;

        debug!("Instantiating WASM module...");
        // We don't need any imports for this specific WASM based on sign.mjs analysis
        let instance = Instance::new(&mut store, &module, &[])
            .map_err(|e| anyhow!("Failed to instantiate WASM module: {}", e))?;

        debug!("Getting WASM exports...");
        let memory = instance
            .get_memory(&mut store, WASM_MEMORY)
            .ok_or_else(|| anyhow!("WASM export '{}' not found", WASM_MEMORY))?;

        let sign_func = instance
            .get_typed_func::<_, ()>(&mut store, WASM_SIGN_FN)
            .map_err(|e| anyhow!("Failed to get typed func '{}': {}", WASM_SIGN_FN, e))?;

        let malloc_func = instance
            .get_typed_func::<_, i32>(&mut store, WASM_MALLOC_FN)
            .map_err(|e| anyhow!("Failed to get typed func '{}': {}", WASM_MALLOC_FN, e))?;

        // let realloc_func = instance
        //     .get_typed_func::<_, i32>(&mut store, WASM_REALLOC_FN).ok(); // Realloc might be optional

        let free_func = instance
            .get_typed_func::<_, ()>(&mut store, WASM_FREE_FN)
            .map_err(|e| anyhow!("Failed to get typed func '{}': {}", WASM_FREE_FN, e))?;

        info!("WASM module instantiated successfully.");
        Ok(Self {
            store,
            // instance,
            memory,
            sign_func,
            malloc_func,
            // realloc_func,
            free_func,
        })
    }

    /// Helper to write string to WASM memory, returns (ptr, len)
    #[instrument(skip(self, data), fields(data_len = data.len()), name = "wasm_write_string")]
    fn write_string_to_wasm(&mut self, data: &str) -> Result<(i32, i32)> {
        let bytes = data.as_bytes();
        let len = bytes.len() as i32;
        // Alignment requirement is 1 for __wbindgen_malloc
        let ptr = self.malloc_func.call(&mut self.store, (len, 1))?;
        debug!("WASM malloc returned ptr: {}", ptr);
        if ptr == 0 {
            return Err(anyhow!("WASM malloc failed (returned 0)"));
        }
        self.memory.write(&mut self.store, ptr as usize, bytes)?;
        Ok((ptr, len))
    }

    /// Helper to read string from WASM memory
    #[instrument(skip(self), name = "wasm_read_string")]
    fn read_string_from_wasm(&self, ptr: i32, len: i32) -> Result<String> {
        if len == 0 {
            return Ok(String::new());
        }
        let mut buffer = vec![0u8; len as usize];
        self.memory.read(&self.store, ptr as usize, &mut buffer)?;
        // Assume UTF-8 encoding as per wasm-bindgen standard
        String::from_utf8(buffer).map_err(|e| anyhow!("Failed to decode UTF-8 string from WASM: {}", e))
    }
}

impl WasmSigner {
    #[instrument(name = "wasm_signer_new")]
    fn new() -> Result<Self> {
        let inner = WasmSignerInner::new()?;
        Ok(WasmSigner { inner: Mutex::new(inner) })
    }

    /// Gets a handle to the globally initialized WasmSigner instance.
    pub fn get_instance() -> Result<&'static Self> {
         match WASM_SIGNER.as_ref() {
             Ok(signer) => Ok(signer),
             Err(e) => {
                 // This attempts to return a reference to the error, which isn't ideal.
                 // A better approach might be to panic on init failure or return Option/Result.
                 // For simplicity here, we log and return the error wrapped in anyhow.
                 // In a real app, consider how unrecoverable init failure should be handled.
                 error!("WasmSigner lazy initialization failed: {}", e);
                 Err(anyhow!("WasmSigner lazy initialization failed: {}", e))
             }
         }
    }

    #[instrument(skip(self, nonce, timestamp, device_id, query), fields(nonce_len=nonce.len(), ts_len=timestamp.len(), device_id_len=device_id.len(), query_len=query.len()))]
    pub fn sign(
        &self,
        nonce: &str,
        timestamp: &str,
        device_id: &str,
        query: &str,
    ) -> Result<String> {
        // Acquire lock once
        let mut guard = self.inner.lock().expect("WASM Signer mutex poisoned");
        let WasmSignerInner {
            store,
            memory,
            sign_func,
            malloc_func,
            free_func,
            .. // Ignore instance if not needed directly
        } = &mut *guard;

        // --- Temporary storage for pointers/lengths --- 
        let nonce_ptr: i32;
        let nonce_len: i32;
        let timestamp_ptr: i32;
        let timestamp_len: i32;
        let device_id_ptr: i32;
        let device_id_len: i32;
        let query_ptr: i32;
        let query_len: i32;
        let ret_ptr_ptr: i32;
        let result_ptr: i32;
        let result_len: i32;
        let result_string: String;

        // --- All operations happen within the lock scope --- 

        // 1. Allocate memory for result pointer
        ret_ptr_ptr = malloc_func.call(&mut *store, (8, 4))
            .context("WASM malloc failed for return pointer allocation")?;
        if ret_ptr_ptr == 0 {
            return Err(anyhow!("WASM malloc failed for return pointer (returned 0)"));
        }
        debug!("Allocated ret_ptr_ptr at: {}", ret_ptr_ptr);

        // 2. Write input strings (using a helper closure)
        let write_string = |s_store: &mut Store<()>, s_data: &str| -> Result<(i32, i32)> {
            let bytes = s_data.as_bytes();
            let len = bytes.len() as i32;
            let ptr = malloc_func.call(&mut *s_store, (len, 1))
                .context("WASM malloc failed for string data")?;
            if ptr == 0 { return Err(anyhow!("WASM malloc failed for string (returned 0)")); }

            memory.write(&mut *s_store, ptr as usize, bytes)
                .context("Failed to write string data to WASM memory")?;
            Ok((ptr, len))
        };

        (nonce_ptr, nonce_len) = write_string(&mut *store, nonce)?;
        (timestamp_ptr, timestamp_len) = write_string(&mut *store, timestamp)?;
        (device_id_ptr, device_id_len) = write_string(&mut *store, device_id)?;
        (query_ptr, query_len) = write_string(&mut *store, query)?;

        // 3. Call the WASM sign function
        debug!("Calling WASM sign function...");
        sign_func.call(
            &mut *store,
            (
                ret_ptr_ptr, nonce_ptr, nonce_len, timestamp_ptr, timestamp_len,
                device_id_ptr, device_id_len, query_ptr, query_len,
            ),
        ).context("WASM sign function call failed")?;
        debug!("WASM sign function returned.");

        // 4. Read the result pointer and length
        // memory.read only needs &Store according to docs
        let mut ret_buf = [0u8; 8];
        memory.read(&mut *store, ret_ptr_ptr as usize, &mut ret_buf)
             .context("Failed to read result pointer/length from WASM memory")?;
        result_ptr = i32::from_le_bytes(ret_buf[0..4].try_into().unwrap());
        result_len = i32::from_le_bytes(ret_buf[4..8].try_into().unwrap());
        debug!("WASM returned result ptr: {}, len: {}", result_ptr, result_len);

        // 5. Read the actual result string (using a helper closure)
        let read_string = |r_store: &Store<()>, ptr: i32, len: i32| -> Result<String> {
            if len == 0 { return Ok(String::new()); }
            let mut buffer = vec![0u8; len as usize];
            // Pass memory explicitly to avoid borrow issues with store if read needs &mut store?
            // Re-check wasmtime docs for Memory::read signature.
            // Assuming Memory::read takes &Store for now.
            memory.read(r_store, ptr as usize, &mut buffer)
                .context("Failed to read result string bytes from WASM memory")?;
            String::from_utf8(buffer).map_err(|e| anyhow!("Failed to decode UTF-8 result string from WASM: {}", e))
        };
        // Use deref coercion to get &Store from &mut Store
        result_string = read_string(store, result_ptr, result_len)?;


        // 6. Free WASM memory
        debug!("Freeing WASM memory...");
        free_func.call(&mut *store, (nonce_ptr, nonce_len, 1))
            .context("Failed to free nonce string in WASM")?;
        free_func.call(&mut *store, (timestamp_ptr, timestamp_len, 1))
             .context("Failed to free timestamp string in WASM")?;
        free_func.call(&mut *store, (device_id_ptr, device_id_len, 1))
              .context("Failed to free device_id string in WASM")?;
        free_func.call(&mut *store, (query_ptr, query_len, 1))
              .context("Failed to free query string in WASM")?;
        free_func.call(&mut *store, (ret_ptr_ptr, 8, 4))
              .context("Failed to free return pointer structure in WASM")?;
        if result_ptr != 0 {
            free_func.call(&mut *store, (result_ptr, result_len, 1))
                 .context("Failed to free result string in WASM")?;
        }
        debug!("WASM memory freed.");

        Ok(result_string)
    }
} 