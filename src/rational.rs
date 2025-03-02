use libavcodec_sys as sys;

#[derive(Debug, Clone, Copy)]
pub struct Rational {
    inner: sys::AVRational,
}

impl Rational {
    pub fn new(num: i32, den: i32) -> Self {
        Rational {
            inner: sys::AVRational { num, den },
        }
    }

    // pub fn as_f64(&self) -> f64 {
    //     unsafe { sys::av_q2d(self.inner) }
    // }

    pub fn num(&self) -> i32 {
        self.inner.num
    }

    pub fn den(&self) -> i32 {
        self.inner.den
    }
}

impl From<sys::AVRational> for Rational {
    fn from(r: sys::AVRational) -> Self {
        Rational { inner: r }
    }
}

impl Into<sys::AVRational> for Rational {
    fn into(self) -> sys::AVRational {
        self.inner
    }
} 
