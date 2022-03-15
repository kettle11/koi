use crate::*;

thread_local! {
    static DOWNLOAD_FILE: JSObjectFromString = JSObjectFromString::new(r#"
    function downloadURL (data, fileName) {
        const a = document.createElement('a')
        a.href = data
        a.download = fileName
        document.body.appendChild(a)
        a.style.display = 'none'
        a.click()
        a.remove()
    }
      
      function downloadBlob (data_ptr, data_len, fileName_index, mimeType_index) {
        let fileName =  self.kwasm_get_object(fileName_index);
        let mimeType =  self.kwasm_get_object(mimeType_index);

        const message_data = new Uint8Array(new Uint8Array(self.kwasm_memory.buffer, data_ptr, data_len));

        const blob = new Blob([message_data], {
          type: mimeType
        })
      
        const url = window.URL.createObjectURL(blob)
      
        downloadURL(url, fileName)
      
        setTimeout(() => window.URL.revokeObjectURL(url), 1000)
      }
      downloadBlob
"#);
}

pub fn download(data: &[u8], file_name: &str, mime_type: &str) {
    let js_name = JSString::new(file_name);
    let js_mime_type = JSString::new(mime_type);
    DOWNLOAD_FILE.with(|v| {
        v.call_raw(&[
            data.as_ptr() as u32,
            data.len() as u32,
            js_name.index(),
            js_mime_type.index(),
        ])
    });
}
