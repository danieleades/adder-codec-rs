use crate::framer::driver::SourceType;
use crate::transcoder::source::video::FramedViewMode;
use crate::{DeltaT, EventCoordless, Intensity, D_SHIFT};
use adder_codec_core::Event;

pub trait FrameValue {
    type Output;
    fn get_frame_value(
        event: &Event,
        source_type: SourceType,
        i1: DeltaT,
        practical_d_max: f32,
        delta_t_max: DeltaT,
        view_mode: FramedViewMode,
    ) -> Self::Output;

    fn max_f32() -> f32;
}

impl FrameValue for EventCoordless {
    type Output = EventCoordless;
    fn get_frame_value(
        event: &Event,
        _source_type: SourceType,
        _tpf: DeltaT,
        _practical_d_max: f32,
        _delta_t_max: DeltaT,
        _view_mode: FramedViewMode,
    ) -> Self::Output {
        EventCoordless {
            d: event.d,
            delta_t: event.delta_t,
        }
    }

    fn max_f32() -> f32 {
        1.0
    }
}

impl FrameValue for u8 {
    type Output = u8;

    fn get_frame_value(
        event: &Event,
        source_type: SourceType,
        tpf: DeltaT,
        practical_d_max: f32,
        delta_t_max: DeltaT,
        view_mode: FramedViewMode,
    ) -> Self::Output {
        match view_mode {
            FramedViewMode::Intensity => {
                let intensity = event_to_intensity(event);
                match source_type {
                    SourceType::U8 => (intensity * f64::from(tpf)) as u8,
                    SourceType::U16 => {
                        (intensity / f64::from(u16::MAX) * f64::from(tpf) * f64::from(u8::MAX))
                            as u8
                    }
                    SourceType::U32 => {
                        (intensity / f64::from(u32::MAX) * f64::from(tpf) * f64::from(u8::MAX))
                            as u8
                    }
                    SourceType::U64 => {
                        (intensity / u64::MAX as f64 * f64::from(tpf) * f64::from(u8::MAX)) as u8
                    }
                    SourceType::F32 => {
                        todo!()
                    }
                    SourceType::F64 => {
                        todo!()
                    }
                }
            }
            FramedViewMode::D => {
                ((f32::from(event.d) / practical_d_max) * f32::from(u8::MAX)) as u8
            }
            FramedViewMode::DeltaT => {
                ((event.delta_t as f32 / delta_t_max as f32) * f32::from(u8::MAX)) as u8
            }
        }
    }

    fn max_f32() -> f32 {
        f32::from(u8::MAX)
    }
}

impl FrameValue for u16 {
    type Output = u16;

    fn get_frame_value(
        event: &Event,
        source_type: SourceType,
        tpf: DeltaT,
        practical_d_max: f32,
        delta_t_max: DeltaT,
        view_mode: FramedViewMode,
    ) -> Self::Output {
        match view_mode {
            FramedViewMode::Intensity => {
                let intensity = event_to_intensity(event);
                match source_type {
                    SourceType::U8 => {
                        (intensity / f64::from(u8::MAX) * f64::from(tpf) * f64::from(u16::MAX))
                            as u16
                    }
                    SourceType::U16 => (intensity * f64::from(tpf)) as u16,
                    SourceType::U32 => {
                        (intensity / f64::from(u32::MAX) * f64::from(tpf) * f64::from(u16::MAX))
                            as u16
                    }
                    SourceType::U64 => {
                        (intensity / u64::MAX as f64 * f64::from(tpf) * f64::from(u16::MAX)) as u16
                    }
                    SourceType::F32 => {
                        todo!()
                    }
                    SourceType::F64 => {
                        todo!()
                    }
                }
            }
            FramedViewMode::D => {
                ((f32::from(event.d) / practical_d_max) * f32::from(u16::MAX)) as u16
            }
            FramedViewMode::DeltaT => {
                ((event.delta_t as f32 / delta_t_max as f32) * f32::from(u16::MAX)) as u16
            }
        }
    }

    fn max_f32() -> f32 {
        f32::from(u16::MAX)
    }
}

impl FrameValue for u32 {
    type Output = u32;

    fn get_frame_value(
        event: &Event,
        source_type: SourceType,
        tpf: DeltaT,
        practical_d_max: f32,
        delta_t_max: DeltaT,
        view_mode: FramedViewMode,
    ) -> Self::Output {
        match view_mode {
            FramedViewMode::Intensity => {
                let intensity = event_to_intensity(event);
                match source_type {
                    SourceType::U8 => {
                        (intensity / f64::from(u8::MAX) * f64::from(tpf) * f64::from(u32::MAX))
                            as u32
                    }
                    SourceType::U16 => {
                        (intensity / f64::from(u16::MAX) * f64::from(tpf) * f64::from(u32::MAX))
                            as u32
                    }
                    SourceType::U32 => (intensity * f64::from(tpf)) as u32,
                    SourceType::U64 => {
                        (intensity / u64::MAX as f64 * f64::from(tpf) * f64::from(u32::MAX)) as u32
                    }
                    SourceType::F32 => {
                        todo!()
                    }
                    SourceType::F64 => {
                        todo!()
                    }
                }
            }
            FramedViewMode::D => ((f32::from(event.d) / practical_d_max) * u32::MAX as f32) as u32,
            FramedViewMode::DeltaT => {
                ((event.delta_t as f32 / delta_t_max as f32) * u32::MAX as f32) as u32
            }
        }
    }

    fn max_f32() -> f32 {
        u32::MAX as f32
    }
}

impl FrameValue for u64 {
    type Output = u64;

    fn get_frame_value(
        event: &Event,
        source_type: SourceType,
        tpf: DeltaT,
        practical_d_max: f32,
        delta_t_max: DeltaT,
        view_mode: FramedViewMode,
    ) -> Self::Output {
        match view_mode {
            FramedViewMode::Intensity => {
                let intensity = event_to_intensity(event);
                match source_type {
                    SourceType::U8 => {
                        (intensity / f64::from(u8::MAX) * f64::from(tpf) * u64::MAX as f64) as u64
                    }
                    SourceType::U16 => {
                        (intensity / f64::from(u16::MAX) * f64::from(tpf) * u64::MAX as f64) as u64
                    }
                    SourceType::U32 => {
                        (intensity / f64::from(u32::MAX) * f64::from(tpf) * u64::MAX as f64) as u64
                    }
                    SourceType::U64 => (intensity * f64::from(tpf)) as u64,
                    SourceType::F32 => {
                        todo!()
                    }
                    SourceType::F64 => {
                        todo!()
                    }
                }
            }
            FramedViewMode::D => ((f32::from(event.d) / practical_d_max) * u64::MAX as f32) as u64,
            FramedViewMode::DeltaT => {
                ((event.delta_t as f32 / delta_t_max as f32) * u64::MAX as f32) as u64
            }
        }
    }

    fn max_f32() -> f32 {
        u64::MAX as f32
    }
}

#[must_use]
pub fn event_to_intensity(event: &Event) -> Intensity {
    match event.d as usize {
        a if a >= D_SHIFT.len() => f64::from(0),
        _ => match event.delta_t {
            0 => D_SHIFT[event.d as usize] as Intensity, // treat it as dt = 1
            _ => D_SHIFT[event.d as usize] as Intensity / f64::from(event.delta_t),
        },
    }
}

fn _eventcoordless_to_intensity(event: EventCoordless) -> Intensity {
    match event.d as usize {
        a if a >= D_SHIFT.len() => f64::from(0),
        _ => D_SHIFT[event.d as usize] as Intensity / f64::from(event.delta_t),
    }
}
