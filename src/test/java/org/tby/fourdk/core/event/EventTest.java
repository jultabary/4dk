package org.tby.fourdk.core.event;

import org.junit.jupiter.api.Test;
import org.tby.fourdk.core.command.CommandId;
import test.fake.event.AnEvent;

import static java.time.ZoneOffset.UTC;
import static org.assertj.core.api.Assertions.assertThat;
import static test.referential.TestReferential.APR_17_1991_AT_1100AM_CET;

public class EventTest {

    @Test
    public void it_should_return_date_in_utc() {
        // Given
        var dateTime = APR_17_1991_AT_1100AM_CET;

        // When
        var event = new AnEvent(CommandId.create(), APR_17_1991_AT_1100AM_CET);

        // Then
        assertThat(event.utcDateTime).isEqualTo(dateTime.withZoneSameInstant(UTC));
    }
}
