package org.tby.fourdk.core.sample.spring.infrastructure.externalsystems;

import org.springframework.stereotype.Component;
import org.tby.fourdk.core.sample.spring.domain.ScreenId;
import org.tby.fourdk.core.sample.spring.domain.ScreenRepository;

@Component
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
