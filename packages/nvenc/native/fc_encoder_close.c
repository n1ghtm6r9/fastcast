#include <stdlib.h>

#include "fc_internal.h"

void fc_encoder_close(fc_encoder* encoder)
{
    if (encoder == NULL) {
        return;
    }

    free(encoder);
}
