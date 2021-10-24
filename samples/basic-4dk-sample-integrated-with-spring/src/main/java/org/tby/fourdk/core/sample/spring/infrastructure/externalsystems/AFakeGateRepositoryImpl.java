package org.tby.fourdk.core.sample.spring.infrastructure.externalsystems;

import org.springframework.stereotype.Component;
import org.tby.fourdk.core.sample.spring.domain.GateId;
import org.tby.fourdk.core.sample.spring.domain.GateRepository;

@Component
public class AFakeGateRepositoryImpl implements GateRepository {

    private Boolean hasBeenCalled;

    @Override
    public void openTheGate(GateId gateId) {
        this.hasBeenCalled = true;
    }

    public Boolean getHasBeenCalled() {
        return hasBeenCalled;
    }
}
