#include "fc_internal.h"

fc_result fc_reserve_output(fc_encoder* encoder, uint32_t size)
{
    size_t capacity;
    uint8_t* grown;

    if (size <= encoder->output_capacity) {
        return fc_result_of(0, FC_CALL_NONE);
    }

    capacity = encoder->output_capacity * 2;
    if (capacity < size) {
        capacity = size;
    }
    grown = (uint8_t*)realloc(encoder->output, capacity);
    if (grown == NULL) {
        return fc_result_of(FC_OUT_OF_MEMORY, FC_CALL_NONE);
    }

    encoder->output = grown;
    encoder->output_capacity = capacity;
    return fc_result_of(0, FC_CALL_NONE);
}
