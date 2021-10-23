package org.tby.fourdk.core.sample.infrastructure;

import org.tby.fourdk.core.sample.domain.GateId;
import org.tby.fourdk.core.sample.domain.GateRepository;

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
