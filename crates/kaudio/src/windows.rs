/// Much of this file is a stripped down version of WinAPI with just the parts relevant to kaudio.
/// https://github.com/retep998/winapi-rs
///
/// WinAPI is licensed under Apache 2.0 or MIT:
///
/// Copyright (c) 2015-2018 The winapi-rs Developers
///
/// Permission is hereby granted, free of charge, to any person obtaining a copy
/// of this software and associated documentation files (the "Software"), to deal
/// in the Software without restriction, including without limitation the rights
/// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
/// copies of the Software, and to permit persons to whom the Software is
/// furnished to do so, subject to the following conditions:
///
/// The above copyright notice and this permission notice shall be included in all
/// copies or substantial portions of the Software.
///
/// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
/// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
/// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
/// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
/// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
/// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
/// SOFTWARE.
///
use crate::*;

type LPWSTR = *mut u16;

pub const CLSCTX_ALL: u32 =
    CLSCTX_INPROC_SERVER | CLSCTX_INPROC_HANDLER | CLSCTX_LOCAL_SERVER | CLSCTX_REMOTE_SERVER;

pub const CLSCTX_INPROC_SERVER: u32 = 0x1;
pub const CLSCTX_INPROC_HANDLER: u32 = 0x2;
pub const CLSCTX_LOCAL_SERVER: u32 = 0x4;
pub const CLSCTX_REMOTE_SERVER: u32 = 0x10;

type HRESULT = i32;
type LPVOID = *mut std::ffi::c_void;
type WORD = u16;
type DWORD = u32;
type REFCLSID = *const GUID;
type REFIID = *const GUID;
type LPCGUID = *const GUID;
type AUDCLNT_SHAREMODE = u32;
type BOOL = i32;
pub type LPCWSTR = *const u16;
pub type LPCSTR = *const i8;

const WAVE_FORMAT_IEEE_FLOAT: u16 = 0x0003;
type REFERENCE_TIME = i64;

const AUDCLNT_SHAREMODE_SHARED: u32 = 0;
pub const AUDCLNT_STREAMFLAGS_EVENTCALLBACK: DWORD = 0x00040000;
pub const AUDCLNT_STREAMFLAGS_RATEADJUST: DWORD = 0x00100000;

pub const INFINITE: DWORD = 0xFFFFFFFF;

#[repr(C)]
struct WAVEFORMATEX {
    pub wFormatTag: WORD,
    pub nChannels: WORD,
    pub nSamplesPerSec: DWORD,
    pub nAvgBytesPerSec: DWORD,
    pub nBlockAlign: WORD,
    pub wBitsPerSample: WORD,
    pub cbSize: WORD,
}

