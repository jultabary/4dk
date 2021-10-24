package org.tby.fourdk.core.sample.spring.domain;

public interface ScreenRepository {
    void refreshTheDisplay(ScreenId screenId, Integer availableSpots);
}
