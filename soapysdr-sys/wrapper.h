#include <stdbool.h>
#include <SoapySDR/Device.h>
#include <SoapySDR/Logger.h>
#include <SoapySDR/Formats.h>
#include <SoapySDR/Modules.h>
#include <SoapySDR/Time.h>
#include <SoapySDR/Version.h>

int _rust_wrapper_SoapySDRDevice_setupStream(
    SoapySDRDevice *device,
    SoapySDRStream **out_stream,
    int direction,
    char const* format,
    size_t const*channels,
    size_t numChans,
    SoapySDRKwargs const* args);

#if SOAPY_SDR_API_VERSION < 0x00080000
void SoapySDR_free(void *ptr);
#endif
