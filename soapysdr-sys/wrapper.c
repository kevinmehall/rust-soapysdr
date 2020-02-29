#include "wrapper.h"

int _rust_wrapper_SoapySDRDevice_setupStream(
    SoapySDRDevice *device,
    SoapySDRStream **out_stream,
    int direction,
    char const* format,
    size_t const*channels,
    size_t numChans,
    SoapySDRKwargs const* args)
{
#if SOAPY_SDR_API_VERSION < 0x00080000
    return SoapySDRDevice_setupStream(device, out_stream, direction, format, channels, numChans, args);
#else
    *out_stream = SoapySDRDevice_setupStream(device, direction, format, channels, numChans, args);
    return SoapySDRDevice_lastStatus();
#endif
}