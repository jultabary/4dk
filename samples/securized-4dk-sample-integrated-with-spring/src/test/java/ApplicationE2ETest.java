import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.springframework.boot.test.autoconfigure.web.servlet.AutoConfigureMockMvc;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.http.MediaType;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.test.web.servlet.ResultActions;
import org.springframework.test.web.servlet.request.MockMvcRequestBuilders;
import org.springframework.web.servlet.config.annotation.EnableWebMvc;
import org.tby.fourdk.security.Permission;
import org.tby.fourdk.security.Role;
import org.tby.fourdk.security.sample.spring.infrastructure.persistence.ParkingInMemoryRepository;
import org.tby.fourdk.security.sample.spring.infrastructure.persistence.RoleInMemoryRepository;

import javax.annotation.Resource;

import static org.springframework.test.web.servlet.result.MockMvcResultHandlers.print;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.status;
import static org.tby.fourdk.security.sample.spring.domain.Permissions.PARKING_CONTROL;
import static referential.TestReferential.*;

@AutoConfigureMockMvc
@EnableWebMvc
@SpringBootTest(classes = {
        AppConfig.class
})
class ApplicationE2ETest {

    @Resource
    private MockMvc mockMvc;

    @Resource
    private ParkingInMemoryRepository parkingRepository;

    @Resource
    private RoleInMemoryRepository roleInMemoryRepository;

    @BeforeEach
    public void reset() {
        this.parkingRepository.reset();
        this.roleInMemoryRepository.reset();
    }

    @Test
    void it_should_dispatch_ParkACarCommand_and_succeed() throws Exception {
        // Given
        this.roleInMemoryRepository.save(PARKING_CONTROL);
        this.roleInMemoryRepository.save(OPERATOR);
        this.parkingRepository.add(A_PARKING);
        String request = "{" +
                "\"carId\": \"" + A_CAR.id() + "\"," +
                "\"parkingId\": \"" + A_PARKING.getId().id() + "\"" +
                "}";
        var httpRequest = MockMvcRequestBuilders.post("/parkACar")
                .contentType(MediaType.APPLICATION_JSON)
                .accept(MediaType.APPLICATION_JSON)
                .header("authorization", "Bearer " + OPERATOR_TOKEN)
                .content(request);

        // When
        ResultActions performGet = this.mockMvc.perform(httpRequest)
                .andDo(print());

        // Then
        performGet.andExpect(status().isOk());
    }

    @Test
    void it_should_dispatch_ParkACarCommand_and_failed_because_user_is_not_authorized() throws Exception {
        // Given
        this.roleInMemoryRepository.save(PARKING_CONTROL);
        this.roleInMemoryRepository.save(OPERATOR);
        this.roleInMemoryRepository.save(CUSTOMER);
        this.parkingRepository.add(A_PARKING);
        String request = "{" +
                "\"carId\": \"" + A_CAR.id() + "\"," +
                "\"parkingId\": \"" + A_PARKING.getId().id() + "\"" +
                "}";
        var httpRequest = MockMvcRequestBuilders.post("/parkACar")
                .contentType(MediaType.APPLICATION_JSON)
                .accept(MediaType.APPLICATION_JSON)
                .header("authorization", "Bearer " + CUSTOMER_TOKEN)
                .content(request);

        // When
        ResultActions performGet = this.mockMvc.perform(httpRequest)
                .andDo(print());

        // Then
        performGet.andExpect(status().isForbidden());
    }

    @Test
    void it_should_dispatch_TheQuery_and_succeed() throws Exception {
        // Given
        this.parkingRepository.add(A_PARKING);
        A_PARKING.parkACar(A_CAR);
        var httpRequest = MockMvcRequestBuilders.get("/parking/"  + A_PARKING.getId().id() + "/parkedCar")
                .contentType(MediaType.APPLICATION_JSON)
                .accept(MediaType.APPLICATION_JSON);

        // When
        ResultActions performGet = this.mockMvc.perform(httpRequest)
                .andDo(print());

        // Then
        performGet.andExpect(status().isOk());
    }

}