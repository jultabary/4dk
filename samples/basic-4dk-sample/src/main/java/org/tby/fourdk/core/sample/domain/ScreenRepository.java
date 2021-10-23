package org.tby.fourdk.core.sample.domain;

public interface ScreenRepository {
    void refreshTheDisplay(ScreenId screenId, Integer availableSpots);
}
