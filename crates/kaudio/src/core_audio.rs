use std::ffi::c_void;

use crate::StreamInfo;
#[link(name = "AudioToolbox", kind = "framework")]
extern "C" {}

type FourCharCode = u32;
type OSType = FourCharCode;

const kAudioUnitType_Output: u32 = 1635086197;
const kAudioUnitSubType_HALOutput: u32 = 1634230636;
const kAudioUnitManufacturer_Apple: u32 = 1634758764;
const kAudioUnitScope_Input: u32 = 1;
const kAudioUnitScope_Output: u32 = 2;
const kAudioUnitProperty_StreamFormat: u32 = 8;
const kAudioFormatLinearPCM: u32 = 1819304813;
const kAudioFormatFlagIsFloat: u32 = 1 << 0;
const kAudioFormatFlagIsPacked: u32 = 8;
// const kAudioDevicePropertyBufferFrameSize: u32 = 1718839674;
// const kAudioUnitScope_Global: u32 = 0;
const kAudioUnitProperty_SetRenderCallback: u32 = 23;
const kAudioFormatFlagsNativeEndian: u32 = 0;
const kAudioFormatFlagsNativeFloatPacked: u32 =
    kAudioFormatFlagIsFloat | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked;
const kOutputBus: u32 = 0;

type OSStatus = i32;
#[repr(C)]
struct OpaqueAudioComponent {
    _unused: [u8; 0],
}

#[repr(C)]
struct ComponentInstanceRecord {
    _unused: [u8; 0],
}

type AudioComponent = *mut OpaqueAudioComponent;
type AudioComponentInstance = *mut ComponentInstanceRecord;
type AudioUnit = AudioComponentInstance;
type AudioUnitPropertyID = u32;
type AudioUnitScope = u32;
type AudioUnitElement = u32;

#[repr(C)]
struct AudioComponentDescription {
    pub componentType: OSType,
    pub componentSubType: OSType,
    pub componentManufacturer: OSType,
    pub componentFlags: u32,
    pub componentFlagsMask: u32,
}

