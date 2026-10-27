package dev.fastcast.client.stream

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class ADiagnoseMessageNamesItsRunTest {

    @Test
    fun `the run is read`() {
        assertEquals(1_761_570_000_123L, DiagnoseMessage.parse("""{"type":"diagnose","run":1761570000123}""")?.run)
    }

    @Test
    fun `a notice of another type is not a diagnose message`() {
        assertNull(DiagnoseMessage.parse("""{"type":"scale","run":5}"""))
    }

    @Test
    fun `a message without a run is not a diagnose message`() {
        assertNull(DiagnoseMessage.parse("""{"type":"diagnose"}"""))
    }

    @Test
    fun `garbage is not a diagnose message`() {
        assertNull(DiagnoseMessage.parse("not json"))
    }
}
