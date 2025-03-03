use libavcodec_sys::*;
use num_derive::FromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AVAudioServiceType {
    Commentary = AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_COMMENTARY,
    Dialogue = AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_DIALOGUE,
    Effects = AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_EFFECTS,
    Emergency = AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_EMERGENCY,
    HearingImpaired = AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED,
    Karaoke = AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_KARAOKE,
    Main = AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_MAIN,
    Nb = AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_NB,
    VisuallyImpaired = AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED,
    VoiceOver = AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_VOICE_OVER,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AVChannelOrder {
    Ambisonic = AVChannelOrder_AV_CHANNEL_ORDER_AMBISONIC,
    Custom = AVChannelOrder_AV_CHANNEL_ORDER_CUSTOM,
    Native = AVChannelOrder_AV_CHANNEL_ORDER_NATIVE,
    Unspecified = AVChannelOrder_AV_CHANNEL_ORDER_UNSPEC,
    Nb = AVChannelOrder_FF_CHANNEL_ORDER_NB,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AVChannel {
    AmbisonicBase = AVChannel_AV_CHAN_AMBISONIC_BASE,
    AmbisonicEnd = AVChannel_AV_CHAN_AMBISONIC_END,
    BackCenter = AVChannel_AV_CHAN_BACK_CENTER,
    BackLeft = AVChannel_AV_CHAN_BACK_LEFT,
    BackRight = AVChannel_AV_CHAN_BACK_RIGHT,
    BottomFrontCenter = AVChannel_AV_CHAN_BOTTOM_FRONT_CENTER,
    BottomFrontLeft = AVChannel_AV_CHAN_BOTTOM_FRONT_LEFT,
    BottomFrontRight = AVChannel_AV_CHAN_BOTTOM_FRONT_RIGHT,
    FrontCenter = AVChannel_AV_CHAN_FRONT_CENTER,
    FrontLeft = AVChannel_AV_CHAN_FRONT_LEFT,
    FrontLeftOfCenter = AVChannel_AV_CHAN_FRONT_LEFT_OF_CENTER,
    FrontRight = AVChannel_AV_CHAN_FRONT_RIGHT,
    FrontRightOfCenter = AVChannel_AV_CHAN_FRONT_RIGHT_OF_CENTER,
    LowFrequency = AVChannel_AV_CHAN_LOW_FREQUENCY,
    LowFrequency2 = AVChannel_AV_CHAN_LOW_FREQUENCY_2,
    None = AVChannel_AV_CHAN_NONE,
    SideLeft = AVChannel_AV_CHAN_SIDE_LEFT,
    SideRight = AVChannel_AV_CHAN_SIDE_RIGHT,
    SideSurroundLeft = AVChannel_AV_CHAN_SIDE_SURROUND_LEFT,
    SideSurroundRight = AVChannel_AV_CHAN_SIDE_SURROUND_RIGHT,
    StereoLeft = AVChannel_AV_CHAN_STEREO_LEFT,
    StereoRight = AVChannel_AV_CHAN_STEREO_RIGHT,
    SurroundDirectLeft = AVChannel_AV_CHAN_SURROUND_DIRECT_LEFT,
    SurroundDirectRight = AVChannel_AV_CHAN_SURROUND_DIRECT_RIGHT,
    TopBackCenter = AVChannel_AV_CHAN_TOP_BACK_CENTER,
    TopBackLeft = AVChannel_AV_CHAN_TOP_BACK_LEFT,
    TopBackRight = AVChannel_AV_CHAN_TOP_BACK_RIGHT,
    TopCenter = AVChannel_AV_CHAN_TOP_CENTER,
    TopFrontCenter = AVChannel_AV_CHAN_TOP_FRONT_CENTER,
    TopFrontLeft = AVChannel_AV_CHAN_TOP_FRONT_LEFT,
    TopFrontRight = AVChannel_AV_CHAN_TOP_FRONT_RIGHT,
    TopSideLeft = AVChannel_AV_CHAN_TOP_SIDE_LEFT,
    TopSideRight = AVChannel_AV_CHAN_TOP_SIDE_RIGHT,
    TopSurroundLeft = AVChannel_AV_CHAN_TOP_SURROUND_LEFT,
    TopSurroundRight = AVChannel_AV_CHAN_TOP_SURROUND_RIGHT,
    Unknown = AVChannel_AV_CHAN_UNKNOWN,
    Unused = AVChannel_AV_CHAN_UNUSED,
    WideLeft = AVChannel_AV_CHAN_WIDE_LEFT,
    WideRight = AVChannel_AV_CHAN_WIDE_RIGHT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AVChromaLocation {
    Bottom = AVChromaLocation_AVCHROMA_LOC_BOTTOM,
    BottomLeft = AVChromaLocation_AVCHROMA_LOC_BOTTOMLEFT,
    Center = AVChromaLocation_AVCHROMA_LOC_CENTER,
    Left = AVChromaLocation_AVCHROMA_LOC_LEFT,
    Nb = AVChromaLocation_AVCHROMA_LOC_NB,
    Top = AVChromaLocation_AVCHROMA_LOC_TOP,
    TopLeft = AVChromaLocation_AVCHROMA_LOC_TOPLEFT,
    Unspecified = AVChromaLocation_AVCHROMA_LOC_UNSPECIFIED,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AVClassCategory {
    BitstreamFilter = AVClassCategory_AV_CLASS_CATEGORY_BITSTREAM_FILTER,
    Decoder = AVClassCategory_AV_CLASS_CATEGORY_DECODER,
    Demuxer = AVClassCategory_AV_CLASS_CATEGORY_DEMUXER,
    DeviceAudioInput = AVClassCategory_AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT,
    DeviceAudioOutput = AVClassCategory_AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT,
    DeviceInput = AVClassCategory_AV_CLASS_CATEGORY_DEVICE_INPUT,
    DeviceOutput = AVClassCategory_AV_CLASS_CATEGORY_DEVICE_OUTPUT,
    DeviceVideoInput = AVClassCategory_AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT,
    DeviceVideoOutput = AVClassCategory_AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT,
    Encoder = AVClassCategory_AV_CLASS_CATEGORY_ENCODER,
    Filter = AVClassCategory_AV_CLASS_CATEGORY_FILTER,
    Input = AVClassCategory_AV_CLASS_CATEGORY_INPUT,
    Muxer = AVClassCategory_AV_CLASS_CATEGORY_MUXER,
    Na = AVClassCategory_AV_CLASS_CATEGORY_NA,
    Nb = AVClassCategory_AV_CLASS_CATEGORY_NB,
    Output = AVClassCategory_AV_CLASS_CATEGORY_OUTPUT,
    Swresampler = AVClassCategory_AV_CLASS_CATEGORY_SWRESAMPLER,
    Swscaler = AVClassCategory_AV_CLASS_CATEGORY_SWSCALER,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum AVCodecId {
    _012v = AVCodecID_AV_CODEC_ID_012V,
    _4Gv = AVCodecID_AV_CODEC_ID_4GV,
    _4Xm = AVCodecID_AV_CODEC_ID_4XM,
    _8Bps = AVCodecID_AV_CODEC_ID_8BPS,
    _8SvxExp = AVCodecID_AV_CODEC_ID_8SVX_EXP,
    _8SvxFib = AVCodecID_AV_CODEC_ID_8SVX_FIB,
    A64Multi = AVCodecID_AV_CODEC_ID_A64_MULTI,
    A64Multi5 = AVCodecID_AV_CODEC_ID_A64_MULTI5,
    Aac = AVCodecID_AV_CODEC_ID_AAC,
    AacLatm = AVCodecID_AV_CODEC_ID_AAC_LATM,
    Aasc = AVCodecID_AV_CODEC_ID_AASC,
    Ac3 = AVCodecID_AV_CODEC_ID_AC3,
    Ac4 = AVCodecID_AV_CODEC_ID_AC4,
    AcelpKelvin = AVCodecID_AV_CODEC_ID_ACELP_KELVIN,
    Adpcm4xm = AVCodecID_AV_CODEC_ID_ADPCM_4XM,
    AdpcmAdx = AVCodecID_AV_CODEC_ID_ADPCM_ADX,
    AdpcmAfc = AVCodecID_AV_CODEC_ID_ADPCM_AFC,
    AdpcmAgm = AVCodecID_AV_CODEC_ID_ADPCM_AGM,
    AdpcmAica = AVCodecID_AV_CODEC_ID_ADPCM_AICA,
    AdpcmArgo = AVCodecID_AV_CODEC_ID_ADPCM_ARGO,
    AdpcmCt = AVCodecID_AV_CODEC_ID_ADPCM_CT,
    AdpcmDtk = AVCodecID_AV_CODEC_ID_ADPCM_DTK,
    AdpcmEa = AVCodecID_AV_CODEC_ID_ADPCM_EA,
    AdpcmEaMaxisXa = AVCodecID_AV_CODEC_ID_ADPCM_EA_MAXIS_XA,
    AdpcmEaR1 = AVCodecID_AV_CODEC_ID_ADPCM_EA_R1,
    AdpcmEaR2 = AVCodecID_AV_CODEC_ID_ADPCM_EA_R2,
    AdpcmEaR3 = AVCodecID_AV_CODEC_ID_ADPCM_EA_R3,
    AdpcmEaXas = AVCodecID_AV_CODEC_ID_ADPCM_EA_XAS,
    AdpcmG722 = AVCodecID_AV_CODEC_ID_ADPCM_G722,
    AdpcmG726 = AVCodecID_AV_CODEC_ID_ADPCM_G726,
    AdpcmG726le = AVCodecID_AV_CODEC_ID_ADPCM_G726LE,
    AdpcmImaAcorn = AVCodecID_AV_CODEC_ID_ADPCM_IMA_ACORN,
    AdpcmImaAlp = AVCodecID_AV_CODEC_ID_ADPCM_IMA_ALP,
    AdpcmImaAmv = AVCodecID_AV_CODEC_ID_ADPCM_IMA_AMV,
    AdpcmImaApc = AVCodecID_AV_CODEC_ID_ADPCM_IMA_APC,
    AdpcmImaApm = AVCodecID_AV_CODEC_ID_ADPCM_IMA_APM,
    AdpcmImaCunning = AVCodecID_AV_CODEC_ID_ADPCM_IMA_CUNNING,
    AdpcmImaDat4 = AVCodecID_AV_CODEC_ID_ADPCM_IMA_DAT4,
    AdpcmImaDk3 = AVCodecID_AV_CODEC_ID_ADPCM_IMA_DK3,
    AdpcmImaDk4 = AVCodecID_AV_CODEC_ID_ADPCM_IMA_DK4,
    AdpcmImaEaEacs = AVCodecID_AV_CODEC_ID_ADPCM_IMA_EA_EACS,
    AdpcmImaEaSead = AVCodecID_AV_CODEC_ID_ADPCM_IMA_EA_SEAD,
    AdpcmImaIss = AVCodecID_AV_CODEC_ID_ADPCM_IMA_ISS,
    AdpcmImaMoflex = AVCodecID_AV_CODEC_ID_ADPCM_IMA_MOFLEX,
    AdpcmImaMtf = AVCodecID_AV_CODEC_ID_ADPCM_IMA_MTF,
    AdpcmImaOki = AVCodecID_AV_CODEC_ID_ADPCM_IMA_OKI,
    AdpcmImaQt = AVCodecID_AV_CODEC_ID_ADPCM_IMA_QT,
    AdpcmImaRad = AVCodecID_AV_CODEC_ID_ADPCM_IMA_RAD,
    AdpcmImaSmjpeg = AVCodecID_AV_CODEC_ID_ADPCM_IMA_SMJPEG,
    AdpcmImaSsi = AVCodecID_AV_CODEC_ID_ADPCM_IMA_SSI,
    AdpcmImaWav = AVCodecID_AV_CODEC_ID_ADPCM_IMA_WAV,
    AdpcmImaWs = AVCodecID_AV_CODEC_ID_ADPCM_IMA_WS,
    AdpcmMs = AVCodecID_AV_CODEC_ID_ADPCM_MS,
    AdpcmMtaf = AVCodecID_AV_CODEC_ID_ADPCM_MTAF,
    AdpcmPsx = AVCodecID_AV_CODEC_ID_ADPCM_PSX,
    AdpcmSbpro2 = AVCodecID_AV_CODEC_ID_ADPCM_SBPRO_2,
    AdpcmSbpro3 = AVCodecID_AV_CODEC_ID_ADPCM_SBPRO_3,
    AdpcmSbpro4 = AVCodecID_AV_CODEC_ID_ADPCM_SBPRO_4,
    AdpcmSwf = AVCodecID_AV_CODEC_ID_ADPCM_SWF,
    AdpcmThp = AVCodecID_AV_CODEC_ID_ADPCM_THP,
    AdpcmThpLe = AVCodecID_AV_CODEC_ID_ADPCM_THP_LE,
    AdpcmVima = AVCodecID_AV_CODEC_ID_ADPCM_VIMA,
    AdpcmXa = AVCodecID_AV_CODEC_ID_ADPCM_XA,
    AdpcmXmd = AVCodecID_AV_CODEC_ID_ADPCM_XMD,
    AdpcmYamaha = AVCodecID_AV_CODEC_ID_ADPCM_YAMAHA,
    AdpcmZork = AVCodecID_AV_CODEC_ID_ADPCM_ZORK,
    Agm = AVCodecID_AV_CODEC_ID_AGM,
    Aic = AVCodecID_AV_CODEC_ID_AIC,
    Alac = AVCodecID_AV_CODEC_ID_ALAC,
    AliasPix = AVCodecID_AV_CODEC_ID_ALIAS_PIX,
    AmrNb = AVCodecID_AV_CODEC_ID_AMR_NB,
    AmrWb = AVCodecID_AV_CODEC_ID_AMR_WB,
    Amv = AVCodecID_AV_CODEC_ID_AMV,
    Anm = AVCodecID_AV_CODEC_ID_ANM,
    Ansi = AVCodecID_AV_CODEC_ID_ANSI,
    Anull = AVCodecID_AV_CODEC_ID_ANULL,
    Apac = AVCodecID_AV_CODEC_ID_APAC,
    Ape = AVCodecID_AV_CODEC_ID_APE,
    Apng = AVCodecID_AV_CODEC_ID_APNG,
    Aptx = AVCodecID_AV_CODEC_ID_APTX,
    AptxHd = AVCodecID_AV_CODEC_ID_APTX_HD,
    Arbc = AVCodecID_AV_CODEC_ID_ARBC,
    Argo = AVCodecID_AV_CODEC_ID_ARGO,
    AribCaption = AVCodecID_AV_CODEC_ID_ARIB_CAPTION,
    Ass = AVCodecID_AV_CODEC_ID_ASS,
    Asv1 = AVCodecID_AV_CODEC_ID_ASV1,
    Asv2 = AVCodecID_AV_CODEC_ID_ASV2,
    Atrac1 = AVCodecID_AV_CODEC_ID_ATRAC1,
    Atrac3 = AVCodecID_AV_CODEC_ID_ATRAC3,
    Atrac9 = AVCodecID_AV_CODEC_ID_ATRAC9,
    Atrac3al = AVCodecID_AV_CODEC_ID_ATRAC3AL,
    Atrac3p = AVCodecID_AV_CODEC_ID_ATRAC3P,
    Atrac3pal = AVCodecID_AV_CODEC_ID_ATRAC3PAL,
    Aura = AVCodecID_AV_CODEC_ID_AURA,
    Aura2 = AVCodecID_AV_CODEC_ID_AURA2,
    Av1 = AVCodecID_AV_CODEC_ID_AV1,
    Avrn = AVCodecID_AV_CODEC_ID_AVRN,
    Avrp = AVCodecID_AV_CODEC_ID_AVRP,
    Avs = AVCodecID_AV_CODEC_ID_AVS,
    Avs2 = AVCodecID_AV_CODEC_ID_AVS2,
    Avs3 = AVCodecID_AV_CODEC_ID_AVS3,
    Avui = AVCodecID_AV_CODEC_ID_AVUI,
    Bethsoftvid = AVCodecID_AV_CODEC_ID_BETHSOFTVID,
    Bfi = AVCodecID_AV_CODEC_ID_BFI,
    BinkaudioDct = AVCodecID_AV_CODEC_ID_BINKAUDIO_DCT,
    BinkaudioRdft = AVCodecID_AV_CODEC_ID_BINKAUDIO_RDFT,
    Binkvideo = AVCodecID_AV_CODEC_ID_BINKVIDEO,
    Bintext = AVCodecID_AV_CODEC_ID_BINTEXT,
    BinData = AVCodecID_AV_CODEC_ID_BIN_DATA,
    Bitpacked = AVCodecID_AV_CODEC_ID_BITPACKED,
    Bmp = AVCodecID_AV_CODEC_ID_BMP,
    BmvAudio = AVCodecID_AV_CODEC_ID_BMV_AUDIO,
    BmvVideo = AVCodecID_AV_CODEC_ID_BMV_VIDEO,
    Bonk = AVCodecID_AV_CODEC_ID_BONK,
    BrenderPix = AVCodecID_AV_CODEC_ID_BRENDER_PIX,
    C93 = AVCodecID_AV_CODEC_ID_C93,
    Cavs = AVCodecID_AV_CODEC_ID_CAVS,
    Cbd2Dpcm = AVCodecID_AV_CODEC_ID_CBD2_DPCM,
    Cdgraphics = AVCodecID_AV_CODEC_ID_CDGRAPHICS,
    Cdtoons = AVCodecID_AV_CODEC_ID_CDTOONS,
    Cdxl = AVCodecID_AV_CODEC_ID_CDXL,
    Celt = AVCodecID_AV_CODEC_ID_CELT,
    Cfhd = AVCodecID_AV_CODEC_ID_CFHD,
    Cinepak = AVCodecID_AV_CODEC_ID_CINEPAK,
    Clearvideo = AVCodecID_AV_CODEC_ID_CLEARVIDEO,
    Cljr = AVCodecID_AV_CODEC_ID_CLJR,
    Cllc = AVCodecID_AV_CODEC_ID_CLLC,
    Cmv = AVCodecID_AV_CODEC_ID_CMV,
    Codec2 = AVCodecID_AV_CODEC_ID_CODEC2,
    ComfortNoise = AVCodecID_AV_CODEC_ID_COMFORT_NOISE,
    Cook = AVCodecID_AV_CODEC_ID_COOK,
    Cpia = AVCodecID_AV_CODEC_ID_CPIA,
    Cri = AVCodecID_AV_CODEC_ID_CRI,
    Cscd = AVCodecID_AV_CODEC_ID_CSCD,
    Cyuv = AVCodecID_AV_CODEC_ID_CYUV,
    Daala = AVCodecID_AV_CODEC_ID_DAALA,
    Dds = AVCodecID_AV_CODEC_ID_DDS,
    DerfDpcm = AVCodecID_AV_CODEC_ID_DERF_DPCM,
    Dfa = AVCodecID_AV_CODEC_ID_DFA,
    Dfpwm = AVCodecID_AV_CODEC_ID_DFPWM,
    Dirac = AVCodecID_AV_CODEC_ID_DIRAC,
    Dnxhd = AVCodecID_AV_CODEC_ID_DNXHD,
    DolbyE = AVCodecID_AV_CODEC_ID_DOLBY_E,
    Dpx = AVCodecID_AV_CODEC_ID_DPX,
    DsdLsbf = AVCodecID_AV_CODEC_ID_DSD_LSBF,
    DsdLsbfPlanar = AVCodecID_AV_CODEC_ID_DSD_LSBF_PLANAR,
    DsdMsbf = AVCodecID_AV_CODEC_ID_DSD_MSBF,
    DsdMsbfPlanar = AVCodecID_AV_CODEC_ID_DSD_MSBF_PLANAR,
    Dsicinaudio = AVCodecID_AV_CODEC_ID_DSICINAUDIO,
    Dsicinvideo = AVCodecID_AV_CODEC_ID_DSICINVIDEO,
    DssSp = AVCodecID_AV_CODEC_ID_DSS_SP,
    Dst = AVCodecID_AV_CODEC_ID_DST,
    Dts = AVCodecID_AV_CODEC_ID_DTS,
    Dvaudio = AVCodecID_AV_CODEC_ID_DVAUDIO,
    DvbSubtitle = AVCodecID_AV_CODEC_ID_DVB_SUBTITLE,
    DvbTeletext = AVCodecID_AV_CODEC_ID_DVB_TELETEXT,
    DvdNav = AVCodecID_AV_CODEC_ID_DVD_NAV,
    DvdSubtitle = AVCodecID_AV_CODEC_ID_DVD_SUBTITLE,
    Dvvideo = AVCodecID_AV_CODEC_ID_DVVIDEO,
    Dxa = AVCodecID_AV_CODEC_ID_DXA,
    Dxtory = AVCodecID_AV_CODEC_ID_DXTORY,
    Dxv = AVCodecID_AV_CODEC_ID_DXV,
    Eac3 = AVCodecID_AV_CODEC_ID_EAC3,
    Eia608 = AVCodecID_AV_CODEC_ID_EIA_608,
    Epg = AVCodecID_AV_CODEC_ID_EPG,
    Escape124 = AVCodecID_AV_CODEC_ID_ESCAPE124,
    Escape130 = AVCodecID_AV_CODEC_ID_ESCAPE130,
    Evc = AVCodecID_AV_CODEC_ID_EVC,
    Evrc = AVCodecID_AV_CODEC_ID_EVRC,
    Exr = AVCodecID_AV_CODEC_ID_EXR,
    Fastaudio = AVCodecID_AV_CODEC_ID_FASTAUDIO,
    Ffmetadata = AVCodecID_AV_CODEC_ID_FFMETADATA,
    Ffv1 = AVCodecID_AV_CODEC_ID_FFV1,
    Ffvhuff = AVCodecID_AV_CODEC_ID_FFVHUFF,
    Ffwavesynth = AVCodecID_AV_CODEC_ID_FFWAVESYNTH,
    Fic = AVCodecID_AV_CODEC_ID_FIC,
    Fits = AVCodecID_AV_CODEC_ID_FITS,
    Flac = AVCodecID_AV_CODEC_ID_FLAC,
    Flashsv = AVCodecID_AV_CODEC_ID_FLASHSV,
    Flashsv2 = AVCodecID_AV_CODEC_ID_FLASHSV2,
    Flic = AVCodecID_AV_CODEC_ID_FLIC,
    Flv1 = AVCodecID_AV_CODEC_ID_FLV1,
    Fmvc = AVCodecID_AV_CODEC_ID_FMVC,
    Fraps = AVCodecID_AV_CODEC_ID_FRAPS,
    Frwu = AVCodecID_AV_CODEC_ID_FRWU,
    Ftr = AVCodecID_AV_CODEC_ID_FTR,
    G2m = AVCodecID_AV_CODEC_ID_G2M,
    G729 = AVCodecID_AV_CODEC_ID_G729,
    G723_1 = AVCodecID_AV_CODEC_ID_G723_1,
    Gdv = AVCodecID_AV_CODEC_ID_GDV,
    Gem = AVCodecID_AV_CODEC_ID_GEM,
    Gif = AVCodecID_AV_CODEC_ID_GIF,
    GremlimDpcm = AVCodecID_AV_CODEC_ID_GREMLIN_DPCM,
    Gsm = AVCodecID_AV_CODEC_ID_GSM,
    GsmMs = AVCodecID_AV_CODEC_ID_GSM_MS,
    H261 = AVCodecID_AV_CODEC_ID_H261,
    H263 = AVCodecID_AV_CODEC_ID_H263,
    H264 = AVCodecID_AV_CODEC_ID_H264,
    H263i = AVCodecID_AV_CODEC_ID_H263I,
    H263p = AVCodecID_AV_CODEC_ID_H263P,
    Hap = AVCodecID_AV_CODEC_ID_HAP,
    Hca = AVCodecID_AV_CODEC_ID_HCA,
    Hcom = AVCodecID_AV_CODEC_ID_HCOM,
    HdmvPgsSubtitle = AVCodecID_AV_CODEC_ID_HDMV_PGS_SUBTITLE,
    HdmvTextSubtitle = AVCodecID_AV_CODEC_ID_HDMV_TEXT_SUBTITLE,
    Hevc = AVCodecID_AV_CODEC_ID_HEVC,
    Hnm4Video = AVCodecID_AV_CODEC_ID_HNM4_VIDEO,
    Hqx = AVCodecID_AV_CODEC_ID_HQX,
    HqHqa = AVCodecID_AV_CODEC_ID_HQ_HQA,
    Huffyuv = AVCodecID_AV_CODEC_ID_HUFFYUV,
    Hymt = AVCodecID_AV_CODEC_ID_HYMT,
    Iac = AVCodecID_AV_CODEC_ID_IAC,
    Idcin = AVCodecID_AV_CODEC_ID_IDCIN,
    Idf = AVCodecID_AV_CODEC_ID_IDF,
    IffIlbm = AVCodecID_AV_CODEC_ID_IFF_ILBM,
    Ilbc = AVCodecID_AV_CODEC_ID_ILBC,
    Imc = AVCodecID_AV_CODEC_ID_IMC,
    Imm4 = AVCodecID_AV_CODEC_ID_IMM4,
    Imm5 = AVCodecID_AV_CODEC_ID_IMM5,
    Indeo2 = AVCodecID_AV_CODEC_ID_INDEO2,
    Indeo3 = AVCodecID_AV_CODEC_ID_INDEO3,
    Indeo4 = AVCodecID_AV_CODEC_ID_INDEO4,
    Indeo5 = AVCodecID_AV_CODEC_ID_INDEO5,
    InterplayAcm = AVCodecID_AV_CODEC_ID_INTERPLAY_ACM,
    InterplayDpcm = AVCodecID_AV_CODEC_ID_INTERPLAY_DPCM,
    InterplayVideo = AVCodecID_AV_CODEC_ID_INTERPLAY_VIDEO,
    Ipu = AVCodecID_AV_CODEC_ID_IPU,
    Jacosub = AVCodecID_AV_CODEC_ID_JACOSUB,
    Jpeg2000 = AVCodecID_AV_CODEC_ID_JPEG2000,
    Jpegls = AVCodecID_AV_CODEC_ID_JPEGLS,
    Jpegxl = AVCodecID_AV_CODEC_ID_JPEGXL,
    Jv = AVCodecID_AV_CODEC_ID_JV,
    Kgv1 = AVCodecID_AV_CODEC_ID_KGV1,
    Kmvc = AVCodecID_AV_CODEC_ID_KMVC,
    Lagarith = AVCodecID_AV_CODEC_ID_LAGARITH,
    Lc3 = AVCodecID_AV_CODEC_ID_LC3,
    Lcevc = AVCodecID_AV_CODEC_ID_LCEVC,
    Lead = AVCodecID_AV_CODEC_ID_LEAD,
    Ljpeg = AVCodecID_AV_CODEC_ID_LJPEG,
    Loco = AVCodecID_AV_CODEC_ID_LOCO,
    Lscr = AVCodecID_AV_CODEC_ID_LSCR,
    M101 = AVCodecID_AV_CODEC_ID_M101,
    Mace3 = AVCodecID_AV_CODEC_ID_MACE3,
    Mace6 = AVCodecID_AV_CODEC_ID_MACE6,
    Mad = AVCodecID_AV_CODEC_ID_MAD,
    Magicyuv = AVCodecID_AV_CODEC_ID_MAGICYUV,
    Mdec = AVCodecID_AV_CODEC_ID_MDEC,
    Media100 = AVCodecID_AV_CODEC_ID_MEDIA100,
    Metasound = AVCodecID_AV_CODEC_ID_METASOUND,
    Microdvd = AVCodecID_AV_CODEC_ID_MICRODVD,
    Mimic = AVCodecID_AV_CODEC_ID_MIMIC,
    Misc4 = AVCodecID_AV_CODEC_ID_MISC4,
    Mjpeg = AVCodecID_AV_CODEC_ID_MJPEG,
    Mjpegb = AVCodecID_AV_CODEC_ID_MJPEGB,
    Mlp = AVCodecID_AV_CODEC_ID_MLP,
    Mmvideo = AVCodecID_AV_CODEC_ID_MMVIDEO,
    Mobiclip = AVCodecID_AV_CODEC_ID_MOBICLIP,
    Motionpixels = AVCodecID_AV_CODEC_ID_MOTIONPIXELS,
    MovText = AVCodecID_AV_CODEC_ID_MOV_TEXT,
    Mp1 = AVCodecID_AV_CODEC_ID_MP1,
    Mp2 = AVCodecID_AV_CODEC_ID_MP2,
    /// preferred ID for decoding MPEG audio layer 1, 2 or 3
    Mp3 = AVCodecID_AV_CODEC_ID_MP3,
    Mp3adu = AVCodecID_AV_CODEC_ID_MP3ADU,
    Mp3on4 = AVCodecID_AV_CODEC_ID_MP3ON4,
    Mp4als = AVCodecID_AV_CODEC_ID_MP4ALS,
    Mpeg4 = AVCodecID_AV_CODEC_ID_MPEG4,
    Mpeg1video = AVCodecID_AV_CODEC_ID_MPEG1VIDEO,
    /// FAKE codec to indicate a raw MPEG-2 TS stream (only used by libavformat)
    Mpeg2ts = AVCodecID_AV_CODEC_ID_MPEG2TS,
    /// preferred ID for MPEG-1/2 video decoding
    Mpeg2video = AVCodecID_AV_CODEC_ID_MPEG2VIDEO,
    /// FAKE codec to indicate a MPEG-4 Systems stream (only used by libavformat)
    Mpeg4systems = AVCodecID_AV_CODEC_ID_MPEG4SYSTEMS,
    Mpegh3dAudio = AVCodecID_AV_CODEC_ID_MPEGH_3D_AUDIO,
    Mpl2 = AVCodecID_AV_CODEC_ID_MPL2,
    Msa1 = AVCodecID_AV_CODEC_ID_MSA1,
    Mscc = AVCodecID_AV_CODEC_ID_MSCC,
    Msmpeg4v1 = AVCodecID_AV_CODEC_ID_MSMPEG4V1,
    Msmpeg4v2 = AVCodecID_AV_CODEC_ID_MSMPEG4V2,
    Msmpeg4v3 = AVCodecID_AV_CODEC_ID_MSMPEG4V3,
    Msnsiren = AVCodecID_AV_CODEC_ID_MSNSIREN,
    Msp2 = AVCodecID_AV_CODEC_ID_MSP2,
    Msrle = AVCodecID_AV_CODEC_ID_MSRLE,
    Mss1 = AVCodecID_AV_CODEC_ID_MSS1,
    Mss2 = AVCodecID_AV_CODEC_ID_MSS2,
    Msvideo1 = AVCodecID_AV_CODEC_ID_MSVIDEO1,
    Mszh = AVCodecID_AV_CODEC_ID_MSZH,
    Mts2 = AVCodecID_AV_CODEC_ID_MTS2,
    Musepack7 = AVCodecID_AV_CODEC_ID_MUSEPACK7,
    Musepack8 = AVCodecID_AV_CODEC_ID_MUSEPACK8,
    Mv30 = AVCodecID_AV_CODEC_ID_MV30,
    Mvc1 = AVCodecID_AV_CODEC_ID_MVC1,
    Mvc2 = AVCodecID_AV_CODEC_ID_MVC2,
    Mvdv = AVCodecID_AV_CODEC_ID_MVDV,
    Mvha = AVCodecID_AV_CODEC_ID_MVHA,
    Mwsc = AVCodecID_AV_CODEC_ID_MWSC,
    Mxpeg = AVCodecID_AV_CODEC_ID_MXPEG,
    Nellymoser = AVCodecID_AV_CODEC_ID_NELLYMOSER,
    None = AVCodecID_AV_CODEC_ID_NONE,
    Notchlc = AVCodecID_AV_CODEC_ID_NOTCHLC,
    Nuv = AVCodecID_AV_CODEC_ID_NUV,
    On2avc = AVCodecID_AV_CODEC_ID_ON2AVC,
    Opus = AVCodecID_AV_CODEC_ID_OPUS,
    Osq = AVCodecID_AV_CODEC_ID_OSQ,
    Otf = AVCodecID_AV_CODEC_ID_OTF,
    PafAudio = AVCodecID_AV_CODEC_ID_PAF_AUDIO,
    PafVideo = AVCodecID_AV_CODEC_ID_PAF_VIDEO,
    Pam = AVCodecID_AV_CODEC_ID_PAM,
    Pbm = AVCodecID_AV_CODEC_ID_PBM,
    PcmAlaw = AVCodecID_AV_CODEC_ID_PCM_ALAW,
    PcmBluray = AVCodecID_AV_CODEC_ID_PCM_BLURAY,
    PcmDvd = AVCodecID_AV_CODEC_ID_PCM_DVD,
    PcmF16le = AVCodecID_AV_CODEC_ID_PCM_F16LE,
    PcmF24le = AVCodecID_AV_CODEC_ID_PCM_F24LE,
    PcmF32be = AVCodecID_AV_CODEC_ID_PCM_F32BE,
    PcmF32le = AVCodecID_AV_CODEC_ID_PCM_F32LE,
    PcmF64be = AVCodecID_AV_CODEC_ID_PCM_F64BE,
    PcmF64le = AVCodecID_AV_CODEC_ID_PCM_F64LE,
    PcmLxf = AVCodecID_AV_CODEC_ID_PCM_LXF,
    PcmMulaw = AVCodecID_AV_CODEC_ID_PCM_MULAW,
    PcmS8 = AVCodecID_AV_CODEC_ID_PCM_S8,
    PcmS8Planar = AVCodecID_AV_CODEC_ID_PCM_S8_PLANAR,
    PcmS16be = AVCodecID_AV_CODEC_ID_PCM_S16BE,
    PcmS16bePlanar = AVCodecID_AV_CODEC_ID_PCM_S16BE_PLANAR,
    PcmS16le = AVCodecID_AV_CODEC_ID_PCM_S16LE,
    PcmS16lePlanar = AVCodecID_AV_CODEC_ID_PCM_S16LE_PLANAR,
    PcmS24be = AVCodecID_AV_CODEC_ID_PCM_S24BE,
    PcmS24daud = AVCodecID_AV_CODEC_ID_PCM_S24DAUD,
    PcmS24le = AVCodecID_AV_CODEC_ID_PCM_S24LE,
    PcmS24lePlanar = AVCodecID_AV_CODEC_ID_PCM_S24LE_PLANAR,
    PcmS32be = AVCodecID_AV_CODEC_ID_PCM_S32BE,
    PcmS32le = AVCodecID_AV_CODEC_ID_PCM_S32LE,
    PcmS32lePlanar = AVCodecID_AV_CODEC_ID_PCM_S32LE_PLANAR,
    PcmS64be = AVCodecID_AV_CODEC_ID_PCM_S64BE,
    PcmS64le = AVCodecID_AV_CODEC_ID_PCM_S64LE,
    PcmSga = AVCodecID_AV_CODEC_ID_PCM_SGA,
    PcmU8 = AVCodecID_AV_CODEC_ID_PCM_U8,
    PcmU16be = AVCodecID_AV_CODEC_ID_PCM_U16BE,
    PcmU16le = AVCodecID_AV_CODEC_ID_PCM_U16LE,
    PcmU24be = AVCodecID_AV_CODEC_ID_PCM_U24BE,
    PcmU24le = AVCodecID_AV_CODEC_ID_PCM_U24LE,
    PcmU32be = AVCodecID_AV_CODEC_ID_PCM_U32BE,
    PcmU32le = AVCodecID_AV_CODEC_ID_PCM_U32LE,
    PcmVidc = AVCodecID_AV_CODEC_ID_PCM_VIDC,
    PcmZork = AVCodecID_AV_CODEC_ID_PCM_ZORK,
    Pcx = AVCodecID_AV_CODEC_ID_PCX,
    Pdv = AVCodecID_AV_CODEC_ID_PDV,
    Pfm = AVCodecID_AV_CODEC_ID_PFM,
    Pgm = AVCodecID_AV_CODEC_ID_PGM,
    Pgmyuv = AVCodecID_AV_CODEC_ID_PGMYUV,
    Pgx = AVCodecID_AV_CODEC_ID_PGX,
    Phm = AVCodecID_AV_CODEC_ID_PHM,
    Photocd = AVCodecID_AV_CODEC_ID_PHOTOCD,
    Pictor = AVCodecID_AV_CODEC_ID_PICTOR,
    Pixlet = AVCodecID_AV_CODEC_ID_PIXLET,
    Pjs = AVCodecID_AV_CODEC_ID_PJS,
    Png = AVCodecID_AV_CODEC_ID_PNG,
    Ppm = AVCodecID_AV_CODEC_ID_PPM,
    /// codec_id is not known (like AV_CODEC_ID_NONE) but lavf should attempt to identify it
    Probe = AVCodecID_AV_CODEC_ID_PROBE,
    Prores = AVCodecID_AV_CODEC_ID_PRORES,
    Prosumer = AVCodecID_AV_CODEC_ID_PROSUMER,
    Psd = AVCodecID_AV_CODEC_ID_PSD,
    Ptx = AVCodecID_AV_CODEC_ID_PTX,
    Qcelp = AVCodecID_AV_CODEC_ID_QCELP,
    Qdm2 = AVCodecID_AV_CODEC_ID_QDM2,
    Qdmc = AVCodecID_AV_CODEC_ID_QDMC,
    Qdraw = AVCodecID_AV_CODEC_ID_QDRAW,
    Qoa = AVCodecID_AV_CODEC_ID_QOA,
    Qoi = AVCodecID_AV_CODEC_ID_QOI,
    Qpeg = AVCodecID_AV_CODEC_ID_QPEG,
    Qtrle = AVCodecID_AV_CODEC_ID_QTRLE,
    R10k = AVCodecID_AV_CODEC_ID_R10K,
    R210 = AVCodecID_AV_CODEC_ID_R210,
    RadianceHdr = AVCodecID_AV_CODEC_ID_RADIANCE_HDR,
    Ralf = AVCodecID_AV_CODEC_ID_RALF,
    Rasc = AVCodecID_AV_CODEC_ID_RASC,
    Rawvideo = AVCodecID_AV_CODEC_ID_RAWVIDEO,
    Ra144 = AVCodecID_AV_CODEC_ID_RA_144,
    Ra288 = AVCodecID_AV_CODEC_ID_RA_288,
    Realtext = AVCodecID_AV_CODEC_ID_REALTEXT,
    Rka = AVCodecID_AV_CODEC_ID_RKA,
    Rl2 = AVCodecID_AV_CODEC_ID_RL2,
    Roq = AVCodecID_AV_CODEC_ID_ROQ,
    RoqDpcm = AVCodecID_AV_CODEC_ID_ROQ_DPCM,
    Rpza = AVCodecID_AV_CODEC_ID_RPZA,
    Rscc = AVCodecID_AV_CODEC_ID_RSCC,
    Rtv1 = AVCodecID_AV_CODEC_ID_RTV1,
    Rv10 = AVCodecID_AV_CODEC_ID_RV10,
    Rv20 = AVCodecID_AV_CODEC_ID_RV20,
    Rv30 = AVCodecID_AV_CODEC_ID_RV30,
    Rv40 = AVCodecID_AV_CODEC_ID_RV40,
    S302m = AVCodecID_AV_CODEC_ID_S302M,
    Sami = AVCodecID_AV_CODEC_ID_SAMI,
    Sanm = AVCodecID_AV_CODEC_ID_SANM,
    Sbc = AVCodecID_AV_CODEC_ID_SBC,
    Scpr = AVCodecID_AV_CODEC_ID_SCPR,
    Screenpresso = AVCodecID_AV_CODEC_ID_SCREENPRESSO,
    /// Contain timestamp estimated through PCR of program stream.
    Scte35 = AVCodecID_AV_CODEC_ID_SCTE_35,
    Sdx2Dpcm = AVCodecID_AV_CODEC_ID_SDX2_DPCM,
    SgaVideo = AVCodecID_AV_CODEC_ID_SGA_VIDEO,
    Sgi = AVCodecID_AV_CODEC_ID_SGI,
    Sgirle = AVCodecID_AV_CODEC_ID_SGIRLE,
    Sheervideo = AVCodecID_AV_CODEC_ID_SHEERVIDEO,
    Shorten = AVCodecID_AV_CODEC_ID_SHORTEN,
    SimbiosisImx = AVCodecID_AV_CODEC_ID_SIMBIOSIS_IMX,
    Sipr = AVCodecID_AV_CODEC_ID_SIPR,
    Siren = AVCodecID_AV_CODEC_ID_SIREN,
    Smackaudio = AVCodecID_AV_CODEC_ID_SMACKAUDIO,
    Smackvideo = AVCodecID_AV_CODEC_ID_SMACKVIDEO,
    Smc = AVCodecID_AV_CODEC_ID_SMC,
    Smpte2038 = AVCodecID_AV_CODEC_ID_SMPTE_2038,
    SmpteKlv = AVCodecID_AV_CODEC_ID_SMPTE_KLV,
    Smv = AVCodecID_AV_CODEC_ID_SMV,
    Smvjpeg = AVCodecID_AV_CODEC_ID_SMVJPEG,
    Snow = AVCodecID_AV_CODEC_ID_SNOW,
    SolDpcm = AVCodecID_AV_CODEC_ID_SOL_DPCM,
    Sonic = AVCodecID_AV_CODEC_ID_SONIC,
    SonicLs = AVCodecID_AV_CODEC_ID_SONIC_LS,
    Sp5x = AVCodecID_AV_CODEC_ID_SP5X,
    Speedhq = AVCodecID_AV_CODEC_ID_SPEEDHQ,
    Speex = AVCodecID_AV_CODEC_ID_SPEEX,
    Srgc = AVCodecID_AV_CODEC_ID_SRGC,
    Srt = AVCodecID_AV_CODEC_ID_SRT,
    Ssa = AVCodecID_AV_CODEC_ID_SSA,
    Stl = AVCodecID_AV_CODEC_ID_STL,
    Subrip = AVCodecID_AV_CODEC_ID_SUBRIP,
    Subviewer = AVCodecID_AV_CODEC_ID_SUBVIEWER,
    Subviewer1 = AVCodecID_AV_CODEC_ID_SUBVIEWER1,
    Sunrast = AVCodecID_AV_CODEC_ID_SUNRAST,
    Svg = AVCodecID_AV_CODEC_ID_SVG,
    Svq1 = AVCodecID_AV_CODEC_ID_SVQ1,
    Svq3 = AVCodecID_AV_CODEC_ID_SVQ3,
    Tak = AVCodecID_AV_CODEC_ID_TAK,
    Targa = AVCodecID_AV_CODEC_ID_TARGA,
    TargaY216 = AVCodecID_AV_CODEC_ID_TARGA_Y216,
    Tdsc = AVCodecID_AV_CODEC_ID_TDSC,
    /// raw UTF-8 text
    Text = AVCodecID_AV_CODEC_ID_TEXT,
    Tgq = AVCodecID_AV_CODEC_ID_TGQ,
    Tgv = AVCodecID_AV_CODEC_ID_TGV,
    Theora = AVCodecID_AV_CODEC_ID_THEORA,
    Thp = AVCodecID_AV_CODEC_ID_THP,
    Tiertexseqvideo = AVCodecID_AV_CODEC_ID_TIERTEXSEQVIDEO,
    Tiff = AVCodecID_AV_CODEC_ID_TIFF,
    TimedId3 = AVCodecID_AV_CODEC_ID_TIMED_ID3,
    Tmv = AVCodecID_AV_CODEC_ID_TMV,
    Tqi = AVCodecID_AV_CODEC_ID_TQI,
    Truehd = AVCodecID_AV_CODEC_ID_TRUEHD,
    Truemotion1 = AVCodecID_AV_CODEC_ID_TRUEMOTION1,
    Truemotion2 = AVCodecID_AV_CODEC_ID_TRUEMOTION2,
    Truemotion2rt = AVCodecID_AV_CODEC_ID_TRUEMOTION2RT,
    Truespeech = AVCodecID_AV_CODEC_ID_TRUESPEECH,
    Tscc = AVCodecID_AV_CODEC_ID_TSCC,
    Tscc2 = AVCodecID_AV_CODEC_ID_TSCC2,
    Tta = AVCodecID_AV_CODEC_ID_TTA,
    Ttf = AVCodecID_AV_CODEC_ID_TTF,
    Ttml = AVCodecID_AV_CODEC_ID_TTML,
    Twinvq = AVCodecID_AV_CODEC_ID_TWINVQ,
    Txd = AVCodecID_AV_CODEC_ID_TXD,
    Ulti = AVCodecID_AV_CODEC_ID_ULTI,
    Utvideo = AVCodecID_AV_CODEC_ID_UTVIDEO,
    V210 = AVCodecID_AV_CODEC_ID_V210,
    V308 = AVCodecID_AV_CODEC_ID_V308,
    V408 = AVCodecID_AV_CODEC_ID_V408,
    V410 = AVCodecID_AV_CODEC_ID_V410,
    V210x = AVCodecID_AV_CODEC_ID_V210X,
    Vb = AVCodecID_AV_CODEC_ID_VB,
    Vble = AVCodecID_AV_CODEC_ID_VBLE,
    Vbn = AVCodecID_AV_CODEC_ID_VBN,
    Vc1 = AVCodecID_AV_CODEC_ID_VC1,
    Vc1image = AVCodecID_AV_CODEC_ID_VC1IMAGE,
    Vcr1 = AVCodecID_AV_CODEC_ID_VCR1,
    Vixl = AVCodecID_AV_CODEC_ID_VIXL,
    Vmdaudio = AVCodecID_AV_CODEC_ID_VMDAUDIO,
    Vmdvideo = AVCodecID_AV_CODEC_ID_VMDVIDEO,
    Vmix = AVCodecID_AV_CODEC_ID_VMIX,
    Vmnc = AVCodecID_AV_CODEC_ID_VMNC,
    /// Dummy null video codec, useful mainly for development and debugging. Null encoder/decoder discard all input and never return any output.
    Vnull = AVCodecID_AV_CODEC_ID_VNULL,
    Vorbis = AVCodecID_AV_CODEC_ID_VORBIS,
    Vp3 = AVCodecID_AV_CODEC_ID_VP3,
    Vp4 = AVCodecID_AV_CODEC_ID_VP4,
    Vp5 = AVCodecID_AV_CODEC_ID_VP5,
    Vp6 = AVCodecID_AV_CODEC_ID_VP6,
    Vp7 = AVCodecID_AV_CODEC_ID_VP7,
    Vp8 = AVCodecID_AV_CODEC_ID_VP8,
    Vp9 = AVCodecID_AV_CODEC_ID_VP9,
    Vp6a = AVCodecID_AV_CODEC_ID_VP6A,
    Vp6f = AVCodecID_AV_CODEC_ID_VP6F,
    Vplayer = AVCodecID_AV_CODEC_ID_VPLAYER,
    Vqc = AVCodecID_AV_CODEC_ID_VQC,
    Vvc = AVCodecID_AV_CODEC_ID_VVC,
    WadyDpcm = AVCodecID_AV_CODEC_ID_WADY_DPCM,
    Wavarc = AVCodecID_AV_CODEC_ID_WAVARC,
    Wavpack = AVCodecID_AV_CODEC_ID_WAVPACK,
    Wbmp = AVCodecID_AV_CODEC_ID_WBMP,
    Wcmv = AVCodecID_AV_CODEC_ID_WCMV,
    Webp = AVCodecID_AV_CODEC_ID_WEBP,
    Webvtt = AVCodecID_AV_CODEC_ID_WEBVTT,
    WestwoodSnd1 = AVCodecID_AV_CODEC_ID_WESTWOOD_SND1,
    Wmalossless = AVCodecID_AV_CODEC_ID_WMALOSSLESS,
    Wmapro = AVCodecID_AV_CODEC_ID_WMAPRO,
    Wmav1 = AVCodecID_AV_CODEC_ID_WMAV1,
    Wmav2 = AVCodecID_AV_CODEC_ID_WMAV2,
    Wmavoice = AVCodecID_AV_CODEC_ID_WMAVOICE,
    Wmv1 = AVCodecID_AV_CODEC_ID_WMV1,
    Wmv2 = AVCodecID_AV_CODEC_ID_WMV2,
    Wmv3 = AVCodecID_AV_CODEC_ID_WMV3,
    Wmv3image = AVCodecID_AV_CODEC_ID_WMV3IMAGE,
    Wnv1 = AVCodecID_AV_CODEC_ID_WNV1,
    /// Passthrough codec, AVFrames wrapped in AVPacket
    WrappedAvframe = AVCodecID_AV_CODEC_ID_WRAPPED_AVFRAME,
    WsVqa = AVCodecID_AV_CODEC_ID_WS_VQA,
    XanDpcm = AVCodecID_AV_CODEC_ID_XAN_DPCM,
    XanWc3 = AVCodecID_AV_CODEC_ID_XAN_WC3,
    XanWc4 = AVCodecID_AV_CODEC_ID_XAN_WC4,
    Xbin = AVCodecID_AV_CODEC_ID_XBIN,
    Xbm = AVCodecID_AV_CODEC_ID_XBM,
    Xface = AVCodecID_AV_CODEC_ID_XFACE,
    Xma1 = AVCodecID_AV_CODEC_ID_XMA1,
    Xma2 = AVCodecID_AV_CODEC_ID_XMA2,
    Xpm = AVCodecID_AV_CODEC_ID_XPM,
    Xsub = AVCodecID_AV_CODEC_ID_XSUB,
    Xwd = AVCodecID_AV_CODEC_ID_XWD,
    Y41p = AVCodecID_AV_CODEC_ID_Y41P,
    Ylc = AVCodecID_AV_CODEC_ID_YLC,
    Yop = AVCodecID_AV_CODEC_ID_YOP,
    Yuv4 = AVCodecID_AV_CODEC_ID_YUV4,
    Zerocodec = AVCodecID_AV_CODEC_ID_ZEROCODEC,
    Zlib = AVCodecID_AV_CODEC_ID_ZLIB,
    Zmbv = AVCodecID_AV_CODEC_ID_ZMBV,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AVColorPrimaries {
    /// also ITU-R BT1361 / IEC 61966-2-4 / SMPTE RP 177 Annex B
    Bt709 = AVColorPrimaries_AVCOL_PRI_BT709,
    /// also ITU-R BT601-6 625 / ITU-R BT1358 625 / ITU-R BT1700 625 PAL & SECAM
    Bt470bg = AVColorPrimaries_AVCOL_PRI_BT470BG,
    /// also FCC Title 47 Code of Federal Regulations 73.682 (a)(20)
    Bt470m = AVColorPrimaries_AVCOL_PRI_BT470M,
    /// ITU-R BT2020
    Bt2020 = AVColorPrimaries_AVCOL_PRI_BT2020,
    /// EBU Tech. 3213-E (nothing there) / one of JEDEC P22 group phosphors
    Ebu3213 = AVColorPrimaries_AVCOL_PRI_EBU3213,
    /// colour filters using Illuminant C
    Film = AVColorPrimaries_AVCOL_PRI_FILM,
    /// Not part of ABI
    Nb = AVColorPrimaries_AVCOL_PRI_NB,
    Reserved = AVColorPrimaries_AVCOL_PRI_RESERVED,
    Reserved0 = AVColorPrimaries_AVCOL_PRI_RESERVED0,
    /// SMPTE ST 428-1 (CIE 1931 XYZ)
    Smpte428 = AVColorPrimaries_AVCOL_PRI_SMPTE428,
    /// SMPTE ST 431-2 (2011) / DCI P3
    Smpte431 = AVColorPrimaries_AVCOL_PRI_SMPTE431,
    /// SMPTE ST 432-1 (2010) / P3 D65 / Display P3
    Smpte432 = AVColorPrimaries_AVCOL_PRI_SMPTE432,
    /// also ITU-R BT601-6 525 / ITU-R BT1358 525 / ITU-R BT1700 NTSC
    Smpte170m = AVColorPrimaries_AVCOL_PRI_SMPTE170M,
    /// identical to above, also called "SMPTE C" even though it uses D65
    Smpte240m = AVColorPrimaries_AVCOL_PRI_SMPTE240M,
    Unspecified = AVColorPrimaries_AVCOL_PRI_UNSPECIFIED,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AVColorRange {
    /// Full range content
    Jpeg = AVColorRange_AVCOL_RANGE_JPEG,
    /// Narrow or limited range content
    Mpeg = AVColorRange_AVCOL_RANGE_MPEG,
    /// Not part of ABI
    Nb = AVColorRange_AVCOL_RANGE_NB,
    Unspecified = AVColorRange_AVCOL_RANGE_UNSPECIFIED,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AVColorSpace {
    /// also ITU-R BT1361 / IEC 61966-2-4 xvYCC709 / derived in SMPTE RP 177 Annex B
    Bt709 = AVColorSpace_AVCOL_SPC_BT709,
    /// also ITU-R BT601-6 625 / ITU-R BT1358 625 / ITU-R BT1700 625 PAL & SECAM / IEC 61966-2-4 xvYCC601
    Bt470bg = AVColorSpace_AVCOL_SPC_BT470BG,
    /// ITU-R BT2020 constant luminance system
    Bt2020Cl = AVColorSpace_AVCOL_SPC_BT2020_CL,
    /// ITU-R BT2020 non-constant luminance system
    Bt2020Ncl = AVColorSpace_AVCOL_SPC_BT2020_NCL,
    /// Chromaticity-derived constant luminance system
    ChromaDerivedCl = AVColorSpace_AVCOL_SPC_CHROMA_DERIVED_CL,
    /// Chromaticity-derived non-constant luminance system
    ChromaDerivedNcl = AVColorSpace_AVCOL_SPC_CHROMA_DERIVED_NCL,
    /// FCC Title 47 Code of Federal Regulations 73.682 (a)(20)
    Fcc = AVColorSpace_AVCOL_SPC_FCC,
    /// ITU-R BT.2100-0, ICtCp
    Ictcp = AVColorSpace_AVCOL_SPC_ICTCP,
    /// SMPTE ST 2128, IPT-C2
    IptC2 = AVColorSpace_AVCOL_SPC_IPT_C2,
    /// Not part of ABI
    Nb = AVColorSpace_AVCOL_SPC_NB,
    /// reserved for future use by ITU-T and ISO/IEC just like 15-255 are
    Reserved = AVColorSpace_AVCOL_SPC_RESERVED,
    /// order of coefficients is actually GBR, also IEC 61966-2-1 (sRGB), YZX and ST 428-1
    Rgb = AVColorSpace_AVCOL_SPC_RGB,
    /// also ITU-R BT601-6 525 / ITU-R BT1358 525 / ITU-R BT1700 NTSC / functionally identical to above
    Smpte170m = AVColorSpace_AVCOL_SPC_SMPTE170M,
    /// derived from 170M primaries and D65 white point, 170M is derived from BT470 System M's primaries
    Smpte240m = AVColorSpace_AVCOL_SPC_SMPTE240M,
    /// SMPTE 2085, Y'D'zD'x
    Smpte2085 = AVColorSpace_AVCOL_SPC_SMPTE2085,
    Unspecified = AVColorSpace_AVCOL_SPC_UNSPECIFIED,
    /// used by Dirac / VC-2 and H.264 FRext, see ITU-T SG16
    Ycgco = AVColorSpace_AVCOL_SPC_YCGCO,
    /// YCgCo-R, even addition of bits
    YcgcoRe = AVColorSpace_AVCOL_SPC_YCGCO_RE,
    /// YCgCo-R, odd addition of bits
    YcgcoRo = AVColorSpace_AVCOL_SPC_YCGCO_RO,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AVColorTransferCharacteristic {
    /// ARIB STD-B67, known as "Hybrid log-gamma"
    AribStdB67 = AVColorTransferCharacteristic_AVCOL_TRC_ARIB_STD_B67,
    /// also ITU-R BT1361
    Bt709 = AVColorTransferCharacteristic_AVCOL_TRC_BT709,
    /// ITU-R BT1361 Extended Colour Gamut
    Bt1361Ecg = AVColorTransferCharacteristic_AVCOL_TRC_BT1361_ECG,
    /// ITU-R BT2020 for 10-bit system
    Bt202010 = AVColorTransferCharacteristic_AVCOL_TRC_BT2020_10,
    /// ITU-R BT2020 for 12-bit system
    Bt202012 = AVColorTransferCharacteristic_AVCOL_TRC_BT2020_12,
    /// also ITU-R BT470M / ITU-R BT1700 625 PAL & SECAM
    Gamma22 = AVColorTransferCharacteristic_AVCOL_TRC_GAMMA22,
    /// also ITU-R BT470BG
    Gamma28 = AVColorTransferCharacteristic_AVCOL_TRC_GAMMA28,
    /// IEC 61966-2-1 (sRGB or sYCC)
    Iec61966_2_1 = AVColorTransferCharacteristic_AVCOL_TRC_IEC61966_2_1,
    /// IEC 61966-2-4
    Iec61966_2_4 = AVColorTransferCharacteristic_AVCOL_TRC_IEC61966_2_4,
    /// "Linear transfer characteristics"
    Linear = AVColorTransferCharacteristic_AVCOL_TRC_LINEAR,
    /// "Logarithmic transfer characteristic (100:1 range)"
    Log = AVColorTransferCharacteristic_AVCOL_TRC_LOG,
    /// "Logarithmic transfer characteristic (100 * Sqrt(10) : 1 range)"
    LogSqrt = AVColorTransferCharacteristic_AVCOL_TRC_LOG_SQRT,
    /// Not part of ABI
    Nb = AVColorTransferCharacteristic_AVCOL_TRC_NB,
    Reserved = AVColorTransferCharacteristic_AVCOL_TRC_RESERVED,
    Reserved0 = AVColorTransferCharacteristic_AVCOL_TRC_RESERVED0,
    /// SMPTE ST 428-1
    Smpte428 = AVColorTransferCharacteristic_AVCOL_TRC_SMPTE428,
    /// also ITU-R BT601-6 525 or 625 / ITU-R BT1358 525 or 625 / ITU-R BT1700 NTSC
    Smpte170m = AVColorTransferCharacteristic_AVCOL_TRC_SMPTE170M,
    Smpte240m = AVColorTransferCharacteristic_AVCOL_TRC_SMPTE240M,
    /// SMPTE ST 2084 for 10-, 12-, 14- and 16-bit systems
    Smpte2084 = AVColorTransferCharacteristic_AVCOL_TRC_SMPTE2084,
    Unspecified = AVColorTransferCharacteristic_AVCOL_TRC_UNSPECIFIED,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AVDiscard {
    /// discard all
    All = AVDiscard_AVDISCARD_ALL,
    /// discard all bidirectional frames
    Bidir = AVDiscard_AVDISCARD_BIDIR,
    /// discard useless packets like 0 size packets in avi
    Default = AVDiscard_AVDISCARD_DEFAULT,
    /// discard nothing
    None = AVDiscard_AVDISCARD_NONE,
    /// discard all non intra frames
    NonIntra = AVDiscard_AVDISCARD_NONINTRA,
    /// discard all frames except keyframes
    NonKey = AVDiscard_AVDISCARD_NONKEY,
    /// discard all non reference
    NonRef = AVDiscard_AVDISCARD_NONREF,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AVFieldOrder {
    /// Bottom coded first, bottom displayed first
    Bb = AVFieldOrder_AV_FIELD_BB,
    /// Bottom coded first, top displayed first
    Bt = AVFieldOrder_AV_FIELD_BT,
    Progressive = AVFieldOrder_AV_FIELD_PROGRESSIVE,
    /// Top coded first, bottom displayed first
    Tb = AVFieldOrder_AV_FIELD_TB,
    /// Top coded first, top displayed first
    Tt = AVFieldOrder_AV_FIELD_TT,
    Unknown = AVFieldOrder_AV_FIELD_UNKNOWN,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AVFrameSideDataType {
    /// ATSC A53 Part 4 Closed Captions. A53 CC bitstream is stored as uint8_t in AVFrameSideData.data. The number of bytes of CC data is AVFrameSideData.size.
    A53Cc = AVFrameSideDataType_AV_FRAME_DATA_A53_CC,
    /// Active Format Description data consisting of a single byte as specified in ETSI TS 101 154 using AVActiveFormatDescription enum.
    Afd = AVFrameSideDataType_AV_FRAME_DATA_AFD,
    /// Ambient viewing environment metadata, as defined by H.274.
    AmbientViewingEnvironment = AVFrameSideDataType_AV_FRAME_DATA_AMBIENT_VIEWING_ENVIRONMENT,
    /// This side data must be associated with an audio frame and corresponds to enum AVAudioServiceType defined in avcodec.h.
    AudioServiceType = AVFrameSideDataType_AV_FRAME_DATA_AUDIO_SERVICE_TYPE,
    /// Content light level (based on CTA-861.3). This payload contains data in the form of the AVContentLightMetadata struct.
    ContentLightLevel = AVFrameSideDataType_AV_FRAME_DATA_CONTENT_LIGHT_LEVEL,
    /// Bounding boxes for object detection and classification, as described by AVDetectionBBoxHeader.
    DetectionBboxes = AVFrameSideDataType_AV_FRAME_DATA_DETECTION_BBOXES,
    /// This side data contains a 3x3 transformation matrix describing an affine transformation that needs to be applied to the frame for correct presentation.
    DisplayMatrix = AVFrameSideDataType_AV_FRAME_DATA_DISPLAYMATRIX,
    /// Parsed Dolby Vision metadata, suitable for passing to a software implementation. The payload is the AVDOVIMetadata struct defined in libavutil/dovi_meta.h.
    DoviMetadata = AVFrameSideDataType_AV_FRAME_DATA_DOVI_METADATA,
    /// Dolby Vision RPU raw data, suitable for passing to x265 or other libraries. Array of uint8_t, with NAL emulation bytes intact.
    DoviRpuBuffer = AVFrameSideDataType_AV_FRAME_DATA_DOVI_RPU_BUFFER,
    /// Metadata relevant to a downmix procedure. The data is the AVDownmixInfo struct defined in libavutil/downmix_info.h.
    DownmixInfo = AVFrameSideDataType_AV_FRAME_DATA_DOWNMIX_INFO,
    /// HDR dynamic metadata associated with a video frame. The payload is an AVDynamicHDRPlus type and contains information for color volume transform - application 4 of SMPTE 2094-40:2016 standard.
    DynamicHdrPlus = AVFrameSideDataType_AV_FRAME_DATA_DYNAMIC_HDR_PLUS,
    /// HDR Vivid dynamic metadata associated with a video frame. The payload is an AVDynamicHDRVivid type and contains information for color volume transform - CUVA 005.1-2021.
    DynamicHdrVivid = AVFrameSideDataType_AV_FRAME_DATA_DYNAMIC_HDR_VIVID,
    /// Film grain parameters for a frame, described by AVFilmGrainParams. Must be present for every frame which should have film grain applied.
    FilmGrainParams = AVFrameSideDataType_AV_FRAME_DATA_FILM_GRAIN_PARAMS,
    /// The GOP timecode in 25 bit timecode format. Data format is 64-bit integer. This is set on the first frame of a GOP that has a temporal reference of 0.
    GopTimecode = AVFrameSideDataType_AV_FRAME_DATA_GOP_TIMECODE,
    /// The data contains an ICC profile as an opaque octet buffer following the format described by ISO 15076-1 with an optional name defined in the metadata key entry "name".
    IccProfile = AVFrameSideDataType_AV_FRAME_DATA_ICC_PROFILE,
    /// Raw LCEVC payload data, as a uint8_t array, with NAL emulation bytes intact.
    Lcevc = AVFrameSideDataType_AV_FRAME_DATA_LCEVC,
    /// Mastering display metadata associated with a video frame. The payload is an AVMasteringDisplayMetadata type and contains information about the mastering display color volume.
    MasteringDisplayMetadata = AVFrameSideDataType_AV_FRAME_DATA_MASTERING_DISPLAY_METADATA,
    /// The data is the AVMatrixEncoding enum defined in libavutil/channel_layout.h.
    MatrixEncoding = AVFrameSideDataType_AV_FRAME_DATA_MATRIXENCODING,
    /// Motion vectors exported by some codecs (on demand through the export_mvs flag set in the libavcodec AVCodecContext flags2 option). The data is the AVMotionVector struct defined in libavutil/motion_vector.h.
    MotionVectors = AVFrameSideDataType_AV_FRAME_DATA_MOTION_VECTORS,
    /// The data is the AVPanScan struct defined in libavcodec.
    PanScan = AVFrameSideDataType_AV_FRAME_DATA_PANSCAN,
    /// Regions Of Interest, the data is an array of AVRegionOfInterest type, the number of array element is implied by AVFrameSideData.size / AVRegionOfInterest.self_size.
    RegionsOfInterest = AVFrameSideDataType_AV_FRAME_DATA_REGIONS_OF_INTEREST,
    /// ReplayGain information in the form of the AVReplayGain struct.
    ReplayGain = AVFrameSideDataType_AV_FRAME_DATA_REPLAYGAIN,
    /// Timecode which conforms to SMPTE ST 12-1. The data is an array of 4 uint32_t where the first uint32_t describes how many (1-3) of the other timecodes are used. The timecode format is described in the documentation of av_timecode_get_smpte_from_framenum() function in libavutil/timecode.h.
    S12mTimecode = AVFrameSideDataType_AV_FRAME_DATA_S12M_TIMECODE,
    /// User data unregistered metadata associated with a video frame. This is the H.26[45] UDU SEI message, and shouldn't be used for any other purpose The data is stored as uint8_t in AVFrameSideData.data which is 16 bytes of uuid_iso_iec_11578 followed by AVFrameSideData.size - 16 bytes of user_data_payload_byte.
    SeiUnregistered = AVFrameSideDataType_AV_FRAME_DATA_SEI_UNREGISTERED,
    /// Recommmends skipping the specified number of samples. This is exported only if the "skip_manual" AVOption is set in libavcodec. This has the same format as AV_PKT_DATA_SKIP_SAMPLES.
    SkipSamples = AVFrameSideDataType_AV_FRAME_DATA_SKIP_SAMPLES,
    /// The data represents the AVSphericalMapping structure defined in libavutil/spherical.h.
    Spherical = AVFrameSideDataType_AV_FRAME_DATA_SPHERICAL,
    /// Stereoscopic 3d metadata. The data is the AVStereo3D struct defined in libavutil/stereo3d.h.
    Stereo3d = AVFrameSideDataType_AV_FRAME_DATA_STEREO3D,
    /// Encoding parameters for a video frame, as described by AVVideoEncParams.
    VideoEncParams = AVFrameSideDataType_AV_FRAME_DATA_VIDEO_ENC_PARAMS,
    /// Provide encoder-specific hinting information about changed/unchanged portions of a frame. It can be used to pass information about which macroblocks can be skipped because they didn't change from the corresponding ones in the previous frame. This could be useful for applications which know this information in advance to speed up encoding.
    VideoHint = AVFrameSideDataType_AV_FRAME_DATA_VIDEO_HINT,
    /// This side data must be associated with a video frame. The presence of this side data indicates that the video stream is composed of multiple views (e.g. stereoscopic 3D content, cf. H.264 Annex H or H.265 Annex G). The data is an int storing the view ID.
    ViewId = AVFrameSideDataType_AV_FRAME_DATA_VIEW_ID,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub enum AVMediaType {
    /// Opaque data information usually sparse
    Attachment = AVMediaType_AVMEDIA_TYPE_ATTACHMENT,
    Audio = AVMediaType_AVMEDIA_TYPE_AUDIO,
    /// Opaque data information usually continuous
    Data = AVMediaType_AVMEDIA_TYPE_DATA,
    Nb = AVMediaType_AVMEDIA_TYPE_NB,
    Subtitle = AVMediaType_AVMEDIA_TYPE_SUBTITLE,
    /// Usually treated as AVMEDIA_TYPE_DATA
    Unknown = AVMediaType_AVMEDIA_TYPE_UNKNOWN,
    Video = AVMediaType_AVMEDIA_TYPE_VIDEO,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AVOptionType {
    /// Underlying C type is a uint8_t* that is either NULL or points to an array allocated with the av_malloc() family of functions. The pointer is immediately followed by an int containing the array length in bytes.
    Binary = AVOptionType_AV_OPT_TYPE_BINARY,
    /// Underlying C type is int.
    Bool = AVOptionType_AV_OPT_TYPE_BOOL,
    /// Underlying C type is AVChannelLayout.
    ChLayout = AVOptionType_AV_OPT_TYPE_CHLAYOUT,
    /// Underlying C type is uint8_t[4].
    Color = AVOptionType_AV_OPT_TYPE_COLOR,
    /// Special option type for declaring named constants. Does not correspond to an actual field in the object, offset must be 0.
    Const = AVOptionType_AV_OPT_TYPE_CONST,
    /// Underlying C type is AVDictionary*.
    Dict = AVOptionType_AV_OPT_TYPE_DICT,
    /// Underlying C type is double.
    Double = AVOptionType_AV_OPT_TYPE_DOUBLE,
    /// Underlying C type is int64_t.
    Duration = AVOptionType_AV_OPT_TYPE_DURATION,
    /// Underlying C type is unsigned int.
    Flags = AVOptionType_AV_OPT_TYPE_FLAGS,
    /// May be combined with another regular option type to declare an array option.
    FlagArray = AVOptionType_AV_OPT_TYPE_FLAG_ARRAY,
    /// Underlying C type is float.
    Float = AVOptionType_AV_OPT_TYPE_FLOAT,
    /// Underlying C type is two consecutive integers.
    ImageSize = AVOptionType_AV_OPT_TYPE_IMAGE_SIZE,
    /// Underlying C type is int.
    Int = AVOptionType_AV_OPT_TYPE_INT,
    /// Underlying C type is int64_t.
    Int64 = AVOptionType_AV_OPT_TYPE_INT64,
    /// Underlying C type is enum AVPixelFormat.
    PixelFmt = AVOptionType_AV_OPT_TYPE_PIXEL_FMT,
    /// Underlying C type is AVRational.
    Rational = AVOptionType_AV_OPT_TYPE_RATIONAL,
    /// Underlying C type is enum AVSampleFormat.
    SampleFmt = AVOptionType_AV_OPT_TYPE_SAMPLE_FMT,
    /// Underlying C type is a uint8_t* that is either NULL or points to a C string allocated with the av_malloc() family of functions.
    String = AVOptionType_AV_OPT_TYPE_STRING,
    /// Underlying C type is unsigned int.
    Uint = AVOptionType_AV_OPT_TYPE_UINT,
    /// Underlying C type is uint64_t.
    Uint64 = AVOptionType_AV_OPT_TYPE_UINT64,
    /// Underlying C type is AVRational.
    VideoRate = AVOptionType_AV_OPT_TYPE_VIDEO_RATE,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AVPacketSideDataType {
    /// ATSC A53 Part 4 Closed Captions. This metadata should be associated with a video stream. A53 CC bitstream is stored as uint8_t in AVPacketSideData.data. The number of bytes of CC data is AVPacketSideData.size.
    A53Cc = AVPacketSideDataType_AV_PKT_DATA_A53_CC,
    /// Active Format Description data consisting of a single byte as specified in ETSI TS 101 154 using AVActiveFormatDescription enum.
    Afd = AVPacketSideDataType_AV_PKT_DATA_AFD,
    /// Ambient viewing environment metadata, as defined by H.274. This metadata should be associated with a video stream and contains data in the form of the AVAmbientViewingEnvironment struct.
    AmbientViewingEnvironment = AVPacketSideDataType_AV_PKT_DATA_AMBIENT_VIEWING_ENVIRONMENT,
    /// This side data should be associated with an audio stream and corresponds to enum AVAudioServiceType.
    AudioServiceType = AVPacketSideDataType_AV_PKT_DATA_AUDIO_SERVICE_TYPE,
    /// Content light level (based on CTA-861.3). This metadata should be associated with a video stream and contains data in the form of the AVContentLightMetadata struct.
    ContentLightLevel = AVPacketSideDataType_AV_PKT_DATA_CONTENT_LIGHT_LEVEL,
    /// This side data corresponds to the AVCPBProperties struct.
    CpbProperties = AVPacketSideDataType_AV_PKT_DATA_CPB_PROPERTIES,
    /// This side data contains a 3x3 transformation matrix describing an affine transformation that needs to be applied to the decoded video frames for correct presentation.
    DisplayMatrix = AVPacketSideDataType_AV_PKT_DATA_DISPLAYMATRIX,
    /// DOVI configuration ref: dolby-vision-bitstreams-within-the-iso-base-media-file-format-v2.1.2, section 2.2 dolby-vision-bitstreams-in-mpeg-2-transport-stream-multiplex-v1.2, section 3.3 Tags are stored in struct AVDOVIDecoderConfigurationRecord.
    DoviConf = AVPacketSideDataType_AV_PKT_DATA_DOVI_CONF,
    /// HDR10+ dynamic metadata associated with a video frame. The metadata is in the form of the AVDynamicHDRPlus struct and contains information for color volume transform - application 4 of SMPTE 2094-40:2016 standard.
    DynamicHdr10Plus = AVPacketSideDataType_AV_PKT_DATA_DYNAMIC_HDR10_PLUS,
    /// This side data contains encryption info for how to decrypt the packet. The format is not part of ABI, use av_encryption_info_* methods to access.
    EncryptionInfo = AVPacketSideDataType_AV_PKT_DATA_ENCRYPTION_INFO,
    /// This side data is encryption initialization data. The format is not part of ABI, use av_encryption_init_info_* methods to access.
    EncryptionInitInfo = AVPacketSideDataType_AV_PKT_DATA_ENCRYPTION_INIT_INFO,
    /// This side data contains an integer value representing the stream index of a "fallback" track. A fallback track indicates an alternate track to use when the current track can not be decoded for some reason. e.g. no decoder available for codec.
    FallbackTrack = AVPacketSideDataType_AV_PKT_DATA_FALLBACK_TRACK,
    /// The number of pixels to discard from the top/bottom/left/right border of the decoded frame to obtain the sub-rectangle intended for presentation.
    FrameCropping = AVPacketSideDataType_AV_PKT_DATA_FRAME_CROPPING,
    /// An AV_PKT_DATA_H263_MB_INFO side data packet contains a number of structures with info about macroblocks relevant to splitting the packet into smaller packets on macroblock edges (e.g. as for RFC 2190). That is, it does not necessarily contain info about all macroblocks, as long as the distance between macroblocks in the info is smaller than the target payload size. Each MB info structure is 12 bytes, and is laid out as follows:
    H263MbInfo = AVPacketSideDataType_AV_PKT_DATA_H263_MB_INFO,
    /// IAMF Demixing Info Parameter Data associated with the audio frame. This metadata is in the form of the AVIAMFParamDefinition struct and contains information defined in sections 3.6.1 and 3.8.2 of the Immersive Audio Model and Formats standard.
    IamfDemixingInfoParam = AVPacketSideDataType_AV_PKT_DATA_IAMF_DEMIXING_INFO_PARAM,
    /// IAMF Mix Gain Parameter Data associated with the audio frame. This metadata is in the form of the AVIAMFParamDefinition struct and contains information defined in sections 3.6.1 and 3.8.1 of the Immersive Audio Model and Formats standard.
    IamfMixGainParam = AVPacketSideDataType_AV_PKT_DATA_IAMF_MIX_GAIN_PARAM,

    /// IAMF Recon Gain Info Parameter Data associated with the audio frame. This metadata is in the form of the AVIAMFParamDefinition struct and contains information defined in sections 3.6.1 and 3.8.3 of the Immersive Audio Model and Formats standard.
    IamfReconGainInfoParam = AVPacketSideDataType_AV_PKT_DATA_IAMF_RECON_GAIN_INFO_PARAM,
    /// ICC profile data consisting of an opaque octet buffer following the format described by ISO 15076-1.
    IccProfile = AVPacketSideDataType_AV_PKT_DATA_ICC_PROFILE,
    /// An AV_PKT_DATA_JP_DUALMONO side data packet indicates that the packet may contain "dual mono" audio specific to Japanese DTV and if it is true, recommends only the selected channel to be used. @code u8 selected channels (0=main/left, 1=sub/right, 2=both) @endcode
    JpDualmono = AVPacketSideDataType_AV_PKT_DATA_JP_DUALMONO,
    /// Raw LCEVC payload data, as a uint8_t array, with NAL emulation bytes intact.
    Lcevc = AVPacketSideDataType_AV_PKT_DATA_LCEVC,
    /// Mastering display metadata (based on SMPTE-2086:2014). This metadata should be associated with a video stream and contains data in the form of the AVMasteringDisplayMetadata struct.
    MasteringDisplayMetadata = AVPacketSideDataType_AV_PKT_DATA_MASTERING_DISPLAY_METADATA,
    /// Data found in BlockAdditional element of matroska container. There is no end marker for the data, so it is required to rely on the side data size to recognize the end. 8 byte id (as found in BlockAddId) followed by data.
    MatroskaBlockadditional = AVPacketSideDataType_AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL,
    /// A list of zero terminated key/value strings. There is no end marker for the list, so it is required to rely on the side data size to stop. This side data includes updated metadata which appeared in the stream.
    MetadataUpdate = AVPacketSideDataType_AV_PKT_DATA_METADATA_UPDATE,
    /// MPEGTS stream ID as uint8_t, this is required to pass the stream ID information from the demuxer to the corresponding muxer.
    MpegtsStreamId = AVPacketSideDataType_AV_PKT_DATA_MPEGTS_STREAM_ID,
    /// The number of side data types. This is not part of the public API/ABI in the sense that it may change when new side data types are added. This must stay the last enum value. If its value becomes huge, some code using it needs to be updated as it assumes it to be smaller than other limits.
    Nb = AVPacketSideDataType_AV_PKT_DATA_NB,
    /// The AV_PKT_DATA_NEW_EXTRADATA is used to notify the codec or the format that the extradata buffer was changed and the receiving side should act upon it appropriately. The new extradata is embedded in the side data buffer and should be immediately used for processing the current frame or packet.
    NewExtradata = AVPacketSideDataType_AV_PKT_DATA_NEW_EXTRADATA,
    /// An AV_PKT_DATA_PALETTE side data packet contains exactly AVPALETTE_SIZE bytes worth of palette. This side data signals that a new palette is present.
    Palette = AVPacketSideDataType_AV_PKT_DATA_PALETTE,
    /// An AV_PKT_DATA_PARAM_CHANGE side data packet is laid out as follows: @code u32le param_flags if (param_flags & AV_SIDE_DATA_PARAM_CHANGE_SAMPLE_RATE) s32le sample_rate if (param_flags & AV_SIDE_DATA_PARAM_CHANGE_DIMENSIONS) s32le width s32le height @endcode
    ParamChange = AVPacketSideDataType_AV_PKT_DATA_PARAM_CHANGE,
    /// Producer Reference Time data corresponding to the AVProducerReferenceTime struct, usually exported by some encoders (on demand through the prft flag set in the AVCodecContext export_side_data field).
    Prft = AVPacketSideDataType_AV_PKT_DATA_PRFT,
    /// This side data contains quality related information from the encoder. @code u32le quality factor of the compressed frame. Allowed range is between 1 (good) and FF_LAMBDA_MAX (bad). u8 picture type u8 error count u16 reserved u64le[error count] sum of squared differences between encoder in and output @endcode
    QualityStats = AVPacketSideDataType_AV_PKT_DATA_QUALITY_STATS,
    /// This side data should be associated with an audio stream and contains ReplayGain information in form of the AVReplayGain struct.
    ReplayGain = AVPacketSideDataType_AV_PKT_DATA_REPLAYGAIN,
    /// Timecode which conforms to SMPTE ST 12-1:2014. The data is an array of 4 uint32_t where the first uint32_t describes how many (1-3) of the other timecodes are used. The timecode format is described in the documentation of av_timecode_get_smpte_from_framenum() function in libavutil/timecode.h.
    S12mTimecode = AVPacketSideDataType_AV_PKT_DATA_S12M_TIMECODE,
    /// Recommmends skipping the specified number of samples @code u32le number of samples to skip from start of this packet u32le number of samples to skip from end of this packet u8 reason for start skip u8 reason for end skip (0=padding silence, 1=convergence) @endcode
    SkipSamples = AVPacketSideDataType_AV_PKT_DATA_SKIP_SAMPLES,
    /// This side data should be associated with a video stream and corresponds to the AVSphericalMapping structure.
    Spherical = AVPacketSideDataType_AV_PKT_DATA_SPHERICAL,
    /// This side data should be associated with a video stream and contains Stereoscopic 3D information in form of the AVStereo3D struct.
    Stereo3d = AVPacketSideDataType_AV_PKT_DATA_STEREO3D,
    /// A list of zero terminated key/value strings. There is no end marker for the list, so it is required to rely on the side data size to stop.
    StringsMetadata = AVPacketSideDataType_AV_PKT_DATA_STRINGS_METADATA,
    /// Subtitle event position @code u32le x1 u32le y1 u32le x2 u32le y2 @endcode
    SubtitlePosition = AVPacketSideDataType_AV_PKT_DATA_SUBTITLE_POSITION,
    /// The optional first identifier line of a WebVTT cue.
    WebvttIdentifier = AVPacketSideDataType_AV_PKT_DATA_WEBVTT_IDENTIFIER,
    /// The optional settings (rendering instructions) that immediately follow the timestamp specifier of a WebVTT cue.
    WebvttSettings = AVPacketSideDataType_AV_PKT_DATA_WEBVTT_SETTINGS,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AVPictureStructure {
    /// coded as bottom field
    BottomField = AVPictureStructure_AV_PICTURE_STRUCTURE_BOTTOM_FIELD,
    /// coded as frame
    Frame = AVPictureStructure_AV_PICTURE_STRUCTURE_FRAME,
    /// coded as top field
    TopField = AVPictureStructure_AV_PICTURE_STRUCTURE_TOP_FIELD,
    /// unknown
    Unknown = AVPictureStructure_AV_PICTURE_STRUCTURE_UNKNOWN,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AVPictureType {
    /// Bi-dir predicted
    B = AVPictureType_AV_PICTURE_TYPE_B,
    /// BI type
    Bi = AVPictureType_AV_PICTURE_TYPE_BI,
    /// Intra
    I = AVPictureType_AV_PICTURE_TYPE_I,
    /// Undefined
    None = AVPictureType_AV_PICTURE_TYPE_NONE,
    /// Predicted
    P = AVPictureType_AV_PICTURE_TYPE_P,
    /// S(GMC)-VOP MPEG-4
    S = AVPictureType_AV_PICTURE_TYPE_S,
    /// Switching Intra
    Si = AVPictureType_AV_PICTURE_TYPE_SI,
    /// Switching Predicted
    Sp = AVPictureType_AV_PICTURE_TYPE_SP,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub enum AVPixelFormat {
    /// packed BGR 8:8:8, 32bpp, XBGRXBGR… X=unused/undefined
    ZeroBgr = AVPixelFormat_AV_PIX_FMT_0BGR,
    /// packed RGB 8:8:8, 32bpp, XRGBXRGB… X=unused/undefined
    ZeroRgb = AVPixelFormat_AV_PIX_FMT_0RGB,
    /// packed ABGR 8:8:8:8, 32bpp, ABGRABGR…
    Abgr = AVPixelFormat_AV_PIX_FMT_ABGR,
    /// packed ARGB 8:8:8:8, 32bpp, ARGBARGB…
    Argb = AVPixelFormat_AV_PIX_FMT_ARGB,
    /// packed AYUV 4:4:4,64bpp (1 Cr & Cb sample per 1x1 Y & A samples), big-endian
    Ayuv64be = AVPixelFormat_AV_PIX_FMT_AYUV64BE,
    /// packed AYUV 4:4:4,64bpp (1 Cr & Cb sample per 1x1 Y & A samples), little-endian
    Ayuv64le = AVPixelFormat_AV_PIX_FMT_AYUV64LE,
    /// bayer, BGBG..(odd line), GRGR..(even line), 8-bit samples
    BayerBggr8 = AVPixelFormat_AV_PIX_FMT_BAYER_BGGR8,
    /// bayer, BGBG..(odd line), GRGR..(even line), 16-bit samples, big-endian
    BayerBggr16be = AVPixelFormat_AV_PIX_FMT_BAYER_BGGR16BE,
    /// bayer, BGBG..(odd line), GRGR..(even line), 16-bit samples, little-endian
    BayerBggr16le = AVPixelFormat_AV_PIX_FMT_BAYER_BGGR16LE,
    /// bayer, GBGB..(odd line), RGRG..(even line), 8-bit samples
    BayerGbrg8 = AVPixelFormat_AV_PIX_FMT_BAYER_GBRG8,
    /// bayer, GBGB..(odd line), RGRG..(even line), 16-bit samples, big-endian
    BayerGbrg16be = AVPixelFormat_AV_PIX_FMT_BAYER_GBRG16BE,
    /// bayer, GBGB..(odd line), RGRG..(even line), 16-bit samples, little-endian
    BayerGbrg16le = AVPixelFormat_AV_PIX_FMT_BAYER_GBRG16LE,
    /// bayer, GRGR..(odd line), BGBG..(even line), 8-bit samples
    BayerGrbg8 = AVPixelFormat_AV_PIX_FMT_BAYER_GRBG8,
    /// bayer, GRGR..(odd line), BGBG..(even line), 16-bit samples, big-endian
    BayerGrbg16be = AVPixelFormat_AV_PIX_FMT_BAYER_GRBG16BE,
    /// bayer, GRGR..(odd line), BGBG..(even line), 16-bit samples, little-endian
    BayerGrbg16le = AVPixelFormat_AV_PIX_FMT_BAYER_GRBG16LE,
    /// bayer, RGRG..(odd line), GBGB..(even line), 8-bit samples
    BayerRggb8 = AVPixelFormat_AV_PIX_FMT_BAYER_RGGB8,
    /// bayer, RGRG..(odd line), GBGB..(even line), 16-bit samples, big-endian
    BayerRggb16be = AVPixelFormat_AV_PIX_FMT_BAYER_RGGB16BE,
    /// bayer, RGRG..(odd line), GBGB..(even line), 16-bit samples, little-endian
    BayerRggb16le = AVPixelFormat_AV_PIX_FMT_BAYER_RGGB16LE,
    /// packed BGR 8:8:8, 32bpp, BGRXBGRX… X=unused/undefined
    Bgr0 = AVPixelFormat_AV_PIX_FMT_BGR0,
    /// packed RGB 1:2:1 bitstream, 4bpp, (msb)1B 2G 1R(lsb), a byte contains two pixels, the first pixel in the byte is the one composed by the 4 msb bits
    Bgr4 = AVPixelFormat_AV_PIX_FMT_BGR4,
    /// packed RGB 3:3:2, 8bpp, (msb)2B 3G 3R(lsb)
    Bgr8 = AVPixelFormat_AV_PIX_FMT_BGR8,
    /// packed RGB 1:2:1, 8bpp, (msb)1B 2G 1R(lsb)
    Bgr4Byte = AVPixelFormat_AV_PIX_FMT_BGR4_BYTE,
    /// packed RGB 8:8:8, 24bpp, BGRBGR…
    Bgr24 = AVPixelFormat_AV_PIX_FMT_BGR24,
    /// packed RGB 16:16:16, 48bpp, 16B, 16G, 16R, the 2-byte value for each R/G/B component is stored as big-endian
    Bgr48be = AVPixelFormat_AV_PIX_FMT_BGR48BE,
    /// packed RGB 16:16:16, 48bpp, 16B, 16G, 16R, the 2-byte value for each R/G/B component is stored as little-endian
    Bgr48le = AVPixelFormat_AV_PIX_FMT_BGR48LE,
    /// packed BGR 4:4:4, 16bpp, (msb)4X 4B 4G 4R(lsb), big-endian, X=unused/undefined
    Bgr444be = AVPixelFormat_AV_PIX_FMT_BGR444BE,
    /// packed BGR 4:4:4, 16bpp, (msb)4X 4B 4G 4R(lsb), little-endian, X=unused/undefined
    Bgr444le = AVPixelFormat_AV_PIX_FMT_BGR444LE,
    /// packed BGR 5:5:5, 16bpp, (msb)1X 5B 5G 5R(lsb), big-endian , X=unused/undefined
    Bgr555be = AVPixelFormat_AV_PIX_FMT_BGR555BE,
    /// packed BGR 5:5:5, 16bpp, (msb)1X 5B 5G 5R(lsb), little-endian, X=unused/undefined
    Bgr555le = AVPixelFormat_AV_PIX_FMT_BGR555LE,
    /// packed BGR 5:6:5, 16bpp, (msb) 5B 6G 5R(lsb), big-endian
    Bgr565be = AVPixelFormat_AV_PIX_FMT_BGR565BE,
    /// packed BGR 5:6:5, 16bpp, (msb) 5B 6G 5R(lsb), little-endian
    Bgr565le = AVPixelFormat_AV_PIX_FMT_BGR565LE,
    /// packed BGRA 8:8:8:8, 32bpp, BGRABGRA…
    Bgra = AVPixelFormat_AV_PIX_FMT_BGRA,
    /// packed RGBA 16:16:16:16, 64bpp, 16B, 16G, 16R, 16A, the 2-byte value for each R/G/B/A component is stored as big-endian
    Bgra64be = AVPixelFormat_AV_PIX_FMT_BGRA64BE,
    /// packed RGBA 16:16:16:16, 64bpp, 16B, 16G, 16R, 16A, the 2-byte value for each R/G/B/A component is stored as little-endian
    Bgra64le = AVPixelFormat_AV_PIX_FMT_BGRA64LE,
    /// HW acceleration through CUDA. data[i] contain CUdeviceptr pointers exactly as for system memory frames.
    Cuda = AVPixelFormat_AV_PIX_FMT_CUDA,
    /// Hardware surfaces for Direct3D11.
    D3d11 = AVPixelFormat_AV_PIX_FMT_D3D11,
    /// Hardware surfaces for Direct3D 12.
    D3d12 = AVPixelFormat_AV_PIX_FMT_D3D12,
    /// HW decoding through Direct3D11 via old API, Picture.data[3] contains a ID3D11VideoDecoderOutputView pointer
    D3d11vaVld = AVPixelFormat_AV_PIX_FMT_D3D11VA_VLD,
    /// DRM-managed buffers exposed through PRIME buffer sharing.
    DrmPrime = AVPixelFormat_AV_PIX_FMT_DRM_PRIME,
    /// HW decoding through DXVA2, Picture.data[3] contains a LPDIRECT3DSURFACE9 pointer
    Dxva2Vld = AVPixelFormat_AV_PIX_FMT_DXVA2_VLD,
    /// planar GBRA 4:4:4:4 32bpp
    Gbrap = AVPixelFormat_AV_PIX_FMT_GBRAP,
    /// planar GBR 4:4:4:4 40bpp, big-endian
    Gbrap10be = AVPixelFormat_AV_PIX_FMT_GBRAP10BE,
    /// planar GBR 4:4:4:4 40bpp, little-endian
    Gbrap10le = AVPixelFormat_AV_PIX_FMT_GBRAP10LE,
    /// planar GBR 4:4:4:4 48bpp, big-endian
    Gbrap12be = AVPixelFormat_AV_PIX_FMT_GBRAP12BE,
    /// planar GBR 4:4:4:4 48bpp, little-endian
    Gbrap12le = AVPixelFormat_AV_PIX_FMT_GBRAP12LE,
    /// planar GBR 4:4:4:4 56bpp, big-endian
    Gbrap14be = AVPixelFormat_AV_PIX_FMT_GBRAP14BE,
    /// planar GBR 4:4:4:4 56bpp, little-endian
    Gbrap14le = AVPixelFormat_AV_PIX_FMT_GBRAP14LE,
    /// planar GBRA 4:4:4:4 64bpp, big-endian
    Gbrap16be = AVPixelFormat_AV_PIX_FMT_GBRAP16BE,
    /// planar GBRA 4:4:4:4 64bpp, little-endian
    Gbrap16le = AVPixelFormat_AV_PIX_FMT_GBRAP16LE,
    /// IEEE-754 single precision planar GBRA 4:4:4:4, 128bpp, big-endian
    Gbrapf32be = AVPixelFormat_AV_PIX_FMT_GBRAPF32BE,
    /// IEEE-754 single precision planar GBRA 4:4:4:4, 128bpp, little-endian
    Gbrapf32le = AVPixelFormat_AV_PIX_FMT_GBRAPF32LE,
    /// planar GBR 4:4:4 24bpp
    Gbrp = AVPixelFormat_AV_PIX_FMT_GBRP,
    /// planar GBR 4:4:4 27bpp, big-endian
    Gbrp9be = AVPixelFormat_AV_PIX_FMT_GBRP9BE,
    /// planar GBR 4:4:4 27bpp, little-endian
    Gbrp9le = AVPixelFormat_AV_PIX_FMT_GBRP9LE,
    /// planar GBR 4:4:4 30bpp, big-endian
    Gbrp10be = AVPixelFormat_AV_PIX_FMT_GBRP10BE,
    /// planar GBR 4:4:4 30bpp, little-endian
    Gbrp10le = AVPixelFormat_AV_PIX_FMT_GBRP10LE,
    /// planar GBR 4:4:4 36bpp, big-endian
    Gbrp12be = AVPixelFormat_AV_PIX_FMT_GBRP12BE,
    /// planar GBR 4:4:4 36bpp, little-endian
    Gbrp12le = AVPixelFormat_AV_PIX_FMT_GBRP12LE,
    /// planar GBR 4:4:4 42bpp, big-endian
    Gbrp14be = AVPixelFormat_AV_PIX_FMT_GBRP14BE,
    /// planar GBR 4:4:4 42bpp, little-endian
    Gbrp14le = AVPixelFormat_AV_PIX_FMT_GBRP14LE,
    /// planar GBR 4:4:4 48bpp, big-endian
    Gbrp16be = AVPixelFormat_AV_PIX_FMT_GBRP16BE,
    /// planar GBR 4:4:4 48bpp, little-endian
    Gbrp16le = AVPixelFormat_AV_PIX_FMT_GBRP16LE,
    /// IEEE-754 single precision planar GBR 4:4:4, 96bpp, big-endian
    Gbrpf32be = AVPixelFormat_AV_PIX_FMT_GBRPF32BE,
    /// IEEE-754 single precision planar GBR 4:4:4, 96bpp, little-endian
    Gbrpf32le = AVPixelFormat_AV_PIX_FMT_GBRPF32LE,
    /// Y , 8bpp
    Gray8 = AVPixelFormat_AV_PIX_FMT_GRAY8,
    /// Y , 9bpp, big-endian
    Gray9be = AVPixelFormat_AV_PIX_FMT_GRAY9BE,
    /// Y , 9bpp, little-endian
    Gray9le = AVPixelFormat_AV_PIX_FMT_GRAY9LE,
    /// Y , 10bpp, big-endian
    Gray10be = AVPixelFormat_AV_PIX_FMT_GRAY10BE,
    /// Y , 10bpp, little-endian
    Gray10le = AVPixelFormat_AV_PIX_FMT_GRAY10LE,
    /// Y , 12bpp, big-endian
    Gray12be = AVPixelFormat_AV_PIX_FMT_GRAY12BE,
    /// Y , 12bpp, little-endian
    Gray12le = AVPixelFormat_AV_PIX_FMT_GRAY12LE,
    /// Y , 14bpp, big-endian
    Gray14be = AVPixelFormat_AV_PIX_FMT_GRAY14BE,
    /// Y , 14bpp, little-endian
    Gray14le = AVPixelFormat_AV_PIX_FMT_GRAY14LE,
    /// Y , 16bpp, big-endian
    Gray16be = AVPixelFormat_AV_PIX_FMT_GRAY16BE,
    /// Y , 16bpp, little-endian
    Gray16le = AVPixelFormat_AV_PIX_FMT_GRAY16LE,
    /// IEEE-754 single precision Y, 32bpp, big-endian
    Grayf32be = AVPixelFormat_AV_PIX_FMT_GRAYF32BE,
    /// IEEE-754 single precision Y, 32bpp, little-endian
    Grayf32le = AVPixelFormat_AV_PIX_FMT_GRAYF32LE,
    /// hardware decoding through MediaCodec
    Mediacodec = AVPixelFormat_AV_PIX_FMT_MEDIACODEC,
    /// HW acceleration though MMAL, data[3] contains a pointer to the MMAL_BUFFER_HEADER_T structure.
    Mmal = AVPixelFormat_AV_PIX_FMT_MMAL,
    /// Y , 1bpp, 0 is black, 1 is white, in each byte pixels are ordered from the msb to the lsb
    Monoblack = AVPixelFormat_AV_PIX_FMT_MONOBLACK,
    /// Y , 1bpp, 0 is white, 1 is black, in each byte pixels are ordered from the msb to the lsb
    Monowhite = AVPixelFormat_AV_PIX_FMT_MONOWHITE,
    None = AVPixelFormat_AV_PIX_FMT_NONE,
    /// planar YUV 4:2:0, 12bpp, 1 plane for Y and 1 plane for the UV components, which are interleaved (first byte U and the following byte V)
    Nv12 = AVPixelFormat_AV_PIX_FMT_NV12,
    /// interleaved chroma YUV 4:2:2, 16bpp, (1 Cr & Cb sample per 2x1 Y samples)
    Nv16 = AVPixelFormat_AV_PIX_FMT_NV16,
    /// as above, but U and V bytes are swapped
    Nv21 = AVPixelFormat_AV_PIX_FMT_NV21,
    /// planar YUV 4:4:4, 24bpp, 1 plane for Y and 1 plane for the UV components, which are interleaved (first byte U and the following byte V)
    Nv24 = AVPixelFormat_AV_PIX_FMT_NV24,
    /// as above, but U and V bytes are swapped
    Nv42 = AVPixelFormat_AV_PIX_FMT_NV42,
    /// interleaved chroma YUV 4:2:2, 20bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian
    Nv20be = AVPixelFormat_AV_PIX_FMT_NV20BE,
    /// interleaved chroma YUV 4:2:2, 20bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian
    Nv20le = AVPixelFormat_AV_PIX_FMT_NV20LE,
    /// Hardware surfaces for OpenCL.
    Opencl = AVPixelFormat_AV_PIX_FMT_OPENCL,
    /// like NV12, with 10bpp per component, data in the high bits, zeros in the low bits, big-endian
    P010be = AVPixelFormat_AV_PIX_FMT_P010BE,
    /// like NV12, with 10bpp per component, data in the high bits, zeros in the low bits, little-endian
    P010le = AVPixelFormat_AV_PIX_FMT_P010LE,
    /// like NV12, with 12bpp per component, data in the high bits, zeros in the low bits, big-endian
    P012be = AVPixelFormat_AV_PIX_FMT_P012BE,
    /// like NV12, with 12bpp per component, data in the high bits, zeros in the low bits, little-endian
    P012le = AVPixelFormat_AV_PIX_FMT_P012LE,
    /// like NV12, with 16bpp per component, big-endian
    P016be = AVPixelFormat_AV_PIX_FMT_P016BE,
    /// like NV12, with 16bpp per component, little-endian
    P016le = AVPixelFormat_AV_PIX_FMT_P016LE,
    /// interleaved chroma YUV 4:2:2, 20bpp, data in the high bits, big-endian
    P210be = AVPixelFormat_AV_PIX_FMT_P210BE,
    /// interleaved chroma YUV 4:2:2, 20bpp, data in the high bits, little-endian
    P210le = AVPixelFormat_AV_PIX_FMT_P210LE,
    /// interleaved chroma YUV 4:2:2, 24bpp, data in the high bits, big-endian
    P212be = AVPixelFormat_AV_PIX_FMT_P212BE,
    /// interleaved chroma YUV 4:2:2, 24bpp, data in the high bits, little-endian
    P212le = AVPixelFormat_AV_PIX_FMT_P212LE,
    /// interleaved chroma YUV 4:2:2, 32bpp, big-endian
    P216be = AVPixelFormat_AV_PIX_FMT_P216BE,
    /// interleaved chroma YUV 4:2:2, 32bpp, little-endian
    P216le = AVPixelFormat_AV_PIX_FMT_P216LE,
    /// interleaved chroma YUV 4:4:4, 30bpp, data in the high bits, big-endian
    P410be = AVPixelFormat_AV_PIX_FMT_P410BE,
    /// interleaved chroma YUV 4:4:4, 30bpp, data in the high bits, little-endian
    P410le = AVPixelFormat_AV_PIX_FMT_P410LE,
    /// interleaved chroma YUV 4:4:4, 36bpp, data in the high bits, big-endian
    P412be = AVPixelFormat_AV_PIX_FMT_P412BE,
    /// interleaved chroma YUV 4:4:4, 36bpp, data in the high bits, little-endian
    P412le = AVPixelFormat_AV_PIX_FMT_P412LE,
    /// interleaved chroma YUV 4:4:4, 48bpp, big-endian
    P416be = AVPixelFormat_AV_PIX_FMT_P416BE,
    /// interleaved chroma YUV 4:4:4, 48bpp, little-endian
    P416le = AVPixelFormat_AV_PIX_FMT_P416LE,
    /// 8 bits with AV_PIX_FMT_RGB32 palette
    Pal8 = AVPixelFormat_AV_PIX_FMT_PAL8,
    /// HW acceleration through QSV, data[3] contains a pointer to the mfxFrameSurface1 structure.
    Qsv = AVPixelFormat_AV_PIX_FMT_QSV,
    /// packed RGB 8:8:8, 32bpp, RGBXRGBX… X=unused/undefined
    Rgb0 = AVPixelFormat_AV_PIX_FMT_RGB0,
    /// packed RGB 1:2:1 bitstream, 4bpp, (msb)1R 2G 1B(lsb), a byte contains two pixels, the first pixel in the byte is the one composed by the 4 msb bits
    Rgb4 = AVPixelFormat_AV_PIX_FMT_RGB4,
    /// packed RGB 3:3:2, 8bpp, (msb)3R 3G 2B(lsb)
    Rgb8 = AVPixelFormat_AV_PIX_FMT_RGB8,
    /// packed RGB 1:2:1, 8bpp, (msb)1R 2G 1B(lsb)
    Rgb4Byte = AVPixelFormat_AV_PIX_FMT_RGB4_BYTE,
    /// packed RGB 8:8:8, 24bpp, RGBRGB…
    Rgb24 = AVPixelFormat_AV_PIX_FMT_RGB24,
    /// packed RGB 16:16:16, 48bpp, 16R, 16G, 16B, the 2-byte value for each R/G/B component is stored as big-endian
    Rgb48be = AVPixelFormat_AV_PIX_FMT_RGB48BE,
    /// packed RGB 16:16:16, 48bpp, 16R, 16G, 16B, the 2-byte value for each R/G/B component is stored as little-endian
    Rgb48le = AVPixelFormat_AV_PIX_FMT_RGB48LE,
    /// packed RGB 4:4:4, 16bpp, (msb)4X 4R 4G 4B(lsb), big-endian, X=unused/undefined
    Rgb444be = AVPixelFormat_AV_PIX_FMT_RGB444BE,
    /// packed RGB 4:4:4, 16bpp, (msb)4X 4R 4G 4B(lsb), little-endian, X=unused/undefined
    Rgb444le = AVPixelFormat_AV_PIX_FMT_RGB444LE,
    /// packed RGB 5:5:5, 16bpp, (msb)1X 5R 5G 5B(lsb), big-endian , X=unused/undefined
    Rgb555be = AVPixelFormat_AV_PIX_FMT_RGB555BE,
    /// packed RGB 5:5:5, 16bpp, (msb)1X 5R 5G 5B(lsb), little-endian, X=unused/undefined
    Rgb555le = AVPixelFormat_AV_PIX_FMT_RGB555LE,
    /// packed RGB 5:6:5, 16bpp, (msb) 5R 6G 5B(lsb), big-endian
    Rgb565be = AVPixelFormat_AV_PIX_FMT_RGB565BE,
    /// packed RGB 5:6:5, 16bpp, (msb) 5R 6G 5B(lsb), little-endian
    Rgb565le = AVPixelFormat_AV_PIX_FMT_RGB565LE,
    /// packed RGBA 8:8:8:8, 32bpp, RGBARGBA…
    Rgba = AVPixelFormat_AV_PIX_FMT_RGBA,
    /// packed RGBA 16:16:16:16, 64bpp, 16R, 16G, 16B, 16A, the 2-byte value for each R/G/B/A component is stored as big-endian
    Rgba64be = AVPixelFormat_AV_PIX_FMT_RGBA64BE,
    /// packed RGBA 16:16:16:16, 64bpp, 16R, 16G, 16B, 16A, the 2-byte value for each R/G/B/A component is stored as little-endian
    Rgba64le = AVPixelFormat_AV_PIX_FMT_RGBA64LE,
    /// IEEE-754 half precision packed RGBA 16:16:16:16, 64bpp, RGBARGBA…, big-endian
    Rgbaf16be = AVPixelFormat_AV_PIX_FMT_RGBAF16BE,
    /// IEEE-754 half precision packed RGBA 16:16:16:16, 64bpp, RGBARGBA…, little-endian
    Rgbaf16le = AVPixelFormat_AV_PIX_FMT_RGBAF16LE,
    /// IEEE-754 single precision packed RGBA 32:32:32:32, 128bpp, RGBARGBA…, big-endian
    Rgbaf32be = AVPixelFormat_AV_PIX_FMT_RGBAF32BE,
    /// IEEE-754 single precision packed RGBA 32:32:32:32, 128bpp, RGBARGBA…, little-endian
    Rgbaf32le = AVPixelFormat_AV_PIX_FMT_RGBAF32LE,
    /// IEEE-754 single precision packed RGB 32:32:32, 96bpp, RGBRGB…, big-endian
    Rgbf32be = AVPixelFormat_AV_PIX_FMT_RGBF32BE,
    /// IEEE-754 single precision packed RGB 32:32:32, 96bpp, RGBRGB…, little-endian
    Rgbf32le = AVPixelFormat_AV_PIX_FMT_RGBF32LE,
    /// packed YUV 4:2:2, 16bpp, Cb Y0 Cr Y1
    Uyvy422 = AVPixelFormat_AV_PIX_FMT_UYVY422,
    /// packed YUV 4:1:1, 12bpp, Cb Y0 Y1 Cr Y2 Y3
    Uyyvyy411 = AVPixelFormat_AV_PIX_FMT_UYYVYY411,
    /// Hardware acceleration through VA-API, data[3] contains a VASurfaceID.
    Vaapi = AVPixelFormat_AV_PIX_FMT_VAAPI,
    /// HW acceleration through VDPAU, Picture.data[3] contains a VdpVideoSurface
    Vdpau = AVPixelFormat_AV_PIX_FMT_VDPAU,
    /// hardware decoding through Videotoolbox
    Videotoolbox = AVPixelFormat_AV_PIX_FMT_VIDEOTOOLBOX,
    /// Vulkan hardware images.
    Vulkan = AVPixelFormat_AV_PIX_FMT_VULKAN,
    /// packed VUYA 4:4:4, 32bpp, VUYAVUYA…
    Vuya = AVPixelFormat_AV_PIX_FMT_VUYA,
    /// packed VUYX 4:4:4, 32bpp, Variant of VUYA where alpha channel is left undefined
    Vuyx = AVPixelFormat_AV_PIX_FMT_VUYX,
    /// packed BGR 10:10:10, 30bpp, (msb)2X 10B 10G 10R(lsb), big-endian, X=unused/undefined
    X2bgr10be = AVPixelFormat_AV_PIX_FMT_X2BGR10BE,
    /// packed BGR 10:10:10, 30bpp, (msb)2X 10B 10G 10R(lsb), little-endian, X=unused/undefined
    X2bgr10le = AVPixelFormat_AV_PIX_FMT_X2BGR10LE,
    /// packed RGB 10:10:10, 30bpp, (msb)2X 10R 10G 10B(lsb), big-endian, X=unused/undefined
    X2rgb10be = AVPixelFormat_AV_PIX_FMT_X2RGB10BE,
    /// packed RGB 10:10:10, 30bpp, (msb)2X 10R 10G 10B(lsb), little-endian, X=unused/undefined
    X2rgb10le = AVPixelFormat_AV_PIX_FMT_X2RGB10LE,
    /// packed XVYU 4:4:4, 32bpp, (msb)2X 10V 10Y 10U(lsb), big-endian, variant of Y410 where alpha channel is left undefined
    Xv30be = AVPixelFormat_AV_PIX_FMT_XV30BE,
    /// packed XVYU 4:4:4, 32bpp, (msb)2X 10V 10Y 10U(lsb), little-endian, variant of Y410 where alpha channel is left undefined
    Xv30le = AVPixelFormat_AV_PIX_FMT_XV30LE,
    /// packed XVYU 4:4:4, 48bpp, data in the high bits, zeros in the low bits, big-endian, variant of Y412 where alpha channel is left undefined
    Xv36be = AVPixelFormat_AV_PIX_FMT_XV36BE,
    /// packed XVYU 4:4:4, 48bpp, data in the high bits, zeros in the low bits, little-endian, variant of Y412 where alpha channel is left undefined
    Xv36le = AVPixelFormat_AV_PIX_FMT_XV36LE,
    /// packed XYZ 4:4:4, 36 bpp, (msb) 12X, 12Y, 12Z (lsb), the 2-byte value for each X/Y/Z is stored as big-endian, the 4 lower bits are set to 0
    Xyz12be = AVPixelFormat_AV_PIX_FMT_XYZ12BE,
    /// packed XYZ 4:4:4, 36 bpp, (msb) 12X, 12Y, 12Z (lsb), the 2-byte value for each X/Y/Z is stored as little-endian, the 4 lower bits are set to 0
    Xyz12le = AVPixelFormat_AV_PIX_FMT_XYZ12LE,
    /// packed YUV 4:2:2 like YUYV422, 20bpp, data in the high bits, big-endian
    Y210be = AVPixelFormat_AV_PIX_FMT_Y210BE,
    /// packed YUV 4:2:2 like YUYV422, 20bpp, data in the high bits, little-endian
    Y210le = AVPixelFormat_AV_PIX_FMT_Y210LE,
    /// packed YUV 4:2:2 like YUYV422, 24bpp, data in the high bits, zeros in the low bits, big-endian
    Y212be = AVPixelFormat_AV_PIX_FMT_Y212BE,
    /// packed YUV 4:2:2 like YUYV422, 24bpp, data in the high bits, zeros in the low bits, little-endian
    Y212le = AVPixelFormat_AV_PIX_FMT_Y212LE,
    /// 8 bits gray, 8 bits alpha
    Ya8 = AVPixelFormat_AV_PIX_FMT_YA8,
    /// 16 bits gray, 16 bits alpha (big-endian)
    Ya16be = AVPixelFormat_AV_PIX_FMT_YA16BE,
    /// 16 bits gray, 16 bits alpha (little-endian)
    Ya16le = AVPixelFormat_AV_PIX_FMT_YA16LE,
    /// planar YUV 4:1:0, 9bpp, (1 Cr & Cb sample per 4x4 Y samples)
    Yuv410p = AVPixelFormat_AV_PIX_FMT_YUV410P,
    /// planar YUV 4:1:1, 12bpp, (1 Cr & Cb sample per 4x1 Y samples)
    Yuv411p = AVPixelFormat_AV_PIX_FMT_YUV411P,
    /// planar YUV 4:2:0, 12bpp, (1 Cr & Cb sample per 2x2 Y samples)
    Yuv420p = AVPixelFormat_AV_PIX_FMT_YUV420P,
    /// planar YUV 4:2:0, 13.5bpp, (1 Cr & Cb sample per 2x2 Y samples), big-endian
    Yuv420p9be = AVPixelFormat_AV_PIX_FMT_YUV420P9BE,
    /// planar YUV 4:2:0, 13.5bpp, (1 Cr & Cb sample per 2x2 Y samples), little-endian
    Yuv420p9le = AVPixelFormat_AV_PIX_FMT_YUV420P9LE,
    /// planar YUV 4:2:0, 15bpp, (1 Cr & Cb sample per 2x2 Y samples), big-endian
    Yuv420p10be = AVPixelFormat_AV_PIX_FMT_YUV420P10BE,
    /// planar YUV 4:2:0, 15bpp, (1 Cr & Cb sample per 2x2 Y samples), little-endian
    Yuv420p10le = AVPixelFormat_AV_PIX_FMT_YUV420P10LE,
    /// planar YUV 4:2:0,18bpp, (1 Cr & Cb sample per 2x2 Y samples), big-endian
    Yuv420p12be = AVPixelFormat_AV_PIX_FMT_YUV420P12BE,
    /// planar YUV 4:2:0,18bpp, (1 Cr & Cb sample per 2x2 Y samples), little-endian
    Yuv420p12le = AVPixelFormat_AV_PIX_FMT_YUV420P12LE,
    /// planar YUV 4:2:0,21bpp, (1 Cr & Cb sample per 2x2 Y samples), big-endian
    Yuv420p14be = AVPixelFormat_AV_PIX_FMT_YUV420P14BE,
    /// planar YUV 4:2:0,21bpp, (1 Cr & Cb sample per 2x2 Y samples), little-endian
    Yuv420p14le = AVPixelFormat_AV_PIX_FMT_YUV420P14LE,
    /// planar YUV 4:2:0, 24bpp, (1 Cr & Cb sample per 2x2 Y samples), big-endian
    Yuv420p16be = AVPixelFormat_AV_PIX_FMT_YUV420P16BE,
    /// planar YUV 4:2:0, 24bpp, (1 Cr & Cb sample per 2x2 Y samples), little-endian
    Yuv420p16le = AVPixelFormat_AV_PIX_FMT_YUV420P16LE,
    /// planar YUV 4:2:2, 16bpp, (1 Cr & Cb sample per 2x1 Y samples)
    Yuv422p = AVPixelFormat_AV_PIX_FMT_YUV422P,
    /// planar YUV 4:2:2, 18bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian
    Yuv422p9be = AVPixelFormat_AV_PIX_FMT_YUV422P9BE,
    /// planar YUV 4:2:2, 18bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian
    Yuv422p9le = AVPixelFormat_AV_PIX_FMT_YUV422P9LE,
    /// planar YUV 4:2:2, 20bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian
    Yuv422p10be = AVPixelFormat_AV_PIX_FMT_YUV422P10BE,
    /// planar YUV 4:2:2, 20bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian
    Yuv422p10le = AVPixelFormat_AV_PIX_FMT_YUV422P10LE,
    /// planar YUV 4:2:2,24bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian
    Yuv422p12be = AVPixelFormat_AV_PIX_FMT_YUV422P12BE,
    /// planar YUV 4:2:2,24bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian
    Yuv422p12le = AVPixelFormat_AV_PIX_FMT_YUV422P12LE,
    /// planar YUV 4:2:2,28bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian
    Yuv422p14be = AVPixelFormat_AV_PIX_FMT_YUV422P14BE,
    /// planar YUV 4:2:2,28bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian
    Yuv422p14le = AVPixelFormat_AV_PIX_FMT_YUV422P14LE,
    /// planar YUV 4:2:2, 32bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian
    Yuv422p16be = AVPixelFormat_AV_PIX_FMT_YUV422P16BE,
    /// planar YUV 4:2:2, 32bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian
    Yuv422p16le = AVPixelFormat_AV_PIX_FMT_YUV422P16LE,
    /// planar YUV 4:4:0 (1 Cr & Cb sample per 1x2 Y samples)
    Yuv440p = AVPixelFormat_AV_PIX_FMT_YUV440P,
    /// planar YUV 4:4:0,20bpp, (1 Cr & Cb sample per 1x2 Y samples), big-endian
    Yuv440p10be = AVPixelFormat_AV_PIX_FMT_YUV440P10BE,
    /// planar YUV 4:4:0,20bpp, (1 Cr & Cb sample per 1x2 Y samples), little-endian
    Yuv440p10le = AVPixelFormat_AV_PIX_FMT_YUV440P10LE,
    /// planar YUV 4:4:0,24bpp, (1 Cr & Cb sample per 1x2 Y samples), big-endian
    Yuv440p12be = AVPixelFormat_AV_PIX_FMT_YUV440P12BE,
    /// planar YUV 4:4:0,24bpp, (1 Cr & Cb sample per 1x2 Y samples), little-endian
    Yuv440p12le = AVPixelFormat_AV_PIX_FMT_YUV440P12LE,
    /// planar YUV 4:4:4, 24bpp, (1 Cr & Cb sample per 1x1 Y samples)
    Yuv444p = AVPixelFormat_AV_PIX_FMT_YUV444P,
    /// planar YUV 4:4:4, 27bpp, (1 Cr & Cb sample per 1x1 Y samples), big-endian
    Yuv444p9be = AVPixelFormat_AV_PIX_FMT_YUV444P9BE,
    /// planar YUV 4:4:4, 27bpp, (1 Cr & Cb sample per 1x1 Y samples), little-endian
    Yuv444p9le = AVPixelFormat_AV_PIX_FMT_YUV444P9LE,
    /// planar YUV 4:4:4, 30bpp, (1 Cr & Cb sample per 1x1 Y samples), big-endian
    Yuv444p10be = AVPixelFormat_AV_PIX_FMT_YUV444P10BE,
    /// planar YUV 4:4:4, 30bpp, (1 Cr & Cb sample per 1x1 Y samples), little-endian
    Yuv444p10le = AVPixelFormat_AV_PIX_FMT_YUV444P10LE,
    /// planar YUV 4:4:4,36bpp, (1 Cr & Cb sample per 1x1 Y samples), big-endian
    Yuv444p12be = AVPixelFormat_AV_PIX_FMT_YUV444P12BE,
    /// planar YUV 4:4:4,36bpp, (1 Cr & Cb sample per 1x1 Y samples), little-endian
    Yuv444p12le = AVPixelFormat_AV_PIX_FMT_YUV444P12LE,
    /// planar YUV 4:4:4,42bpp, (1 Cr & Cb sample per 1x1 Y samples), big-endian
    Yuv444p14be = AVPixelFormat_AV_PIX_FMT_YUV444P14BE,
    /// planar YUV 4:4:4,42bpp, (1 Cr & Cb sample per 1x1 Y samples), little-endian
    Yuv444p14le = AVPixelFormat_AV_PIX_FMT_YUV444P14LE,
    /// planar YUV 4:4:4, 48bpp, (1 Cr & Cb sample per 1x1 Y samples), big-endian
    Yuv444p16be = AVPixelFormat_AV_PIX_FMT_YUV444P16BE,
    /// planar YUV 4:4:4, 48bpp, (1 Cr & Cb sample per 1x1 Y samples), little-endian
    Yuv444p16le = AVPixelFormat_AV_PIX_FMT_YUV444P16LE,
    /// planar YUV 4:2:0, 20bpp, (1 Cr & Cb sample per 2x2 Y & A samples)
    Yuva420p = AVPixelFormat_AV_PIX_FMT_YUVA420P,
    /// planar YUV 4:2:0 22.5bpp, (1 Cr & Cb sample per 2x2 Y & A samples), big-endian
    Yuva420p9be = AVPixelFormat_AV_PIX_FMT_YUVA420P9BE,
    /// planar YUV 4:2:0 22.5bpp, (1 Cr & Cb sample per 2x2 Y & A samples), little-endian
    Yuva420p9le = AVPixelFormat_AV_PIX_FMT_YUVA420P9LE,
    /// planar YUV 4:2:0 25bpp, (1 Cr & Cb sample per 2x2 Y & A samples, big-endian)
    Yuva420p10be = AVPixelFormat_AV_PIX_FMT_YUVA420P10BE,
    /// planar YUV 4:2:0 25bpp, (1 Cr & Cb sample per 2x2 Y & A samples, little-endian)
    Yuva420p10le = AVPixelFormat_AV_PIX_FMT_YUVA420P10LE,
    /// planar YUV 4:2:0 40bpp, (1 Cr & Cb sample per 2x2 Y & A samples, big-endian)
    Yuva420p16be = AVPixelFormat_AV_PIX_FMT_YUVA420P16BE,
    /// planar YUV 4:2:0 40bpp, (1 Cr & Cb sample per 2x2 Y & A samples, little-endian)
    Yuva420p16le = AVPixelFormat_AV_PIX_FMT_YUVA420P16LE,
    /// planar YUV 4:2:2 24bpp, (1 Cr & Cb sample per 2x1 Y & A samples)
    Yuva422p = AVPixelFormat_AV_PIX_FMT_YUVA422P,
    /// planar YUV 4:2:2 27bpp, (1 Cr & Cb sample per 2x1 Y & A samples), big-endian
    Yuva422p9be = AVPixelFormat_AV_PIX_FMT_YUVA422P9BE,
    /// planar YUV 4:2:2 27bpp, (1 Cr & Cb sample per 2x1 Y & A samples), little-endian
    Yuva422p9le = AVPixelFormat_AV_PIX_FMT_YUVA422P9LE,
    /// planar YUV 4:2:2 30bpp, (1 Cr & Cb sample per 2x1 Y & A samples, big-endian)
    Yuva422p10be = AVPixelFormat_AV_PIX_FMT_YUVA422P10BE,
    /// planar YUV 4:2:2 30bpp, (1 Cr & Cb sample per 2x1 Y & A samples, little-endian)
    Yuva422p10le = AVPixelFormat_AV_PIX_FMT_YUVA422P10LE,
    /// planar YUV 4:2:2,24bpp, (1 Cr & Cb sample per 2x1 Y samples), 12b alpha, big-endian
    Yuva422p12be = AVPixelFormat_AV_PIX_FMT_YUVA422P12BE,
    /// < planar YUV 4:2:2,24bpp, (1 Cr & Cb sample per 2x1 Y samples), 12b alpha, little-endian
    Yuva422p12le = AVPixelFormat_AV_PIX_FMT_YUVA422P12LE,
    /// planar YUV 4:2:2 48bpp, (1 Cr & Cb sample per 2x1 Y & A samples, big-endian)
    Yuva422p16be = AVPixelFormat_AV_PIX_FMT_YUVA422P16BE,
    /// planar YUV 4:2:2 48bpp, (1 Cr & Cb sample per 2x1 Y & A samples, little-endian)
    Yuva422p16le = AVPixelFormat_AV_PIX_FMT_YUVA422P16LE,
    /// planar YUV 4:4:4 32bpp, (1 Cr & Cb sample per 1x1 Y & A samples)
    Yuva444p = AVPixelFormat_AV_PIX_FMT_YUVA444P,
    /// planar YUV 4:4:4 36bpp, (1 Cr & Cb sample per 1x1 Y & A samples), big-endian
    Yuva444p9be = AVPixelFormat_AV_PIX_FMT_YUVA444P9BE,
    /// planar YUV 4:4:4 36bpp, (1 Cr & Cb sample per 1x1 Y & A samples), little-endian
    Yuva444p9le = AVPixelFormat_AV_PIX_FMT_YUVA444P9LE,
    /// planar YUV 4:4:4 40bpp, (1 Cr & Cb sample per 1x1 Y & A samples, big-endian)
    Yuva444p10be = AVPixelFormat_AV_PIX_FMT_YUVA444P10BE,
    /// planar YUV 4:4:4 40bpp, (1 Cr & Cb sample per 1x1 Y & A samples, little-endian)
    Yuva444p10le = AVPixelFormat_AV_PIX_FMT_YUVA444P10LE,
    /// planar YUV 4:4:4,36bpp, (1 Cr & Cb sample per 1x1 Y samples), 12b alpha, big-endian
    Yuva444p12be = AVPixelFormat_AV_PIX_FMT_YUVA444P12BE,
    /// planar YUV 4:4:4,36bpp, (1 Cr & Cb sample per 1x1 Y samples), 12b alpha, little-endian
    Yuva444p12le = AVPixelFormat_AV_PIX_FMT_YUVA444P12LE,
    /// planar YUV 4:4:4 64bpp, (1 Cr & Cb sample per 1x1 Y & A samples, big-endian)
    Yuva444p16be = AVPixelFormat_AV_PIX_FMT_YUVA444P16BE,
    /// planar YUV 4:4:4 64bpp, (1 Cr & Cb sample per 1x1 Y & A samples, little-endian)
    Yuva444p16le = AVPixelFormat_AV_PIX_FMT_YUVA444P16LE,
    /// planar YUV 4:1:1, 12bpp, (1 Cr & Cb sample per 4x1 Y samples) full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV411P and setting color_range
    Yuvj411p = AVPixelFormat_AV_PIX_FMT_YUVJ411P,
    /// planar YUV 4:2:0, 12bpp, full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV420P and setting color_range
    Yuvj420p = AVPixelFormat_AV_PIX_FMT_YUVJ420P,
    /// planar YUV 4:2:2, 16bpp, full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV422P and setting color_range
    Yuvj422p = AVPixelFormat_AV_PIX_FMT_YUVJ422P,
    /// planar YUV 4:4:0 full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV440P and setting color_range
    Yuvj440p = AVPixelFormat_AV_PIX_FMT_YUVJ440P,
    /// planar YUV 4:4:4, 24bpp, full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV444P and setting color_range
    Yuvj444p = AVPixelFormat_AV_PIX_FMT_YUVJ444P,
    /// packed YUV 4:2:2, 16bpp, Y0 Cb Y1 Cr
    Yuyv422 = AVPixelFormat_AV_PIX_FMT_YUYV422,
    /// packed YUV 4:2:2, 16bpp, Y0 Cr Y1 Cb
    Yvyu422 = AVPixelFormat_AV_PIX_FMT_YVYU422,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AVSampleFormat {
    /// double
    Dbl = AVSampleFormat_AV_SAMPLE_FMT_DBL,
    /// double, planar
    Dblp = AVSampleFormat_AV_SAMPLE_FMT_DBLP,
    /// float
    Flt = AVSampleFormat_AV_SAMPLE_FMT_FLT,
    /// float, planar
    Fltp = AVSampleFormat_AV_SAMPLE_FMT_FLTP,
    /// Number of sample formats. DO NOT USE if linking dynamically
    Nb = AVSampleFormat_AV_SAMPLE_FMT_NB,
    /// None
    None = AVSampleFormat_AV_SAMPLE_FMT_NONE,
    /// signed 16 bits
    S16 = AVSampleFormat_AV_SAMPLE_FMT_S16,
    /// signed 32 bits
    S32 = AVSampleFormat_AV_SAMPLE_FMT_S32,
    /// signed 64 bits
    S64 = AVSampleFormat_AV_SAMPLE_FMT_S64,
    /// signed 16 bits, planar
    S16p = AVSampleFormat_AV_SAMPLE_FMT_S16P,
    /// signed 32 bits, planar
    S32p = AVSampleFormat_AV_SAMPLE_FMT_S32P,
    /// signed 64 bits, planar
    S64p = AVSampleFormat_AV_SAMPLE_FMT_S64P,
    /// unsigned 8 bits
    U8 = AVSampleFormat_AV_SAMPLE_FMT_U8,
    /// unsigned 8 bits, planar
    U8p = AVSampleFormat_AV_SAMPLE_FMT_U8P,
}

pub const EAGAIN: i32 = -11;
