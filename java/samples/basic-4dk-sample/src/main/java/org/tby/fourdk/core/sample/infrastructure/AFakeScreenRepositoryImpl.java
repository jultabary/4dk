package org.tby.fourdk.core.sample.infrastructure;

import org.tby.fourdk.core.sample.domain.ScreenId;
import org.tby.fourdk.core.sample.domain.ScreenRepository;

public class AFakeScreenRepositoryImpl implements ScreenRepository {

    private Boolean hasBeenCalled;

    @Override
    public void refreshTheDisplay(ScreenId screenId, Integer availableSpots) {
        this.hasBeenCalled = true;
    }

    public Boolean getHasBeenCalled() {
        return hasBeenCalled;
    }
}