extern "C" {
    fn AudioComponentFindNext(
        inComponent: AudioComponent,
        inDesc: *const AudioComponentDescription,
    ) -> AudioComponent;
    fn AudioComponentInstanceNew(
        inComponent: AudioComponent,
        outInstance: *mut AudioComponentInstance,
    ) -> OSStatus;
    fn AudioUnitInitialize(inUnit: AudioUnit) -> OSStatus;
    fn AudioUnitGetProperty(
        inUnit: AudioUnit,
        inID: AudioUnitPropertyID,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
        outData: *mut ::std::os::raw::c_void,
        ioDataSize: *mut u32,
    ) -> OSStatus;

    fn AudioUnitSetProperty(
        inUnit: AudioUnit,
        inID: AudioUnitPropertyID,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
        inData: *const ::std::os::raw::c_void,
        inDataSize: u32,
    ) -> OSStatus;
    fn AudioOutputUnitStart(ci: AudioUnit) -> OSStatus;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct AURenderCallbackStruct {
    pub inputProc: AURenderCallback,
    pub inputProcRefCon: *mut ::std::os::raw::c_void,
}

type AURenderCallback = unsafe extern "C" fn(
    inRefCon: *mut ::std::os::raw::c_void,
    ioActionFlags: *mut AudioUnitRenderActionFlags,
    inTimeStamp: *const AudioTimeStamp,
    inBusNumber: u32,
    inNumberFrames: u32,
    ioData: *mut AudioBufferList,
) -> OSStatus;

#[repr(C)]
struct AudioBufferList {
    pub mNumberBuffers: u32,
    pub mBuffers: [AudioBuffer; 1usize],
}

#[derive(Debug)]
#[repr(C)]
struct AudioBuffer {
    pub mNumberChannels: u32,
    pub mDataByteSize: u32,
    pub mData: *mut ::std::os::raw::c_void,
}

#[repr(C)]
struct AudioTimeStamp {
    pub mSampleTime: f64,
    pub mHostTime: u64,
    pub mRateScalar: f64,
    pub mWordClockTime: u64,
    pub mSMPTETime: u32,
    pub mFlags: u32,
    pub mReserved: u32,
}

type AudioUnitRenderActionFlags = u32;
type AudioFormatID = u32;
type AudioFormatFlags = u32;

/// Official documentation:
/// https://developer.apple.com/documentation/coreaudiotypes/audiostreambasicdescription
#[repr(C)]
#[derive(Default, Debug)]
struct AudioStreamBasicDescription {
    pub mSampleRate: f64,
    pub mFormatID: AudioFormatID,
    pub mFormatFlags: AudioFormatFlags,
    pub mBytesPerPacket: u32,
    pub mFramesPerPacket: u32,
    pub mBytesPerFrame: u32,
    pub mChannelsPerFrame: u32,
    pub mBitsPerChannel: u32,
    pub mReserved: u32,
}

type AudioOutputFormat = f32;
const SAMPLE_RATE: u32 = 44100;
pub fn begin_audio_thread(
    audio_callback: impl FnMut(&mut [AudioOutputFormat], StreamInfo) + Send + 'static,
) {
    // Relevant Apple example documentation here:
    // https://developer.apple.com/library/archive/technotes/tn2091/_index.html

    let audio_component_description = AudioComponentDescription {
        componentType: kAudioUnitType_Output,
        componentSubType: kAudioUnitSubType_HALOutput,
        componentManufacturer: kAudioUnitManufacturer_Apple,
        componentFlags: 0,
        componentFlagsMask: 0,
    };

    unsafe {
        // Can this be used to enumerate available audio devices? Or is that a separate thing?
        let component = AudioComponentFindNext(std::ptr::null_mut(), &audio_component_description);
        if component == std::ptr::null_mut() {
            panic!("Could not find audio device");
        }

        let mut audio_unit: AudioComponentInstance = std::ptr::null_mut();
        let result =
            AudioComponentInstanceNew(component, &mut audio_unit as *mut AudioComponentInstance);

        if result != 0 {
            panic!("ERROR CREATING AUDIO COMPONENT");
        }

        let mut stream_description = AudioStreamBasicDescription {
            ..Default::default()
        };
        let mut size: u32 = std::mem::size_of::<AudioStreamBasicDescription>() as u32;
        let result = AudioUnitGetProperty(
            audio_unit,
            kAudioUnitProperty_StreamFormat,
            kAudioUnitScope_Output,
            kOutputBus,
            &mut stream_description as *mut AudioStreamBasicDescription as *mut c_void,
            &mut size,
        );

        if result != 0 {
            panic!("ERROR CREATING AUDIO COMPONENT");
        }

        //println!("Stream Description: {:?}", stream_description);

        let channels = 2;

        let bytes_per_frame = channels * std::mem::size_of::<AudioOutputFormat>() as u32;
        let frames_per_packet = 1;
        // Now initialize the stream with the formats we want.
        let mut stream_description = AudioStreamBasicDescription {
            mFormatID: kAudioFormatLinearPCM,
            mFormatFlags: kAudioFormatFlagsNativeFloatPacked,
            mChannelsPerFrame: channels, // This should be adjustable later
            mSampleRate: SAMPLE_RATE as f64,
            mFramesPerPacket: frames_per_packet,
            mBitsPerChannel: (std::mem::size_of::<AudioOutputFormat>() * 8) as u32, // Mac's output is f32, iOS's is i16. So this incurs some overhead on iOS.
            mBytesPerFrame: bytes_per_frame,
            mBytesPerPacket: bytes_per_frame * frames_per_packet,
            mReserved: 0,
        };

        let result = AudioUnitSetProperty(
            audio_unit,
            kAudioUnitProperty_StreamFormat,
            kAudioUnitScope_Input,
            kOutputBus,
            &mut stream_description as *mut AudioStreamBasicDescription as *mut c_void,
            size,
        );

        if result != 0 {
            panic!("ERROR CREATING AUDIO COMPONENT");
        }

        /*
        let frame_size = 512; // This should be adjustable
        let result = AudioUnitSetProperty(
            audio_unit,
            kAudioDevicePropertyBufferFrameSize,
            kAudioUnitScope_Global, // Global means that this setting applies to the entire audio unit.
            kOutputBus,
            &frame_size as *const i32 as *const c_void,
            std::mem::size_of::<u32>() as u32,
        );

        if result != 0 {
            panic!("ERROR CREATING AUDIO COMPONENT");
        }
        */

        // audio_source.initialize(frame_size as usize);

        let callback = AURenderCallbackStruct {
            inputProc: callback,
            inputProcRefCon: Box::into_raw(Box::new(CallbackWrapper {
                audio_source: Box::new(audio_callback),
            })) as *mut c_void,
        };

        let result = AudioUnitSetProperty(
            audio_unit,
            kAudioUnitProperty_SetRenderCallback,
            kAudioUnitScope_Input,
            kOutputBus,
            &callback as *const AURenderCallbackStruct as *const c_void,
            std::mem::size_of::<AURenderCallbackStruct>() as u32,
        );

        if result != 0 {
            panic!("ERROR CREATING AUDIO COMPONENT");
        }

        let result = AudioUnitInitialize(audio_unit);

        if result != 0 {
            panic!("ERROR CREATING AUDIO COMPONENT");
        }

        let result = AudioOutputUnitStart(audio_unit);

        if result != 0 {
            panic!("ERROR CREATING AUDIO COMPONENT");
        }
    }
}

unsafe extern "C" fn callback(
    in_ref_con: *mut ::std::os::raw::c_void,
    _io_action_flags: *mut AudioUnitRenderActionFlags,
    _in_time_stamp: *const AudioTimeStamp,
    _in_bus_number: u32,
    _in_number_frames: u32,
    io_data: *mut AudioBufferList,
) -> i32 {
    let callback_wrapper: *mut CallbackWrapper = in_ref_con as *mut CallbackWrapper;
    let data: &mut [AudioOutputFormat] = std::slice::from_raw_parts_mut(
        (*io_data).mBuffers[0].mData as *mut AudioOutputFormat,
        ((*io_data).mBuffers[0].mDataByteSize / std::mem::size_of::<AudioOutputFormat>() as u32)
            as usize,
    );

    // The data does not necessarily have to be zeroed if the user callback will zero it,
    // but uninitialized memory can be quite painful on the ears if allowed to slip through.
    for b in data.iter_mut() {
        *b = 0.;
    }

    let channels = (*io_data).mBuffers[0].mNumberChannels;
    let stream_info = StreamInfo {
        channels,
        sample_rate: 44100,
    };

    // Call user callback.
    ((*callback_wrapper).audio_source)(data, stream_info);

    return 0;
}

struct CallbackWrapper {
    audio_source: Box<dyn FnMut(&mut [AudioOutputFormat], StreamInfo) + Send + 'static>,
}
