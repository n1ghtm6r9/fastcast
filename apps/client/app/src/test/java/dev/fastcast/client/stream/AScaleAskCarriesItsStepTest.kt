package dev.fastcast.client.stream

import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Test

class AScaleAskCarriesItsStepTest {

    @Test
    fun `an ask names the scale type and its step`() {
        val json = JSONObject(ScaleAsk(2).toJson())

        assertEquals("scale", json.getString("type"))
        assertEquals(2, json.getInt("step"))
    }

    @Test
    fun `a step back is sent as a negative step`() {
        assertEquals(-1, JSONObject(ScaleAsk(-1).toJson()).getInt("step"))
    }
}
