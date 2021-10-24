package org.tby.fourdk.core.sample.spring.usecases.events;

import org.tby.fourdk.core.event.Event;

import java.time.ZonedDateTime;

public abstract class SampleEvent extends Event {

    public final Boolean isTransactionSucceed;

    public SampleEvent(ZonedDateTime dateTime, Boolean isTransactionSucceed) {
        super(dateTime);
        this.isTransactionSucceed = isTransactionSucceed;
    }
}
