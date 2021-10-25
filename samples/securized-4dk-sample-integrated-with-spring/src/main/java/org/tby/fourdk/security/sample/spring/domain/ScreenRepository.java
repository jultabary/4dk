package org.tby.fourdk.security.sample.spring.domain;

public interface ScreenRepository {
    void refreshTheDisplay(ScreenId screenId, Integer availableSpots);
}
