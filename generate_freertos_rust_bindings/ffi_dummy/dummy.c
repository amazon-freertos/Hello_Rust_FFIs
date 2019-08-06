// Ensure that all FreeRTOS header files are included.
#include "FreeRTOS.h"
#include "atomic.h"
#include "deprecated_definitions.h"
#include "event_groups.h"
#include "list.h"
#include "message_buffer.h"
//#include "mpu_prototypes.h"
//#include "mpu_wrappers.h"
#include "portable.h"
#include "projdefs.h"
#include "queue.h"
#include "semphr.h"
#include "stack_macros.h"
#include "stream_buffer.h"
#include "task.h"
#include "timers.h"

// Note: This isn't a real program.
// Its only purpose is to generate a list of FreeRTOS methods
// which we can make available to Rust, hence all the includes.
// So instead of running the compiler (gcc), we'll run
// the preprocessor (cpp). There's no C++ involved,
// it's just an acronym for 'C PreProcessor'.
// Because this wasn't confusing enough already...
int main() {}
