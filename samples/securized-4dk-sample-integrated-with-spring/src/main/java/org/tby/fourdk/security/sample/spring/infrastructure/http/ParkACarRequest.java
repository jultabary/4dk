package org.tby.fourdk.security.sample.spring.infrastructure.http;

import java.util.UUID;

public record ParkACarRequest (UUID carId, UUID parkingId){

}