macro_rules! RIDL {
    (#[uuid($l:expr, $w1:expr, $w2:expr,
        $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr, $b6:expr, $b7:expr, $b8:expr)]
    class $class:ident;) => (
        pub enum $class {}
        impl $crate::Class for $class {
            #[inline]
            fn uuidof() -> $crate::shared::guiddef::GUID {
                $crate::shared::guiddef::GUID {
                    Data1: $l,
                    Data2: $w1,
                    Data3: $w2,
                    Data4: [$b1, $b2, $b3, $b4, $b5, $b6, $b7, $b8],
                }
            }
        }
    );
    (#[uuid($($uuid:expr),+)]
    interface $interface:ident ($vtbl:ident) {$(
        $(#[$($attrs:tt)*])* fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,
    )+}) => (
        RIDL!{@vtbl $interface $vtbl () $(
            $(#[$($attrs)*])* fn $method($($p: $t,)*) -> $rtr,
        )+}
        #[repr(C)]
        struct $interface {
            pub lpVtbl: *const $vtbl,
        }
        impl $interface {
            $(RIDL!{@method $(#[$($attrs)*])* fn $method($($p: $t,)*) -> $rtr})+
        }
        RIDL!{@uuid $interface $($uuid),+}
    );
    (#[uuid($($uuid:expr),+)]
    interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) {}) => (
        RIDL!{@vtbl $interface $vtbl (pub parent: $pvtbl,)}
        #[repr(C)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl,
        }
        RIDL!{@deref $interface $pinterface}
        RIDL!{@uuid $interface $($uuid),+}
    );
    (#[uuid($($uuid:expr),+)]
    interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) {$(
        $(#[$($attrs:tt)*])* fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,
    )+}) => (
        RIDL!{@vtbl $interface $vtbl (pub parent: $pvtbl,) $(
            $(#[$($attrs)*])* fn $method($($p: $t,)*) -> $rtr,
        )+}
        #[repr(C)]
        struct $interface {
            pub lpVtbl: *const $vtbl,
        }
        impl $interface {
            $(RIDL!{@method $(#[$($attrs)*])* fn $method($($p: $t,)*) -> $rtr})+
        }
        RIDL!{@deref $interface $pinterface}
        RIDL!{@uuid $interface $($uuid),+}
    );
    (@deref $interface:ident $pinterface:ident) => (
        impl std::ops::Deref for $interface {
            type Target = $pinterface;
            #[inline]
            fn deref(&self) -> &$pinterface {
                unsafe { &*(self as *const $interface as *const $pinterface) }
            }
        }
    );
    (@method fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty) => (
        #[allow(dead_code)]
        #[inline] unsafe fn $method(&self, $($p: $t,)*) -> $rtr {
            ((*self.lpVtbl).$method)(self as *const _ as *mut _, $($p,)*)
        }
    );
    (@method #[fixme] fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty) => (
        #[inline] pub unsafe fn $method(&self, $($p: $t,)*) -> $rtr {
            let mut ret = $crate::_core::mem::uninitialized();
            ((*self.lpVtbl).$method)(self as *const _ as *mut _, &mut ret, $($p,)*);
            ret
        }
    );
    (@vtbl $interface:ident $vtbl:ident ($($fields:tt)*)
        $(fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,)*
    ) => (
        RIDL!{@item #[repr(C)]
        struct $vtbl {
            $($fields)*
            $(pub $method: unsafe extern "system" fn(
                This: *mut $interface,
                $($p: $t,)*
            ) -> $rtr,)*
        }}
    );
    (@vtbl $interface:ident $vtbl:ident ($($fields:tt)*)
        fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,
    $($tail:tt)*) => (
        RIDL!{@vtbl $interface $vtbl (
            $($fields)*
            pub $method: unsafe extern "system" fn(
                This: *mut $interface,
                $($p: $t,)*
            ) -> $rtr,
        ) $($tail)*}
    );
    (@vtbl $interface:ident $vtbl:ident ($($fields:tt)*)
        #[fixme] fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,
    $($tail:tt)*) => (
        RIDL!{@vtbl $interface $vtbl (
            $($fields)*
            pub $method: unsafe extern "system" fn(
                This: *mut $interface,
                ret: *mut $rtr,
                $($p: $t,)*
            ) -> *mut $rtr,
        ) $($tail)*}
    );

    (@uuid $interface:ident
        $l:expr, $w1:expr, $w2:expr,
        $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr, $b6:expr, $b7:expr, $b8:expr
    ) => (
        impl Interface for $interface {
            #[inline]
            fn uuidof() -> GUID {
                GUID (
                    $l,
                    $w1,
                    $w2,
                    [$b1, $b2, $b3, $b4, $b5, $b6, $b7, $b8],
                )
            }
        }
    );

    (@item $thing:item) => ($thing);
}

trait Interface {
    fn uuidof() -> GUID;
}

RIDL! {#[uuid(0x00000000, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IUnknown(IUnknownVtbl) {
    fn QueryInterface(
        riid: REFIID,
        ppvObject: *mut *mut std::ffi::c_void,
    ) -> HRESULT,
    fn AddRef() -> u32,
    fn Release() -> u32,
}}

RIDL! {#[uuid(0xa95664d2, 0x9614, 0x4f35, 0xa7, 0x46, 0xde, 0x8d, 0xb6, 0x36, 0x17, 0xe6)]
interface IMMDeviceEnumerator(IMMDeviceEnumeratorVtbl): IUnknown(IUnknownVtbl) {
    fn EnumAudioEndpoints(
        dataFlow: u32,
        dwStateMask: DWORD,
        ppDevices: *mut *mut std::ffi::c_void,
    ) -> HRESULT,
    fn GetDefaultAudioEndpoint(
        dataFlow: u32,
        role: u32,
        ppEndpoint: *mut *mut IMMDevice,
    ) -> HRESULT,
    fn GetDevice(
        pwstrId: u16,
        ppDevices: *mut *mut std::ffi::c_void,
    ) -> HRESULT,
    fn RegisterEndpointNotificationCallback(
        pClient: *mut std::ffi::c_void,
    ) -> HRESULT,
    fn UnregisterEndpointNotificationCallback(
        pClient: *mut std::ffi::c_void,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0xd666063f, 0x1587, 0x4e43, 0x81, 0xf1, 0xb9, 0x48, 0xe8, 0x07, 0x36, 0x3f)]
interface IMMDevice(IMMDeviceVtbl): IUnknown(IUnknownVtbl) {
    fn Activate(
        iid: REFIID,
        dwClsCtx: DWORD,
        pActivationParams: *mut std::ffi::c_void,
        ppInterface: *mut LPVOID,
    ) -> HRESULT,
    fn OpenPropertyStore(
        stgmAccess: DWORD,
        ppProperties: *mut *mut std::ffi::c_void,
    ) -> HRESULT,
    fn GetId(
        ppstrId: *mut LPWSTR,
    ) -> HRESULT,
    fn GetState(
        pdwState: *mut DWORD,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0x1cb9ad4c, 0xdbfa, 0x4c32, 0xb1, 0x78, 0xc2, 0xf5, 0x68, 0xa7, 0x03, 0xb2)]
interface IAudioClient(IAudioClientVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        ShareMode: AUDCLNT_SHAREMODE,
        StreamFlags: AUDCLNT_SHAREMODE,
        hnsBufferDuration: REFERENCE_TIME,
        hnsPeriodicity: REFERENCE_TIME,
        pFormat: *const WAVEFORMATEX,
        AudioSessionGuid: LPCGUID,
    ) -> HRESULT,
    fn GetBufferSize(
        pNumBufferFrames: *mut u32,
    ) -> HRESULT,
    fn GetStreamLatency(
        phnsLatency: *mut REFERENCE_TIME,
    ) -> HRESULT,
    fn GetCurrentPadding(
        pNumPaddingFrames: *mut u32,
    ) -> HRESULT,
    fn IsFormatSupported(
        ShareMode: AUDCLNT_SHAREMODE,
        pFormat: *const WAVEFORMATEX,
        ppClosestMatch: *mut *mut WAVEFORMATEX,
    ) -> HRESULT,
    fn GetMixFormat(
        ppDeviceFormat: *mut *mut WAVEFORMATEX,
    ) -> HRESULT,
    fn GetDevicePeriod(
        phnsDefaultDevicePeriod: *mut REFERENCE_TIME,
        phnsMinimumDevicePeriod: *mut REFERENCE_TIME,
    ) -> HRESULT,
    fn Start() -> HRESULT,
    fn Stop() -> HRESULT,
    fn Reset() -> HRESULT,
    fn SetEventHandle(
        eventHandle: *mut std::ffi::c_void,
    ) -> HRESULT,
    fn GetService(
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT,
}}

RIDL! {#[uuid(0xf294acfc, 0x3146, 0x4483, 0xa7, 0xbf, 0xad, 0xdc, 0xa7, 0xc2, 0x60, 0xe2)]
interface IAudioRenderClient(IAudioRenderClientVtbl): IUnknown(IUnknownVtbl) {
    fn GetBuffer(
        NumFramesRequested: u32,
        ppData: *mut *mut u8,
    ) -> HRESULT,
    fn ReleaseBuffer(
        NumFramesWritten: u32,
        dwFlags: DWORD,
    ) -> HRESULT,
}}

#[link(name = "ole32")]
extern "system" {
    fn CoCreateInstance(
        rclsid: REFCLSID,
        pUnkOuter: *mut std::ffi::c_void,
        dwClsContext: DWORD,
        riid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
    fn CoInitializeEx(pvReserved: LPVOID, dwCoInit: DWORD) -> HRESULT;
    pub fn WaitForSingleObject(hHandle: *mut std::ffi::c_void, dwMilliseconds: DWORD) -> DWORD;
    pub fn CreateEventA(
        lpEventAttributes: *mut std::ffi::c_void,
        bManualReset: BOOL,
        bInitialState: BOOL,
        lpName: LPCSTR,
    ) -> *mut std::ffi::c_void;

}

#[repr(C)]
struct GUID(u32, u16, u16, [u8; 8]);

const IMMDeviceEnumerator: GUID = GUID(
    0xa95664d2,
    0x9614,
    0x4f35,
    [0xa7, 0x46, 0xde, 0x8d, 0xb6, 0x36, 0x17, 0xe6],
);

const CLSID_MMDeviceEnumerator: GUID = GUID(
    0xBCDE0395,
    0xE52F,
    0x467C,
    [0x8E, 0x3D, 0xC4, 0x57, 0x92, 0x91, 0x69, 0x2E],
);

const IID_IAudioClient: GUID = GUID(
    0x1CB9AD4C,
    0xDBFA,
    0x4c32,
    [0xB1, 0x78, 0xC2, 0xF5, 0x68, 0xA7, 0x03, 0xB2],
);

const IID_IAudioRenderClient: GUID = GUID(
    0xF294ACFC,
    0x3146,
    0x4483,
    [0xA7, 0xBF, 0xAD, 0xDC, 0xA7, 0xC2, 0x60, 0xE2],
);

/*
  static const IID _saudio_IID_IMMDeviceEnumerator = { 0xa95664d2, 0x9614, 0x4f35, { 0xa7, 0x46, 0xde, 0x8d, 0xb6, 0x36, 0x17, 0xe6 } };
    static const CLSID _saudio_CLSID_IMMDeviceEnumerator = { 0xbcde0395, 0xe52f, 0x467c, { 0x8e, 0x3d, 0xc4, 0x57, 0x92, 0x91, 0x69, 0x2e } };
*/

pub trait AudioSource {
    fn provide_samples(&mut self, samples: &mut [f32]);
    fn handle_event() {}
}

fn check_result(result: HRESULT) -> Result<(), std::io::Error> {
    if result < 0 {
        println!("ERROR: {:?}", result);
        Err(std::io::Error::from_raw_os_error(result))
    } else {
        Ok(())
    }
}

type AudioOutputFormat = f32;

// A backend for Windows Audio Session Application Programming Interface (WASAPI)
// Inspired by sokol_audio.h:
// https://github.com/floooh/sokol/blob/master/sokol_audio.h
// And cpal: https://github.com/RustAudio/cpal/blob/master/src/host/wasapi/device.rs
pub fn begin_audio_thread(
    mut audio_callback: impl FnMut(&mut [AudioOutputFormat], StreamInfo) + Send + 'static,
) {
    unsafe {
        let hresult = CoInitializeEx(std::ptr::null_mut(), 0 /* COINIT_MULTITHREADED */);
        check_result(hresult).unwrap();

        // First find an audio device to play to
        let mut enumerator: *mut IMMDeviceEnumerator =
            std::mem::MaybeUninit::uninit().assume_init();

        let hresult = CoCreateInstance(
            &CLSID_MMDeviceEnumerator,
            std::ptr::null_mut(),
            CLSCTX_ALL,
            &IMMDeviceEnumerator,
            &mut enumerator as *mut *mut IMMDeviceEnumerator as *mut _,
        );

        check_result(hresult).unwrap();

        // Select the default audio device.
        let mut device = std::mem::MaybeUninit::uninit().assume_init();
        let hresult = (*enumerator).GetDefaultAudioEndpoint(
            0, /* eRender */
            0, /* eConsole */
            &mut device,
        );

        check_result(hresult).unwrap();

        // Create an audio client.
        let mut audio_client = std::mem::MaybeUninit::uninit().assume_init();
        let hresult = (*device).Activate(
            &IID_IAudioClient,
            CLSCTX_ALL,
            std::ptr::null_mut(),
            &mut audio_client,
        );
        check_result(hresult).unwrap();

        let audio_client = audio_client as *mut IAudioClient;

        // Setup the streaming format for the audio.
        let wFormatTag = WAVE_FORMAT_IEEE_FLOAT;
        let nChannels = 2;
        let nSamplesPerSec = SAMPLE_RATE as _;
        let wBitsPerSample = 32;

        let nBlockAlign = (nChannels as u32 * wBitsPerSample as u32) / 8; // Size of a sample. Required equation. See below link
        let nAvgBytesPerSec = nSamplesPerSec as u32 * nBlockAlign as u32; // Required equation. See below link

        let cbSize = 0; // Denotes space for extra data appended to the end.

        // https://docs.microsoft.com/en-us/windows/win32/api/mmeapi/ns-mmeapi-waveformatex
        let format = WAVEFORMATEX {
            wFormatTag,
            nChannels,
            nSamplesPerSec,
            nAvgBytesPerSec: nAvgBytesPerSec as u32,
            nBlockAlign: nBlockAlign as u16,
            wBitsPerSample,
            cbSize,
        };

        // https://docs.microsoft.com/en-us/windows/win32/api/audioclient/nf-audioclient-iaudioclient-initialize
        let buffer_frames = 2048; // Number of frames in streaming buffer. Introduces 46 ms of latency.

        let duration = (buffer_frames as f64) / (SAMPLE_RATE as f64 * (1.0 / 10000000.0));

        // AUDCLNT_STREAMFLAGS_RATEADJUST lets us use a sample rate that's different from the hardware and the OS
        // will resample for us.
        // Todo: Probably should add this as well: AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY 0x08000000
        let hresult = (*audio_client).Initialize(
            AUDCLNT_SHAREMODE_SHARED, // ShareMode
            AUDCLNT_STREAMFLAGS_EVENTCALLBACK | AUDCLNT_STREAMFLAGS_RATEADJUST, // StreamFlgs
            duration as i64,          // hnsBufferDuration
            0,                        // hnsPeriodicity
            &format,                  // *pFormat
            std::ptr::null(),         // AudioSessionGuid
        );
        check_result(hresult).unwrap();

        let mut pNumBufferFrames = std::mem::MaybeUninit::uninit().assume_init();
        let hresult = (*audio_client).GetBufferSize(&mut pNumBufferFrames);
        check_result(hresult).unwrap();

        let mut render_client: *mut IAudioRenderClient =
            std::mem::MaybeUninit::uninit().assume_init();

        let hresult = (*audio_client).GetService(
            &IID_IAudioRenderClient,
            &mut render_client as *mut *mut IAudioRenderClient as *mut _,
        );
        check_result(hresult).unwrap();

        // Create the event
        // https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventa
        let event = CreateEventA(
            std::ptr::null_mut(), // lpEventAttributes
            0,                    // bManualReset (BOOL)
            0,                    // bInitialState (BOOL)
            std::ptr::null(),     // lpName
        );

        let hresult = (*audio_client).SetEventHandle(event as *mut std::ffi::c_void);
        check_result(hresult).unwrap();

        // This wrapper is a workaround because Rust designates raw pointers as unsafe to send
        // 'unsafe impl Send' is implemented for ThreadData
        let thread_data = ThreadData {
            event,
            audio_client,
            render_client,
        };

        // Run forever requesting audio
        std::thread::spawn(move || loop {
            let thread_data = thread_data;
            loop {
                let _result = WaitForSingleObject(thread_data.event, INFINITE);

                let mut padding = std::mem::MaybeUninit::uninit().assume_init();
                let hresult = (*thread_data.audio_client).GetCurrentPadding(&mut padding);
                check_result(hresult).unwrap();

                let frames_to_write = pNumBufferFrames - padding;

                let mut buffer: *mut u8 = std::mem::MaybeUninit::uninit().assume_init();
                let hresult = (*thread_data.render_client)
                    .GetBuffer(frames_to_write, &mut buffer as *mut *mut _);
                check_result(hresult).unwrap();

                let buffer_len = frames_to_write as usize * nChannels as usize;
                let samples_slice: &mut [f32] =
                    std::slice::from_raw_parts_mut(buffer as *mut f32, buffer_len);
                samples_slice.fill(0.0);

                let stream_info = StreamInfo {
                    channels: 2,
                    sample_rate: SAMPLE_RATE as _,
                };

                (audio_callback)(samples_slice, stream_info);
                let hresult = (*thread_data.render_client).ReleaseBuffer(frames_to_write, 0);
                check_result(hresult).unwrap();
            }
        });

        let hresult = (*audio_client).Start();
        check_result(hresult).unwrap();

        // These should be stored in some sort of structure and released when the structure is dropped.
        // (*enumerator).Release();
        // (*device).Release();
        // (*render_client).Release();
    }
}

struct ThreadData {
    event: *mut std::ffi::c_void,
    audio_client: *mut IAudioClient,
    render_client: *mut IAudioRenderClient,
}

unsafe impl Send for ThreadData {}
